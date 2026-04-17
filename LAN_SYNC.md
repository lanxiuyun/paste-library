# LAN Sync Guide

## Status

- `android-lan-sync/` is the current Android app in this repository.
- Treat it as the Android implementation for LAN sync work, not as a disposable demo folder.
- When a task is about Android LAN sync, inspect `android-lan-sync/` first.

## Desktop LAN Sync Overview

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

## Current Desktop Flow

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

## Android Spec v1

### Goal

Android is a lightweight LAN clipboard client, not a full desktop feature clone.

It should:

- record local clipboard history for text and images
- show a single searchable list
- keep the latest 100 untagged records locally
- keep tagged records long-term
- broadcast clipboard records to trusted devices on the same LAN
- broadcast tag changes to trusted devices on the same LAN
- ignore data created while devices were offline or outside the same LAN

### Scope

Included in v1:

- local clipboard capture
- text and image support
- single mixed history list
- search by text and tag
- tag add/remove
- LAN device discovery and pairing
- online LAN broadcast sync

Not included in v1:

- offline history backfill
- full multi-device history reconciliation
- cloud sync
- file clipboard sync
- OCR
- advanced filters
- batch actions

### Core Rules

1. Every record has a stable unique `hash_id` generated from content.
2. Text and image records both dedupe by `hash_id`.
3. Tags are attributes of a record, not part of the primary identity.
4. Only online LAN events are synchronized.
5. Untagged records keep only the latest 100 local entries.
6. Tagged records are retained long-term.
7. Tag changes made on one online device should converge on other online trusted devices.

### Record Types

Supported content types:

- text
- image

Suggested text fields:

- `hash_id`
- `content_type = text`
- `text_content`
- `tags`
- `created_at`
- `updated_at`
- `last_seen_at`
- `source_device_id`
- `source_device_name`
- `is_remote`

Suggested image fields:

- `hash_id`
- `content_type = image`
- `image_hash`
- `image_path`
- `thumbnail_path`
- `width`
- `height`
- `tags`
- `created_at`
- `updated_at`
- `last_seen_at`
- `source_device_id`
- `source_device_name`
- `is_remote`

### Local Storage Rules

- local copy creates or refreshes a record by `hash_id`
- if a record already exists, update `last_seen_at`
- if no record exists, insert a new record
- untagged records participate in latest-100 cleanup
- tagged records do not participate in latest-100 cleanup
- cleanup only removes untagged records older than the newest 100 local records

### Sync Model

Sync applies only when devices are:

- on the same LAN
- already paired and trusted
- currently online

There are two event types.

Clipboard events:

- when a device copies text or image, it broadcasts the record
- receivers upsert by `hash_id`
- no history replay
- no catch-up after reconnect

Tag events:

- add tag to record
- remove tag from record
- receivers update tags by `hash_id`
- if the target record does not exist locally, ignore the event

### Dedupe Rules

- text dedupes by `hash_id`
- image dedupes by `hash_id`
- only one local record exists for one `hash_id`
- repeated receipt or repeated local copy updates `last_seen_at`

Assumption:

- `hash_id` is content-derived and stable across devices

### Tag Rules

- one record can have multiple tags
- tags sync across devices
- tag names should be normalized
- normalize by `trim`
- compare tags case-insensitively
- empty tags are invalid
- removing the final tag returns the record to untagged cleanup behavior

### List Rules

Android uses a single mixed list.

The list includes:

- local records
- records received from other online devices
- all tagged retained records

Default sort:

- `last_seen_at` descending

Each row should show:

- content preview
- content type
- source device
- time
- tags
- image thumbnail when applicable

### Search Rules

Search covers the full local database.

v1 supports:

- text content search
- tag search
- image records searchable by tag

v1 does not require:

- advanced query syntax
- device filters

### Device Management

Android should keep and productize the existing `android-lan-sync/` pairing features:

- device discovery
- request pairing
- approve or reject pairing
- trusted device management
- online status display
- broadcast only to trusted devices

Settings should include:

- device name
- LAN sync switch
- auto-discovery switch
- discovered devices
- pending pairing requests
- trusted devices

### Image Support

Image support is included in v1, but only at a basic level:

- detect clipboard image
- persist a local copy
- generate thumbnail
- compute stable `hash_id`
- broadcast image payload or equivalent recoverable data
- store received image locally

Known cost:

- image sync is more expensive than text sync
- local cache cleanup will be needed
- transfer failures and size limits should be expected

### Failure and Edge Behavior

- if receive succeeds but local persistence fails, log the error and continue
- if a tag event targets a missing record, ignore it
- offline devices do not receive replays
- duplicate broadcasts must be idempotent via `hash_id`
- internal clipboard restore must not create sync loops

### Loop Prevention

At minimum, record source should distinguish:

- real local system copy
- internal app restore-to-clipboard
- LAN-synced content

Rules:

- internal restore-to-clipboard must not rebroadcast
- LAN-received clipboard writes must not rebroadcast
- only genuine new local copies can trigger broadcast

### Suggested Screens

1. Main list screen
- search box
- mixed history list
- text and image preview
- tag actions

2. Device or settings screen
- LAN sync switch
- auto-discovery switch
- pairing management
- trusted device management

3. Record detail screen or bottom sheet
- full text
- image preview
- tag editing
- delete action

### MVP Success Criteria

Android v1 is considered successful if:

1. it captures local text and image clipboard content
2. it stores and shows recent history locally
3. untagged records are limited to the latest 100
4. tagged records are retained long-term
5. phone and desktop can exchange copied content in real time on the same LAN after pairing
6. tag changes made on phone propagate to desktop while both are online
7. search works for text and tags
8. sync loops do not occur

### Suggested Delivery Order

1. define Android local database and models
2. integrate Android clipboard capture and restore
3. build local list, search, and tag actions
4. merge `android-lan-sync/` discovery and pairing into a formal settings flow
5. implement text broadcast sync
6. implement tag broadcast sync
7. implement image sync last
