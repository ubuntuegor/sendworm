<script lang="ts">
  import {
    mockSendFile,
    mockConfirmSend,
    mockCancelSend,
  } from "$lib/mocks/send"
  import type { TransitInfo } from "$lib/types/common"
  import type { SendEvent } from "$lib/types/send"
  import { Channel, invoke } from "@tauri-apps/api/core"
  import { writeText } from "@tauri-apps/plugin-clipboard-manager"
  import { fade, scale } from "svelte/transition"
  import QRCode from "qrcode"
  import { quadOut } from "svelte/easing"
  import { getFileSize, getUIPath, isFolder } from "$lib/utils/files"
  import Spinner from "./Spinner.svelte"
  import ProgressBlock from "./ProgressBlock.svelte"
  import { MockChannel } from "$lib/mocks/common"
  import FileInfoBlock from "./FileInfoBlock.svelte"
  import ClipboardIcon from "$lib/icons/ClipboardIcon.svelte"
  import { getAskBeforeSend } from "$lib/settings"
  import { basename } from "@tauri-apps/api/path"

  // Mock ongoing transfer to debug UI
  const MOCK = false

  interface Props {
    filePath: string
    goBack: () => void
  }

  const { filePath, goBack }: Props = $props()

  type CenterState =
    | {
        state: "loading"
      }
    | {
        state: "code"
        code: string
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
  let uiFilePath: string | null = $state(null)
  let error: string | null = $state(null)

  $effect(() => {
    getUIPath(filePath).then((result) => {
      uiFilePath = result
    })
  })

  let transferEnded = $derived.by(() => {
    return (
      error !== null ||
      (centerState.state === "progress" && centerState.finished)
    )
  })

  function getQrCode(code: string): Promise<string> {
    return QRCode.toString(`wormhole-transfer:${code}`, {
      type: "svg",
      errorCorrectionLevel: "M",
      margin: 3,
    })
  }

  function copyCodeToClipboard() {
    if (centerState.state === "code") {
      writeText(centerState.code)
    }
  }

  $effect(() => {
    error = null
    centerState = { state: "loading" }

    sendFileOrFolder(filePath)

    return () => {
      cancelSend()
    }
  })

  function clearSelection() {
    window.getSelection()?.empty()
  }

  async function sendFileOrFolder(path: string) {
    let onEvent
    if (MOCK) {
      onEvent = new MockChannel<SendEvent>()
    } else {
      onEvent = new Channel<SendEvent>()
    }
    onEvent.onmessage = async (message) => {
      switch (message.event) {
        case "code":
          centerState = { state: "code", code: message.data.code }
          break
        case "connected":
          if (await getAskBeforeSend()) {
            centerState = { state: "confirmation" }
          } else {
            confirmSend()
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
      mockSendFile(onEvent as MockChannel<SendEvent>)
    } else {
      invoke("send_file_or_folder", { filePath: path, onEvent })
    }
  }

  function confirmSend() {
    centerState = {
      state: "progress",
      progress: null,
      transitInfo: null,
      finished: false,
    }

    if (MOCK) {
      mockConfirmSend()
    } else {
      invoke("confirm_send")
    }
  }

  function cancelSend() {
    if (MOCK) {
      mockCancelSend()
    } else {
      invoke("cancel_send")
    }
  }

  function cancelAndGoBack() {
    cancelSend()
    goBack()
  }
</script>

<svelte:body onmousedown={clearSelection} />

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
    {:else if centerState.state === "code"}
      <div
        class="center-variant code-block"
        transition:scale={{ duration: 200, easing: quadOut }}
      >
        <p class="sub-title">Share this code</p>
        <div class="code-box">
          {centerState.code}
          <button
            class="code-copy"
            title="Copy to clipboard"
            aria-label="Copy code to clipboard"
            onclick={copyCodeToClipboard}
          >
            <ClipboardIcon size={16} />
          </button>
        </div>
        <div class="qrcode">
          {#await getQrCode(centerState.code) then qrCode}
            {@html qrCode}
          {/await}
        </div>
      </div>
    {:else if centerState.state === "confirmation"}
      <div
        class="center-variant confirmation-block"
        transition:scale={{ duration: 200, easing: quadOut }}
      >
        <p>Recipient has entered the code</p>
        <button onclick={confirmSend}>Confirm sending</button>
      </div>
    {:else if centerState.state === "progress"}
      <div
        class="center-variant"
        transition:scale={{ duration: 200, easing: quadOut }}
      >
        <ProgressBlock
          mode="send"
          {...centerState}
          {error}
          --percentage-inactive="rgba(204, 218, 255, 0.25)"
          --filling-color="#1c2e5f"
        />
      </div>
    {/if}
  </div>
  <div class="top">
    {#await Promise.all( [basename(filePath), getFileSize(filePath), isFolder(filePath)] ) then [fileName, fileSize, isDir]}
      <FileInfoBlock
        mode="send"
        {isDir}
        {fileName}
        fileNameTooltip={uiFilePath}
        {fileSize}
      />{/await}
  </div>
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
    color: #ccdaff;
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

  .code-block {
    .sub-title {
      margin-bottom: 4px;
      font-size: 12px;
      opacity: 0.5;
    }

    .code-box {
      position: relative;
      user-select: text;
      -webkit-user-select: text;
      box-sizing: border-box;
      min-width: 220px;
      padding: 32px;
      padding-top: 6px;
      padding-bottom: 8px;
      border: solid 1px rgba(204, 218, 255, 0.25);
      border-radius: 12px;
      background-color: #111f45;

      transition: border ease-out 0.1s;

      text-align: center;
      font-size: 14px;
      font-weight: bold;

      &::selection {
        background-color: #535967;
      }

      .code-copy {
        color: rgba(204, 218, 255, 0.5);
        position: absolute;
        width: 32px;
        right: 0px;
        top: 0px;
        bottom: 0px;
        cursor: pointer;
        display: grid;
        place-items: center;

        transition: color ease-out 0.1s;

        &:hover:not(:active) {
          color: #ccdaff;
        }
      }

      &:has(.code-copy:hover:not(:active)) {
        border: solid 1px rgba(204, 218, 255, 0.5);
      }
    }

    .qrcode {
      margin-top: 12px;
      width: 160px;
      height: 160px;
      overflow: hidden;
      background-color: white;
      border: solid 1px rgba(0, 0, 0, 0.25);
      border-radius: 12px;
      box-shadow: 0px 2px 4px rgba(0, 0, 0, 0.25);
    }
  }

  .confirmation-block {
    gap: 12px;

    p {
      font-size: 12px;
      opacity: 0.5;
    }

    button {
      cursor: pointer;
      min-width: 160px;
      text-align: center;
      font-size: 14px;
      background-color: #ccdaff;
      color: #202943;
      border-radius: 8px;
      padding: 8px;
      padding-top: 5px;
      padding-bottom: 6px;

      transition: background-color ease-out 0.1s;

      &:hover {
        background-color: #e6ecff;
      }

      &:active {
        background-color: #9eabcf;
      }
    }
  }

  .cancel-button {
    min-width: 160px;
    text-align: center;
    font-size: 14px;
    background-color: #1c2e5f;
    border: solid 1px rgba(204, 218, 255, 0.25);
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
      border: solid 1px rgba(204, 218, 255, 0.5);
      background-color: #24366b;
    }

    &:active {
      background-color: #162757;
    }
  }
</style>
