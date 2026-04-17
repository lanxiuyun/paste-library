# Android LAN Sync Spec

## Goal

Android is a lightweight LAN clipboard client, not a full desktop feature clone.

It should:

- record local clipboard history for text and images
- show a single searchable list
- keep the latest 100 untagged records locally
- keep tagged records long-term
- broadcast clipboard records to trusted devices on the same LAN
- broadcast tag changes to trusted devices on the same LAN
- ignore data created while devices were offline or outside the same LAN

## Scope

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

## Core Rules

1. Every record has a stable unique `hash` generated from content.
2. Text and image records both dedupe by `hash`.
3. Tags are attributes of a record, not part of the primary identity.
4. Only online LAN events are synchronized.
5. Untagged records keep only the latest 100 local entries.
6. Tagged records are retained long-term.
7. Tag changes made on one online device should converge on other online trusted devices.

## Record Types

Supported content types:

- text
- image

Suggested text fields:

- `hash`
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

- `hash`
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

## Local Storage Rules

- local copy creates or refreshes a record by `hash`
- if a record already exists, update `last_seen_at`
- if no record exists, insert a new record
- untagged records participate in latest-100 cleanup
- tagged records do not participate in latest-100 cleanup
- cleanup only removes untagged records older than the newest 100 local records

## Sync Model

Sync applies only when devices are:

- on the same LAN
- already paired and trusted
- currently online

There are two event types.

Clipboard events:

- when a device copies text or image, it broadcasts the record
- receivers upsert by `hash`
- no history replay
- no catch-up after reconnect

Tag events:

- add tag to record
- remove tag from record
- receivers update tags by `hash`
- if the target record does not exist locally, ignore the event

## Dedupe Rules

- text dedupes by `hash`
- image dedupes by `hash`
- only one local record exists for one `hash`
- repeated receipt or repeated local copy updates `last_seen_at`

Assumption:

- `hash` is content-derived and stable across devices

## Tag Rules

- one record can have multiple tags
- tags sync across devices
- tag names should be normalized
- normalize by `trim`
- compare tags case-insensitively
- empty tags are invalid
- removing the final tag returns the record to untagged cleanup behavior

## List Rules

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

## Search Rules

Search covers the full local database.

v1 supports:

- text content search
- tag search
- image records searchable by tag

v1 does not require:

- advanced query syntax
- device filters

## Device Management

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

## Image Support

Image support is included in v1, but only at a basic level:

- detect clipboard image
- persist a local copy
- generate thumbnail
- compute stable `hash`
- broadcast image payload or equivalent recoverable data
- store received image locally

## Failure and Edge Behavior

- if receive succeeds but local persistence fails, log the error and continue
- if a tag event targets a missing record, ignore it
- offline devices do not receive replays
- duplicate broadcasts must be idempotent via `hash`
- internal clipboard restore must not create sync loops

## Loop Prevention

At minimum, record source should distinguish:

- real local system copy
- internal app restore-to-clipboard
- LAN-synced content

Rules:

- internal restore-to-clipboard must not rebroadcast
- LAN-received clipboard writes must not rebroadcast
- only genuine new local copies can trigger broadcast

## Android Data Model v1

### Storage Choice

Recommended choice:

- Room over raw SQLite

### Tables

#### `clipboard_records`

- `hash TEXT PRIMARY KEY`
- `content_type TEXT NOT NULL`
- `text_content TEXT`
- `image_hash TEXT`
- `image_path TEXT`
- `thumbnail_path TEXT`
- `width INTEGER`
- `height INTEGER`
- `source_device_id TEXT NOT NULL`
- `source_device_name TEXT NOT NULL`
- `is_remote INTEGER NOT NULL DEFAULT 0`
- `created_at INTEGER NOT NULL`
- `updated_at INTEGER NOT NULL`
- `last_seen_at INTEGER NOT NULL`

#### `record_tags`

- `hash TEXT NOT NULL`
- `tag TEXT NOT NULL`
- `created_at INTEGER NOT NULL`
- primary key: `hash, tag`

#### `trusted_devices`

- `device_id TEXT PRIMARY KEY`
- `device_name TEXT NOT NULL`
- `last_known_address TEXT NOT NULL`
- `tcp_port INTEGER NOT NULL`
- `last_seen_at INTEGER NOT NULL`
- `is_trusted INTEGER NOT NULL DEFAULT 1`

#### `app_settings`

- `device_name`
- `lan_sync_enabled`
- `auto_discovery_enabled`

## Normalization Rules

Text:

- use the desktop-provided `hash`
- do not recompute a different identity locally if remote payload already has one

Tag:

- `trim`
- lowercase for compare and storage
- reject empty string after trim

Image:

- use the payload `hash` as the record identity
- `image_hash` may equal `hash`, but keep both fields only if protocol needs both

## Android Sync Event Model v1

### Clipboard Event

- `kind`
- `protocol_version`
- `device_id`
- `device_name`
- `tcp_port`
- `hash`
- `content_type`
- `created_at`
- `text`
- `image_hash`
- `image_bytes` or transport equivalent
- `width`
- `height`

### Tag Event

- `kind = tag_update`
- `protocol_version`
- `device_id`
- `device_name`
- `hash`
- `operation`
- `tag`
- `created_at`

## Delivery Plan

1. Foundation
2. Local Clipboard MVP
3. Pairing Integration
4. Text Sync
5. Tag Sync
6. Image Local Support
7. Image Sync
