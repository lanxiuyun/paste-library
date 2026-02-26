declare module 'vue-virtual-scroller' {
  import type { DefineComponent } from 'vue';

  interface RecycleScrollerProps {
    items: unknown[];
    itemSize: number;
    keyField?: string;
    direction?: 'vertical' | 'horizontal';
    buffer?: number;
  }

  interface RecycleScrollerMethods {
    scrollToItem(index: number, position?: 'start' | 'center' | 'end'): void;
    scrollToPosition(position: number): void;
  }

  interface DynamicScrollerProps {
    items: unknown[];
    minItemSize: number;
    keyField?: string;
    direction?: 'vertical' | 'horizontal';
    buffer?: number;
  }

  interface DynamicScrollerItemProps {
    item: unknown;
    active: boolean;
    sizeDependencies?: unknown[];
  }

  export const RecycleScroller: DefineComponent<RecycleScrollerProps, RecycleScrollerMethods>;
  export const DynamicScroller: DefineComponent<DynamicScrollerProps, RecycleScrollerMethods>;
  export const DynamicScrollerItem: DefineComponent<DynamicScrollerItemProps>;
}
