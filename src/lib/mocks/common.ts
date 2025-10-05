export class MockChannel<T> {
  onmessage: (response: T) => void = () => {}

  send(msg: T) {
    this.onmessage(msg)
  }
}

export function delay(timeout: number): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, timeout))
}
