<template>
  <div class="settings-section">
    <h2 class="section-title">窗口设置</h2>
    
    <div class="setting-group">
      <SettingItem title="窗口位置">
        <select v-model="form.window_position" class="select-input">
          <option value="remember">记住位置</option>
          <option value="center">居中显示</option>
          <option value="cursor">跟随鼠标</option>
        </select>
      </SettingItem>

    </div>

    <h2 class="section-title">内容设置</h2>

    <div class="setting-group">
      <SettingItem title="单击动作" description="鼠标单击剪贴板项目时执行的动作">
        <select v-model="form.click_action" class="select-input">
          <option value="copy">复制</option>
          <option value="paste">粘贴</option>
          <option value="none">不操作</option>
        </select>
      </SettingItem>

      <SettingItem title="双击动作" description="鼠标双击剪贴板项目时执行的动作">
        <select v-model="form.double_click_action" class="select-input">
          <option value="copy">复制</option>
          <option value="paste">粘贴</option>
          <option value="none">不操作</option>
        </select>
      </SettingItem>

      <SettingItem title="复制后隐藏窗口" description="复制后自动隐藏剪贴板窗口">
        <label class="switch">
          <input type="checkbox" v-model="form.hide_window_after_copy" />
          <span class="slider"></span>
        </label>
      </SettingItem>

      <SettingItem
        title="粘贴快捷键"
        description="执行粘贴动作时使用的快捷键（终端推荐使用 Shift+Insert）"
      >
        <select v-model="form.paste_shortcut" class="select-input">
          <option value="ctrl_v">Ctrl+V</option>
          <option value="shift_insert">Shift+Insert</option>
        </select>
      </SettingItem>

      <!-- <SettingItem title="图片OCR">
        <label class="switch">
          <input type="checkbox" v-model="form.image_ocr" />
          <span class="slider"></span>
        </label>
      </SettingItem> -->

      <SettingItem title="复制为纯文本" description="HTML格式在复制时仅保留纯文本内容">
        <label class="switch">
          <input type="checkbox" v-model="form.copy_as_plain_text" />
          <span class="slider"></span>
        </label>
      </SettingItem>

      <SettingItem title="粘贴为纯文本" description="HTML格式在粘贴时仅保留纯文本内容">
        <label class="switch">
          <input type="checkbox" v-model="form.paste_as_plain_text" />
          <span class="slider"></span>
        </label>
      </SettingItem>

      <SettingItem title="删除确认" description="删除剪贴板内容时弹出确认对话框">
        <label class="switch">
          <input type="checkbox" v-model="form.confirm_delete" />
          <span class="slider"></span>
        </label>
      </SettingItem>

      <SettingItem title="自动排序" description="复制已存在的内容时排列到最前面">
        <label class="switch">
          <input type="checkbox" v-model="form.auto_sort" />
          <span class="slider"></span>
        </label>
      </SettingItem>
    </div>
  </div>
</template>

<script setup lang="ts">
import SettingItem from '../components/SettingItem.vue';
import type { AppSettings } from '@/types';

interface Props {
  form: AppSettings;
}

defineProps<Props>();
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

.select-input {
  padding: 6px 28px 6px 10px;
  border: 1px solid #d9d9d9;
  border-radius: 4px;
  font-size: 13px;
  background: #fff;
  cursor: pointer;
  outline: none;
  min-width: 120px;
}

.select-input:focus {
  border-color: #262626;
}

/* Switch toggle */
.switch {
  position: relative;
  display: inline-block;
  width: 44px;
  height: 24px;
}

.switch input {
  opacity: 0;
  width: 0;
  height: 0;
}

.slider {
  position: absolute;
  cursor: pointer;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: #d9d9d9;
  transition: 0.2s;
  border-radius: 24px;
}

.slider:before {
  position: absolute;
  content: "";
  height: 18px;
  width: 18px;
  left: 3px;
  bottom: 3px;
  background-color: white;
  transition: 0.2s;
  border-radius: 50%;
}

input:checked + .slider {
  background-color: #262626;
}

input:checked + .slider:before {
  transform: translateX(20px);
}
</style>
