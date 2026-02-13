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
        <div class="about-header">
          <h2 class="section-title">关于</h2>
        </div>
        
        <div class="about-grid">
          <!-- 应用信息卡片 -->
          <div class="about-card app-card">
            <div class="app-logo">
              <svg viewBox="0 0 48 48" fill="none">
                <rect width="48" height="48" rx="12" fill="#262626"/>
                <path d="M14 14h8v8h-8v-8zm12 0h8v8h-8v-8zm-12 12h8v12h-8V26zm12 0h8v12h-8V26z" fill="#fff"/>
              </svg>
            </div>
            <div class="app-details">
              <h3 class="app-name">Paste Library</h3>
              <p class="app-version">版本 {{ currentVersion }}</p>
              <p class="app-tagline">现代化的剪贴板管理工具</p>
            </div>
          </div>

          <!-- 更新卡片 -->
          <div class="about-card update-card">
            <div class="card-header">
              <div class="card-title">
                <svg viewBox="0 0 20 20" fill="currentColor">
                  <path fill-rule="evenodd" d="M4 2a1 1 0 011 1v2.101a7.002 7.002 0 0111.601 2.566 1 1 0 11-1.885.666A5.002 5.002 0 005.999 7H9a1 1 0 010 2H4a1 1 0 01-1-1V3a1 1 0 011-1zm.008 9.057a1 1 0 011.276.61A5.002 5.002 0 0014.001 13H11a1 1 0 110-2h5a1 1 0 011 1v5a1 1 0 11-2 0v-2.101a7.002 7.002 0 01-11.601-2.566 1 1 0 01.61-1.276z" clip-rule="evenodd"/>
                </svg>
                <span>检查更新</span>
              </div>
              <span class="version-badge" v-if="updateStatus === 'uptodate'">已是最新</span>
              <span class="version-badge update-available" v-else-if="updateStatus === 'available'">有新版本</span>
            </div>
            
            <div class="update-content">
              <!-- 检查中状态 -->
              <div v-if="updateStatus === 'checking'" class="update-state checking">
                <div class="state-icon checking">
                  <div class="spinner"></div>
                </div>
                <div class="state-text">
                  <p class="state-title">正在检查更新...</p>
                  <p class="state-desc">正在获取最新版本信息</p>
                </div>
              </div>

              <!-- 有可用更新 -->
              <div v-else-if="updateStatus === 'available'" class="update-state available">
                <div class="state-icon available">
                  <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <path d="M12 2L2 7l10 5 10-5-10-5zM2 17l10 5 10-5M2 12l10 5 10-5"/>
                  </svg>
                </div>
                <div class="state-text">
                  <p class="state-title">发现新版本 <strong>v{{ latestVersion }}</strong></p>
                  <p class="state-desc">{{ formatUpdateDate(updateDate) }}</p>
                </div>
                <div class="update-notes-preview" v-if="updateNotes">
                  <p>{{ truncateNotes(updateNotes) }}</p>
                </div>
                
                <!-- 下载进度 -->
                <div v-if="isDownloading" class="download-progress">
                  <div class="progress-header">
                    <span>正在下载更新...</span>
                    <span class="progress-percent">{{ downloadProgress }}%</span>
                  </div>
                  <div class="progress-bar">
                    <div class="progress-fill" :style="{ width: downloadProgress + '%' }"></div>
                  </div>
                </div>

                <div class="update-actions">
                  <button class="btn-primary btn-large" @click="downloadUpdate" :disabled="isDownloading">
                    <svg v-if="!isDownloading" viewBox="0 0 20 20" fill="currentColor">
                      <path fill-rule="evenodd" d="M3 17a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zm3.293-7.707a1 1 0 011.414 0L9 10.586V3a1 1 0 112 0v7.586l1.293-1.293a1 1 0 111.414 1.414l-3 3a1 1 0 01-1.414 0l-3-3a1 1 0 010-1.414z" clip-rule="evenodd"/>
                    </svg>
                    <div v-else class="btn-spinner"></div>
                    {{ isDownloading ? '正在下载...' : '下载并安装' }}
                  </button>
                  <button class="btn-ghost" @click="skipUpdate" v-if="!isDownloading">暂不更新</button>
                </div>
              </div>

              <!-- 已是最新 -->
              <div v-else-if="updateStatus === 'uptodate'" class="update-state success">
                <div class="state-icon success">
                  <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <path d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"/>
                  </svg>
                </div>
                <div class="state-text">
                  <p class="state-title">当前已是最新版本</p>
                  <p class="state-desc">您正在使用最新功能</p>
                </div>
                <button class="btn-ghost btn-small" @click="checkForUpdates">
                  <svg viewBox="0 0 20 20" fill="currentColor">
                    <path fill-rule="evenodd" d="M4 2a1 1 0 011 1v2.101a7.002 7.002 0 0111.601 2.566 1 1 0 11-1.885.666A5.002 5.002 0 005.999 7H9a1 1 0 010 2H4a1 1 0 01-1-1V3a1 1 0 011-1zm.008 9.057a1 1 0 011.276.61A5.002 5.002 0 0014.001 13H11a1 1 0 110-2h5a1 1 0 011 1v5a1 1 0 11-2 0v-2.101a7.002 7.002 0 01-11.601-2.566 1 1 0 01.61-1.276z" clip-rule="evenodd"/>
                  </svg>
                  重新检查
                </button>
              </div>

              <!-- 检查失败 -->
              <div v-else-if="updateStatus === 'error'" class="update-state error">
                <div class="state-icon error">
                  <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <path d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"/>
                  </svg>
                </div>
                <div class="state-text">
                  <p class="state-title">检查更新失败</p>
                  <p class="state-desc">请检查网络连接后重试</p>
                </div>
                <button class="btn-ghost btn-small" @click="checkForUpdates">
                  <svg viewBox="0 0 20 20" fill="currentColor">
                    <path fill-rule="evenodd" d="M4 2a1 1 0 011 1v2.101a7.002 7.002 0 0111.601 2.566 1 1 0 11-1.885.666A5.002 5.002 0 005.999 7H9a1 1 0 010 2H4a1 1 0 01-1-1V3a1 1 0 011-1zm.008 9.057a1 1 0 011.276.61A5.002 5.002 0 0014.001 13H11a1 1 0 110-2h5a1 1 0 011 1v5a1 1 0 11-2 0v-2.101a7.002 7.002 0 01-11.601-2.566 1 1 0 01.61-1.276z" clip-rule="evenodd"/>
                  </svg>
                  重试
                </button>
              </div>

              <!-- 初始状态 -->
              <div v-else class="update-state idle">
                <div class="state-icon idle">
                  <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <path d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"/>
                  </svg>
                </div>
                <div class="state-text">
                  <p class="state-title">检查更新</p>
                  <p class="state-desc">获取最新版本信息</p>
                </div>
                <button class="btn-primary btn-large" @click="checkForUpdates">
                  <svg viewBox="0 0 20 20" fill="currentColor">
                    <path fill-rule="evenodd" d="M4 2a1 1 0 011 1v2.101a7.002 7.002 0 0111.601 2.566 1 1 0 11-1.885.666A5.002 5.002 0 005.999 7H9a1 1 0 010 2H4a1 1 0 01-1-1V3a1 1 0 011-1zm.008 9.057a1 1 0 011.276.61A5.002 5.002 0 0014.001 13H11a1 1 0 110-2h5a1 1 0 011 1v5a1 1 0 11-2 0v-2.101a7.002 7.002 0 01-11.601-2.566 1 1 0 01.61-1.276z" clip-rule="evenodd"/>
                  </svg>
                  检查更新
                </button>
              </div>
            </div>
          </div>

          <!-- 系统信息卡片 -->
          <div class="about-card info-card full-width">
            <div class="card-header">
              <div class="card-title">
                <svg viewBox="0 0 20 20" fill="currentColor">
                  <path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7-4a1 1 0 11-2 0 1 1 0 012 0zM9 9a1 1 0 000 2v3a1 1 0 001 1h1a1 1 0 100-2v-3a1 1 0 00-1-1H9z" clip-rule="evenodd"/>
                </svg>
                <span>系统信息</span>
              </div>
            </div>
            <div class="info-list">
              <div class="info-item">
                <span class="info-label">数据目录</span>
                <span class="info-value">{{ storagePaths.data_dir || '加载中...' }}</span>
              </div>
              <div class="info-item">
                <span class="info-label">日志目录</span>
                <span class="info-value">{{ storagePaths.log_dir || '加载中...' }}</span>
              </div>
            </div>
          </div>
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

// 截断更新说明
const truncateNotes = (notes: string, maxLength = 100): string => {
  if (notes.length <= maxLength) return notes;
  return notes.substring(0, maxLength) + '...';
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

/* 关于页面 - 新布局 */
.about-section {
  padding: 20px 24px !important;
}

.about-header {
  margin-bottom: 20px;
}

.about-grid {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 16px;
}

.about-card {
  background: #fff;
  border-radius: 12px;
  padding: 20px;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.04);
  border: 1px solid #f0f0f0;
}

.about-card.app-card,
.about-card.update-card,
.about-card.info-card {
  grid-column: span 2;
}

.about-card.app-card {
  display: flex;
  align-items: center;
  gap: 20px;
  padding: 28px;
}

.app-logo {
  width: 72px;
  height: 72px;
  flex-shrink: 0;
}

.app-logo svg {
  width: 100%;
  height: 100%;
}

.app-details {
  flex: 1;
}

.app-name {
  font-size: 22px;
  font-weight: 600;
  color: #262626;
  margin: 0 0 6px 0;
}

.app-version {
  font-size: 13px;
  color: #8c8c8c;
  margin: 0 0 4px 0;
}

.app-tagline {
  font-size: 14px;
  color: #595959;
  margin: 0;
}

.card-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 16px;
}

.card-title {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 14px;
  font-weight: 600;
  color: #262626;
}

.card-title svg {
  width: 18px;
  height: 18px;
  color: #595959;
}

.version-badge {
  font-size: 11px;
  padding: 3px 8px;
  border-radius: 10px;
  background: #f5f5f5;
  color: #8c8c8c;
  font-weight: 500;
}

.version-badge.update-available {
  background: #e6f7ff;
  color: #1890ff;
}

/* 更新卡片 */
.update-card {
  grid-column: span 2;
}

.update-content {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.update-state {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.update-state.checking,
.update-state.success,
.update-state.error {
  flex-direction: row;
  align-items: center;
  padding: 16px;
  background: #fafafa;
  border-radius: 8px;
}

.state-icon {
  width: 44px;
  height: 44px;
  border-radius: 12px;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
}

.state-icon svg {
  width: 22px;
  height: 22px;
}

.state-icon.checking {
  background: #e6f7ff;
}

.state-icon.checking .spinner {
  width: 20px;
  height: 20px;
  border: 2px solid #1890ff;
  border-top-color: transparent;
  border-radius: 50%;
  animation: spin 0.8s linear infinite;
}

.state-icon.available {
  background: #e6f7ff;
  color: #1890ff;
}

.state-icon.success {
  background: #f6ffed;
  color: #52c41a;
}

.state-icon.error {
  background: #fff2f0;
  color: #ff4d4f;
}

.state-icon.idle {
  background: #f5f5f5;
  color: #8c8c8c;
}

.state-text {
  flex: 1;
}

.state-title {
  font-size: 14px;
  font-weight: 500;
  color: #262626;
  margin: 0 0 2px 0;
}

.state-title strong {
  color: #1890ff;
}

.state-desc {
  font-size: 12px;
  color: #8c8c8c;
  margin: 0;
}

.update-notes-preview {
  padding: 12px;
  background: #fafafa;
  border-radius: 6px;
  font-size: 12px;
  color: #595959;
  line-height: 1.5;
}

.update-actions {
  display: flex;
  gap: 10px;
  margin-top: 4px;
}

.btn-large {
  padding: 10px 20px;
  font-size: 13px;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
}

.btn-large svg {
  width: 16px;
  height: 16px;
}

.btn-small {
  padding: 6px 12px;
  font-size: 12px;
  display: flex;
  align-items: center;
  gap: 6px;
}

.btn-small svg {
  width: 14px;
  height: 14px;
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

/* 下载进度 */
.download-progress {
  padding: 12px;
  background: #fafafa;
  border-radius: 8px;
}

.progress-header {
  display: flex;
  justify-content: space-between;
  font-size: 12px;
  color: #595959;
  margin-bottom: 8px;
}

.progress-percent {
  font-weight: 500;
  color: #1890ff;
}

.download-progress .progress-bar {
  height: 6px;
}

.download-progress .progress-fill {
  background: linear-gradient(90deg, #1890ff, #40a9ff);
}

/* 系统信息卡片 */
.info-card .info-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.info-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 10px 12px;
  background: #fafafa;
  border-radius: 6px;
}

.info-label {
  font-size: 12px;
  color: #8c8c8c;
}

.info-value {
  font-size: 12px;
  color: #595959;
  font-family: ui-monospace, SFMono-Regular, 'SF Mono', Menlo, Consolas, monospace;
  max-width: 180px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

/* 响应式 */
@media (max-width: 600px) {
  .about-grid {
    grid-template-columns: 1fr;
  }
  
  .about-card.app-card,
  .about-card.update-card {
    grid-column: span 1;
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
