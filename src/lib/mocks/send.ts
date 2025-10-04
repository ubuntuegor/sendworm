import type { SendEvent } from "$lib/types/send"
import { delay } from "./common"

export class MockChannel<T> {
  onmessage: (response: T) => void = () => {}

  send(msg: T) {
    this.onmessage(msg)
  }
}

let currentChannel: MockChannel<SendEvent> | null = null

export async function mockSendFile(
  filePath: string,
  channel: MockChannel<SendEvent>
) {
  currentChannel = channel
  await delay(1000)
  channel.send({
    event: "code",
    data: {
      code: "9-knight-mice",
    },
  })
  await delay(3000)

  // channel.send({
  //   event: "error",
  //   data: {
  //     message: "Wormhole error: something happened",
  //   },
  // })
  // return

  channel.send({
    event: "connected",
  })
}

export async function mockConfirmSend() {
  const channel = currentChannel as MockChannel<SendEvent>
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

    await delay(500)
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

export function mockCancelSend() {
  if (currentChannel) currentChannel.onmessage = () => {}
  currentChannel = null
}
