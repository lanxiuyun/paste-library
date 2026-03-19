<template>
  <div class="extracted-info-panel">
    <div
      v-for="(info, index) in extractedInfo"
      :key="index"
      class="extracted-info-item"
    >
      <span class="info-type-badge" :class="info.type">{{ info.label }}</span>
      <span class="info-value" :title="info.value">{{ info.value }}</span>
      <div class="info-actions">
        <button
          class="info-action-btn"
          title="复制"
          @click="handleCopy(info.value)"
        >
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <rect x="9" y="9" width="13" height="13" rx="2" ry="2"/>
            <path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"/>
          </svg>
        </button>
        <button
          class="info-action-btn"
          title="粘贴"
          @click="handlePaste(info.value)"
        >
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M16 4h2a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2h2"/>
            <rect x="8" y="2" width="8" height="4" rx="1"/>
          </svg>
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { decodeHtmlEntities } from '@/utils/htmlUtils';

interface Props {
  content: string;
  contentType: string;
}

const props = defineProps<Props>();

const emit = defineEmits<{
  copy: [value: string];
  paste: [value: string];
}>();

interface ExtractedInfo {
  type: string;
  label: string;
  value: string;
}

// 去除 HTML 标签，获取纯文本
const stripHtml = (html: string): string => {
  if (!html) return '';
  const decoded = decodeHtmlEntities(html);
  return decoded.replace(/<[^>]+>/g, ' ').replace(/\s+/g, ' ').trim();
};

// 从文本中提取关键信息
const extractedInfo = computed<ExtractedInfo[]>(() => {
  if (!props.content) return [];

  // 如果是 HTML 内容，先去除标签
  const content = props.contentType === 'html'
    ? stripHtml(props.content)
    : props.content;

  if (!content) return [];

  const results: ExtractedInfo[] = [];
  const seen = new Set<string>();

  const addResult = (type: string, label: string, value: string) => {
    const key = `${type}:${value}`;
    if (!seen.has(key)) {
      seen.add(key);
      results.push({ type, label, value });
    }
  };

  // 身份证号（18位）
  const idCardRegex = /\b\d{17}[\dXx]\b/g;
  let match;
  while ((match = idCardRegex.exec(content)) !== null) {
    addResult('idcard', '身份证', match[0]);
  }

  // 手机号（中国大陆）
  const phoneRegex = /\b1[3-9]\d{9}\b/g;
  while ((match = phoneRegex.exec(content)) !== null) {
    addResult('phone', '电话', match[0]);
  }

  // 座机号
  const landlineRegex = /\b0\d{2,3}-?\d{7,8}\b/g;
  while ((match = landlineRegex.exec(content)) !== null) {
    addResult('landline', '座机', match[0]);
  }

  // 邮箱
  const emailRegex = /\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}\b/g;
  while ((match = emailRegex.exec(content)) !== null) {
    addResult('email', '邮箱', match[0]);
  }

  // 地址（包含常见地址关键词）
  const addressRegex = /(?:([\u4e00-\u9fa5]{2,}(?:省|市|自治区|州|区|县|镇|乡|街道|路|街|巷|号|单元|室|栋|层|楼))[^，。,\n]*(?:省|市|自治区|州|区|县|镇|乡|街道|路|街|巷|号|单元|室|栋|层|楼))/g;
  while ((match = addressRegex.exec(content)) !== null) {
    const start = Math.max(0, match.index - 10);
    const end = Math.min(content.length, match.index + match[0].length + 30);
    let address = content.slice(start, end).trim();
    address = address.replace(/^[，。,\s]+|[，。,\s]+$/g, '');
    if (address.length > 5) {
      addResult('address', '地址', address);
    }
  }

  // URL / 链接
  const urlRegex = /\b(?:https?:\/\/|www\.)[^\s<>'"]+\b/gi;
  while ((match = urlRegex.exec(content)) !== null) {
    addResult('url', '链接', match[0]);
  }

  // IP 地址
  const ipRegex = /\b(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\b/g;
  while ((match = ipRegex.exec(content)) !== null) {
    addResult('ip', 'IP', match[0]);
  }

  // 银行卡号（16-19位数字）
  const bankCardRegex = /\b(?:\d{4}[ -]?){3,4}\d{1,4}\b/g;
  while ((match = bankCardRegex.exec(content)) !== null) {
    const cardNum = match[0].replace(/[ -]/g, '');
    if (cardNum.length >= 16 && cardNum.length <= 19) {
      addResult('bankcard', '银行卡', match[0]);
    }
  }

  // 日期
  const dateRegex = /\b\d{4}[年/-]\d{1,2}[月/-]\d{1,2}[日]?\b/g;
  while ((match = dateRegex.exec(content)) !== null) {
    addResult('date', '日期', match[0]);
  }

  return results.slice(0, 20);
});

const handleCopy = (value: string) => {
  emit('copy', value);
};

const handlePaste = (value: string) => {
  emit('paste', value);
};
</script>

<style scoped>
.extracted-info-panel {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.extracted-info-item {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 12px;
  background: #fafafa;
  border: 1px solid #e8e8e8;
  border-radius: 6px;
}

.info-type-badge {
  flex-shrink: 0;
  padding: 2px 8px;
  font-size: 12px;
  font-weight: 500;
  border-radius: 4px;
  white-space: nowrap;
  background: #f0f0f0;
  color: #595959;
}

.info-type-badge.idcard {
  background: #fff2e8;
  color: #fa8c16;
}

.info-type-badge.phone,
.info-type-badge.landline {
  background: #e6f7ff;
  color: #1890ff;
}

.info-type-badge.email {
  background: #f6ffed;
  color: #52c41a;
}

.info-type-badge.address {
  background: #fff0f6;
  color: #eb2f96;
}

.info-type-badge.url {
  background: #f9f0ff;
  color: #722ed1;
}

.info-type-badge.ip {
  background: #fff2f0;
  color: #ff4d4f;
}

.info-type-badge.bankcard {
  background: #e6fffb;
  color: #13c2c2;
}

.info-type-badge.date {
  background: #fff7e6;
  color: #faad14;
}

.info-value {
  flex: 1;
  min-width: 0;
  font-size: 13px;
  color: #262626;
  font-family: ui-monospace, SFMono-Regular, 'SF Mono', Menlo, Consolas, monospace;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.info-actions {
  display: flex;
  gap: 4px;
}

.info-action-btn {
  width: 28px;
  height: 28px;
  display: flex;
  align-items: center;
  justify-content: center;
  border: 1px solid #d9d9d9;
  background: #fff;
  cursor: pointer;
  border-radius: 4px;
  color: #595959;
  transition: all 0.15s ease;
}

.info-action-btn:hover {
  border-color: #262626;
  color: #262626;
}

.info-action-btn svg {
  width: 14px;
  height: 14px;
}
</style>
