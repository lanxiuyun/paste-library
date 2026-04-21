# Android LAN Sync Development Plan

## Goal

Align the current Android implementation to the MVP contract, then stabilize it enough for iterative testing.

This plan intentionally prioritizes:

1. correct user interaction contract
2. loop-safe sync behavior
3. stable local state

It does not attempt to jump to the long-term Android spec.

## Current Baseline

Already present in the codebase:

- UDP discovery
- TCP text sync
- manual pairing and trusted device management
- in-memory synced text list with `hash` dedupe
- foreground service
- accessibility overlay entry and record picker
- focused-input text injection attempt

Main gaps:

- no stable persisted device identity
- no persisted synced records or trusted devices
- clipboard observation responsibility is split across multiple places

## Branch Progress

Already implemented in this branch:

1. remote receive no longer auto-restores into the Android clipboard as the primary path
2. overlay record selection now tries direct focused-node insertion first
3. clipboard fallback writes are marked as internal so they do not rebroadcast
4. notification copy now describes explicit paste instead of automatic clipboard restore
5. accessibility overlay event handling now ignores null-package and self-package noise to reduce flicker

## Phase 1: Align To Explicit Paste Contract

### Objective

Make Android remote text handling match the intended UX:

- receive
- list
- explicit user-triggered paste

### Changes

1. stop automatic remote clipboard restore on receive
2. keep remote text in synced records for later explicit paste
3. make overlay click try `ACTION_SET_TEXT` first
4. only write to clipboard for `ACTION_PASTE` fallback
5. mark internal fallback clipboard writes so they do not rebroadcast
6. update notification and UI wording to match the new behavior

### Files

- `android-lan-sync/app/src/main/java/com/pastelibrary/lansync/LanSyncController.kt`
- `android-lan-sync/app/src/main/java/com/pastelibrary/lansync/ClipboardMonitorAccessibilityService.kt`
- `android-lan-sync/app/src/main/java/com/pastelibrary/lansync/LanSyncForegroundService.kt`
- `android-lan-sync/app/src/main/java/com/pastelibrary/lansync/MainActivity.kt`

### Verification

1. desktop sends text to Android
2. Android list updates
3. Android clipboard does not change immediately just because remote text arrived
4. focus another app input
5. overlay button appears
6. open overlay list and tap a record
7. text is inserted into the focused field
8. no outbound rebroadcast is caused by the fallback clipboard write path

Current troubleshooting note:

- if the floating `Paste` entry flickers, check whether the service is receiving repeated `package=null` accessibility events
- the current implementation now ignores null-package and self-package events and no longer reacts to noisy click/window-change event types

Recommended log filters during Phase 1 testing:

- Android Studio Logcat: `package:com.pastelibrary.lansync tag:LanSync`
- adb: `adb logcat LanSync:D *:S`

## Phase 2: Stabilize Clipboard Observation

### Objective

Reduce ambiguity around which component is allowed to observe and forward local clipboard changes.

### Changes

1. decide whether foreground `MainActivity` clipboard listening remains necessary
2. keep one clear primary path for background-capable local clipboard monitoring
3. ensure duplicate observation does not create duplicate list updates or duplicate outbound sync

### Files

- `android-lan-sync/app/src/main/java/com/pastelibrary/lansync/MainActivity.kt`
- `android-lan-sync/app/src/main/java/com/pastelibrary/lansync/ClipboardMonitorAccessibilityService.kt`
- `android-lan-sync/app/src/main/java/com/pastelibrary/lansync/LanSyncController.kt`

### Verification

1. copy text locally with the app foregrounded
2. copy text locally with the app backgrounded and accessibility enabled
3. only one outbound sync is generated per local copy

## Phase 3: Persist Stable Identity And Minimal State

### Objective

Make the Android client survive restart without losing identity and core sync state.

### Changes

1. persist stable `device_id`
2. persist `device_name`
3. persist trusted devices
4. persist recent synced text records

Recommended first cut:

- `SharedPreferences` or small JSON persistence for MVP
- do not introduce Room yet unless the persistence surface grows materially

### Files

- `android-lan-sync/app/src/main/java/com/pastelibrary/lansync/LanSyncController.kt`
- new storage helper file under `android-lan-sync/app/src/main/java/com/pastelibrary/lansync/`
- `android-lan-sync/app/src/main/java/com/pastelibrary/lansync/MainActivity.kt`

### Verification

1. pair with a desktop device
2. sync some text
3. restart Android app or service
4. device identity is unchanged
5. trusted device state remains
6. recent records remain visible

## Phase 4: Harden Overlay Record Picker

### Objective

Make the overlay record picker reliable enough for daily manual testing.

### Changes

1. improve record row formatting
2. make empty state clearer
3. decide whether successful insert closes the panel or keeps it open
4. show insertion failure feedback
5. keep the focused-node resolution resilient across app switches
6. continue reducing overlay jitter caused by noisy accessibility event streams on device-specific ROMs

### Files

- `android-lan-sync/app/src/main/java/com/pastelibrary/lansync/ClipboardMonitorAccessibilityService.kt`

### Verification

1. test with multiple target apps
2. verify both `ACTION_SET_TEXT` and `ACTION_PASTE` fallback paths
3. confirm panel behavior is predictable after repeated inserts

## Implementation Order For This Branch

Do these in order:

1. Phase 1
2. Phase 2 only if Phase 1 reveals duplicate-observation issues
3. Phase 3
4. Phase 4 polish

## Success Criteria

This development pass is successful when all of the following are true:

1. Android no longer depends on automatic remote clipboard restore as the main remote paste behavior
2. the overlay flow is the primary path for pasting synced text into another app
3. overlay clipboard fallback does not create sync loops
4. the code and MVP documentation describe the same behavior
