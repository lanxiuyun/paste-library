import type { PinnedSearch } from "@/types";
import { computed, ref, type Ref } from "vue";

const PINNED_SEARCH_STORAGE_KEY = "paste_library_pinned_searches";

export function usePinnedSearches(
  searchQuery: Ref<string>,
) {
  const pinnedSearches = ref<PinnedSearch[]>([]);

  const loadPinnedSearches = () => {
    try {
      const stored = localStorage.getItem(PINNED_SEARCH_STORAGE_KEY);
      if (stored) {
        pinnedSearches.value = JSON.parse(stored);
      }
    } catch {
      pinnedSearches.value = [];
    }
  };

  const savePinnedSearches = () => {
    try {
      localStorage.setItem(
        PINNED_SEARCH_STORAGE_KEY,
        JSON.stringify(pinnedSearches.value),
      );
    } catch {
      // 忽略存储错误
    }
  };

  const canPinCurrentSearch = computed(() => {
    const query = searchQuery.value.trim();
    if (!query) return false;
    return !pinnedSearches.value.some((ps) => ps.query === query);
  });

  const pinCurrentSearch = () => {
    const query = searchQuery.value.trim();
    if (!query) return;

    let label = query;
    if (query.length > 10) {
      label = query.slice(0, 10) + "...";
    }

    const newPinned: PinnedSearch = {
      id: `pinned_${Date.now()}`,
      label,
      query,
      created_at: Date.now(),
    };

    pinnedSearches.value.push(newPinned);
    savePinnedSearches();
    searchQuery.value = newPinned.query;
  };

  const unpinSearch = (id: string) => {
    const index = pinnedSearches.value.findIndex((ps) => ps.id === id);
    if (index > -1) {
      const removedQuery = pinnedSearches.value[index].query;
      pinnedSearches.value.splice(index, 1);
      savePinnedSearches();

      if (searchQuery.value.trim() === removedQuery) {
        searchQuery.value = "";
      }
    }
  };

  const reorderPinnedSearches = (fromIndex: number, toIndex: number) => {
    const item = pinnedSearches.value.splice(fromIndex, 1)[0];
    pinnedSearches.value.splice(toIndex, 0, item);
    savePinnedSearches();
  };

  return {
    pinnedSearches,
    canPinCurrentSearch,
    loadPinnedSearches,
    savePinnedSearches,
    pinCurrentSearch,
    unpinSearch,
    reorderPinnedSearches,
  };
}
