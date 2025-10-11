<script lang="ts">
  import IconBase from "./IconBase.svelte"
  const { size }: IconProps = $props()
</script>

<IconBase {size}>
  <path
    fill="currentColor"
    d="m9.55 18l-5.7-5.7l1.425-1.425L9.55 15.15l9.175-9.175L20.15 7.4z"
  />
</IconBase>
