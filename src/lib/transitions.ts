import { quadInOut } from "svelte/easing"
import type { TransitionConfig } from "svelte/transition"

export function shrink(
  _: HTMLElement,
  { delay = 0, duration = 300, easing = quadInOut } = {}
): TransitionConfig {
  return {
    delay,
    duration,
    css: (t: number) => `flex-basis: ${Math.floor(t * 100)}%; opacity: ${t}`,
    easing,
  }
}
