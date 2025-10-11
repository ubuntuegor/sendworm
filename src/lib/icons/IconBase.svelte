<script lang="ts">
  import type { Snippet } from "svelte"

  interface Props {
    size: number
    children: Snippet
  }

  const { size, children }: Props = $props()
</script>

<svg
  xmlns="http://www.w3.org/2000/svg"
  width={size}
  height={size}
  viewBox="0 0 24 24"
>
  {@render children()}
</svg>

<style>
  svg {
    display: block;
  }
</style>
