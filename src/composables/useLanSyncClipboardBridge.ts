const REMOTE_CLIPBOARD_TTL_MS = 4000

type PendingRemoteClipboard = {
  text: string
  expiresAt: number
}

let pendingRemoteClipboard: PendingRemoteClipboard | null = null

export function markPendingRemoteClipboardText(text: string): void {
  pendingRemoteClipboard = {
    text,
    expiresAt: Date.now() + REMOTE_CLIPBOARD_TTL_MS,
  }
}

export function consumePendingRemoteClipboardText(text: string): boolean {
  if (!pendingRemoteClipboard) {
    return false
  }

  if (pendingRemoteClipboard.expiresAt <= Date.now()) {
    pendingRemoteClipboard = null
    return false
  }

  if (pendingRemoteClipboard.text !== text) {
    return false
  }

  pendingRemoteClipboard = null
  return true
}
