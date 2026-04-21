# Android LAN Sync MVP

## Immediate Goal

Android MVP is a text-only LAN sync client with explicit user-triggered paste.

Before search, tags, history cleanup, image support, or Room persistence, the first working milestone should be:

1. desktop copies text and broadcasts it, Android receives it
2. Android copies text and broadcasts it, desktop receives it
3. Android shows received and sent text in a list
4. when the user focuses an input field in another app, Android shows a floating paste entry
5. when the user taps the floating entry, Android shows recent synced text records
6. when the user taps one record, Android injects it into the focused input field

## Current Repository Status

The repository already contains a partial Android MVP implementation:

- device discovery and pairing exist
- trusted-device-only text sync exists
- Android has an in-memory synced text list
- Android already has an `AccessibilityService` overlay entry and record picker
- remote receive now keeps text for explicit paste instead of treating background clipboard restore as the primary path
- overlay injection now tries direct focused-node insertion first and only uses clipboard fallback when needed

But the implementation is not fully aligned with the intended Android contract yet:

- synced records are still in-memory only
- device identity is not yet stable across app restarts
- overlay behavior still needs real-device validation across different apps and Android ROMs

The next development work should align the implementation to the explicit paste contract first, then stabilize persistence.

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

This can be in-memory first, but persistence is the first follow-up after the explicit paste flow is aligned.

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
6. Android keeps the received text available for explicit paste into the currently focused input field
7. Android does not rely on automatic background clipboard restore as the primary UX

Android to Desktop:

1. user copies text on Android
2. Android detects a real local clipboard change
3. Android generates or reuses `hash`
4. Android broadcasts `sync_text`
5. desktop receives payload

Focused input paste flow:

1. user focuses an editable field in another app
2. `AccessibilityService` detects the focused editable node
3. Android shows a lightweight floating paste entry
4. user taps the floating entry
5. Android shows recent synced text records
6. user taps one record
7. Android injects the text into the focused field with `ACTION_SET_TEXT`
8. if `ACTION_SET_TEXT` fails, Android falls back to `ACTION_PASTE`
9. clipboard-based fallback must not rebroadcast the injected text

## Android Interaction Constraint

For this repository, do not assume Android can stably restore remote text into the system clipboard while the app is backgrounded.

Preferred Android interaction model:

1. receive and cache remote text in the foreground service/controller
2. show synced records in the app list
3. when the user focuses an input field in another app, expose an accessibility overlay entry
4. when the user taps a record from that overlay, inject it into the focused field

Clipboard write-back can still be used as a fallback inside the explicit accessibility paste flow, but it is not the primary UX contract.

## Deduping Rule

- same `hash` means same record
- receiving an existing `hash` must not create a new row
- receiving an existing `hash` should move the existing row to the top

## Loop Prevention

Mandatory rule:

- only real local clipboard copies can trigger outbound broadcast
- remote LAN clipboard write-back must not trigger broadcast
- overlay-triggered clipboard fallback must not trigger broadcast

Record source must distinguish:

- real local clipboard copy
- remote LAN write-back
- internal overlay fallback write

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
- `android-lan-sync/app/src/main/java/com/pastelibrary/lansync/LanSyncController.kt`
- `android-lan-sync/app/src/main/java/com/pastelibrary/lansync/LanSyncForegroundService.kt`
- `android-lan-sync/app/src/main/java/com/pastelibrary/lansync/ClipboardMonitorAccessibilityService.kt`
- `android-lan-sync/app/src/main/res/layout/activity_main.xml`
- `android-lan-sync/app/src/main/AndroidManifest.xml`

Android-side work:

1. keep protocol parsing aligned to `sync_text` with `hash`
2. keep the simple synced text list and dedupe behavior
3. make the accessibility overlay flow the primary remote paste path
4. remove automatic remote clipboard restore as the primary behavior
5. make clipboard fallback suppression explicit inside the overlay paste flow
6. keep local clipboard send working for real local copies only
7. stabilize identity and basic persistence after the explicit paste flow is aligned

## Suggested Implementation Order

### Step 1: Protocol Alignment

- add `hash` to the desktop outbound payload if missing
- keep Android parsing and serialization aligned to the same fields

### Step 2: Explicit Paste Contract Alignment

- stop automatically writing remote text into the Android clipboard on receive
- keep received text in the synced record list for explicit user-triggered paste
- when the user taps a record in the overlay, try `ACTION_SET_TEXT` first
- only use clipboard fallback when direct injection fails
- suppress rebroadcast for overlay-triggered clipboard fallback writes

### Step 3: Android Receive List

- keep the visible text list in the Android UI
- insert a row on each new inbound `sync_text`
- dedupe repeated `hash`
- if the same `hash` arrives again, move the existing item to the top

### Step 4: Android Local Clipboard Send

- keep the Android clipboard listener
- detect actual local text copy
- broadcast `sync_text`
- add local sent item into the same list

### Step 5: Persistence and Stable Identity

- persist a stable Android `device_id`
- persist trusted devices
- persist the recent synced text list

### Step 6: Hardening

- verify loop prevention across remote receive, overlay fallback, and local clipboard send
- reduce duplicate clipboard listener responsibility between foreground activity and accessibility service if needed
- continue tightening accessibility-event filtering if device-specific overlay flicker appears during testing

## Testing Notes

For Android manual testing, prefer filtering logs down to the app's own LAN sync tag:

- Android Studio Logcat: `package:com.pastelibrary.lansync tag:LanSync`
- adb: `adb logcat LanSync:D *:S`

If the floating `Paste` entry flickers, first check whether the service is processing repeated accessibility events with `package=null` or other overlay-generated noise.

## Fastest Safe First Cut

Implement exactly this first:

1. desktop outbound `hash` in `sync_text`
2. Android inbound text parsing
3. Android visible text list
4. Android clipboard listener for local text
5. Android outbound `sync_text`
6. explicit accessibility overlay entry for focused input
7. record picker overlay
8. direct focused-node injection
9. clipboard fallback only when injection fails
10. loop suppression for fallback clipboard writes
