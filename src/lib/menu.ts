import {
  CheckMenuItem,
  Menu,
  MenuItem,
  PredefinedMenuItem,
  Submenu,
} from "@tauri-apps/api/menu"
import { open } from "@tauri-apps/plugin-dialog"
import {
  getAskBeforeReceive,
  getAskBeforeSend,
  getReceiveFolder,
  onAskBeforeReceiveChange,
  onAskBeforeSendChange,
  onReceiveFolderChange,
  setAskBeforeReceive,
  setAskBeforeSend,
  setReceiveFolder,
} from "./settings"
import { platform } from "@tauri-apps/plugin-os"
import { getCurrentWindow } from "@tauri-apps/api/window"
import type { UnlistenFn } from "@tauri-apps/api/event"

export function initializeMenu() {
  const myPromise = (async function () {
    const unlistens: Promise<UnlistenFn>[] = []

    const askBeforeSend = await CheckMenuItem.new({
      text: "Ask for confirmation before sending",
      checked: await getAskBeforeSend(),
      action: async () => {
        setAskBeforeSend(!(await getAskBeforeSend()))
      },
    })

    unlistens.push(
      onAskBeforeSendChange((newValue) => {
        if (newValue !== undefined) askBeforeSend.setChecked(newValue)
      })
    )

    const askBeforeReceive = await CheckMenuItem.new({
      text: "Ask for confirmation before receiving",
      checked: await getAskBeforeReceive(),
      action: async () => {
        setAskBeforeReceive(!(await getAskBeforeReceive()))
      },
    })

    unlistens.push(
      onAskBeforeReceiveChange((newValue) => {
        if (newValue !== undefined) askBeforeReceive.setChecked(newValue)
      })
    )

    const separator = await PredefinedMenuItem.new({
      item: "Separator",
    })

    const receiveFolderTitle = await MenuItem.new({
      text: "Receive files to this folder:",
      enabled: false,
    })

    const receiveFolder = await MenuItem.new({
      text: await getReceiveFolder(),
      enabled: false,
    })

    unlistens.push(
      onReceiveFolderChange((newValue) => {
        if (newValue !== undefined) receiveFolder.setText(newValue)
      })
    )

    const changeReceiveFolder = await MenuItem.new({
      text: "Change folder",
      action: async () => {
        const path = await open({
          directory: true,
        })

        if (path) setReceiveFolder(path)
      },
    })

    const menuItems = []

    if (platform() === "macos") {
      // filler item because in macos first submenu goes into the "application name" menu
      menuItems.push(
        await Submenu.new({
          text: "About",
          items: [
            await MenuItem.new({
              text: "Quit",
              accelerator: "Cmd+Q",
              action: async () => {
                await getCurrentWindow().close()
              },
            }),
          ],
        })
      )

      async function closeWindowListener(event: KeyboardEvent) {
        if (event.code === "KeyW" && event.metaKey) {
          await getCurrentWindow().close()
        }
      }

      window.addEventListener("keydown", closeWindowListener)

      unlistens.push(
        Promise.resolve(() => {
          window.removeEventListener("keydown", closeWindowListener)
        })
      )
    }

    menuItems.push(
      await Submenu.new({
        text: "Settings",
        items: [
          askBeforeSend,
          askBeforeReceive,
          separator,
          receiveFolderTitle,
          receiveFolder,
          changeReceiveFolder,
        ],
      })
    )

    const menu = await Menu.new({
      items: menuItems,
    })

    menu.setAsAppMenu()

    return unlistens
  })()

  return () => {
    myPromise.then((unlistenPromises) => {
      Promise.all(unlistenPromises).then((unlistens) =>
        unlistens.forEach((unlisten) => unlisten())
      )
    })
  }
}
