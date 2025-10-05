import type { TransitInfo } from "./common"

export type FileInfo = {
  fileName: string
  fileSize: number
}

export type ReceiveEvent =
  | {
      event: "fileInfo"
      data: FileInfo
    }
  | {
      event: "transitInfo"
      data: TransitInfo
    }
  | {
      event: "progress"
      data: {
        sent: number
        total: number
      }
    }
  | {
      event: "finished"
    }
  | {
      event: "error"
      data: {
        message: string
      }
    }
