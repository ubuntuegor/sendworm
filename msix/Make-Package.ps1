Copy-Item ..\src-tauri\target\release\sendworm.exe .\package\
Copy-Item ..\src-tauri\target\release\sendworm.pdb .\package\

$Manifest = Get-Content ..\src-tauri\tauri.conf.json | ConvertFrom-Json
$Version = $Manifest.version + ".0"

If ($env:PROCESSOR_ARCHITECTURE -Eq "AMD64") {
    $Arch = "x64"
}
elseif ($env:PROCESSOR_ARCHITECTURE -Eq "ARM64") {
    $Arch = "arm64"
}
else {
    throw "Unknown architecture $env:PROCESSOR_ARCHITECTURE"
}

$OutputFilename = "Sendworm-$Version-$Arch.msix"

(Get-Content appxmanifest.template.xml).Replace('$VERSION$', $Version).Replace('$ARCH$', $Arch) | Out-File .\package\appxmanifest.xml

$installationPath = vswhere.exe -latest -property installationPath
if ($installationPath -and (test-path "$installationPath\Common7\Tools\vsdevcmd.bat")) {
    & "${env:COMSPEC}" /s /c "`"$installationPath\Common7\Tools\vsdevcmd.bat`" -no_logo && set" | foreach-object {
        $name, $value = $_ -split '=', 2
        set-content env:\"$name" $value
    }
}

Set-Location package

makepri.exe createconfig /cf priconfig.xml /dq en-US
makepri.exe new /pr . /cf .\priconfig.xml
MakeAppx.exe pack /o /d . /p "..\$OutputFilename"

Set-Location ..

SignTool.exe sign /fd SHA256 /a /f .\TemporaryKey.pfx $OutputFilename
