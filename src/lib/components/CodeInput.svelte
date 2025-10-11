<script lang="ts">
  import ChevronRightIcon from "$lib/icons/ChevronRightIcon.svelte"
  import { scaleVertically } from "$lib/transitions"
  import { invoke } from "@tauri-apps/api/core"

  interface Props {
    onsubmit: (code: string) => void
  }

  const { onsubmit }: Props = $props()

  let code = $state("")
  let completions: string[] = $state([])

  let codeField: HTMLInputElement | undefined = $state()

  $effect(() => {
    getCompletions(code).then((result) => {
      completions = result.filter((item) => item !== code)
    })
  })

  function getCompletions(code: string): Promise<string[]> {
    return invoke("get_completions", { code })
  }

  function keyboardNavigation(node: HTMLElement) {
    function getNavigatableItems(): HTMLElement[] {
      const inputField = node.querySelector(".code-input")
      const autocompleteItems = node.querySelectorAll(".autocomplete-item")

      return [inputField].concat(Array.from(autocompleteItems)) as HTMLElement[]
    }

    function handleKeydown(event: KeyboardEvent) {
      if (event.key !== "ArrowDown" && event.key !== "ArrowUp") {
        return
      }
      event.preventDefault()

      const items = getNavigatableItems()
      const current = document.activeElement

      let index = items.indexOf(current as any)
      if (index === -1) {
        index = 0
      }

      let next = 0

      if (event.key === "ArrowDown") {
        next = index + 1
        if (next >= items.length) {
          next = 0
        }
      } else if (event.key === "ArrowUp") {
        next = index - 1
        if (next < 0) {
          next = items.length - 1
        }
      }

      items[next].focus()
    }

    $effect(() => {
      node.addEventListener("keydown", handleKeydown)

      return () => {
        node.removeEventListener("keydown", handleKeydown)
      }
    })
  }

  function applyCompletion(item: string) {
    code = item
    codeField!.focus()
    setTimeout(() => {
      codeField!.scrollLeft = codeField!.scrollWidth
    })
  }

  function scrollWhenCompletionsChange(node: HTMLElement) {
    $effect(() => {
      completions

      node.scrollLeft = node.scrollWidth
    })
  }

  function submitForm() {
    onsubmit(code.trim())
  }
</script>

<form class="code-form" onsubmit={submitForm} use:keyboardNavigation>
  <input
    type="text"
    class="code-input"
    placeholder="Start typing"
    spellcheck="false"
    autocomplete="off"
    bind:value={code}
    bind:this={codeField}
  />
  <button type="submit" aria-label="Receive file" disabled={code === ""}>
    <ChevronRightIcon size={24} />
  </button>

  {#if completions.length > 0}
    <div
      class="autocomplete"
      transition:scaleVertically={{ duration: 100 }}
      use:scrollWhenCompletionsChange
    >
      {#each completions as item}
        <button
          class="autocomplete-item"
          type="button"
          onclick={() => {
            applyCompletion(item)
          }}
        >
          {item.slice(0, code.length)}<b>{item.slice(code.length)}</b>
        </button>
      {/each}
    </div>
  {/if}
</form>

<style>
  .code-form {
    position: relative;
    width: 240px;
    display: flex;
    gap: 6px;

    & > input {
      all: unset;
      height: 32px;
      box-sizing: border-box;
      flex: 1 1 auto;
      padding: 0px 11px;
      padding-bottom: 2px;

      background-color: #1d1b18;
      color: #ffe0b5;
      border: solid 1px rgba(255, 224, 181, 0.25);
      border-radius: 12px;

      transition: border ease-out 0.1s;

      font-weight: 300;
      font-size: 14px;

      &::placeholder {
        color: rgba(255, 224, 181, 0.25);
      }

      &::selection {
        background-color: #725b3a;
      }

      &:focus {
        border: solid 1px rgba(255, 224, 181, 0.75);
        outline: none;
      }
    }

    & > button {
      cursor: pointer;
      flex: 0 0 auto;
      width: 32px;
      height: 32px;
      box-sizing: border-box;
      display: grid;
      place-content: center;
      padding-left: 2px;

      background-color: #ffe0b5;
      color: #131210;
      border-radius: 50%;

      transition:
        opacity ease-out 0.1s,
        background-color ease-out 0.1s;

      &[disabled] {
        opacity: 0.5;
        cursor: default;
      }

      &:hover:not([disabled]) {
        background-color: #ffe6c3;
      }

      &:active:not([disabled]) {
        background-color: #e3c396;
      }
    }
  }

  .autocomplete {
    position: absolute;
    top: 38px;
    width: 202px;
    padding: 6px;
    box-sizing: border-box;
    overflow: hidden;

    background-color: #1d1b18;
    border: solid 1px rgba(255, 224, 181, 0.25);
    border-radius: 12px;

    transform-origin: top;

    @media screen and (max-width: 600px) {
      top: unset;
      bottom: 38px;
      transform-origin: bottom;
    }

    .autocomplete-item {
      display: block;
      min-width: 100%;
      height: 28px;
      padding: 0px 5px;
      box-sizing: border-box;
      border-radius: 6px;
      outline: unset;

      font-size: 14px;
      font-weight: 300;
      color: rgba(255, 224, 181, 0.8);

      transition: background-color ease-out 0.1s;

      &:hover {
        background-color: rgba(255, 224, 181, 0.15);
      }

      &:focus {
        background-color: rgba(255, 224, 181, 0.15);
        color: #ffe0b5;
      }

      b {
        font-weight: 500;
      }
    }
  }
</style>
