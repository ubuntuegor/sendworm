<script lang="ts">
  import CodeInput from "$lib/components/CodeInput.svelte"
  import ReceivePanel from "$lib/components/ReceivePanel.svelte"
  import SendPanel from "$lib/components/SendPanel.svelte"
  import { shrink } from "$lib/transitions"
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
      }
    | {
        state: "receiving"
        code: string
      }

  const shrinkDuration = 300
  const fadeDuration = 300 / 2

  let myState: State = $state({ state: "idle" })

  let isDragHovering = $state(false)
  let shouldDoHoverEffect = $derived(isDragHovering && myState.state === "idle")

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
            <svg
              xmlns="http://www.w3.org/2000/svg"
              width="96"
              height="96"
              viewBox="0 0 24 24"
            >
              <path
                fill="currentColor"
                d="M11 16V7.85l-2.6 2.6L7 9l5-5l5 5l-1.4 1.45l-2.6-2.6V16zm-5 4q-.825 0-1.412-.587T4 18v-3h2v3h12v-3h2v3q0 .825-.587 1.413T18 20z"
              />
            </svg>
          </div>
          <div class="bottom-half">
            <h2>Drag a file here</h2>
            <p class="sub-text">or click to select a file</p>
            <button class="send-folder-button" onclick={selectAndSendFolder}>
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
            <svg
              xmlns="http://www.w3.org/2000/svg"
              width="96"
              height="96"
              viewBox="0 0 24 24"
            >
              <path
                fill="currentColor"
                d="m12 16l-5-5l1.4-1.45l2.6 2.6V4h2v8.15l2.6-2.6L17 11zm-6 4q-.825 0-1.412-.587T4 18v-3h2v3h12v-3h2v3q0 .825-.587 1.413T18 20z"
              />
            </svg>
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
          <ReceivePanel code={myState.code} goBack={goToIdle} />
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
