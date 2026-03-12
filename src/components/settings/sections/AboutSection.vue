<template>
  <div class="settings-section about-section">
    <!-- Hero 区域：应用品牌展示 -->
    <div class="about-hero">
      <div class="hero-logo">
        <svg viewBox="0 0 80 80" fill="none">
          <rect width="80" height="80" rx="20" fill="#262626"/>
          <rect x="20" y="20" width="18" height="18" rx="4" fill="#fff" fill-opacity="0.9"/>
          <rect x="42" y="20" width="18" height="18" rx="4" fill="#fff" fill-opacity="0.9"/>
          <rect x="20" y="42" width="18" height="22" rx="4" fill="#fff" fill-opacity="0.9"/>
          <rect x="42" y="42" width="18" height="22" rx="4" fill="#fff" fill-opacity="0.9"/>
        </svg>
      </div>
      <div class="hero-content">
        <h1 class="hero-title">Paste Library</h1>
        <p class="hero-version">版本 {{ currentVersion }}</p>
        <p class="hero-tagline">现代化的智能剪贴板管理工具</p>
      </div>
    </div>

    <!-- 更新卡片 -->
    <div class="about-card update-card" :class="updateStatus">
      <!-- 检查中状态 -->
      <div v-if="updateStatus === 'checking'" class="update-state">
        <div class="state-visual checking">
          <div class="spinner-large"></div>
        </div>
        <div class="state-content">
          <h3 class="state-title">正在检查更新</h3>
          <p class="state-desc">正在获取最新版本信息...</p>
        </div>
      </div>

      <!-- 有可用更新 -->
      <div v-else-if="updateStatus === 'available'" class="update-state">
        <div class="state-visual available">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M12 2L2 7l10 5 10-5-10-5zM2 17l10 5 10-5M2 12l10 5 10-5"/>
          </svg>
        </div>
        <div class="state-content">
          <div class="state-header">
            <h3 class="state-title">发现新版本</h3>
            <span class="version-tag">v{{ latestVersion }}</span>
          </div>
          <p class="state-desc" v-if="updateDate">{{ formatUpdateDate(updateDate) }}</p>
          
          <!-- 更新说明 -->
          <div class="update-notes" v-if="updateNotes">
            <p>{{ updateNotes }}</p>
          </div>

          <!-- 下载进度 -->
          <div v-if="isDownloading" class="download-progress-section">
            <div class="progress-info">
              <span>正在下载更新...</span>
              <span class="progress-text">{{ downloadProgress }}%</span>
            </div>
            <div class="progress-track">
              <div class="progress-bar" :style="{ width: downloadProgress + '%' }"></div>
            </div>
          </div>

          <div class="state-actions">
            <button class="btn-primary btn-update" @click="handleDownload" :disabled="isDownloading">
              <span v-if="isDownloading" class="btn-spinner"></span>
              <svg v-else viewBox="0 0 20 20" fill="currentColor">
                <path fill-rule="evenodd" d="M3 17a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zm3.293-7.707a1 1 0 011.414 0L9 10.586V3a1 1 0 112 0v7.586l1.293-1.293a1 1 0 111.414 1.414l-3 3a1 1 0 01-1.414 0l-3-3a1 1 0 010-1.414z" clip-rule="evenodd"/>
              </svg>
              {{ isDownloading ? '正在下载...' : '下载并安装' }}
            </button>
            <button class="btn-text" @click="handleSkip" v-if="!isDownloading">跳过此版本</button>
          </div>
        </div>
      </div>

      <!-- 已是最新 -->
      <div v-else-if="updateStatus === 'uptodate'" class="update-state">
        <div class="state-visual success">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5">
            <polyline points="20 6 9 17 4 12"/>
          </svg>
        </div>
        <div class="state-content">
          <h3 class="state-title">已是最新版本</h3>
          <p class="state-desc">您正在使用最新功能，无需更新</p>
          <div class="state-actions">
            <button class="btn-ghost btn-small" @click="handleCheck">
              <svg viewBox="0 0 20 20" fill="currentColor">
                <path fill-rule="evenodd" d="M4 2a1 1 0 011 1v2.101a7.002 7.002 0 0111.601 2.566 1 1 0 11-1.885.666A5.002 5.002 0 005.999 7H9a1 1 0 010 2H4a1 1 0 01-1-1V3a1 1 0 011-1zm.008 9.057a1 1 0 011.276.61A5.002 5.002 0 0014.001 13H11a1 1 0 110-2h5a1 1 0 011 1v5a1 1 0 11-2 0v-2.101a7.002 7.002 0 01-11.601-2.566 1 1 0 01.61-1.276z" clip-rule="evenodd"/>
              </svg>
              重新检查
            </button>
          </div>
        </div>
      </div>

      <!-- 检查失败 -->
      <div v-else-if="updateStatus === 'error'" class="update-state">
        <div class="state-visual error">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <circle cx="12" cy="12" r="10"/>
            <line x1="15" y1="9" x2="9" y2="15"/>
            <line x1="9" y1="9" x2="15" y2="15"/>
          </svg>
        </div>
        <div class="state-content">
          <h3 class="state-title">检查更新失败</h3>
          <p class="state-desc">请检查网络连接后重试</p>
          <div class="state-actions">
            <button class="btn-ghost btn-small" @click="handleCheck">
              <svg viewBox="0 0 20 20" fill="currentColor">
                <path fill-rule="evenodd" d="M4 2a1 1 0 011 1v2.101a7.002 7.002 0 0111.601 2.566 1 1 0 11-1.885.666A5.002 5.002 0 005.999 7H9a1 1 0 010 2H4a1 1 0 01-1-1V3a1 1 0 011-1zm.008 9.057a1 1 0 011.276.61A5.002 5.002 0 0014.001 13H11a1 1 0 110-2h5a1 1 0 011 1v5a1 1 0 11-2 0v-2.101a7.002 7.002 0 01-11.601-2.566 1 1 0 01.61-1.276z" clip-rule="evenodd"/>
              </svg>
              重试
            </button>
          </div>
        </div>
      </div>

      <!-- 初始状态 -->
      <div v-else class="update-state">
        <div class="state-visual idle">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M21 12a9 9 0 0 0-9-9 9.75 9.75 0 0 0-6.74 2.74L3 8"/>
            <path d="M3 3v5h5"/>
            <path d="M3 12a9 9 0 0 0 9 9 9.75 9.75 0 0 0 6.74-2.74L21 16"/>
            <path d="M16 16h5v5"/>
          </svg>
        </div>
        <div class="state-content">
          <h3 class="state-title">检查更新</h3>
          <p class="state-desc">获取最新版本信息和功能改进</p>
          <div class="state-actions">
            <button class="btn-primary btn-update" @click="handleCheck">
              <svg viewBox="0 0 20 20" fill="currentColor">
                <path fill-rule="evenodd" d="M4 2a1 1 0 011 1v2.101a7.002 7.002 0 0111.601 2.566 1 1 0 11-1.885.666A5.002 5.002 0 005.999 7H9a1 1 0 010 2H4a1 1 0 01-1-1V3a1 1 0 011-1zm.008 9.057a1 1 0 011.276.61A5.002 5.002 0 0014.001 13H11a1 1 0 110-2h5a1 1 0 011 1v5a1 1 0 11-2 0v-2.101a7.002 7.002 0 01-11.601-2.566 1 1 0 01.61-1.276z" clip-rule="evenodd"/>
              </svg>
              检查更新
            </button>
          </div>
        </div>
      </div>
    </div>

    <!-- 链接区域 -->
    <div class="links-section">
      <h3 class="section-subtitle">相关链接</h3>
      <div class="links-grid">
        <a href="#" class="link-card" @click.prevent="handleOpenLink('https://github.com/lanxiuyun/paste-library')">
          <div class="link-icon">
            <svg viewBox="0 0 24 24" fill="currentColor">
              <path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/>
            </svg>
          </div>
          <div class="link-content">
            <span class="link-title">GitHub</span>
            <span class="link-desc">查看源码和提交 Issue</span>
          </div>
          <svg class="link-arrow" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M5 12h14M12 5l7 7-7 7"/>
          </svg>
        </a>

        <a href="#" class="link-card" @click.prevent="handleOpenLink('https://github.com/lanxiuyun/paste-library/releases')">
          <div class="link-icon blue">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M12 2L2 7l10 5 10-5-10-5zM2 17l10 5 10-5M2 12l10 5 10-5"/>
            </svg>
          </div>
          <div class="link-content">
            <span class="link-title">更新日志</span>
            <span class="link-desc">查看版本更新记录</span>
          </div>
          <svg class="link-arrow" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M5 12h14M12 5l7 7-7 7"/>
          </svg>
        </a>

        <a href="#" class="link-card" @click.prevent="handleOpenLink('https://github.com/lanxiuyun/paste-library/issues/new')">
          <div class="link-icon orange">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <circle cx="12" cy="12" r="10"/>
              <line x1="12" y1="8" x2="12" y2="12"/>
              <line x1="12" y1="16" x2="12.01" y2="16"/>
            </svg>
          </div>
          <div class="link-content">
            <span class="link-title">问题反馈</span>
            <span class="link-desc">报告 Bug 或建议新功能</span>
          </div>
          <svg class="link-arrow" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M5 12h14M12 5l7 7-7 7"/>
          </svg>
        </a>
      </div>
    </div>

    <!-- 底部版权信息 -->
    <div class="about-footer">
      <p>&copy; {{ new Date().getFullYear() }} Paste Library. All rights reserved.</p>
      <p>Made with ❤️ for efficient workflows</p>
    </div>
  </div>
</template>

<script setup lang="ts">
interface Props {
  currentVersion: string;
  updateStatus: 'idle' | 'checking' | 'available' | 'uptodate' | 'error';
  latestVersion: string;
  updateNotes: string;
  updateDate: string;
  isDownloading: boolean;
  downloadProgress: number;
}

defineProps<Props>();

const emit = defineEmits<{
  'check': [];
  'download': [];
  'skip': [];
  'open-link': [url: string];
}>();

const formatUpdateDate = (dateStr: string): string => {
  if (!dateStr) return '';
  try {
    const date = new Date(dateStr);
    return date.toLocaleDateString('zh-CN', {
      year: 'numeric',
      month: 'long',
      day: 'numeric',
    });
  } catch {
    return dateStr;
  }
};

const handleCheck = () => {
  emit('check');
};

const handleDownload = () => {
  emit('download');
};

const handleSkip = () => {
  emit('skip');
};

const handleOpenLink = (url: string) => {
  emit('open-link', url);
};
</script>

<style scoped>
.about-section {
  padding: 24px 32px !important;
  overflow-y: auto;
}

/* Hero 区域 */
.about-hero {
  display: flex;
  align-items: center;
  gap: 20px;
  margin-bottom: 24px;
  padding-bottom: 24px;
  border-bottom: 1px solid #f0f0f0;
}

.hero-logo {
  width: 88px;
  height: 88px;
  flex-shrink: 0;
  border-radius: 20px;
  overflow: hidden;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.08);
}

.hero-logo svg {
  width: 100%;
  height: 100%;
}

.hero-content {
  flex: 1;
}

.hero-title {
  font-size: 28px;
  font-weight: 700;
  color: #262626;
  margin: 0 0 6px 0;
  letter-spacing: -0.5px;
}

.hero-version {
  font-size: 14px;
  color: #8c8c8c;
  margin: 0 0 8px 0;
  font-weight: 500;
}

.hero-tagline {
  font-size: 15px;
  color: #595959;
  margin: 0;
  line-height: 1.5;
}

/* 卡片基础样式 */
.about-card {
  background: #fff;
  border-radius: 16px;
  border: 1px solid #f0f0f0;
  overflow: hidden;
  transition: all 0.3s ease;
}

/* 更新卡片 */
.update-card {
  margin-bottom: 24px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.04);
}

.update-card:hover {
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.06);
}

.update-card.available {
  border-color: #bae7ff;
  background: linear-gradient(135deg, #fff 0%, #f0f9ff 100%);
}

.update-card.uptodate {
  border-color: #d9f7be;
  background: linear-gradient(135deg, #fff 0%, #f6ffed 100%);
}

.update-card.error {
  border-color: #ffccc7;
  background: linear-gradient(135deg, #fff 0%, #fff2f0 100%);
}

/* 更新状态 */
.update-state {
  display: flex;
  align-items: flex-start;
  gap: 16px;
  padding: 20px;
}

.state-visual {
  width: 56px;
  height: 56px;
  border-radius: 16px;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
}

.state-visual svg {
  width: 28px;
  height: 28px;
}

.state-visual.checking {
  background: #e6f7ff;
  color: #1890ff;
}

.state-visual.available {
  background: #e6f7ff;
  color: #1890ff;
}

.state-visual.success {
  background: #f6ffed;
  color: #52c41a;
}

.state-visual.error {
  background: #fff2f0;
  color: #ff4d4f;
}

.state-visual.idle {
  background: #f5f5f5;
  color: #8c8c8c;
}

.spinner-large {
  width: 28px;
  height: 28px;
  border: 3px solid #1890ff;
  border-top-color: transparent;
  border-radius: 50%;
  animation: spin 0.8s linear infinite;
}

.state-content {
  flex: 1;
  min-width: 0;
}

.state-header {
  display: flex;
  align-items: center;
  gap: 12px;
  margin-bottom: 4px;
}

.state-title {
  font-size: 17px;
  font-weight: 600;
  color: #262626;
  margin: 0;
}

.version-tag {
  font-size: 12px;
  font-weight: 600;
  padding: 3px 10px;
  background: #1890ff;
  color: #fff;
  border-radius: 20px;
}

.state-desc {
  font-size: 14px;
  color: #8c8c8c;
  margin: 0 0 16px 0;
}

/* 更新说明 */
.update-notes {
  margin: 16px 0;
  padding: 16px;
  background: rgba(255, 255, 255, 0.7);
  border-radius: 10px;
  border: 1px solid #f0f0f0;
}

.update-notes p {
  margin: 0;
  font-size: 13px;
  color: #595959;
  line-height: 1.7;
  white-space: pre-wrap;
}

/* 下载进度 */
.download-progress-section {
  margin: 16px 0;
  padding: 16px;
  background: rgba(255, 255, 255, 0.8);
  border-radius: 10px;
  border: 1px solid #e6f7ff;
}

.progress-info {
  display: flex;
  justify-content: space-between;
  font-size: 13px;
  color: #595959;
  margin-bottom: 10px;
}

.progress-text {
  font-weight: 600;
  color: #1890ff;
}

.progress-track {
  height: 6px;
  background: #f0f0f0;
  border-radius: 3px;
  overflow: hidden;
}

.progress-bar {
  height: 100%;
  background: linear-gradient(90deg, #1890ff 0%, #40a9ff 100%);
  border-radius: 3px;
  transition: width 0.3s ease;
}

/* 状态操作 */
.state-actions {
  display: flex;
  align-items: center;
  gap: 12px;
  margin-top: 16px;
}

.btn-update {
  padding: 10px 20px;
  background: #262626;
  border: none;
  border-radius: 6px;
  font-size: 14px;
  font-weight: 500;
  color: #fff;
  cursor: pointer;
  transition: background 0.2s;
  display: inline-flex;
  align-items: center;
  gap: 8px;
}

.btn-update:hover:not(:disabled) {
  background: #404040;
}

.btn-update:disabled {
  background: #d9d9d9;
  cursor: not-allowed;
}

.btn-update svg {
  width: 18px;
  height: 18px;
}

.btn-text {
  padding: 10px 16px;
  background: transparent;
  border: none;
  font-size: 14px;
  color: #8c8c8c;
  cursor: pointer;
  transition: color 0.2s;
}

.btn-text:hover {
  color: #595959;
}

/* 小按钮 */
.btn-small {
  padding: 8px 16px;
  font-size: 13px;
  display: inline-flex;
  align-items: center;
  gap: 6px;
}

.btn-small svg {
  width: 16px;
  height: 16px;
}

.btn-ghost {
  padding: 8px 16px;
  background: transparent;
  border: 1px solid #d9d9d9;
  border-radius: 6px;
  font-size: 13px;
  color: #595959;
  cursor: pointer;
  transition: all 0.2s;
}

.btn-ghost:hover {
  border-color: #262626;
  color: #262626;
}

.btn-spinner {
  width: 16px;
  height: 16px;
  border: 2px solid #fff;
  border-top-color: transparent;
  border-radius: 50%;
  animation: spin 0.8s linear infinite;
}

@keyframes spin {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}

/* 链接区域 */
.links-section {
  margin-bottom: 24px;
}

.section-subtitle {
  font-size: 14px;
  font-weight: 600;
  color: #262626;
  margin: 0 0 12px 0;
}

.links-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(160px, 1fr));
  gap: 12px;
}

.link-card {
  display: flex;
  align-items: center;
  gap: 14px;
  padding: 16px;
  background: #fff;
  border-radius: 12px;
  border: 1px solid #f0f0f0;
  text-decoration: none;
  transition: all 0.2s;
  cursor: pointer;
}

.link-card:hover {
  border-color: #d9d9d9;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.06);
  transform: translateY(-2px);
}

.link-icon {
  width: 40px;
  height: 40px;
  border-radius: 10px;
  background: #f5f5f5;
  display: flex;
  align-items: center;
  justify-content: center;
  color: #262626;
  flex-shrink: 0;
}

.link-icon.blue {
  background: #e6f7ff;
  color: #1890ff;
}

.link-icon.orange {
  background: #fff7e6;
  color: #fa8c16;
}

.link-icon svg {
  width: 20px;
  height: 20px;
}

.link-content {
  flex: 1;
  min-width: 0;
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.link-title {
  font-size: 14px;
  font-weight: 600;
  color: #262626;
}

.link-desc {
  font-size: 12px;
  color: #8c8c8c;
}

.link-arrow {
  width: 16px;
  height: 16px;
  color: #d9d9d9;
  flex-shrink: 0;
  transition: all 0.2s;
}

.link-card:hover .link-arrow {
  color: #595959;
  transform: translateX(2px);
}

/* 底部版权 */
.about-footer {
  text-align: center;
  padding-top: 24px;
  border-top: 1px solid #f0f0f0;
}

.about-footer p {
  margin: 0;
  font-size: 12px;
  color: #bfbfbf;
}

.about-footer p:first-child {
  margin-bottom: 4px;
}

/* 响应式 */
@media (max-width: 600px) {
  .about-section {
    padding: 24px !important;
  }
  
  .about-hero {
    flex-direction: column;
    text-align: center;
    gap: 16px;
  }
  
  .hero-logo {
    width: 72px;
    height: 72px;
  }
  
  .hero-title {
    font-size: 24px;
  }
  
  .update-state {
    flex-direction: column;
    text-align: center;
    gap: 16px;
  }
  
  .state-actions {
    justify-content: center;
  }
  
  .links-grid {
    grid-template-columns: 1fr;
  }
}
</style>
