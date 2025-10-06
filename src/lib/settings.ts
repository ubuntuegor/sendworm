import { downloadDir } from "@tauri-apps/api/path"
import { load, Store } from "@tauri-apps/plugin-store"

const RECEIVE_FOLDER = "receiveFolder"

let store: Store | undefined

load("settings.json").then((result) => {
  store = result
})

export async function getReceiveFolder(): Promise<string> {
  const folder = await store?.get<string>(RECEIVE_FOLDER)

  if (folder !== undefined) {
    return folder
  }

  return downloadDir()
}

export async function setReceiveFolder(folder: string) {
  store?.set(RECEIVE_FOLDER, folder)
}
