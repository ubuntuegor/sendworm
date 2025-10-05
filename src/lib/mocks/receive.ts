import type { ReceiveEvent } from "$lib/types/receive"
import { delay, MockChannel } from "./common"

let currentChannel: MockChannel<ReceiveEvent> | null = null

export async function mockReceiveFile(channel: MockChannel<ReceiveEvent>) {
  currentChannel = channel
  await delay(2000)
  channel.send({
    event: "fileInfo",
    data: {
      fileName: "windowsdesktop-runtime-9.0.9-win-x64.exe",
      fileSize: 60_950_768,
    },
  })

  // await delay(2000)
  // channel.send({
  //   event: "error",
  //   data: {
  //     message: "Wormhole error: something happened",
  //   },
  // })
}

export async function mockConfirmReceive() {
  const channel = currentChannel as MockChannel<ReceiveEvent>
  channel.send({
    event: "transitInfo",
    data: {
      address: "127.0.0.1",
      connectionType: "direct",
    },
  })
  const totalSize = 25565

  for (var i = 0; i < totalSize; i += totalSize / 100) {
    channel.send({
      event: "progress",
      data: {
        sent: Math.floor(i),
        total: totalSize,
      },
    })

    // if (i > 5000) {
    //   channel.send({
    //     event: "error",
    //     data: {
    //       message: "Wormhole error: disconnected",
    //     },
    //   })
    //   return
    // }

    await delay(50)
  }
  channel.send({
    event: "progress",
    data: {
      sent: totalSize,
      total: totalSize,
    },
  })

  await delay(5)

  channel.send({
    event: "finished",
  })
}

export function mockCancelReceive() {
  if (currentChannel) currentChannel.onmessage = () => {}
  currentChannel = null
}
