<template>
  <div class="settings-view">
    <!-- 左侧导航栏 -->
    <div class="sidebar">
      <nav class="nav-menu">
        <button 
          v-for="item in menuItems" 
          :key="item.key"
          class="nav-item"
          :class="{ active: activeMenu === item.key }"
          @click="activeMenu = item.key"
        >
          <span class="nav-icon">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path v-if="item.key === 'clipboard'" d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2"/>
              <path v-else-if="item.key === 'history'" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"/>
              <path v-else-if="item.key === 'general'" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"/>
              <path v-else-if="item.key === 'hotkey'" d="M12 2L2 7l10 5 10-5-10-5zM2 17l10 5 10-5M2 12l10 5 10-5"/>
              <path v-else-if="item.key === 'backup'" d="M4 7v10c0 2.21 3.582 4 8 4s8-1.79 8-4V7M4 7c0 2.21 3.582 4 8 4s8-1.79 8-4M4 7c0-2.21 3.582-4 8-4s8 1.79 8 4m0 5c0 2.21-3.582 4-8 4s-8-1.79-8-4"/>
              <path v-else-if="item.key === 'about'" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"/>
            </svg>
          </span>
          <span class="nav-label">{{ item.label }}</span>
        </button>
      </nav>
    </div>

    <!-- 右侧内容区 -->
    <div class="content">
      <!-- 剪贴板设置 -->
      <div v-if="activeMenu === 'clipboard'" class="settings-section">
        <h2 class="section-title">窗口设置</h2>
        
        <div class="setting-group">
          <div class="setting-item">
            <div class="setting-info">
              <div class="setting-title">窗口位置</div>
            </div>
            <div class="setting-control">
              <select v-model="form.window_position" class="select-input">
                <option value="remember">记住位置</option>
                <option value="center">居中显示</option>
                <option value="cursor">跟随鼠标</option>
              </select>
            </div>
          </div>

          <div class="setting-item">
            <div class="setting-info">
              <div class="setting-title">智能激活</div>
              <div class="setting-desc">5秒内复制过内容时，激活窗口自动回到顶部、切换至全部、聚焦搜索框</div>
            </div>
            <div class="setting-control">
              <label class="switch">
                <input type="checkbox" v-model="form.smart_activate" />
                <span class="slider"></span>
              </label>
            </div>
          </div>
        </div>

        <!-- 音效设置 - 已隐藏
        <h2 class="section-title">音效设置</h2>
        
        <div class="setting-group">
          <div class="setting-item">
            <div class="setting-info">
              <div class="setting-title">复制音效</div>
            </div>
            <div class="setting-control">
              <label class="switch">
                <input type="checkbox" v-model="form.copy_sound" />
                <span class="slider"></span>
              </label>
              <button class="icon-btn" title="预览音效">
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <polygon points="11 5 6 9 2 9 2 15 6 15 11 19 11 5"/>
                  <path d="M19.07 4.93a10 10 0 010 14.14M15.54 8.46a5 5 0 010 7.07"/>
                </svg>
              </button>
            </div>
          </div>
        </div>
        -->

        <!-- 搜索设置 - 搜索框位置已隐藏，默认顶部 -->
        <h2 class="section-title">搜索设置</h2>
        
        <div class="setting-group">
          <!-- 搜索框位置 - 已隐藏，默认顶部
          <div class="setting-item">
            <div class="setting-info">
              <div class="setting-title">搜索框位置</div>
            </div>
            <div class="setting-control">
              <select v-model="form.search_position" class="select-input">
                <option value="top">顶部</option>
                <option value="bottom">底部</option>
              </select>
            </div>
          </div>
          -->

          <div class="setting-item">
            <div class="setting-info">
              <div class="setting-title">默认聚焦</div>
              <div class="setting-desc">激活窗口时，默认聚焦搜索框</div>
            </div>
            <div class="setting-control">
              <label class="switch">
                <input type="checkbox" v-model="form.focus_search_on_activate" />
                <span class="slider"></span>
              </label>
            </div>
          </div>

        </div>

        <h2 class="section-title">内容设置</h2>

        <div class="setting-group">
          <div class="setting-item">
            <div class="setting-info">
              <div class="setting-title">单击动作</div>
              <div class="setting-desc">鼠标单击剪贴板项目时执行的动作</div>
            </div>
            <div class="setting-control">
              <select v-model="form.click_action" class="select-input">
                <option value="copy">复制</option>
                <option value="paste">粘贴</option>
                <option value="none">不操作</option>
              </select>
            </div>
          </div>

          <div class="setting-item">
            <div class="setting-info">
              <div class="setting-title">双击动作</div>
              <div class="setting-desc">鼠标双击剪贴板项目时执行的动作</div>
            </div>
            <div class="setting-control">
              <select v-model="form.double_click_action" class="select-input">
                <option value="copy">复制</option>
                <option value="paste">粘贴</option>
                <option value="none">不操作</option>
              </select>
            </div>
          </div>

          <div class="setting-item">
            <div class="setting-info">
              <div class="setting-title">复制后隐藏窗口</div>
              <div class="setting-desc">复制后自动隐藏剪贴板窗口</div>
            </div>
            <div class="setting-control">
              <label class="switch">
                <input type="checkbox" v-model="form.hide_window_after_copy" />
                <span class="slider"></span>
              </label>
            </div>
          </div>

          <div class="setting-item">
            <div class="setting-info">
              <div class="setting-title">粘贴快捷键</div>
              <div class="setting-desc">执行粘贴动作时使用的快捷键（终端推荐使用 Shift+Insert）</div>
            </div>
            <div class="setting-control">
              <select v-model="form.paste_shortcut" class="select-input">
                <option value="ctrl_v">Ctrl+V</option>
                <option value="shift_insert">Shift+Insert</option>
              </select>
            </div>
          </div>

          <div class="setting-item">
            <div class="setting-info">
              <div class="setting-title">图片OCR</div>
            </div>
            <div class="setting-control">
              <label class="switch">
                <input type="checkbox" v-model="form.image_ocr" />
                <span class="slider"></span>
              </label>
            </div>
          </div>

          <div class="setting-item">
            <div class="setting-info">
              <div class="setting-title">复制为纯文本</div>
              <div class="setting-desc">HTML格式在复制时仅保留纯文本内容</div>
            </div>
            <div class="setting-control">
              <label class="switch">
                <input type="checkbox" v-model="form.copy_as_plain_text" />
                <span class="slider"></span>
              </label>
            </div>
          </div>

          <div class="setting-item">
            <div class="setting-info">
              <div class="setting-title">粘贴为纯文本</div>
              <div class="setting-desc">HTML格式在粘贴时仅保留纯文本内容</div>
            </div>
            <div class="setting-control">
              <label class="switch">
                <input type="checkbox" v-model="form.paste_as_plain_text" />
                <span class="slider"></span>
              </label>
            </div>
          </div>

          <div class="setting-item">
            <div class="setting-info">
              <div class="setting-title">删除确认</div>
              <div class="setting-desc">删除剪贴板内容时弹出确认对话框</div>
            </div>
            <div class="setting-control">
              <label class="switch">
                <input type="checkbox" v-model="form.confirm_delete" />
                <span class="slider"></span>
              </label>
            </div>
          </div>

          <div class="setting-item">
            <div class="setting-info">
              <div class="setting-title">自动排序</div>
              <div class="setting-desc">复制已存在的内容时排列到最前面</div>
            </div>
            <div class="setting-control">
              <label class="switch">
                <input type="checkbox" v-model="form.auto_sort" />
                <span class="slider"></span>
              </label>
            </div>
          </div>
        </div>
      </div>

      <!-- 历史记录设置 -->
      <div v-if="activeMenu === 'history'" class="settings-section">
        <h2 class="section-title">历史记录设置</h2>
        
        <div class="setting-group">
          <div class="setting-item">
            <div class="setting-info">
              <div class="setting-title">最大历史记录数</div>
              <div class="setting-desc">达到上限后自动删除最早的数据</div>
            </div>
            <div class="setting-control">
              <input 
                type="number" 
                v-model.number="form.max_history_count"
                min="100"
                max="10000"
                step="100"
                class="number-input"
              />
            </div>
          </div>

          <div class="setting-item">
            <div class="setting-info">
              <div class="setting-title">自动清理</div>
              <div class="setting-desc">自动删除超过指定天数的历史记录</div>
            </div>
            <div class="setting-control">
              <select v-model.number="form.auto_cleanup_days" class="select-input">
                <option :value="0">不自动清理</option>
                <option :value="7">7天</option>
                <option :value="30">30天</option>
                <option :value="90">90天</option>
              </select>
            </div>
          </div>

          <div class="setting-item">
            <div class="setting-info">
              <div class="setting-title">删除历史记录</div>
              <div class="setting-desc">永久删除所有剪贴板历史记录（此操作不可撤销）</div>
            </div>
            <div class="setting-control">
              <button class="btn-danger" @click="clearAllHistory">
                删除全部
              </button>
            </div>
          </div>
        </div>
      </div>

      <!-- 通用设置 -->
      <div v-if="activeMenu === 'general'" class="settings-section">
        <h2 class="section-title">通用设置</h2>
        
        <div class="setting-group">
          <div class="setting-item">
            <div class="setting-info">
              <div class="setting-title">开机自启</div>
              <div class="setting-desc">系统启动时自动运行 Paste Library</div>
            </div>
            <div class="setting-control">
              <label class="switch">
                <input type="checkbox" v-model="form.auto_start" />
                <span class="slider"></span>
              </label>
            </div>
          </div>

        </div>
      </div>

      <!-- 快捷键设置 -->
      <div v-if="activeMenu === 'hotkey'" class="settings-section">
        <h2 class="section-title">快捷键设置</h2>
        
        <div class="setting-group">
          <div class="setting-item">
            <div class="setting-info">
              <div class="setting-title">唤醒快捷键</div>
              <div class="setting-desc">按下此快捷键可快速打开或关闭剪贴板窗口</div>
              <div class="setting-note">点击右边按钮开始录制，然后按下想要的快捷键组合</div>
              <div class="setting-warning">修改快捷键后需要重启应用才能生效</div>
              <div v-if="shortcutError" class="error-message">{{ shortcutError }}</div>
            </div>
            <div class="setting-control">
              <button
                class="hotkey-record-btn"
                :class="{ 'recording': isRecordingHotkey, 'has-value': form.hotkey && !isRecordingHotkey }"
                @click="toggleHotkeyRecording"
              >
                {{ isRecordingHotkey ? '请按下快捷键...' : (form.hotkey || '点击录制') }}
              </button>
            </div>
          </div>

          <div class="setting-item">
            <div class="setting-info">
              <div class="setting-title">数字键快捷粘贴</div>
              <div class="setting-desc">按下 1-9 数字键粘贴对应位置剪贴板内容时需要同时按住的修饰键</div>
              <div class="setting-note">点击右边按钮开始录制，然后按下想要的修饰键组合（如 Ctrl、Ctrl+Shift、Alt 等）</div>
            </div>
            <div class="setting-control">
              <button
                class="hotkey-record-btn small"
                :class="{ 'recording': isRecordingNumberKeyShortcut, 'has-value': form.number_key_shortcut && !isRecordingNumberKeyShortcut }"
                @click="toggleNumberKeyShortcutRecording"
              >
                {{ isRecordingNumberKeyShortcut ? '请按下修饰键...' : (formatNumberKeyShortcut(form.number_key_shortcut) || '点击录制') }}
              </button>
            </div>
          </div>

        </div>
      </div>

      <!-- 数据备份 -->
      <div v-if="activeMenu === 'backup'" class="settings-section">
        <h2 class="section-title">数据备份</h2>
        
        <div class="setting-group">
          <div class="setting-item">
            <div class="setting-info">
              <div class="setting-title">导出数据</div>
              <div class="setting-desc">将剪贴板历史导出为 JSON 文件</div>
            </div>
            <div class="setting-control">
              <button class="btn-secondary" @click="exportData">
                导出
              </button>
            </div>
          </div>

          <div class="setting-item">
            <div class="setting-info">
              <div class="setting-title">导入数据</div>
              <div class="setting-desc">从 JSON 文件导入剪贴板历史</div>
            </div>
            <div class="setting-control">
              <button class="btn-secondary" @click="importData">
                导入
              </button>
            </div>
          </div>
        </div>

        <h2 class="section-title">存储路径</h2>
        
        <div class="setting-group">
          <div class="setting-item">
            <div class="setting-info">
              <div class="setting-title">数据存储路径</div>
              <div class="setting-desc">剪贴板历史和设置数据存储位置</div>
            </div>
          </div>
          <div class="setting-item full-width">
            <div class="path-display">
              <span class="path-text">{{ storagePaths.data_dir }}</span>
              <button class="icon-btn" title="复制路径" @click="copyToClipboard(storagePaths.data_dir)">
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <rect x="9" y="9" width="13" height="13" rx="2" ry="2"/>
                  <path d="M5 15H4a2 2 0 01-2-2V4a2 2 0 012-2h9a2 2 0 012 2v1"/>
                </svg>
              </button>
              <button class="icon-btn" title="打开文件夹" @click="openFolder(storagePaths.data_dir)">
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"/>
                </svg>
              </button>
            </div>
          </div>

          <div class="setting-item">
            <div class="setting-info">
              <div class="setting-title">日志存储路径</div>
              <div class="setting-desc">应用日志文件存储位置</div>
            </div>
          </div>
          <div class="setting-item full-width">
            <div class="path-display">
              <span class="path-text">{{ storagePaths.log_dir }}</span>
              <button class="icon-btn" title="复制路径" @click="copyToClipboard(storagePaths.log_dir)">
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <rect x="9" y="9" width="13" height="13" rx="2" ry="2"/>
                  <path d="M5 15H4a2 2 0 01-2-2V4a2 2 0 012-2h9a2 2 0 012 2v1"/>
                </svg>
              </button>
              <button class="icon-btn" title="打开文件夹" @click="openFolder(storagePaths.log_dir)">
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"/>
                </svg>
              </button>
            </div>
          </div>
        </div>
      </div>

      <!-- 关于 -->
      <div v-if="activeMenu === 'about'" class="settings-section about-section">
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
                <button class="btn-primary btn-update" @click="downloadUpdate" :disabled="isDownloading">
                  <span v-if="isDownloading" class="btn-spinner"></span>
                  <svg v-else viewBox="0 0 20 20" fill="currentColor">
                    <path fill-rule="evenodd" d="M3 17a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zm3.293-7.707a1 1 0 011.414 0L9 10.586V3a1 1 0 112 0v7.586l1.293-1.293a1 1 0 111.414 1.414l-3 3a1 1 0 01-1.414 0l-3-3a1 1 0 010-1.414z" clip-rule="evenodd"/>
                  </svg>
                  {{ isDownloading ? '正在下载...' : '下载并安装' }}
                </button>
                <button class="btn-text" @click="skipUpdate" v-if="!isDownloading">跳过此版本</button>
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
                <button class="btn-ghost btn-small" @click="checkForUpdates">
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
                <button class="btn-ghost btn-small" @click="checkForUpdates">
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
                <button class="btn-primary btn-update" @click="checkForUpdates">
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
            <a href="#" class="link-card" @click.prevent="openExternal('https://github.com/lanxiuyun/paste-library')">
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

            <a href="#" class="link-card" @click.prevent="openExternal('https://github.com/lanxiuyun/paste-library/releases')">
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

            <a href="#" class="link-card" @click.prevent="openExternal('https://github.com/lanxiuyun/paste-library/issues/new')">
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

      <!-- 底部操作栏 -->
      <div v-if="activeMenu !== 'about'" class="settings-footer">
        <button class="btn-secondary" @click="resetSettings">恢复默认设置</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, reactive, onUnmounted, watch } from 'vue';
import { useSettings } from '@/composables/useSettings';
import { useClipboard } from '@/composables/useClipboard';
import { invoke } from '@tauri-apps/api/core';
import { listen, type UnlistenFn } from '@tauri-apps/api/event';
import { check, type Update } from '@tauri-apps/plugin-updater';
import { relaunch } from '@tauri-apps/plugin-process';
import type { AppSettings } from '@/types';

const { settings, loadSettings, saveSettings } = useSettings();
const { loadHistory } = useClipboard();

const activeMenu = ref('clipboard');

const menuItems = [
  { key: 'clipboard', label: '剪贴板' },
  { key: 'history', label: '历史记录' },
  { key: 'general', label: '通用设置' },
  { key: 'hotkey', label: '快捷键' },
  { key: 'backup', label: '数据备份' },
  { key: 'about', label: '关于' },
];

const form = reactive<AppSettings>({
  max_history_count: 5000,
  auto_cleanup_days: 30,
  window_position: 'remember',
  smart_activate: true,
  copy_sound: false,
  search_position: 'bottom',
  focus_search_on_activate: false,
  click_action: 'copy',
  double_click_action: 'paste',
  paste_shortcut: 'ctrl_v',
  hide_window_after_copy: false,
  image_ocr: false,
  copy_as_plain_text: false,
  paste_as_plain_text: true,
  confirm_delete: true,
  auto_sort: false,
  hotkey: 'Alt+V',
  auto_start: false,
  number_key_shortcut: 'ctrl',
});

const shortcutError = ref('');
const isRecordingHotkey = ref(false);
const isRecordingNumberKeyShortcut = ref(false);
const storagePaths = ref<Record<string, string>>({
  data_dir: '',
  log_dir: '',
});
let unlistenShortcutError: UnlistenFn | null = null;
let isInitializing = true;

// 格式化数字键修饰键显示
const formatNumberKeyShortcut = (shortcut: string): string => {
  if (!shortcut || shortcut === 'none') return '直接按数字键';
  return shortcut.split('+').map(s => s.charAt(0).toUpperCase() + s.slice(1)).join('+') + '+数字键';
};

// 录制数字键修饰键
const toggleNumberKeyShortcutRecording = () => {
  if (isRecordingNumberKeyShortcut.value) {
    // 停止录制
    isRecordingNumberKeyShortcut.value = false;
    window.removeEventListener('keydown', handleNumberKeyShortcutRecord);
  } else {
    // 开始录制
    isRecordingNumberKeyShortcut.value = true;
    window.addEventListener('keydown', handleNumberKeyShortcutRecord, { capture: true });
  }
};

const handleNumberKeyShortcutRecord = (e: KeyboardEvent) => {
  e.preventDefault();
  e.stopPropagation();

  const modifiers: string[] = [];
  if (e.ctrlKey) modifiers.push('ctrl');
  if (e.altKey) modifiers.push('alt');
  if (e.shiftKey) modifiers.push('shift');
  if (e.metaKey) modifiers.push('meta');

  // 获取按键
  let key = e.key.toLowerCase();

  // 如果是数字键，使用当前的修饰键组合
  if (key >= '1' && key <= '9') {
    // 如果只按了数字键没有修饰键，设置为 "none"
    if (modifiers.length === 0) {
      form.number_key_shortcut = 'none';
    } else {
      form.number_key_shortcut = modifiers.join('+');
    }

    // 停止录制
    isRecordingNumberKeyShortcut.value = false;
    window.removeEventListener('keydown', handleNumberKeyShortcutRecord, { capture: true });
    return;
  }

  // 如果按下了其他键（非数字键），则记录修饰键组合
  // 忽略单独的修饰键
  if (key === 'control' || key === 'alt' || key === 'shift' || key === 'meta') {
    return;
  }

  // 按其他键也停止录制并保存当前修饰键状态
  if (modifiers.length === 0) {
    form.number_key_shortcut = 'none';
  } else {
    form.number_key_shortcut = modifiers.join('+');
  }

  // 停止录制
  isRecordingNumberKeyShortcut.value = false;
  window.removeEventListener('keydown', handleNumberKeyShortcutRecord, { capture: true });
};

// 录制快捷键
const toggleHotkeyRecording = () => {
  if (isRecordingHotkey.value) {
    // 停止录制
    isRecordingHotkey.value = false;
    window.removeEventListener('keydown', handleHotkeyRecord);
  } else {
    // 开始录制
    isRecordingHotkey.value = true;
    window.addEventListener('keydown', handleHotkeyRecord, { capture: true });
  }
};

const handleHotkeyRecord = (e: KeyboardEvent) => {
  e.preventDefault();
  e.stopPropagation();

  const modifiers: string[] = [];
  if (e.ctrlKey) modifiers.push('Ctrl');
  if (e.altKey) modifiers.push('Alt');
  if (e.shiftKey) modifiers.push('Shift');
  if (e.metaKey) modifiers.push('Win');

  // 获取按键名称
  let key = e.key;

  // 忽略单独的修饰键
  if (key === 'Control' || key === 'Alt' || key === 'Shift' || key === 'Meta') {
    return;
  }

  // 标准化按键名称
  if (key === ' ') key = 'Space';
  if (key.length === 1) key = key.toUpperCase();

  // 组合快捷键
  const hotkeyParts = [...modifiers, key];
  const hotkeyString = hotkeyParts.join('+');

  // 验证并设置
  form.hotkey = hotkeyString;
  validateHotkey();

  // 停止录制
  isRecordingHotkey.value = false;
  window.removeEventListener('keydown', handleHotkeyRecord, { capture: true });
};

onMounted(async () => {
  await loadSettings();
  syncFromSettings();

  // 加载存储路径
  try {
    const paths = await invoke<Record<string, string>>('get_storage_paths');
    storagePaths.value = paths;
  } catch (error) {
    console.error('Failed to load storage paths:', error);
  }

  // 加载版本号并自动检查更新
  await loadAppVersion();
  // 延迟检查更新，避免影响页面加载速度
  setTimeout(() => {
    checkForUpdates();
  }, 1000);

  // 监听快捷键注册失败事件
  unlistenShortcutError = await listen<string>('shortcut-registration-failed', (event) => {
    shortcutError.value = `快捷键 "${event.payload}" 已被其他程序占用，请使用备用快捷键 Ctrl+Shift+V，或修改快捷键后重启应用`;
  });

  // 初始化完成后，开始监听变化
  isInitializing = false;
});

onUnmounted(() => {
  if (unlistenShortcutError) {
    unlistenShortcutError();
  }
  // 清理录制监听器
  if (isRecordingHotkey.value) {
    window.removeEventListener('keydown', handleHotkeyRecord, { capture: true });
  }
  if (isRecordingNumberKeyShortcut.value) {
    window.removeEventListener('keydown', handleNumberKeyShortcutRecord, { capture: true });
  }
});

const syncFromSettings = () => {
  Object.assign(form, settings.value);
};

// 自动保存：监听 form 变化，变化时自动保存
watch(form, async (newValue) => {
  if (isInitializing) return;
  
  try {
    await saveSettings({ ...newValue });
  } catch (error) {
    console.error('自动保存设置失败:', error);
  }
}, { deep: true });

const resetSettings = async () => {
  if (confirm('确定要恢复默认设置吗？')) {
    // 先标记为初始化中，避免重复保存
    isInitializing = true;

    form.max_history_count = 5000;
    form.auto_cleanup_days = 30;
    form.window_position = 'remember';
    form.smart_activate = true;
    form.copy_sound = false;
    form.search_position = 'top';
    form.focus_search_on_activate = false;
    form.click_action = 'copy'; // 'copy' | 'paste' | 'none'
    form.double_click_action = 'paste'; // 'copy' | 'paste' | 'none'
    form.paste_shortcut = 'ctrl_v';
    form.hide_window_after_copy = false;
    form.image_ocr = false;
    form.copy_as_plain_text = false;
    form.paste_as_plain_text = true;
    form.confirm_delete = true;
    form.auto_sort = false;
    form.auto_start = false;
    form.number_key_shortcut = 'ctrl';

    // 立即保存
    try {
      await saveSettings({ ...form });
    } catch (error) {
      console.error('保存默认设置失败:', error);
    }

    // 恢复监听
    isInitializing = false;
  }
};

const exportData = async () => {
  try {
    const jsonData = await invoke<string>('export_clipboard_data');
    
    // 创建 Blob 并下载
    const blob = new Blob([jsonData], { type: 'application/json' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `clipboard-backup-${new Date().toISOString().split('T')[0]}.json`;
    document.body.appendChild(a);
    a.click();
    document.body.removeChild(a);
    URL.revokeObjectURL(url);
    
    alert('导出成功！');
  } catch (error) {
    console.error('导出失败:', error);
    alert('导出失败，请重试');
  }
};

const importData = async () => {
  try {
    const input = document.createElement('input');
    input.type = 'file';
    input.accept = '.json';
    input.onchange = async (e: Event) => {
      const file = (e.target as HTMLInputElement).files?.[0];
      if (!file) return;
      
      const reader = new FileReader();
      reader.onload = async (event) => {
        try {
          const jsonData = event.target?.result as string;
          const count = await invoke<number>('import_clipboard_data', { jsonData });
          alert(`导入成功！共导入 ${count} 条记录`);
          await loadHistory();
        } catch (error) {
          console.error('导入失败:', error);
          alert('导入失败，请检查文件格式');
        }
      };
      reader.readAsText(file);
    };
    input.click();
  } catch (error) {
    console.error('导入失败:', error);
    alert('导入失败，请重试');
  }
};

const copyToClipboard = async (path: string) => {
  try {
    await navigator.clipboard.writeText(path);
    alert('路径已复制到剪贴板');
  } catch (error) {
    console.error('复制失败:', error);
    alert('复制失败，请重试');
  }
};

const openFolder = async (path: string) => {
  try {
    await invoke('show_in_folder', { path });
  } catch (error) {
    console.error('打开文件夹失败:', error);
    alert('打开文件夹失败，请重试');
  }
};

const clearAllHistory = async () => {
  const confirmed = confirm('确定要删除所有历史记录吗？此操作不可撤销！');
  if (!confirmed) return;

  try {
    await invoke('clear_clipboard_history', { 
      request: { keep_count: null, keep_days: null }
    });
    alert('历史记录已清空');
    await loadHistory();
  } catch (error) {
    console.error('清空历史记录失败:', error);
    alert('清空失败，请重试');
  }
};

const hotkeyError = ref('');

const validateHotkey = async () => {
  try {
    await invoke('validate_shortcut', { hotkey: form.hotkey });
    hotkeyError.value = '';
  } catch (error) {
    hotkeyError.value = '快捷键格式无效';
    // 恢复到默认值
    form.hotkey = 'Alt+V';
  }
};

// 更新检查相关状态
const currentVersion = ref('0.1.0');
const updateStatus = ref<'idle' | 'checking' | 'available' | 'uptodate' | 'error'>('idle');
const latestVersion = ref('');
const updateNotes = ref('');
const updateDate = ref('');
const isDownloading = ref(false);
const skippedVersion = ref<string | null>(null);
const downloadProgress = ref(0);
const downloadedBytes = ref(0);
const totalBytes = ref(0);
let updateInstance: Update | null = null;

// 获取当前版本号
const loadAppVersion = async () => {
  try {
    const version = await invoke<string>('get_app_version');
    currentVersion.value = version;
  } catch (error) {
    console.error('获取版本号失败:', error);
  }
};

// 检查更新
const checkForUpdates = async () => {
  updateStatus.value = 'checking';
  try {
    const update = await check({
      headers: {
        "X-AccessKey": "9SzxzOb3pQgkOB-LU-QU1Q",
      },
      timeout: 5000,
    });

    if (update) {
      console.log(`发现新版本 ${update.version}，发布于 ${update.date}`);

      // 检查是否已跳过此版本
      if (skippedVersion.value === update.version) {
        updateStatus.value = 'uptodate';
      } else {
        updateInstance = update;
        latestVersion.value = update.version;
        updateDate.value = update.date || '';
        updateNotes.value = update.body || '';
        updateStatus.value = 'available';
      }
    } else {
      updateStatus.value = 'uptodate';
    }
  } catch (error) {
    console.error('检查更新失败:', error);
    updateStatus.value = 'error';
  }
};

// 格式化更新日期
const formatUpdateDate = (dateStr: string | undefined): string => {
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


// 下载并安装更新
const downloadUpdate = async () => {
  if (!updateInstance) return;

  isDownloading.value = true;
  downloadProgress.value = 0;
  downloadedBytes.value = 0;
  totalBytes.value = 0;

  try {
    await updateInstance.downloadAndInstall((event) => {
      switch (event.event) {
        case 'Started':
          totalBytes.value = event.data.contentLength || 0;
          break;
        case 'Progress':
          downloadedBytes.value += event.data.chunkLength;
          if (totalBytes.value > 0) {
            downloadProgress.value = Math.round((downloadedBytes.value / totalBytes.value) * 100);
          }
          break;
        case 'Finished':
          console.log('下载完成');
          break;
      }
    });

    console.log('更新已安装，准备重启应用');
    await relaunch();
  } catch (error) {
    console.error('安装更新失败:', error);
    alert('安装更新失败，请稍后重试');
    isDownloading.value = false;
  }
};

// 跳过此版本
const skipUpdate = () => {
  skippedVersion.value = latestVersion.value;
  updateStatus.value = 'uptodate';
};

// 打开外部链接
const openExternal = async (url: string) => {
  try {
    await invoke('open_external_link', { url });
  } catch (error) {
    console.error('打开链接失败:', error);
    // 降级方案：使用浏览器打开
    window.open(url, '_blank');
  }
};
</script>

<style scoped>
.settings-view {
  height: 100vh;
  display: flex;
  background-color: #f5f5f5;
}

/* 左侧导航栏 */
.sidebar {
  width: 220px;
  background: #fff;
  border-right: 1px solid #e8e8e8;
  display: flex;
  flex-direction: column;
}

.nav-menu {
  flex: 1;
  padding: 12px;
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.nav-item {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 14px 16px;
  border: none;
  background: transparent;
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.2s;
  text-align: left;
}

.nav-item:hover {
  background: #f5f5f5;
}

.nav-item.active {
  background: #262626;
  color: #fff;
}

.nav-icon {
  width: 20px;
  height: 20px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.nav-icon svg {
  width: 100%;
  height: 100%;
}

.nav-label {
  font-size: 14px;
}

/* 右侧内容区 */
.content {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

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

.setting-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 16px 0;
  border-bottom: 1px solid #f0f0f0;
}

.setting-item:last-child {
  border-bottom: none;
}

.setting-item.full-width {
  flex-direction: column;
  align-items: flex-start;
  gap: 12px;
}

.setting-item.full-width .setting-control {
  width: 100%;
}

.setting-info {
  flex: 1;
}

.setting-title {
  font-size: 14px;
  font-weight: 500;
  color: #262626;
  margin-bottom: 4px;
}

.setting-desc {
  font-size: 12px;
  color: #8c8c8c;
}

.setting-control {
  display: flex;
  align-items: center;
  gap: 12px;
}

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

.hotkey-input {
  width: 120px;
  padding: 6px 10px;
  border: 1px solid #d9d9d9;
  border-radius: 4px;
  font-family: ui-monospace, SFMono-Regular, 'SF Mono', Menlo, Consolas, monospace;
  font-size: 13px;
  color: #262626;
  text-align: center;
}

.hotkey-input:focus {
  border-color: #262626;
  outline: none;
}

.setting-note {
  font-size: 11px;
  color: #faad14;
  margin-top: 4px;
}

.setting-warning {
  font-size: 11px;
  color: #ff4d4f;
  margin-top: 4px;
}

.error-message {
  font-size: 11px;
  color: #ff4d4f;
  margin-top: 4px;
  padding: 4px 8px;
  background: #fff2f0;
  border: 1px solid #ffccc7;
  border-radius: 4px;
}

.hotkey-input.has-error {
  border-color: #ff4d4f;
  background: #fff2f0;
}

.number-input {
  width: 80px;
  padding: 6px 10px;
  border: 1px solid #d9d9d9;
  border-radius: 4px;
  font-size: 13px;
  text-align: center;
  outline: none;
}

.number-input.small {
  width: 60px;
}

.number-input:focus {
  border-color: #262626;
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

.size-control {
  display: flex;
  align-items: center;
  gap: 8px;
}

.size-separator {
  color: #8c8c8c;
  font-size: 13px;
}

.textarea-input {
  width: 100%;
  padding: 10px 12px;
  border: 1px solid #d9d9d9;
  border-radius: 4px;
  font-size: 13px;
  resize: vertical;
  outline: none;
  font-family: inherit;
}

.textarea-input:focus {
  border-color: #262626;
}

.textarea-input::placeholder {
  color: #bfbfbf;
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

/* Icon button */
.icon-btn {
  width: 32px;
  height: 32px;
  border: none;
  background: transparent;
  cursor: pointer;
  border-radius: 4px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: #8c8c8c;
  transition: all 0.2s;
}

.icon-btn:hover {
  background: #f5f5f5;
  color: #262626;
}

.icon-btn svg {
  width: 16px;
  height: 16px;
}

/* 路径显示 */
.path-display {
  display: flex;
  align-items: center;
  gap: 8px;
  width: 100%;
  padding: 10px 12px;
  background: #fafafa;
  border: 1px solid #e8e8e8;
  border-radius: 4px;
  font-family: ui-monospace, SFMono-Regular, 'SF Mono', Menlo, Consolas, monospace;
  font-size: 12px;
}

.path-text {
  flex: 1;
  color: #595959;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

/* 关于页面 - 全新设计 */
.about-section {
  padding: 24px 32px !important;
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
  font-size: 14px;
  font-weight: 500;
  display: inline-flex;
  align-items: center;
  gap: 8px;
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
  
  .info-card-item {
    flex-wrap: wrap;
  }
  
  .info-actions {
    width: 100%;
    justify-content: flex-end;
    margin-top: 8px;
  }
  
  .links-grid {
    grid-template-columns: 1fr;
  }
}

/* 底部操作栏 */
.settings-footer {
  display: flex;
  align-items: flex-end;
  justify-content: flex-end;
  background: #fff;
  padding: 12px;
  border-top: 1px solid #e8e8e8;
}

.save-hint {
  font-size: 12px;
  color: #8c8c8c;
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

.btn-primary {
  padding: 8px 16px;
  background: #262626;
  border: none;
  border-radius: 6px;
  font-size: 13px;
  color: #fff;
  cursor: pointer;
  transition: background 0.2s;
}

.btn-primary:hover:not(:disabled) {
  background: #404040;
}

.btn-primary:disabled {
  background: #d9d9d9;
  cursor: not-allowed;
}

.btn-danger {
  padding: 8px 16px;
  background: #ff4d4f;
  border: none;
  border-radius: 6px;
  font-size: 13px;
  color: #fff;
  cursor: pointer;
  transition: background 0.2s;
}

.btn-danger:hover {
  background: #ff7875;
}

.btn-danger:active {
  background: #d9363e;
}

/* Scrollbar */
::-webkit-scrollbar {
  width: 6px;
}

::-webkit-scrollbar-track {
  background: transparent;
}

::-webkit-scrollbar-thumb {
  background: #c0c0c0;
  border-radius: 3px;
}

::-webkit-scrollbar-thumb:hover {
  background: #a0a0a0;
}
</style>
