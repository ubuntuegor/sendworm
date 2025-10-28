<script lang="ts">
  import { Channel, invoke } from "@tauri-apps/api/core"
  import { fade, scale } from "svelte/transition"
  import { quadOut } from "svelte/easing"
  import Spinner from "./Spinner.svelte"
  import ProgressBlock from "./ProgressBlock.svelte"
  import type { TransitInfo } from "$lib/types/common"
  import type { FileInfo, ReceiveEvent } from "$lib/types/receive"
  import { MockChannel } from "$lib/mocks/common"
  import {
    mockCancelReceive,
    mockConfirmReceive,
    mockReceiveFile,
  } from "$lib/mocks/receive"
  import { basename, dirname, join } from "@tauri-apps/api/path"
  import FileInfoBlock from "./FileInfoBlock.svelte"
  import {
    getAskBeforeReceive,
    getReceiveFolder,
    setReceiveFolder,
  } from "$lib/settings"
  import { save } from "@tauri-apps/plugin-dialog"
  import FolderIcon from "$lib/icons/FolderIcon.svelte"
  import { scaleVertically } from "$lib/transitions"
  import { platform } from "@tauri-apps/plugin-os"
  import { computeNonexistingPath, getUIPath } from "$lib/utils/files"

  // Mock ongoing transfer to debug UI
  const MOCK = false

  interface Props {
    code: string
    goBack: () => void
    onFileInfo: (info: FileInfo) => void
  }

  const { code, goBack, onFileInfo }: Props = $props()

  type CenterState =
    | {
        state: "loading"
      }
    | {
        state: "confirmation"
      }
    | {
        state: "progress"
        progress: [number, number] | null
        transitInfo: TransitInfo | null
        finished: boolean
      }

  let centerState: CenterState = $state({ state: "loading" })
  let fileInfo: FileInfo | null = $state(null)
  let resultFilePath: string | null = $state(null)
  let uiFilePath: string | null = $state(null)
  let uiFolderName: string | null = $state(null)
  let error: string | null = $state(null)

  let transferEnded = $derived.by(() => {
    return (
      error !== null ||
      (centerState.state === "progress" && centerState.finished)
    )
  })

  $effect(() => {
    error = null
    fileInfo = null
    centerState = { state: "loading" }

    receiveFile(code)

    return () => {
      cancelReceive()
    }
  })

  $effect(() => {
    if (resultFilePath === null) return

    let folderName = "Unknown directory"
    dirname(resultFilePath)
      .then((dir) => {
        folderName = dir
        return getUIPath(dir)
      })
      .then((uiDir) => {
        folderName = uiDir
        return basename(uiDir)
      })
      .then((result) => {
        uiFolderName = result
      })
      .catch((_) => {
        uiFolderName = folderName
      })

    let filePath = resultFilePath
    dirname(resultFilePath)
      .then((dir) => {
        return Promise.all([getUIPath(dir), basename(filePath)])
      })
      .then(([uiDir, fileName]) => {
        return join(uiDir, fileName)
      })
      .then((result) => {
        uiFilePath = result
      })
      .catch((_) => {
        uiFilePath = filePath
      })
  })

  async function receiveFile(code: string) {
    let onEvent
    if (MOCK) {
      onEvent = new MockChannel<ReceiveEvent>()
    } else {
      onEvent = new Channel<ReceiveEvent>()
    }
    onEvent.onmessage = async (message) => {
      switch (message.event) {
        case "fileInfo":
          fileInfo = message.data
          onFileInfo(message.data)

          resultFilePath = await computeNonexistingPath(
            await getReceiveFolder(),
            fileInfo.fileName
          )

          if (await getAskBeforeReceive()) {
            askBeforeReceive()
          } else {
            confirmReceive()
          }

          break
        case "transitInfo":
          if (centerState.state === "progress") {
            centerState.transitInfo = message.data
          }
          break
        case "progress":
          if (centerState.state === "progress") {
            centerState.progress = [message.data.sent, message.data.total]
          }
          break
        case "finished":
          if (centerState.state === "progress") {
            centerState.finished = true
          }
          break
        case "error":
          error = message.data.message
          break
      }
    }

    if (MOCK) {
      mockReceiveFile(onEvent as MockChannel<ReceiveEvent>)
    } else {
      invoke("receive_file", { code, onEvent })
    }
  }

  async function askBeforeReceive() {
    if (platform() === "linux") {
      const path = await selectReceivePath()

      if (path !== null) {
        confirmReceive()
      } else {
        centerState = { state: "confirmation" }
      }
    } else {
      centerState = { state: "confirmation" }
    }
  }

  function confirmReceive() {
    centerState = {
      state: "progress",
      progress: null,
      transitInfo: null,
      finished: false,
    }

    if (MOCK) {
      mockConfirmReceive()
    } else {
      invoke("confirm_receive", { filePath: resultFilePath })
    }
  }

  function cancelReceive() {
    if (MOCK) {
      mockCancelReceive()
    } else {
      invoke("cancel_receive")
    }
  }

  async function selectReceivePath() {
    const path = await save({
      defaultPath: resultFilePath!,
    })

    if (path) {
      resultFilePath = path

      try {
        const folder = await dirname(path)
        setReceiveFolder(folder)
      } catch (e) {
        console.error(e)
      }
    }

    return path
  }

  async function openReceivedFile() {
    if (centerState.state === "progress") {
      invoke("open_file", { filePath: resultFilePath })
    }
  }

  async function revealReceivedFile() {
    if (centerState.state === "progress") {
      invoke("reveal_file", { filePath: resultFilePath })
    }
  }

  function cancelAndGoBack() {
    cancelReceive()
    goBack()
  }
</script>

<div class="container">
  <div class="center">
    {#if error !== null && centerState.state !== "progress"}
      <div
        class="center-variant error-block"
        transition:scale={{ duration: 200, easing: quadOut }}
      >
        <p>{error}</p>
      </div>
    {:else if centerState.state === "loading"}
      <div class="center-variant" transition:fade={{ duration: 200 }}>
        <Spinner />
      </div>
    {:else if centerState.state === "confirmation"}
      <div
        class="center-variant confirmation-block"
        transition:scale={{ duration: 200, easing: quadOut }}
      >
        <p>Save to this folder</p>
        <button
          class="folder-chooser"
          title={uiFilePath}
          aria-label="Click to choose file path. Current folder for the file is {uiFolderName}"
          onclick={selectReceivePath}
        >
          <span class="text">{uiFolderName}</span>
          <span class="icon">
            <FolderIcon size={16} />
          </span>
        </button>
        <button class="continue-button" onclick={confirmReceive}>
          Continue
        </button>
      </div>
    {:else if centerState.state === "progress"}
      <div
        class="center-variant"
        transition:scale={{ duration: 200, easing: quadOut }}
      >
        <ProgressBlock
          mode="receive"
          {...centerState}
          {error}
          --percentage-inactive="rgba(255, 224, 181, 0.25)"
          --filling-color="#131210"
        />
      </div>
    {/if}
  </div>
  {#if fileInfo}
    <div class="top" transition:fade={{ duration: 200 }}>
      <FileInfoBlock
        mode="receive"
        isDir={false}
        {...fileInfo}
        fileNameTooltip={fileInfo.fileName}
      />
      {#if centerState.state === "progress" && centerState.finished}
        <div
          class="file-open-buttons"
          transition:scaleVertically={{ duration: 100, easing: quadOut }}
        >
          <button
            class="open-button"
            onclick={openReceivedFile}
            title={`Open ${uiFilePath}`}>Open</button
          >
          <button
            class="reveal-button"
            onclick={revealReceivedFile}
            title={`Reveal ${uiFilePath} in file explorer`}
          >
            Reveal
          </button>
        </div>
      {/if}
    </div>
  {/if}
  <div class="bottom">
    <button
      class="cancel-button"
      class:red={!transferEnded}
      onclick={cancelAndGoBack}
    >
      {#if transferEnded}
        Return
      {:else}
        Cancel
      {/if}
    </button>
  </div>
</div>

<style>
  .container {
    height: 100%;
    display: grid;
    place-content: center;
    color: #ffe0b5;
  }

  .center {
    padding: 6px;
  }

  .top {
    position: absolute;
    top: 40px;
    left: 0px;
    right: 0px;
    display: grid;
    justify-items: center;
    grid-template-rows: auto auto;
  }

  .file-open-buttons {
    margin-top: 16px;
    width: 340px;
    display: flex;
    gap: 6px;

    transform-origin: top;

    button {
      flex: 1 1 100%;
    }

    .open-button {
      cursor: pointer;
      text-align: center;
      font-size: 14px;
      background-color: #ffe0b5;
      color: #131210;
      border-radius: 8px;
      padding: 8px;
      padding-top: 5px;
      padding-bottom: 6px;
      overflow: hidden;
      text-overflow: ellipsis;

      transition: background-color ease-out 0.1s;

      &:hover {
        background-color: #ffe6c3;
      }

      &:active {
        background-color: #e3c396;
      }
    }

    .reveal-button {
      cursor: pointer;
      text-align: center;
      font-size: 14px;
      background-color: #131210;
      border: solid 1px rgba(255, 224, 181, 0.25);
      border-radius: 8px;
      padding: 8px;
      padding-top: 5px;
      padding-bottom: 6px;
      overflow: hidden;
      text-overflow: ellipsis;

      transition:
        background-color ease-out 0.1s,
        border ease-out 0.1s;

      &:hover:not(:active) {
        border: solid 1px rgba(255, 224, 181, 0.5);
        background-color: #1e1c19;
      }

      &:active {
        background-color: #0a0909;
      }
    }
  }

  .bottom {
    position: absolute;
    bottom: 60px;
    left: 0px;
    right: 0px;
    display: grid;
    place-content: center;
  }

  .center-variant {
    position: absolute;
    top: 0;
    right: 0;
    bottom: 0;
    left: 0;
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
  }

  .error-block {
    font-size: 12px;
    color: #ff9ca8;
    text-align: center;
  }

  .confirmation-block {
    p {
      margin-bottom: 4px;
      font-size: 12px;
      opacity: 0.5;
    }

    .folder-chooser {
      margin-bottom: 12px;
      cursor: pointer;
      display: flex;
      align-items: center;
      width: 220px;
      height: 32px;
      padding: 0px 11px;
      border: solid 1px rgba(255, 224, 181, 0.25);
      border-radius: 12px;
      box-sizing: border-box;

      background-color: #1d1b18;
      transition: border ease-out 0.1s;

      text-align: left;
      font-size: 14px;

      &:hover:not(:active) {
        border: solid 1px rgba(255, 224, 181, 0.75);
      }

      .text {
        flex: 1 1 auto;
        padding-bottom: 2px;

        text-overflow: ellipsis;
        overflow: hidden;
      }

      .icon {
        flex: 0 0 auto;
      }
    }

    .continue-button {
      cursor: pointer;
      min-width: 160px;
      text-align: center;
      font-size: 14px;
      background-color: #ffe0b5;
      color: #131210;
      border-radius: 8px;
      padding: 8px;
      padding-top: 5px;
      padding-bottom: 6px;

      transition: background-color ease-out 0.1s;

      &:hover {
        background-color: #ffe6c3;
      }

      &:active {
        background-color: #e3c396;
      }
    }
  }

  .cancel-button {
    min-width: 160px;
    text-align: center;
    font-size: 14px;
    background-color: #131210;
    border: solid 1px rgba(255, 224, 181, 0.25);
    border-radius: 8px;
    padding: 8px;
    padding-top: 5px;
    padding-bottom: 6px;
    cursor: pointer;
    box-shadow: 0 1px 4px rgba(0, 0, 0, 0.25);

    transition:
      background-color ease-out 0.1s,
      border ease-out 0.1s;

    &.red {
      color: #ff9ca8;
    }

    &:hover:not(:active) {
      border: solid 1px rgba(255, 224, 181, 0.5);
      background-color: #1e1c19;
    }

    &:active {
      background-color: #0a0909;
    }
  }
</style>
