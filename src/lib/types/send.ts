export type TransitInfo = {
  connectionType: "direct" | "relay" | "unknown"
  address: string
}

export type SendEvent =
  | {
      event: "code"
      data: {
        code: string
      }
    }
  | {
      event: "connected"
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
