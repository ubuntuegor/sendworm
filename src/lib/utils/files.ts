import { invoke } from "@tauri-apps/api/core"

export async function isFolder(filePath: string): Promise<boolean> {
  return (await invoke("is_folder", { filePath })) || false
}

export async function computeFileName(
  filePath: string
): Promise<string | null> {
  return await invoke("compute_file_name", { filePath })
}

export async function getFileSize(filePath: string): Promise<number | null> {
  return await invoke("get_file_size", { filePath })
}

export function formatSize(size: number): string {
  const units = ["B", "KB", "MB", "GB", "TB"]

  for (let i = 0; i < units.length; i++) {
    if (size < 1000) {
      return `${size.toFixed(1)} ${units[i]}`
    }

    size /= 1000
  }

  return `${size.toFixed(1)} PB`
}
