<script lang="ts">
  import IconBase from "./IconBase.svelte"
  const { size }: IconProps = $props()
</script>

<IconBase {size}>
  <path
    fill="currentColor"
    d="m12 16l-5-5l1.4-1.45l2.6 2.6V4h2v8.15l2.6-2.6L17 11zm-6 4q-.825 0-1.412-.587T4 18v-3h2v3h12v-3h2v3q0 .825-.587 1.413T18 20z"
  />
</IconBase>
