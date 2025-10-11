<script lang="ts">
  import IconBase from "./IconBase.svelte"
  const { size }: IconProps = $props()
</script>

<IconBase {size}>
  <path fill="currentColor" d="M12.6 12L8 7.4L9.4 6l6 6l-6 6L8 16.6z" />
</IconBase>
