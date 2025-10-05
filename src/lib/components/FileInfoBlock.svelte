<script lang="ts">
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
  {#if isDir}
    <svg
      xmlns="http://www.w3.org/2000/svg"
      width="24"
      height="24"
      viewBox="0 0 24 24"
    >
      <path
        fill="currentColor"
        d="M4 20q-.825 0-1.412-.587T2 18V6q0-.825.588-1.412T4 4h6l2 2h8q.825 0 1.413.588T22 8v10q0 .825-.587 1.413T20 20zm0-2h16V8h-8.825l-2-2H4zm0 0V6z"
      />
    </svg>
  {:else}
    <svg
      xmlns="http://www.w3.org/2000/svg"
      width="24"
      height="24"
      viewBox="0 0 24 24"
    >
      <path
        fill="currentColor"
        d="M6 22q-.825 0-1.412-.587T4 20V4q0-.825.588-1.412T6 2h8l6 6v12q0 .825-.587 1.413T18 22zm7-13V4H6v16h12V9zM6 4v5zv16z"
      />
    </svg>
  {/if}

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

    svg {
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
