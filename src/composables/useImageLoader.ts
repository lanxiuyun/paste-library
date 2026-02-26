import { ref } from 'vue';
import { convertFileSrc } from '@tauri-apps/api/core';

export interface ImageLoadState {
  status: 'idle' | 'loading' | 'success' | 'error';
  retries: number;
  error?: string;
}

const MAX_RETRIES = 5;
const RETRY_DELAY = 300;

export function useImageLoader() {
  const imageStates = ref<Map<string, ImageLoadState>>(new Map());

  const getImageState = (src: string): ImageLoadState => {
    return imageStates.value.get(src) || { status: 'idle', retries: 0 };
  };

  const loadImage = async (src: string, maxRetries: number = MAX_RETRIES): Promise<boolean> => {
    // 如果已经加载成功，直接返回
    const currentState = getImageState(src);
    if (currentState.status === 'success') {
      return true;
    }

    // 更新状态为加载中
    imageStates.value.set(src, { status: 'loading', retries: currentState.retries });

    try {
      // 使用 convertFileSrc 处理本地文件路径
      const imageSrc = src.startsWith('http') ? src : convertFileSrc(src);
      
      const img = new Image();
      
      await new Promise<void>((resolve, reject) => {
        img.onload = () => resolve();
        img.onerror = () => reject(new Error('Failed to load image'));
        img.src = imageSrc;
      });

      imageStates.value.set(src, { status: 'success', retries: 0 });
      return true;
    } catch (error) {
      const retries = currentState.retries + 1;
      
      if (retries < maxRetries) {
        imageStates.value.set(src, { status: 'loading', retries });
        // 延迟重试
        await new Promise(resolve => setTimeout(resolve, RETRY_DELAY));
        return loadImage(src, maxRetries);
      }

      imageStates.value.set(src, { 
        status: 'error', 
        retries,
        error: error instanceof Error ? error.message : 'Unknown error'
      });
      return false;
    }
  };

  const retryLoad = async (src: string): Promise<boolean> => {
    // 重置状态并重试
    imageStates.value.set(src, { status: 'idle', retries: 0 });
    return loadImage(src);
  };

  const resetState = (src?: string) => {
    if (src) {
      imageStates.value.delete(src);
    } else {
      imageStates.value.clear();
    }
  };

  return {
    imageStates,
    getImageState,
    loadImage,
    retryLoad,
    resetState,
  };
}
