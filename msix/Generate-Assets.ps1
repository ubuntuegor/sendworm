#
# Generate-TerminalAssets.ps1
#
# Typical usage:
#   .\Generate-Assets.ps1 -Path .\Terminal.png -Destination .\images
#

Param(
  [Parameter(Mandatory = $true, ValueFromPipeline = $true)]
  [string]$Path,
  [string]$Destination,
  [int[]]$Altforms = (16, 20, 24, 30, 32, 36, 40, 48, 60, 64, 72, 80, 96, 256),
  [int[]]$Win32IconSizes = (16, 20, 24, 32, 48, 64, 256),
  [float[]]$Scales = (1.0, 1.25, 1.5, 2.0, 4.0)
)

$assetTypes = @(
  [pscustomobject]@{Name = "LargeTile"; W = 310; H = 310; IconSize = 96 }
  [pscustomobject]@{Name = "LockScreenLogo"; W = 24; H = 24; IconSize = 24 }
  [pscustomobject]@{Name = "SmallTile"; W = 71; H = 71; IconSize = 36 }
  [pscustomobject]@{Name = "SplashScreen"; W = 620; H = 300; IconSize = 96 }
  [pscustomobject]@{Name = "Square44x44Logo"; W = 44; H = 44; IconSize = 32 }
  [pscustomobject]@{Name = "Square150x150Logo"; W = 150; H = 150; IconSize = 48 }
  [pscustomobject]@{Name = "StoreLogo"; W = 50; H = 50; IconSize = 36 }
  [pscustomobject]@{Name = "Wide310x150Logo"; W = 310; H = 150; IconSize = 48 }
)

function CeilToEven ([int]$i) { if ($i % 2 -eq 0) { [int]($i) } else { [int]($i + 1) } }

$inflatedAssetSizes = $assetTypes | ForEach-Object {
  $as = $_;
  $scales | ForEach-Object {
    [pscustomobject]@{
      Name     = $as.Name + ".scale-$($_*100)"
      W        = [math]::Round($as.W * $_, [System.MidpointRounding]::ToPositiveInfinity)
      H        = [math]::Round($as.H * $_, [System.MidpointRounding]::ToPositiveInfinity)
      IconSize = CeilToEven ($as.IconSize * $_)
    }
  }
}

$allAssetSizes = $inflatedAssetSizes + ($Altforms | ForEach-Object {
    [pscustomobject]@{
      Name     = "Square44x44Logo.targetsize-${_}"
      W        = [int]$_
      H        = [int]$_
      IconSize = [int]$_
    }
    [pscustomobject]@{
      Name     = "Square44x44Logo.targetsize-${_}_altform-unplated"
      W        = [int]$_
      H        = [int]$_
      IconSize = [int]$_
    }
  })

$allSizes = $allAssetSizes.IconSize | Group-Object | Select-Object -Expand Name

$TranslatedSourcePath = & wsl wslpath -u ((Get-Item $Path -ErrorAction:Stop).FullName -Replace "\\", "/")
& wsl which convert | Out-Null
If ($LASTEXITCODE -Ne 0) { throw "imagemagick is not installed in WSL" }

If (-Not [string]::IsNullOrEmpty($Destination)) {
  New-Item -Type Directory $Destination -EA:Ignore
  $TranslatedOutDir = & wsl wslpath -u ((Get-Item $Destination -EA:Stop).FullName -Replace "\\", "/")
}
Else {
  $TranslatedOutDir = "."
}

$intermediates = [System.Collections.Concurrent.ConcurrentBag[PSCustomObject]]::new()
$intermediateFiles = [System.Collections.Concurrent.ConcurrentBag[string]]::new()

# Generate the base icons
$allSizes | ForEach-Object -Parallel {
  $sz = $_;

  $destinationNt = $using:Destination
  $destinationWsl = $using:TranslatedOutDir
  $sourceWsl = $using:TranslatedSourcePath

  $intermediateNt = "$destinationNt\_intermediate.$($sz).png"
  $intermediateWsl = "$destinationWsl/_intermediate.$($sz).png"

  wsl convert $sourceWsl -resize $sz "$intermediateWsl"

  ($using:intermediateFiles).Add($intermediateNt)
  ($using:intermediates).Add([PSCustomObject]@{
      Size    = $sz
      PathWSL = $intermediateWsl
    })
}

# Once the base icons are done, splat them into the middles of larger canvases.
$allAssetSizes | ForEach-Object -Parallel {
  $asset = $_
  If ($asset.W -Eq $asset.H -And $asset.IconSize -eq $asset.W) {
    Write-Host "Copying base icon for size=$($asset.IconSize) to $($asset.Name)"
    Copy-Item "${using:Destination}\_intermediate.$($asset.IconSize).png" "${using:Destination}\$($asset.Name).png" -Force
  }
  Else {
    wsl convert "$($using:TranslatedOutDir)/_intermediate.$($asset.IconSize).png" -gravity center -background transparent -extent "$($asset.W)x$($asset.H)" "$($using:TranslatedOutDir)/$($asset.Name).png"
  }
}

$intermediateFiles | ForEach-Object {
  Write-Host "Cleaning up intermediate file $_"
  Remove-Item $_
}

