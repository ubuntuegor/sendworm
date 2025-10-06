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
  import { basename } from "@tauri-apps/api/path"
  import FileInfoBlock from "./FileInfoBlock.svelte"
  import { getReceiveFolder, setReceiveFolder } from "$lib/settings"
  import { open } from "@tauri-apps/plugin-dialog"

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
  let folder: string | null = $state(null)
  let folderName: string | null = $state(null)
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

    getReceiveFolder().then((path) => {
      folder = path
    })

    receiveFile(code)

    return () => {
      cancelReceive()
    }
  })

  $effect(() => {
    if (folder === null) return
    basename(folder)
      .then((baseName) => {
        folderName = baseName
      })
      .catch((_) => {
        folderName = folder
      })
  })

  async function receiveFile(code: string) {
    let onEvent
    if (MOCK) {
      onEvent = new MockChannel<ReceiveEvent>()
    } else {
      onEvent = new Channel<ReceiveEvent>()
    }
    onEvent.onmessage = (message) => {
      switch (message.event) {
        case "fileInfo":
          centerState = { state: "confirmation" }
          fileInfo = message.data
          onFileInfo(message.data)
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
      invoke("confirm_receive", { folder })
    }
  }

  function cancelReceive() {
    if (MOCK) {
      mockCancelReceive()
    } else {
      invoke("cancel_receive")
    }
  }

  async function selectFolder() {
    const path = await open({
      directory: true,
    })

    if (path) {
      folder = path
      setReceiveFolder(path)
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
          title={folder}
          aria-label="Click to choose folder. Current folder is {folderName}"
          onclick={selectFolder}
        >
          <span class="text">{folderName}</span>
          <svg
            xmlns="http://www.w3.org/2000/svg"
            width="16"
            height="16"
            viewBox="0 0 24 24"
          >
            <path
              fill="currentColor"
              d="M4 20q-.825 0-1.412-.587T2 18V6q0-.825.588-1.412T4 4h6l2 2h8q.825 0 1.413.588T22 8v10q0 .825-.587 1.413T20 20zm0-2h16V8h-8.825l-2-2H4zm0 0V6z"
            />
          </svg>
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
    place-content: center;
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

      svg {
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
