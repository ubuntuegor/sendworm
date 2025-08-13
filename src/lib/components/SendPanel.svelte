<script lang="ts">
  import {
    MockChannel,
    mockSendFile,
    mockConfirmSend,
    mockCancelSend,
  } from "$lib/mocks/send"
  import type { TransitInfo, SendEvent } from "$lib/types/send"
  import { Channel, invoke } from "@tauri-apps/api/core"
  import { fade, scale } from "svelte/transition"
  import QRCode from "qrcode"
  import { quadOut } from "svelte/easing"

  // TODO:
  // - finished and error screens
  // - qr code is not working (should it be a url?)
  // - long filename tooltip
  // - copy code button
  // - get real size
  // - format sizes
  // - format transitInfo
  // - redesign probably

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
      }
    | {
        state: "progress"
        progress: [number, number]
      }
    | {
        state: "finished"
      }
    | {
        state: "error"
        message: string
      }

  let fileName = $derived(computeFileName(filePath))
  let fileSize = "562 kB"
  let code: string | null = $state(null)
  let qrcode = $derived(
    code
      ? QRCode.toString(code, {
          type: "svg",
          errorCorrectionLevel: "M",
          margin: 3,
        })
      : null
  )
  let transitInfo: TransitInfo | null = $state(null)
  let centerState: CenterState = $state({ state: "loading" })

  let ongoing = $derived.by(() => {
    return centerState.state !== "finished" && centerState.state !== "error"
  })

  async function computeFileName(filePath: string): Promise<string | null> {
    return await invoke("compute_file_name", { filePath })
  }

  $effect(() => {
    code = null
    centerState = { state: "loading" }

    sendFile(filePath)

    return () => {
      cancelSend()
    }
  })

  function clearSelection() {
    window.getSelection()?.empty()
  }

  function getPercentage(progress: [number, number]): number {
    return Math.floor((progress[0] / progress[1]) * 100)
  }

  async function sendFile(path: string) {
    let onEvent
    if (MOCK) {
      onEvent = new MockChannel<SendEvent>()
    } else {
      onEvent = new Channel<SendEvent>()
    }
    onEvent.onmessage = (message) => {
      switch (message.event) {
        case "code":
          code = message.data.code
          centerState = { state: "code" }
          break
        case "connected":
          centerState = { state: "progress", progress: [0, 0] }
          confirmSend()
          break
        case "transitInfo":
          transitInfo = message.data
          break
        case "progress":
          centerState = {
            state: "progress",
            progress: [message.data.sent, message.data.total],
          }
          break
        case "finished":
          centerState = { state: "finished" }
          break
        case "error":
          centerState = { state: "error", message: message.data.message }
          break
      }
    }

    if (MOCK) {
      mockSendFile(path, onEvent as MockChannel<SendEvent>)
    } else {
      invoke("send_file", { filePath: path, onEvent })
    }
  }

  function confirmSend() {
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
  <div class="top">
    <div class="file-info">
      <svg
        xmlns="http://www.w3.org/2000/svg"
        class="file-info-icon"
        width="24"
        height="24"
        viewBox="0 0 24 24"
        ><path
          fill="currentColor"
          d="M6 22q-.825 0-1.412-.587T4 20V4q0-.825.588-1.412T6 2h8l6 6v12q0 .825-.587 1.413T18 22zm7-13V4H6v16h12V9zM6 4v5zv16z"
        /></svg
      >

      <p class="sub-title">Sending a file ({fileSize})</p>
      {#await fileName then name}
        <div class="name-and-more">
          <b>
            {name}
          </b>
          <!-- <span class="suffix">and 2 more</span> -->
        </div>
      {/await}
    </div>
    <div class="code">
      <input type="text" value={code} placeholder="Loading..." disabled />
      <button class="code-copy" aria-label="Copy code">
        <svg
          xmlns="http://www.w3.org/2000/svg"
          width="16"
          height="16"
          viewBox="0 0 24 24"
          ><path
            fill="currentColor"
            d="M9 18q-.825 0-1.412-.587T7 16V4q0-.825.588-1.412T9 2h9q.825 0 1.413.588T20 4v12q0 .825-.587 1.413T18 18zm0-2h9V4H9zm-4 6q-.825 0-1.412-.587T3 20V6h2v14h11v2zm4-6V4z"
          /></svg
        >
      </button>
    </div>
  </div>
  <div class="center">
    {#if centerState.state === "loading"}
      <div class="center-variant">
        <div class="loader" transition:fade={{ duration: 200 }}></div>
      </div>
    {:else if centerState.state === "code"}
      {#await qrcode then svgcode}
        <div
          class="center-variant qrcode-block"
          transition:scale={{ duration: 200, easing: quadOut }}
        >
          <div class="qrcode">
            {@html svgcode}
          </div>
          <p>Share this code</p>
        </div>
      {/await}
    {:else if centerState.state === "progress"}
      <div
        class="center-variant progress-block"
        transition:scale={{ duration: 200, easing: quadOut }}
      >
        <div
          class="progress-circle"
          role="progressbar"
          aria-valuenow={getPercentage(centerState.progress)}
          style:--percentage={getPercentage(centerState.progress) + "%"}
        >
          {getPercentage(centerState.progress)}%
        </div>
        <p>Sent {centerState.progress[0]} out of {centerState.progress[1]}</p>
        {#if transitInfo}
          <p class="sub-text">
            Connected to {transitInfo.address} via {transitInfo.connectionType}
          </p>
        {/if}
      </div>
    {/if}
  </div>
  <div class="bottom">
    <button class="cancel-button" class:red={ongoing} onclick={cancelAndGoBack}>
      {#if ongoing}
        Cancel
      {:else}
        Return
      {/if}
    </button>
  </div>
</div>

<style>
  .container {
    max-width: 320px;
    height: 100%;
    margin: auto;
    display: grid;
    grid-template-rows: 1fr 200px 1fr;
    justify-items: center;
    color: #ccdaff;
    gap: 12px;
  }

  .top {
    justify-self: stretch;
    align-self: end;
  }

  .file-info {
    display: grid;
    align-items: center;
    grid-template-columns: auto 1fr;
    column-gap: 12px;

    .file-info-icon {
      grid-column: 1 / 2;
      grid-row: 1 / 3;
    }

    .sub-title {
      font-size: 12px;
      color: rgba(204, 218, 255, 0.5);
    }

    .name-and-more {
      display: flex;
      overflow: hidden;
      white-space: nowrap;
      font-size: 14px;

      b {
        overflow: hidden;
        text-overflow: ellipsis;
      }
    }
  }

  .code {
    margin-top: 12px;
    position: relative;
    background-color: #111f45;
    border: solid 1px rgba(204, 218, 255, 0.25);
    border-radius: 12px;

    transition:
      background-color ease-out 0.1s,
      border ease-out 0.1s;

    input {
      all: unset;
      cursor: text;
      width: 100%;
      text-align: center;
      font-size: 14px;
      font-weight: bold;
      padding-top: 6px;
      padding-bottom: 8px;

      &::selection {
        background-color: #535967;
      }

      &::placeholder {
        color: #7b87a4;
      }
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

  .center {
    position: relative;
    place-self: stretch;
  }

  @keyframes rotation {
    0% {
      transform: rotate(0deg);
    }
    100% {
      transform: rotate(360deg);
    }
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

  .loader {
    width: 40px;
    height: 40px;
    border-top: solid 2px #ccdaff;
    border-right: solid 2px transparent;
    border-radius: 50%;
    animation: rotation 1s linear infinite;
  }

  .qrcode-block {
    gap: 8px;

    .qrcode {
      width: 160px;
      height: 160px;
      overflow: hidden;
      background-color: white;
      border: solid 1px rgba(0, 0, 0, 0.25);
      border-radius: 12px;
      box-shadow: 0px 2px 4px rgba(0, 0, 0, 0.25);
    }

    p {
      font-size: 12px;
    }
  }

  .progress-block {
    .progress-circle {
      margin-bottom: 10px;
      width: 60px;
      height: 60px;
      border-radius: 50%;
      background-image: radial-gradient(#1c2e5f 60%, transparent 62%),
        conic-gradient(#ccdaff var(--percentage), transparent var(--percentage)),
        linear-gradient(rgba(204, 218, 255, 0.25));

      line-height: 60px;
      text-align: center;
      font-size: 16px;
      font-weight: 500;
    }

    p {
      font-size: 14px;
    }

    .sub-text {
      margin-top: 1px;
      font-size: 12px;
      color: rgba(204, 218, 255, 0.5);
    }
  }

  .cancel-button {
    width: 160px;
    text-align: center;
    font-size: 14px;
    background-color: #111f45;
    border: solid 1px rgba(204, 218, 255, 0.25);
    border-radius: 8px;
    padding-top: 5px;
    padding-bottom: 6px;
    cursor: pointer;
    box-shadow: 0 1px 4px rgba(0, 0, 0, 0.25);

    transition:
      background-color ease-out 0.1s,
      border ease-out 0.1s;

    &.red {
      color: #ff9ca8;
      border: solid 1px rgba(255, 156, 168, 0.3);
    }

    &:hover:not(:active) {
      border: solid 1px rgba(204, 218, 255, 0.5);
      background-color: #13234f;

      &.red {
        border: solid 1px rgba(255, 156, 168, 0.5);
      }
    }

    &:active {
      background-color: #0e1a38;
    }
  }
</style>
