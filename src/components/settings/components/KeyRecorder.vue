<template>
  <button
    class="hotkey-record-btn"
    :class="{ 'recording': isRecording, 'has-value': modelValue && !isRecording, 'small': size === 'small' }"
    @click="toggleRecording"
  >
    {{ displayText }}
  </button>
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
  emptyText: '点击录制',
  isModifierOnly: false,
});

const emit = defineEmits<{
  'update:modelValue': [value: string];
  'record': [value: string];
}>();

const isRecording = ref(false);

const displayText = computed(() => {
  if (isRecording.value) {
    return props.recordingText;
  }
  if (props.modelValue) {
    return props.isModifierOnly ? formatModifierShortcut(props.modelValue) : props.modelValue;
  }
  return props.emptyText;
});

const formatModifierShortcut = (shortcut: string): string => {
  if (!shortcut || shortcut === 'none') return '直接按数字键';
  return shortcut.split('+').map(s => s.charAt(0).toUpperCase() + s.slice(1)).join('+') + '+数字键';
};

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

  // 忽略单独的修饰键
  if (key === 'Control' || key === 'Alt' || key === 'Shift' || key === 'Meta') {
    return;
  }

  if (props.isModifierOnly) {
    // 如果是数字键，使用当前的修饰键组合
    const keyLower = key.toLowerCase();
    if (keyLower >= '1' && keyLower <= '9') {
      const value = modifiers.length === 0 ? 'none' : modifiers.join('+');
      emit('update:modelValue', value);
      emit('record', value);
      stopRecording();
      return;
    }

    // 按其他键也停止录制并保存当前修饰键状态
    const value = modifiers.length === 0 ? 'none' : modifiers.join('+');
    emit('update:modelValue', value);
    emit('record', value);
    stopRecording();
  } else {
    // 标准化按键名称
    if (key === ' ') key = 'Space';
    if (key.length === 1) key = key.toUpperCase();

    // 组合快捷键
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
.hotkey-record-btn {
  min-width: 120px;
  padding: 8px 16px;
  border: 2px dashed #d9d9d9;
  border-radius: 4px;
  background: #fafafa;
  font-family: ui-monospace, SFMono-Regular, 'SF Mono', Menlo, Consolas, monospace;
  font-size: 13px;
  color: #8c8c8c;
  cursor: pointer;
  transition: all 0.2s;
  text-align: center;
}

.hotkey-record-btn:hover {
  border-color: #262626;
  color: #262626;
}

.hotkey-record-btn.recording {
  border-color: #fa8c16;
  border-style: solid;
  background: #fff7e6;
  color: #fa8c16;
  animation: pulse 1s infinite;
}

.hotkey-record-btn.has-value {
  border-style: solid;
  border-color: #52c41a;
  background: #f6ffed;
  color: #52c41a;
}

.hotkey-record-btn.small {
  min-width: 100px;
  padding: 6px 12px;
  font-size: 12px;
}

@keyframes pulse {
  0%, 100% { opacity: 1; }
  50% { opacity: 0.6; }
}
</style>
