<script lang="ts">
  import { Channel, invoke } from "@tauri-apps/api/core"
  import { getCurrentWindow } from "@tauri-apps/api/window"
  import { open } from "@tauri-apps/plugin-dialog"

  let filePath: string | null = $state(null)
  let code: string | null = $state(null)
  let awaitingConfirmation: boolean = $state(false)
  let transitInfo: {
    connectionType: string
    address: string
  } | null = $state(null)
  let progress: [number, number] | null = $state(null)
  let finished: boolean = $state(false)
  let error: string | null = $state(null)

  let tasksNumber: number = $state(0)

  $effect(() => {
    const id = setInterval(async () => {
      tasksNumber = await invoke("get_tasks_number")
    }, 500)

    return () => {
      clearInterval(id)
    }
  })

  $effect(() => {
    const unlistenPromise = getCurrentWindow().onDragDropEvent((event) => {
      const { payload } = event
      if (payload.type === "drop") {
        filePath = payload.paths[0] || null
      }
    })

    return () => {
      unlistenPromise.then((unlisten) => unlisten())
    }
  })

  async function selectFile() {
    filePath = await open({
      multiple: false,
    })
  }

  type SendEvent =
    | {
        event: "code"
        data: {
          code: string
        }
      }
    | {
        event: "connected"
      }
    | {
        event: "transitInfo"
        data: {
          connectionType: string
          address: string
        }
      }
    | {
        event: "progress"
        data: {
          sent: number
          total: number
        }
      }
    | {
        event: "finished"
      }
    | {
        event: "error"
        data: {
          message: string
        }
      }

  async function sendFile() {
    const fileName = await invoke("compute_file_name", {
      filePath: filePath,
    })

    if (fileName === null) {
      throw new Error("watafff ")
    }

    const onEvent = new Channel<SendEvent>()
    onEvent.onmessage = (message) => {
      switch (message.event) {
        case "code":
          code = message.data.code
          break
        case "connected":
          awaitingConfirmation = true
          break
        case "transitInfo":
          transitInfo = message.data
          break
        case "progress":
          awaitingConfirmation = false
          progress = [message.data.sent, message.data.total]
          break
        case "finished":
          finished = true
          break
        case "error":
          error = message.data.message
          break
      }
    }

    code = null
    awaitingConfirmation = false
    transitInfo = null
    progress = null
    finished = false
    error = null

    invoke("send_file", { filePath, onEvent }).then((res) => console.log(res))
  }

  function confirmSend() {
    invoke("confirm_send")
  }

  function cancelSend() {
    invoke("cancel_send")
  }
</script>

<main>
  <p>Tasks number: {tasksNumber}</p>
  <p>Filepath: {filePath}</p>
  <p><button onclick={selectFile}>Select file</button></p>
  {#if filePath !== null}
    <p><button onclick={sendFile}>Send</button></p>
  {/if}
  {#if code !== null}
    <p>Code: {code}</p>
  {/if}
  {#if awaitingConfirmation}
    <p><button onclick={confirmSend}>Confirm</button></p>
  {/if}
  {#if transitInfo !== null}
    <p>Transit info: {JSON.stringify(transitInfo)}</p>
  {/if}
  {#if progress !== null}
    <p>Progress: {progress[0]} out of {progress[1]}</p>
  {/if}
  {#if finished}
    <p>Finished!</p>
  {/if}
  {#if error !== null}
    <p>ERROR: {error}</p>
  {/if}
  <p><button onclick={cancelSend}>Cancel</button></p>
</main>
