<script lang="ts">
  import FolderIcon from "$lib/icons/FolderIcon.svelte"
  import FileIcon from "$lib/icons/FileIcon.svelte"
  import { formatSize } from "$lib/utils/files"

  interface Props {
    mode: "send" | "receive"
    isDir: boolean
    fileName: string | null
    fileNameTooltip: string | null
    fileSize: number | null
  }

  const { mode, isDir, fileName, fileNameTooltip, fileSize }: Props = $props()
</script>

<div class="file-info">
  <div class="icon">
    {#if isDir}
      <FolderIcon size={24} />
    {:else}
      <FileIcon size={24} />
    {/if}
  </div>

  <div class="name-and-more">
    <p class="sub-title">
      {mode === "send" ? "Sending" : "Receiving"} a {isDir ? "folder" : "file"}
      {fileSize ? `(${formatSize(fileSize)})` : ""}
    </p>
    <p class="title" title={fileNameTooltip}>
      {fileName}
    </p>
  </div>
</div>

<style>
  .file-info {
    min-width: 200px;
    max-width: 340px;
    display: flex;
    align-items: center;
    gap: 12px;

    .icon {
      flex: 0 0 auto;
    }

    .name-and-more {
      display: flex;
      flex-direction: column;
      white-space: nowrap;
      overflow: hidden;
      font-size: 14px;

      .sub-title {
        font-size: 12px;
        opacity: 0.5;
      }

      .title {
        font-weight: bold;
        overflow: hidden;
        text-overflow: ellipsis;
      }
    }
  }
</style>
