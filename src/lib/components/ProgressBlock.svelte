<script lang="ts">
  import DoneIcon from "$lib/icons/DoneIcon.svelte"
  import type { TransitInfo } from "$lib/types/common"
  import { formatSize } from "$lib/utils/files"
  import { quadOut } from "svelte/easing"
  import { fade, scale } from "svelte/transition"

  interface Props {
    mode: "send" | "receive"
    progress: [number, number] | null
    transitInfo: TransitInfo | null
    finished: boolean
    error: string | null
  }

  const { mode, progress, transitInfo, finished, error }: Props = $props()

  const action = $derived(mode === "send" ? "Sent" : "Received")

  const percentage = $derived.by(() => {
    if (finished) return 100
    if (!progress) return 0
    return Math.floor((progress[0] / progress[1]) * 100)
  })

  const subText = $derived.by(() => {
    if (error !== null) return error
    if (finished) return "Transfer complete"
    if (!transitInfo) return "Connecting..."

    switch (transitInfo.connectionType) {
      case "direct":
        return `Connected to ${transitInfo.address} directly`
      case "relay":
        return `Connected to ${transitInfo.address} via relay`
      case "unknown":
        return `Connected to ${transitInfo.address}`
    }
  })
</script>

<div
  class="progress-circle"
  class:semitransparent={error !== null}
  role="progressbar"
  aria-valuenow={percentage}
  style:--percentage={percentage + "%"}
>
  {#if !finished}
    <div
      class="filling"
      transition:scale={{ duration: 200, opacity: 1, easing: quadOut }}
    ></div>
    <div class="percentage" transition:fade={{ duration: 200 }}>
      <b>{percentage}</b>%
    </div>
  {:else}
    <div class="done-icon">
      <DoneIcon size={54} />
    </div>
  {/if}
</div>
<p class:semitransparent={error !== null}>
  {#if progress && finished}
    {action} {formatSize(progress[1])} out of {formatSize(progress[1])}
  {:else if progress}
    {action} {formatSize(progress[0])} out of {formatSize(progress[1])}
  {:else}
    {action} 0 B out of 0 B
  {/if}
</p>
<p class="sub-text" class:red={error !== null}>
  {subText}
</p>

<style>
  .semitransparent {
    opacity: 0.5;
  }

  .progress-circle {
    margin-bottom: 10px;
    position: relative;
    width: 80px;
    height: 80px;
    border-radius: 50%;
    background-image: conic-gradient(
        currentColor var(--percentage),
        transparent var(--percentage)
      ),
      linear-gradient(var(--percentage-inactive));

    line-height: 80px;
    font-size: 20px;
    text-align: center;

    div {
      position: absolute;
      top: 0px;
      right: 0px;
      bottom: 0px;
      left: 0px;
    }

    .filling {
      margin: 6px;
      border-radius: 50%;
      background-color: var(--filling-color);
    }

    .done-icon {
      margin-top: 16px;
      color: var(--filling-color);
    }
  }

  p {
    font-size: 14px;
    text-align: center;
  }

  .sub-text {
    margin-top: 1px;
    font-size: 12px;
    opacity: 0.5;
    text-align: center;

    &.red {
      color: #ff9ca8;
      opacity: 1;
    }
  }
</style>
