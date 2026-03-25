<script setup lang="ts">
import { ref, computed, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";

// 常见文件后缀列表
const commonExtensions = [
  ".jpg", ".jpeg", ".png", ".ARW",
  ".mp4", ".avi", ".mkv", ".mov", ".flv", ".wmv",
  ".mp3", ".flac", ".wav", ".aac",
];

interface OrganizeResult {
  moved_files: string[];
  skipped_files: string[];
  created_folders: string[];
}

type BackupStrategy = "source" | "backup" | "largest" | "newest";

const selectedFolder = ref<string>("");
const activePage = ref<"main" | "settings">("main");
const operationMode = ref<"copy" | "move">("copy");
const backupStrategy = ref<BackupStrategy>("source");
const dryRun = ref(false);
const processing = ref(false);
const result = ref<OrganizeResult | null>(null);
const settingsKey = "file-deduplication-settings";

interface SettingsData {
  selectedFolder: string;
  operationMode: "copy" | "move";
  backupStrategy: BackupStrategy;
  dryRun: boolean;
  sourceFolder: string;
  backupFolder: string;
  sourceExtensions: string[];
  backupExtensions: string[];
}

function loadSettings() {
  try {
    const json = localStorage.getItem(settingsKey);
    if (!json) return;
    const data = JSON.parse(json) as SettingsData;
    selectedFolder.value = data.selectedFolder || "";
    operationMode.value = data.operationMode || "copy";
    backupStrategy.value = data.backupStrategy || "source";
    dryRun.value = data.dryRun || false;
    sourcePanel.value.selectedFolder = data.sourceFolder || "";
    backupPanel.value.selectedFolder = data.backupFolder || "";
    sourcePanel.value.selectedExtensions = new Set(data.sourceExtensions || []);
    backupPanel.value.selectedExtensions = new Set(data.backupExtensions || []);
  } catch (err) {
    console.warn("加载设置失败", err);
  }
}

function saveSettings() {
  try {
    const data: SettingsData = {
      selectedFolder: selectedFolder.value,
      operationMode: operationMode.value,
      backupStrategy: backupStrategy.value,
      dryRun: dryRun.value,
      sourceFolder: sourcePanel.value.selectedFolder,
      backupFolder: backupPanel.value.selectedFolder,
      sourceExtensions: Array.from(sourcePanel.value.selectedExtensions),
      backupExtensions: Array.from(backupPanel.value.selectedExtensions),
    };
    localStorage.setItem(settingsKey, JSON.stringify(data));
    alert("设置已保存");
  } catch (err) {
    alert("保存设置失败: " + err);
  }
}

onMounted(() => {
  loadSettings();
});
const sourcePanel = ref({
  selectedFolder: "",
  selectedExtensions: new Set<string>(),
  customExtension: "",
  folders: [] as string[],
});
const backupPanel = ref({
  selectedFolder: "",
  selectedExtensions: new Set<string>(),
  customExtension: "",
  folders: [] as string[],
});

const confirmButtonDisabled = computed(
  () => processing.value || !sourcePanel.value.selectedFolder || !backupPanel.value.selectedFolder
);

async function selectMainFolder() {
  try {
    const folder = await invoke<string>("select_folder");
    if (folder) {
      selectedFolder.value = folder;
      const folders = await invoke<string[]>("list_folder", { path: folder });
      sourcePanel.value.folders = folders;
      backupPanel.value.folders = folders;
    }
  } catch (error) {
    console.error("选择文件夹失败:", error);
  }
}

function toggleExtension(panel: "source" | "backup", ext: string) {
  const panelRef = panel === "source" ? sourcePanel : backupPanel;
  if (panelRef.value.selectedExtensions.has(ext)) {
    panelRef.value.selectedExtensions.delete(ext);
  } else {
    panelRef.value.selectedExtensions.add(ext);
  }
}

function addCustomExtension(panel: "source" | "backup") {
  const panelRef = panel === "source" ? sourcePanel : backupPanel;
  const ext = panelRef.value.customExtension.trim();
  if (ext && !ext.startsWith(".")) {
    panelRef.value.selectedExtensions.add("." + ext);
  } else if (ext) {
    panelRef.value.selectedExtensions.add(ext);
  }
  panelRef.value.customExtension = "";
}

async function handleConfirm() {
  if (!sourcePanel.value.selectedFolder || !backupPanel.value.selectedFolder) {
    alert("请同时选择源目录和备目录");
    return;
  }

  try {
    processing.value = true;
    result.value = null;
    const sourceExts = Array.from(sourcePanel.value.selectedExtensions);
    const backupExts = Array.from(backupPanel.value.selectedExtensions);

    const res = await invoke<OrganizeResult>("process_files", {
      mainFolder: selectedFolder.value,
      sourceFolder: sourcePanel.value.selectedFolder,
      backupFolder: backupPanel.value.selectedFolder,
      sourceExtensions: sourceExts,
      backupExtensions: backupExts,
      mode: operationMode.value,
      backupStrategy: backupStrategy.value,
      dryRun: dryRun.value,
    });

    result.value = res;
    const action = dryRun.value ? "预览" : "整理";
    alert(`文件${action}完成！\n已处理：${res.moved_files.length}，跳过：${res.skipped_files.length}`);
  } catch (error) {
    alert("文件整理失败: " + error);
  } finally {
    processing.value = false;
  }
}

function clearResult() {
  result.value = null;
}

function exportResult() {
  if (!result.value) {
    alert("当前没有可导出的结果");
    return;
  }

  const text = JSON.stringify(result.value, null, 2);
  const blob = new Blob([text], { type: "application/json" });
  const url = URL.createObjectURL(blob);
  const link = document.createElement("a");
  link.href = url;
  link.download = "deduplication-result.json";
  document.body.appendChild(link);
  link.click();
  document.body.removeChild(link);
  URL.revokeObjectURL(url);
}
</script>

<template>
  <div class="app-container">
    <aside class="app-sidebar">
      <button :class="['sidebar-item', { active: activePage === 'main' }]" @click="activePage = 'main'" title="主程序">📂</button>
      <button :class="['sidebar-item', { active: activePage === 'settings' }]" @click="activePage = 'settings'" title="设置">⚙️</button>
    </aside>

    <main class="app-main">
      <header class="main-header">
        <button @click="selectMainFolder" class="btn btn-primary">选择文件夹</button>
        <span v-if="selectedFolder" class="folder-path">{{ selectedFolder }}</span>
      </header>

      <section class="main-content" v-if="activePage === 'main'">
        <div class="main-card">
          <h3>主程序</h3>
          <p>当前已选择文件夹: {{ selectedFolder || '未选择' }}</p>
          <button @click="handleConfirm" :disabled="confirmButtonDisabled" class="btn btn-confirm">执行整理</button>
          <p class="tip">完成前请先到“设置”页面检查源、备选项与后缀。</p>
        </div>

        <div class="status-panel">
          <p v-if="processing">正在处理，请稍候...</p>
          <div class="action-buttons" v-if="result">
            <button class="btn btn-secondary" @click="exportResult">导出结果</button>
            <button class="btn btn-secondary" @click="clearResult" style="margin-left:8px">清空结果</button>
          </div>
          <div v-if="result">
            <p>已创建文件夹: {{ result.created_folders.length }}</p>
            <p>已处理文件: {{ result.moved_files.length }}</p>
            <p>已跳过: {{ result.skipped_files.length }}</p>
            <p v-if="dryRun" class="dry-run-notice">注意：这是预览模式，未执行实际操作</p>
            <div class="result-list">
              <h4>已处理文件</h4>
              <ul>
                <li v-for="item in result.moved_files" :key="item">{{ item }}</li>
              </ul>
              <h4>已跳过文件</h4>
              <ul>
                <li v-for="item in result.skipped_files" :key="item">{{ item }}</li>
              </ul>
            </div>
          </div>
        </div>
      </section>

      <section class="settings-content" v-if="activePage === 'settings'">
        <div class="setting-panel">
          <h3>基本配置</h3>
          <div class="mode-selector">
            <label>
              <input type="radio" value="copy" v-model="operationMode" /> 复制
            </label>
            <label>
              <input type="radio" value="move" v-model="operationMode" /> 移动
            </label>
          </div>
          <div class="dry-run-selector">
            <label>
              <input type="checkbox" v-model="dryRun" /> 预览模式（不执行实际操作）
            </label>
          </div>
          <div class="strategy-selector">
            <label>冲突优先策略:</label>
            <select v-model="backupStrategy">
              <option value="source">优先源</option>
              <option value="backup">优先备</option>
              <option value="largest">最大的文件</option>
              <option value="newest">最新修改</option>
            </select>
          </div>
        </div>

        <div class="setting-panel">
          <h3>源目录配置</h3>
          <div class="folder-area">
            <h4>源目录</h4>
            <div class="folder-list">
              <div v-for="folder in sourcePanel.folders" :key="folder" class="folder-item" :class="{ selected: sourcePanel.selectedFolder === folder }" @click="sourcePanel.selectedFolder = folder">📁 {{ folder }}</div>
            </div>
          </div>

          <h4>选择后缀</h4>
          <div class="extension-list">
            <div v-for="ext in commonExtensions" :key="ext" class="extension-group">
              <label>
                <input type="checkbox" @change="toggleExtension('source', ext)" :checked="sourcePanel.selectedExtensions.has(ext)" /> {{ ext }}
              </label>
            </div>
          </div>
          <div class="custom-extension">
            <input v-model="sourcePanel.customExtension" type="text" placeholder="自定义后缀（如 .txt）" /><button @click="addCustomExtension('source')" class="btn btn-small">添加</button>
          </div>
          <div v-if="sourcePanel.selectedExtensions.size > 0" class="selected-extensions">已选择: <span v-for="ext of sourcePanel.selectedExtensions" :key="ext" class="ext-tag">{{ ext }}</span></div>
        </div>

        <div class="setting-panel">
          <button class="btn btn-secondary" @click="saveSettings">保存设置</button>
          <button class="btn btn-secondary" @click="loadSettings" style="margin-left: 10px">加载设置</button>
        </div>

        <div class="setting-panel">
          <h3>备目录配置</h3>
          <div class="folder-area">
            <h4>备目录</h4>
            <div class="folder-list">
              <div v-for="folder in backupPanel.folders" :key="folder" class="folder-item" :class="{ selected: backupPanel.selectedFolder === folder }" @click="backupPanel.selectedFolder = folder">📁 {{ folder }}</div>
            </div>
          </div>

          <h4>选择后缀</h4>
          <div class="extension-list">
            <div v-for="ext in commonExtensions" :key="ext" class="extension-group">
              <label>
                <input type="checkbox" @change="toggleExtension('backup', ext)" :checked="backupPanel.selectedExtensions.has(ext)" /> {{ ext }}
              </label>
            </div>
          </div>
          <div class="custom-extension">
            <input v-model="backupPanel.customExtension" type="text" placeholder="自定义后缀（如 .txt）" /><button @click="addCustomExtension('backup')" class="btn btn-small">添加</button>
          </div>
          <div v-if="backupPanel.selectedExtensions.size > 0" class="selected-extensions">已选择: <span v-for="ext of backupPanel.selectedExtensions" :key="ext" class="ext-tag">{{ ext }}</span></div>
        </div>
      </section>
    </main>
  </div>
</template>

<style scoped>
.app-container {
  display: flex;
  height: 100vh;
  background: #f5f5f5;
}

.main-header {
  background: #fff;
  border-bottom: 1px solid #ddd;
  padding: 12px;
  display: flex;
  align-items: center;
  gap: 10px;
}

.folder-path {
  flex: 1;
  color: #666;
  font-size: 14px;
  word-break: break-all;
}

.mode-selector {
  display: flex;
  align-items: center;
  gap: 10px;
}
.mode-selector label {
  display: flex;
  align-items: center;
  gap: 4px;
  font-size: 14px;
}

.app-container {
  display: flex;
  height: 100vh;
}

.app-sidebar {
  width: 70px;
  background: #252f3f;
  color: #fff;
  padding: 8px;
  box-sizing: border-box;
  display: flex;
  flex-direction: column;
  gap: 8px;
  align-items: center;
}

.sidebar-title {
  font-size: 16px;
  font-weight: 600;
  margin-bottom: 12px;
}

.sidebar-item {
  width: 44px;
  height: 44px;
  padding: 0;
  font-size: 18px;
  display: flex;
  justify-content: center;
  align-items: center;
  border: none;
  background: transparent;
  color: #fff;
  text-align: center;
  border-radius: 4px;
  cursor: pointer;
}

.sidebar-item.active,
.sidebar-item:hover {
  background: #355269;
}

.app-main {
  flex: 1;
  display: flex;
  flex-direction: column;
  background: #f5f5f5;
  overflow: hidden;
}

.main-header {
  background: #fff;
  border-bottom: 1px solid #ddd;
  padding: 12px;
  display: flex;
  align-items: center;
  gap: 10px;
}

.main-content,
.settings-content {
  padding: 16px;
  overflow: auto;
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 16px;
  height: 0;
}

.setting-panel,
.main-card,
.status-panel {
  background: #fff;
  border: 1px solid #ddd;
  border-radius: 6px;
  padding: 14px;
}

.setting-panel h3,
.main-card h3 {
  margin-top: 0;
}

.strategy-selector {
  margin-top: 10px;
}

.strategy-selector label {
  margin-right: 8px;
}

.strategy-selector select {
  padding: 4px 6px;
  border: 1px solid #ccc;
  border-radius: 4px;
}

.folder-list {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
}

.folder-item {
  padding: 5px 8px;
  border: 1px solid #ccc;
  border-radius: 4px;
  cursor: pointer;
}

.folder-item.selected {
  background: #e6f1ff;
  border-color: #7aa8dd;
}

.tip {
  color: #666;
  margin-top: 8px;
}

.result-list {
  max-height: 220px;
  overflow: auto;
  margin-top: 10px;
}

.result-list h4 {
  margin: 10px 0 4px;
}

.result-list ul {
  list-style: none;
  padding-left: 0;
  margin: 0;
}

.result-list li {
  font-size: 13px;
  line-height: 1.4;
  color: #333;
}

.action-buttons {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 10px;
}

.dry-run-notice {
  color: #d32f2f;
  font-weight: bold;
  margin-top: 8px;
}

.bottom-panel {
  display: flex;
  flex: 1;
  gap: 0;
  overflow: hidden;
}

.status-panel {
  background: #fff;
  border-bottom: 1px solid #ddd;
  padding: 10px;
}

.result-list {
  max-height: 220px;
  overflow: auto;
  margin-top: 10px;
}

.result-list h4 {
  margin: 10px 0 4px;
}

.result-list ul {
  list-style: none;
  padding-left: 0;
  margin: 0;
}

.result-list li {
  font-size: 13px;
  line-height: 1.4;
  color: #333;
}

.side-panel {
  flex: 1;
  display: flex;
  flex-direction: column;
  border-right: 1px solid #ddd;
  background: white;
  overflow: hidden;
}

.side-panel:last-child {
  border-right: none;
}

.side-panel h3 {
  background: #f9f9f9;
  padding: 10px 15px;
  margin: 0;
  border-bottom: 1px solid #eee;
  font-size: 16px;
}

.extension-area {
  padding: 15px;
  border-bottom: 1px solid #eee;
  background: #fafafa;
}

.extension-list {
  margin-bottom: 10px;
  display: flex;
    flex-wrap: wrap;
    gap: 10px;
}

.extension-group {
  display: flex;
    align-items: center;
}

.extension-group label {
  display: flex;
  align-items: center;
    gap: 6px;
    cursor: pointer;
    font-size: 14px;
    white-space: nowrap;
}

.extension-group input {
  cursor: pointer;
}

.custom-extension {
  display: flex;
  gap: 5px;
  margin-bottom: 10px;
}

.custom-extension input {
  flex: 1;
  padding: 6px;
  border: 1px solid #ddd;
  border-radius: 4px;
  font-size: 12px;
}

.selected-extensions {
  font-size: 12px;
  color: #666;
}

.ext-tag {
  display: inline-block;
  background: #e3f2fd;
  color: #1976d2;
  padding: 2px 8px;
  border-radius: 3px;
  margin-left: 5px;
}

.folder-area {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.folder-area h4 {
  padding: 10px 15px;
  margin: 0;
  font-size: 14px;
  background: #f9f9f9;
  border-bottom: 1px solid #eee;
}

.folder-list {
  flex: 1;
  overflow-y: auto;
  padding: 10px 0;
}

.folder-item {
  padding: 10px 15px;
  cursor: pointer;
  border-left: 3px solid transparent;
    transition: background-color 0.2s;
}

.folder-item:hover {
  background: #f5f5f5;
}
.folder-item.selected {
  background: #e3f2fd;
  border-left-color: #1976d2;
  color: #1976d2;
}

.btn {
  padding: 8px 16px;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 14px;
  transition: background-color 0.2s;
}

.btn-primary {
  background: #1976d2;
  color: white;
}

.btn-primary:hover {
  background: #1565c0;
}

.btn-confirm {
  background: #388e3c;
  color: white;
}

.btn-confirm:hover:not(:disabled) {
  background: #2e7d32;
}
.btn-confirm:disabled {
  background: #ccc;
  cursor: not-allowed;
  opacity: 0.6;
}
.btn-small {
  background: #1976d2;
  color: white;
  padding: 4px 10px;
  font-size: 12px;
}

.btn-small:hover {
  background: #1565c0;
}
</style>

<style>
html, body, #app {
  margin: 0;
  padding: 0;
  width: 100%;
  height: 100%;
  overflow: hidden;
}
</style>