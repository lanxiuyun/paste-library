import { ref, computed } from 'vue';
import type { ClipboardItem } from '@/types';

export type SeparatorType = 'newline' | 'none' | 'custom';

const queue = ref<ClipboardItem[]>([]);
const separator = ref<SeparatorType>('newline');
const customSeparator = ref('');
const autoClear = ref(false);

export function usePasteQueue() {
  const addToQueue = (item: ClipboardItem) => {
    // Check if item already exists in queue
    const exists = queue.value.some(q => q.id === item.id);
    if (!exists) {
      queue.value.push(item);
    }
  };

  const removeFromQueue = (id: number) => {
    const index = queue.value.findIndex(item => item.id === id);
    if (index > -1) {
      queue.value.splice(index, 1);
    }
  };

  const moveItem = (fromIndex: number, toIndex: number) => {
    if (toIndex < 0 || toIndex >= queue.value.length) return;
    const item = queue.value.splice(fromIndex, 1)[0];
    queue.value.splice(toIndex, 0, item);
  };

  const clearQueue = () => {
    queue.value = [];
  };

  const getMergedContent = () => {
    const sep = separator.value === 'custom' 
      ? customSeparator.value 
      : separator.value === 'newline' 
        ? '\n' 
        : '';
    
    return queue.value.map(item => {
      // Strip HTML for text content
      if (item.content_type === 'html') {
        return item.content.replace(/<[^>]*>/g, '');
      }
      return item.content;
    }).join(sep);
  };

  const queueCount = computed(() => queue.value.length);

  const isInQueue = (id: number) => {
    return queue.value.some(item => item.id === id);
  };

  return {
    queue,
    separator,
    customSeparator,
    autoClear,
    queueCount,
    addToQueue,
    removeFromQueue,
    moveItem,
    clearQueue,
    getMergedContent,
    isInQueue,
  };
}
