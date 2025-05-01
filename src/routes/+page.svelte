<script lang="ts">
  import { getCurrentWindow } from "@tauri-apps/api/window"
  import { open } from "@tauri-apps/plugin-dialog"

  let filepath: string | null = $state(null)

  $effect(() => {
    const unlistenPromise = getCurrentWindow().onDragDropEvent((event) => {
      const { payload } = event
      if (payload.type === "drop") {
        filepath = payload.paths[0] || null
      }
    })

    return () => {
      unlistenPromise.then((unlisten) => unlisten())
    }
  })

  async function selectFile() {
    filepath = await open({
      multiple: false,
    })
  }
</script>

<main>
  <p>Filepath: {filepath}</p>
  <p><button onclick={selectFile}>Select file</button></p>
</main>
