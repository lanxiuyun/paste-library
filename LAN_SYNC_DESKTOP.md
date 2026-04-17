# Desktop LAN Sync

## Overview

The current LAN sync feature is designed for syncing clipboard content inside the same local network.

Current desktop implementation characteristics:

- UDP broadcast device discovery
- TCP point-to-point message delivery
- manual pairing approval on first trust
- Rust background service that does not depend on the clipboard window being open

## Key Files

- `src-tauri/src/sync_service.rs`: LAN discovery, pairing, broadcast, inbound handling, status events
- `src-tauri/src/lib.rs`: Tauri lifecycle wiring, command registration, service restart after settings changes
- `src-tauri/src/clipboard.rs`: clipboard persistence flow reused by sync ingestion
- `src-tauri/src/storage.rs`: settings persistence and history cleanup
- `src-tauri/src/models.rs`: `AppSettings`, `LanSyncStatus`, `ClipboardSource`
- `src/composables/useClipboard.ts`: local clipboard listening and source tagging
- `src/composables/useLanSync.ts`: frontend sync state and device management commands
- `src/composables/useLanSyncClipboardBridge.ts`: bridge for writing inbound synced text back to clipboard
- `src/components/settings/sections/SyncSection.vue`: sync settings UI
- `src/components/clipboard/ClipboardList.vue`: refresh after `lan-sync-updated`
- `src/main.ts`: handles `lan-sync-copy-text`

## Current Flow

Local copy flow:

1. Frontend listens to system clipboard changes.
2. `useClipboard.ts` calls the add-item flow.
3. Rust `ClipboardManager` persists and deduplicates the item.
4. `LanSyncService` broadcasts trusted-device sync messages.

Inbound sync flow:

1. `LanSyncService` receives a TCP sync message.
2. It validates protocol version, sender identity, trust state, and message id.
3. It calls `ClipboardManager` to persist the synced content.
4. On success it emits:
   - `lan-sync-copy-text`
   - `lan-sync-updated`
5. Frontend writes content back to the system clipboard and refreshes history.

## Source Semantics

Keep these source categories distinct:

- `ClipboardSource::LocalSystem`: true local system clipboard content, allowed to broadcast
- `ClipboardSource::InternalCopy`: app-internal clipboard restore, must not broadcast
- `ClipboardSource::LanSync`: inbound LAN content, must not rebroadcast

Do not collapse these into a single boolean flag.

## Current Protocol

Discovery and sync messages currently include:

- `Heartbeat`
- `PairRequest`
- `PairResponse`
- `SyncText`

Current sync payload requirements include:

- `device_id`
- `device_name`
- `tcp_port`
- `message_id`
- `created_at`
- `text`

Note:

- desktop local history already has `content_hash`
- current LAN wire payload does not yet send `hash`
- current inbound duplicate suppression is still based on `message_id`

## Frontend Events

- `lan-sync-status-changed`: online devices, pending requests, trusted devices, error state updates
- `lan-sync-updated`: refresh clipboard list after inbound sync persistence
- `lan-sync-copy-text`: write inbound synced text into the system clipboard

## Tauri Commands

- `get_lan_sync_status`
- `list_discovered_devices`
- `request_lan_pairing`
- `approve_lan_device`
- `reject_lan_device`
- `remove_trusted_lan_device`

## Current Capabilities

- one-to-many LAN broadcast
- text sync only
- manual first-time pairing approval
- trusted device list persistence
- background sync continues even when clipboard window is closed

## Current Constraints

- do not write directly into SQLite from sync service; always go through `ClipboardManager`
- do not write `lan-sync-copy-text` from multiple windows
- settings autosave can trigger repeated service restart; avoid pointless rebinding
- before adding new synced content types, define dedupe, loop prevention, and clipboard restore behavior first

## Known Limitations

- no end-to-end encryption
- no offline queue or retry queue
- no device grouping or selective broadcast
- discovery depends on LAN broadcast and may be unstable on complex networks
