import type { Ref } from "vue";

export interface ScrollHandlerOptions {
  isSearching: Ref<boolean>;
  searchHasMore: Ref<boolean>;
  loadMoreResults: () => Promise<void>;
  scrollerElement?: Ref<HTMLElement | null>;
}

export function useScrollHandler(options: ScrollHandlerOptions) {
  const { isSearching, searchHasMore, loadMoreResults } = options;

  let scrollThrottleTimer: number | null = null;

  const handleScroll = (event?: Event) => {
    if (scrollThrottleTimer) return;

    scrollThrottleTimer = window.setTimeout(() => {
      scrollThrottleTimer = null;

      const scroller =
        options.scrollerElement?.value ||
        (event?.target as HTMLElement) ||
        null;
      if (!scroller) return;

      const scrollTop = scroller.scrollTop;
      const scrollHeight = scroller.scrollHeight;
      const clientHeight = scroller.clientHeight;

      if (isSearching.value || !searchHasMore.value) {
        return;
      }

      // 距离底部 200px 时触发加载更多
      if (scrollTop + clientHeight >= scrollHeight - 200) {
        loadMoreResults();
      }
    }, 100);
  };

  const cleanup = () => {
    if (scrollThrottleTimer) {
      clearTimeout(scrollThrottleTimer);
      scrollThrottleTimer = null;
    }
  };

  return {
    handleScroll,
    cleanup,
  };
}
