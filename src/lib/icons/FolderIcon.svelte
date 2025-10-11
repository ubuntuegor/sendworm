<script lang="ts">
  import IconBase from "./IconBase.svelte"
  const { size }: IconProps = $props()
</script>

<IconBase {size}>
  <path
    fill="currentColor"
    d="M4 20q-.825 0-1.412-.587T2 18V6q0-.825.588-1.412T4 4h6l2 2h8q.825 0 1.413.588T22 8v10q0 .825-.587 1.413T20 20zm0-2h16V8h-8.825l-2-2H4zm0 0V6z"
  />
</IconBase>
