# src/composables

Shared Vue composition functions (hooks).

## FILES

| File | Purpose |
|------|---------|
| useClipboard.ts | Clipboard monitoring, CRUD operations |
| useContentType.ts | Content type detection |
| useFileOperations.ts | File/folder open, reveal in explorer |
| useImageLoader.ts | Image loading with retry (5 attempts) |
| usePasteQueue.ts | Paste queue state (shopping cart mode) |
| useSettings.ts | Settings read/write via Tauri invoke |
| useSmartSearch.ts | Fuzzy search with pinyin support |
| useWindow.ts | Window visibility toggle |

## CONVENTIONS

- Named `use*.ts` (Vue 3 composition API pattern)
- All async Tauri calls wrapped in try-catch
- Return typed refs: `const items = ref<ClipboardItem[]>([])`
- Use `@/` path alias (maps to `src/`)

## WHERE TO LOOK

| Task | File |
|------|------|
| Add new clipboard type | useContentType.ts |
| Modify search behavior | useSmartSearch.ts |
| Window management | useWindow.ts |

## NOTES

- No test framework yet
- All state is component-local or Tauri-backed
