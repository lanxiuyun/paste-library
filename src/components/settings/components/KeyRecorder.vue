<template>
  <div
    class="hotkey-recorder"
    :class="{ recording: isRecording }"
    @click="toggleRecording"
  >
    <template v-if="isRecording">
      <span class="recording-text">请按下快捷键...</span>
    </template>
    <template v-else-if="displayKeys.length > 0">
      <span v-for="(key, i) in displayKeys" :key="i" class="key-badge">{{ key }}</span>
      <span v-if="isModifierOnly" class="key-suffix">1~9</span>
    </template>
    <template v-else-if="isModifierOnly && modelValue === 'none'">
      <span class="key-hint">直接按数字键</span>
    </template>
    <template v-else>
      <span class="key-hint">{{ emptyText }}</span>
    </template>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onUnmounted } from 'vue';

interface Props {
  modelValue: string;
  size?: 'default' | 'small';
  recordingText?: string;
  emptyText?: string;
  isModifierOnly?: boolean;
}

const props = withDefaults(defineProps<Props>(), {
  size: 'default',
  recordingText: '请按下快捷键...',
  emptyText: '点击录制快捷键',
  isModifierOnly: false,
});

const emit = defineEmits<{
  'update:modelValue': [value: string];
  'record': [value: string];
}>();

const isRecording = ref(false);

const displayKeys = computed<string[]>(() => {
  if (!props.modelValue || props.modelValue === 'none') return [];
  if (props.isModifierOnly) {
    return props.modelValue.split('+').map(s => s.charAt(0).toUpperCase() + s.slice(1));
  }
  return props.modelValue.split('+');
});

const toggleRecording = () => {
  if (isRecording.value) {
    stopRecording();
  } else {
    startRecording();
  }
};

const startRecording = () => {
  isRecording.value = true;
  window.addEventListener('keydown', handleKeyRecord, { capture: true });
};

const stopRecording = () => {
  isRecording.value = false;
  window.removeEventListener('keydown', handleKeyRecord, { capture: true });
};

const handleKeyRecord = (e: KeyboardEvent) => {
  e.preventDefault();
  e.stopPropagation();

  const modifiers: string[] = [];
  if (e.ctrlKey) modifiers.push(props.isModifierOnly ? 'ctrl' : 'Ctrl');
  if (e.altKey) modifiers.push(props.isModifierOnly ? 'alt' : 'Alt');
  if (e.shiftKey) modifiers.push(props.isModifierOnly ? 'shift' : 'Shift');
  if (e.metaKey) modifiers.push(props.isModifierOnly ? 'meta' : 'Win');

  let key = e.key;

  if (key === 'Control' || key === 'Alt' || key === 'Shift' || key === 'Meta') {
    return;
  }

  if (props.isModifierOnly) {
    const keyLower = key.toLowerCase();
    if (keyLower >= '1' && keyLower <= '9') {
      const value = modifiers.length === 0 ? 'none' : modifiers.join('+');
      emit('update:modelValue', value);
      emit('record', value);
      stopRecording();
      return;
    }

    const value = modifiers.length === 0 ? 'none' : modifiers.join('+');
    emit('update:modelValue', value);
    emit('record', value);
    stopRecording();
  } else {
    if (key === ' ') key = 'Space';
    if (key.length === 1) key = key.toUpperCase();

    const hotkeyParts = [...modifiers, key];
    const hotkeyString = hotkeyParts.join('+');

    emit('update:modelValue', hotkeyString);
    emit('record', hotkeyString);
    stopRecording();
  }
};

onUnmounted(() => {
  if (isRecording.value) {
    stopRecording();
  }
});
</script>

<style scoped>
.hotkey-recorder {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  padding: 4px 8px;
  border-radius: 6px;
  cursor: pointer;
  transition: background 0.15s;
  user-select: none;
}

.hotkey-recorder:hover {
  background: #f0f0f0;
}

.hotkey-recorder.recording {
  background: #fff7e6;
}

.key-badge {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  min-width: 28px;
  height: 28px;
  padding: 0 10px;
  border: 1px solid #d0d0d0;
  border-radius: 6px;
  background: #fff;
  font-family: ui-monospace, SFMono-Regular, 'SF Mono', Menlo, Consolas, monospace;
  font-size: 13px;
  font-weight: 500;
  color: #262626;
  box-shadow: 0 1px 2px rgba(0, 0, 0, 0.04);
}

.key-suffix {
  font-size: 13px;
  font-weight: 500;
  color: #8c8c8c;
  margin-left: 2px;
}

.key-hint {
  font-size: 13px;
  color: #bfbfbf;
  padding: 4px 0;
}

.recording-text {
  font-size: 13px;
  color: #fa8c16;
  padding: 4px 0;
  animation: blink 1s ease-in-out infinite;
}

@keyframes blink {
  0%, 100% { opacity: 1; }
  50% { opacity: 0.5; }
}
</style>
