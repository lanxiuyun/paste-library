# src/components

Vue components (UI + settings).

## STRUCTURE

```
components/
├── ClipboardItem.vue    # Card for single clipboard entry
├── ClipboardList.vue    # Main list with tabs/search
├── ContextMenu.vue      # Right-click menu
├── DragHandle.vue       # Window drag region (36px)
├── DrawerEditor.vue     # Text/image preview modal
├── PasteQueuePanel.vue  # Batch paste panel
├── SmartSearch.vue      # Search input component
├── TagManager.vue       # Tag CRUD popup
├── clipboard/           # Clipboard-specific components
│   └── ...
└── settings/            # Settings panel components
    ├── components/      # Reusable setting widgets
    └── sections/        # Setting categories
```

## CONVENTIONS

- PascalCase in script, kebab-case in templates
- Props typed with `interface Props`
- Emit types: `defineEmits<{ action: [id: string] }>()`
- Scoped styles: `<style scoped>`
- Max ~200 lines per file

## WHERE TO LOOK

| Task | File |
|------|------|
| Add tab | ClipboardList.vue (tabs array) |
| Add setting | settings/sections/*.vue |
| Modify context menu | ContextMenu.vue |
| Item card styling | ClipboardItem.vue |

## NOTES

- Two root components: App.vue (settings), ClipboardView.vue (clipboard)
- URL-based routing in main.ts: `/clipboard` → ClipboardView, else → App
