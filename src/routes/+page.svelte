<script lang="ts">
  import CodeInput from "$lib/components/CodeInput.svelte"
  import ReceivePanel from "$lib/components/ReceivePanel.svelte"
  import SendPanel from "$lib/components/SendPanel.svelte"
  import FolderIcon from "$lib/icons/FolderIcon.svelte"
  import ReceiveIcon from "$lib/icons/ReceiveIcon.svelte"
  import SendIcon from "$lib/icons/SendIcon.svelte"
  import { initializeMenu } from "$lib/menu"
  import { shrink } from "$lib/transitions"
  import { invoke } from "@tauri-apps/api/core"
  import { basename } from "@tauri-apps/api/path"
  import { getCurrentWindow } from "@tauri-apps/api/window"
  import { open } from "@tauri-apps/plugin-dialog"
  import { fade } from "svelte/transition"

  type State =
    | {
        state: "idle"
      }
    | {
        state: "sending"
        filePath: string
        fileName?: string
      }
    | {
        state: "receiving"
        code: string
        fileName?: string
      }

  const shrinkDuration = 300
  const fadeDuration = 300 / 2

  let myState: State = $state({ state: "idle" })

  let isDragHovering = $state(false)
  let shouldDoHoverEffect = $derived(isDragHovering && myState.state === "idle")

  let windowTitle = $derived.by(() => {
    if (myState.state === "sending") {
      return `Sending ${myState.fileName || "file"}`
    } else if (myState.state === "receiving") {
      return `Receiving ${myState.fileName || "file"}`
    } else {
      return "Sendworm"
    }
  })

  $effect(() => {
    invoke("get_file_to_send").then((filePath) => {
      if (filePath !== null) {
        sendFileOrFolder(filePath as string)
      }
    })
  })

  $effect(() => {
    const unlisten = initializeMenu()

    return () => {
      unlisten()
    }
  })

  $effect(() => {
    const unlistenPromise = getCurrentWindow().onDragDropEvent((event) => {
      const { payload } = event

      switch (payload.type) {
        case "enter":
        case "over":
          isDragHovering = true
          break
        case "drop":
        case "leave":
          isDragHovering = false
          break
      }

      if (myState.state === "idle" && payload.type === "drop") {
        const filePath = payload.paths[0]
        if (filePath) sendFileOrFolder(filePath)
      }
    })

    return () => {
      unlistenPromise.then((unlisten) => unlisten())
    }
  })

  $effect(() => {
    if (myState.state === "sending") {
      basename(myState.filePath).then((name) => {
        if (myState.state === "sending") {
          myState.fileName = name
        }
      })
    }
  })

  $effect(() => {
    getCurrentWindow().setTitle(windowTitle)
  })

  function sendFileOrFolder(filePath: string) {
    myState = {
      state: "sending",
      filePath,
    }
  }

  async function selectAndSendFile() {
    const path = await open({
      multiple: false,
    })

    if (path) sendFileOrFolder(path)
  }

  async function selectAndSendFolder() {
    const path = await open({
      directory: true,
    })

    if (path) sendFileOrFolder(path)
  }

  function receiveFile(code: string) {
    myState = {
      state: "receiving",
      code,
    }
  }

  function goToIdle() {
    myState = { state: "idle" }
  }

  function preventContextMenu(e: MouseEvent) {
    const target = e.target as HTMLElement
    if (target.nodeName == "INPUT" && target.getAttribute("type") == "text") {
      return
    }

    e.preventDefault()
  }
</script>

<svelte:window oncontextmenu={preventContextMenu} />

<main>
  {#if myState.state === "idle" || myState.state === "sending"}
    <div
      class="area send-area"
      class:drag-hover={shouldDoHoverEffect}
      transition:shrink={{ duration: shrinkDuration }}
    >
      {#if myState.state !== "sending"}
        <div
          class="area-content send-menu-content halved"
          in:fade|global={{ duration: fadeDuration, delay: fadeDuration }}
          out:fade|global={{ duration: fadeDuration }}
        >
          <div class="drop-border"></div>
          <div class="top-half">
            <SendIcon size={96} />
          </div>
          <div class="bottom-half">
            <h2>Drag a file here</h2>
            <p class="sub-text">or click to select a file</p>
            <button class="send-folder-button" onclick={selectAndSendFolder}>
              <FolderIcon size={16} />
              <span>Select folder</span>
            </button>
          </div>
        </div>

        <button
          class="send-button-overlay"
          onclick={selectAndSendFile}
          aria-label="Select and send file"
        ></button>
      {:else}
        <div
          class="area-content"
          in:fade={{ duration: fadeDuration, delay: fadeDuration }}
          out:fade={{ duration: fadeDuration }}
        >
          <SendPanel filePath={myState.filePath} goBack={goToIdle} />
        </div>
      {/if}
    </div>
  {/if}
  {#if myState.state === "idle" || myState.state === "receiving"}
    <div
      class="area receive-area"
      transition:shrink={{ duration: shrinkDuration }}
    >
      {#if myState.state !== "receiving"}
        <div
          class="area-content receive-menu-content halved"
          in:fade|global={{ duration: fadeDuration, delay: fadeDuration }}
          out:fade|global={{ duration: fadeDuration }}
        >
          <div class="top-half">
            <ReceiveIcon size={96} />
          </div>
          <div class="bottom-half">
            <h2>Enter code to receive</h2>
            <CodeInput onsubmit={receiveFile} />
          </div>
        </div>
      {:else}
        <div
          class="area-content"
          in:fade={{ duration: fadeDuration, delay: fadeDuration }}
          out:fade={{ duration: fadeDuration }}
        >
          <ReceivePanel
            code={myState.code}
            goBack={goToIdle}
            onFileInfo={(info) => {
              if (myState.state === "receiving")
                myState.fileName = info.fileName
            }}
          />
        </div>
      {/if}
    </div>
  {/if}
</main>

<style>
  :global(body) {
    background-color: #0e0e0b;
    color: #ebebeb;
    height: 100vh;
    display: grid;
  }

  main {
    display: flex;
    padding: 12px;
    gap: 12px;

    @media screen and (max-width: 600px) {
      flex-direction: column;
    }
  }

  .halved {
    display: grid;
    grid-template-rows: 1fr 1fr;

    .top-half {
      display: flex;
      flex-direction: column;
      align-items: center;
      justify-content: flex-end;
    }

    .bottom-half {
      display: flex;
      flex-direction: column;
      align-items: center;
      justify-content: flex-start;
    }
  }

  .area {
    position: relative;
    height: 100%;
    flex: 1 1 100%;
    box-shadow: 0px 4px 10px rgba(0, 0, 0, 0.25);
    border-radius: 12px;
  }

  .send-area {
    background-color: #1c2e5f;
    color: #ccdaff;
    border: solid 1px rgba(204, 218, 255, 0.25);

    transition:
      background-color ease-out 0.1s,
      border ease-out 0.1s,
      flex-basis ease-in-out 0.2s;

    &:has(.send-button-overlay:hover:not(:active)) {
      background-color: #243a78;
      border: solid 1px rgba(204, 218, 255, 0.5);

      .drop-border {
        border: dashed rgba(204, 218, 255, 0.5) 4px;
      }
    }

    &.drag-hover {
      flex-basis: 120%;
      background-color: #243a78;
      border: solid 1px rgba(204, 218, 255, 0.5);

      .drop-border {
        border: dashed rgba(204, 218, 255, 0.5) 4px;
      }
    }

    &:has(.send-button-overlay:active) {
      background-color: #15244a;
    }
  }

  .send-button-overlay {
    position: absolute;
    cursor: pointer;
    top: 0;
    right: 0;
    bottom: 0;
    left: 0;
    border-radius: 12px;
  }

  .area-content {
    position: absolute;
    top: 0;
    right: 0;
    bottom: 0;
    left: 0;
  }

  .send-menu-content {
    overflow: hidden;
    white-space: nowrap;

    .drop-border {
      position: absolute;
      top: 18px;
      right: 18px;
      bottom: 18px;
      left: 18px;
      border: dashed rgba(204, 218, 255, 0.25) 4px;
      border-radius: 2px;
      pointer-events: none;
      transition: border ease-out 0.1s;
    }

    h2 {
      font-size: 24px;
      font-weight: 500;
      margin-bottom: 4px;
    }

    .sub-text {
      font-size: 14px;
      font-weight: 300;
      color: rgba(204, 218, 255, 0.5);
    }

    .send-folder-button {
      cursor: pointer;
      z-index: 2;
      margin-top: 12px;
      display: flex;
      align-items: center;
      gap: 6px;
      padding: 4px 8px;
      border-radius: 8px;
      background-color: #ccdaff;
      color: #202943;
      font-weight: 500;
      font-size: 14px;
      transition: background-color ease-out 0.1s;

      span {
        margin-bottom: 2px;
      }

      &:hover {
        background-color: #e6ecff;
      }

      &:active {
        background-color: #9eabcf;
      }
    }
  }

  .receive-area {
    background-color: #131210;
    color: #ffe0b5;
    border: solid 1px rgba(255, 224, 181, 0.25);
  }

  .receive-menu-content {
    overflow: hidden;
    white-space: nowrap;

    h2 {
      font-size: 24px;
      font-weight: 500;
      margin-bottom: 18px;
    }
  }
</style>
