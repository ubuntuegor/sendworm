import type { UnlistenFn } from "@tauri-apps/api/event"
import { downloadDir } from "@tauri-apps/api/path"
import { load } from "@tauri-apps/plugin-store"

const RECEIVE_FOLDER = "receiveFolder"
const ASK_BEFORE_SEND = "askBeforeSend"
const ASK_BEFORE_RECEIVE = "askBeforeReceive"

const store = load("settings.json")

export async function getReceiveFolder(): Promise<string> {
  const result = await (await store).get<string>(RECEIVE_FOLDER)

  return result ?? downloadDir()
}

export async function setReceiveFolder(value: string) {
  await (await store).set(RECEIVE_FOLDER, value)
}

export async function onReceiveFolderChange(
  listener: (value: string | undefined) => void
): Promise<UnlistenFn> {
  return (await store).onKeyChange(RECEIVE_FOLDER, listener)
}

export async function getAskBeforeSend(): Promise<boolean> {
  const result = await (await store).get<boolean>(ASK_BEFORE_SEND)

  return result ?? false
}

export async function setAskBeforeSend(value: boolean) {
  await (await store).set(ASK_BEFORE_SEND, value)
}

export async function onAskBeforeSendChange(
  listener: (value: boolean | undefined) => void
): Promise<UnlistenFn> {
  return (await store).onKeyChange(ASK_BEFORE_SEND, listener)
}

export async function getAskBeforeReceive(): Promise<boolean> {
  const result = await (await store).get<boolean>(ASK_BEFORE_RECEIVE)

  return result ?? true
}

export async function setAskBeforeReceive(value: boolean) {
  await (await store).set(ASK_BEFORE_RECEIVE, value)
}

export async function onAskBeforeReceiveChange(
  listener: (value: boolean | undefined) => void
): Promise<UnlistenFn> {
  return (await store).onKeyChange(ASK_BEFORE_RECEIVE, listener)
}
