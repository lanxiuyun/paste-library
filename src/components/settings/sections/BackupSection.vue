<template>
  <div class="settings-section">
    <h2 class="section-title">数据备份</h2>
    
    <div class="setting-group">
      <SettingItem title="导出数据" description="将剪贴板历史导出为 JSON 文件">
        <button class="btn-secondary" @click="handleExport">
          导出
        </button>
      </SettingItem>

      <SettingItem title="导入数据" description="从 JSON 文件导入剪贴板历史">
        <button class="btn-secondary" @click="handleImport">
          导入
        </button>
      </SettingItem>
    </div>

    <h2 class="section-title">存储路径</h2>
    
    <div class="setting-group">
      <SettingItem title="数据存储路径" description="剪贴板历史和设置数据存储位置" full-width>
        <PathDisplay
          :path="storagePaths.data_dir"
          @copy="handlePathCopy"
          @open="handlePathOpen"
        />
      </SettingItem>

      <SettingItem title="日志存储路径" description="应用日志文件存储位置" full-width>
        <PathDisplay
          :path="storagePaths.log_dir"
          @copy="handlePathCopy"
          @open="handlePathOpen"
        />
      </SettingItem>
    </div>
  </div>
</template>

<script setup lang="ts">
import SettingItem from '../components/SettingItem.vue';
import PathDisplay from '../components/PathDisplay.vue';

interface Props {
  storagePaths: {
    data_dir: string;
    log_dir: string;
  };
}

defineProps<Props>();

const emit = defineEmits<{
  'export': [];
  'import': [];
  'path-copy': [path: string];
  'path-open': [path: string];
}>();

const handleExport = () => {
  emit('export');
};

const handleImport = () => {
  emit('import');
};

const handlePathCopy = (path: string) => {
  emit('path-copy', path);
};

const handlePathOpen = (path: string) => {
  emit('path-open', path);
};
</script>

<style scoped>
.settings-section {
  flex: 1;
  padding: 24px 32px;
  overflow-y: auto;
}

.section-title {
  font-size: 16px;
  font-weight: 600;
  color: #262626;
  margin: 0 0 12px 0;
}

.section-title:not(:first-child) {
  margin-top: 12px;
}

.setting-group {
  background: #fff;
  border-radius: 8px;
  padding: 0 20px;
}

.btn-secondary {
  padding: 8px 16px;
  background: #fff;
  border: 1px solid #d9d9d9;
  border-radius: 6px;
  font-size: 13px;
  color: #595959;
  cursor: pointer;
  transition: all 0.2s;
}

.btn-secondary:hover {
  border-color: #262626;
  color: #262626;
}
</style>
