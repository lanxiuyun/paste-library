# Android LAN Sync MVP

## Immediate Goal

Before search, tags, history cleanup, or image support, the first working milestone should be:

1. desktop copies text and broadcasts it, Android receives it
2. Android copies text and broadcasts it, desktop receives it
3. Android shows received and sent text in a list

## Explicitly Out Of Scope

- search
- latest-100 cleanup
- tags
- image support
- full Room database
- polished settings flow
- offline replay
- full history management

## Supported Content

Only text.

## Minimum Record Shape

Android list items only need:

- `hash`
- `text`
- `source_device_id`
- `source_device_name`
- `created_at`
- `direction`

Suggested direction values:

- `received`
- `sent`

This can be in-memory first.

## Minimum Sync Payload

Use one event type only:

- `sync_text`

Suggested required fields:

- `kind`
- `protocol_version`
- `device_id`
- `device_name`
- `tcp_port`
- `hash`
- `created_at`
- `text`

Assumption:

- desktop already has or can produce a stable content-derived `hash`

## MVP Behavior

Desktop to Android:

1. user copies text on desktop
2. desktop broadcasts `sync_text`
3. Android receives payload
4. Android dedupes by `hash`
5. Android inserts the item into the visible list
6. Android keeps the received text ready for explicit paste into the currently focused input field

Android to Desktop:

1. user copies text on Android
2. Android detects a real local clipboard change
3. Android generates or reuses `hash`
4. Android broadcasts `sync_text`
5. desktop receives payload

## Android Interaction Constraint

For this repository, do not assume Android can stably restore remote text into the system clipboard while the app is backgrounded.

Preferred Android interaction model:

1. receive and cache remote text in the foreground service/controller
2. show synced records in the app list
3. when the user focuses an input field in another app, expose an accessibility overlay entry
4. when the user taps a record from that overlay, inject it into the focused field

Clipboard write-back can still be attempted as a fallback, but it is not the primary UX contract.

## Deduping Rule

- same `hash` means same record
- receiving an existing `hash` must not create a new row
- receiving an existing `hash` should move the existing row to the top

## Loop Prevention

Mandatory rule:

- only real local clipboard copies can trigger outbound broadcast
- remote LAN clipboard write-back must not trigger broadcast

Record source must distinguish:

- real local clipboard copy
- remote LAN write-back

## Desktop Side Changes Needed

Primary files likely involved:

- `src-tauri/src/sync_service.rs`
- `src-tauri/src/clipboard.rs`
- `src-tauri/src/models.rs`
- `src/composables/useClipboard.ts`
- `src/composables/useLanSyncClipboardBridge.ts`
- `src/main.ts`

Desktop-side work:

1. ensure text sync payload includes `hash`
2. ensure outbound text sync uses the same `hash` that Android will consume
3. keep source tagging intact so LAN-received text is not rebroadcast
4. keep inbound Android text going through existing clipboard/history flow if that flow already prevents loops

## Android Side Changes Needed

Primary current files:

- `android-lan-sync/app/src/main/java/com/pastelibrary/lansync/MainActivity.kt`
- `android-lan-sync/app/src/main/res/layout/activity_main.xml`
- `android-lan-sync/app/src/main/AndroidManifest.xml`

Android-side work:

1. extend protocol parsing to accept `hash` and `sync_text`
2. add a simple in-memory list for synced records
3. render the list in the current UI
4. listen to Android clipboard text changes
5. when a real local copy occurs, broadcast `sync_text`
6. when remote text is received, mark it so clipboard write-back does not rebroadcast

## Suggested Implementation Order

### Step 1: Protocol Alignment

- add `hash` to the desktop outbound payload if missing
- update Android parsing and serialization to the same fields

### Step 2: Android Receive List

- add a text message list in Android UI
- insert a row on each new inbound `sync_text`
- dedupe repeated `hash`
- if the same `hash` arrives again, move the existing item to the top

### Step 3: Android Local Clipboard Send

- add Android clipboard listener
- detect actual local text copy
- broadcast `sync_text`
- add local sent item into the same list

### Step 4: Loop Prevention

- mark remote clipboard writes
- suppress rebroadcast after remote write-back
- dedupe repeated `hash` on both sides

### Step 5: Focused Input Injection

- track the currently focused editable node via `AccessibilityService`
- show an overlay entry when an editable field is focused
- on user tap, inject the selected synced text with `ACTION_SET_TEXT`
- if `ACTION_SET_TEXT` fails, fall back to `ACTION_PASTE`

## Fastest Safe First Cut

Implement exactly this:

1. desktop outbound `hash` in `sync_text`
2. Android inbound text parsing
3. Android visible text list
4. Android clipboard listener for local text
5. Android outbound `sync_text`
6. loop suppression
7. accessibility overlay entry for explicit paste into the focused input
