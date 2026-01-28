<script setup lang="ts">
import { ref, computed } from "vue";
import { invoke } from "@tauri-apps/api/core";

// 常见文件后缀列表
const commonExtensions = [
  ".jpg", ".jpeg", ".png", ".ARW",
  ".mp4", ".avi", ".mkv", ".mov", ".flv", ".wmv",
  ".mp3", ".flac", ".wav", ".aac",
];

const selectedFolder = ref<string>("");
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
  () => !sourcePanel.value.selectedFolder || !backupPanel.value.selectedFolder
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
    const sourceExts = Array.from(sourcePanel.value.selectedExtensions);
    const backupExts = Array.from(backupPanel.value.selectedExtensions);

    const result = await invoke("process_files", {
      mainFolder: selectedFolder.value,
      sourceFolder: sourcePanel.value.selectedFolder,
      backupFolder: backupPanel.value.selectedFolder,
      sourceExtensions: sourceExts,
      backupExtensions: backupExts,
    });

    alert("文件整理完成！\n" + JSON.stringify(result, null, 2));
  } catch (error) {
    alert("文件整理失败: " + error);
  }
}
</script>

<template>
 <div class="app-container">
    <!-- 上面板 -->
    <div class="top-panel">
      <button @click="selectMainFolder" class="btn btn-primary">选择文件夹</button>
      <span v-if="selectedFolder" class="folder-path">{{ selectedFolder }}</span>
      <button @click="handleConfirm" :disabled="confirmButtonDisabled" class="btn btn-confirm">
        确认
      </button>
    </div>

  <!-- 下面板 -->
    <div class="bottom-panel">
      <!-- 源面板 -->
      <div class="side-panel source-panel">
        <h3>源面板</h3>

        <!-- 后缀选择区域 -->
        <div class="extension-area">
          <div class="extension-list">
           <div v-for="ext in commonExtensions" :key="ext" class="extension-group">
              <label>
               <input type="checkbox" @change="toggleExtension('source', ext)"
                  :checked="sourcePanel.selectedExtensions.has(ext)" />
                {{ ext }}
              </label>
            </div>
         </div>
          <div class="custom-extension">
            <input v-model="sourcePanel.customExtension" type="text" placeholder="自定义后缀（如 .txt）" />
            <button @click="addCustomExtension('source')" class="btn btn-small">添加</button>
          </div>
          <div v-if="sourcePanel.selectedExtensions.size > 0" class="selected-extensions">
            已选择:
            <span v-for="ext of sourcePanel.selectedExtensions" :key="ext" class="ext-tag">
              {{ ext }}
            </span>
          </div>
        </div>

      <!-- 文件夹选择区域 -->
        <div class="folder-area">
          <h4>选择文件夹</h4>
          <div class="folder-list">
            <div v-for="folder in sourcePanel.folders" :key="folder" class="folder-item"
              :class="{ selected: sourcePanel.selectedFolder === folder }" @click="sourcePanel.selectedFolder = folder">
              📁 {{ folder }}
            </div>
          </div>
        </div>
      </div>

      <!-- 备面板 -->
      <div class="side-panel backup-panel">
        <h3>备面板</h3>

        <!-- 后缀选择区域 -->
        <div class="extension-area">
          <div class="extension-list">
           <div v-for="ext in commonExtensions" :key="ext" class="extension-group">
              <label>
               <input type="checkbox" @change="toggleExtension('backup', ext)"
                  :checked="backupPanel.selectedExtensions.has(ext)" />
                {{ ext }}
              </label>
            </div>
          </div>
          <div class="custom-extension">
            <input v-model="backupPanel.customExtension" type="text" placeholder="自定义后缀（如 .txt）" />
            <button @click="addCustomExtension('backup')" class="btn btn-small">添加</button>
          </div>
          <div v-if="backupPanel.selectedExtensions.size > 0" class="selected-extensions">
            已选择:
            <span v-for="ext of backupPanel.selectedExtensions" :key="ext" class="ext-tag">
              {{ ext }}
            </span>
          </div>
        </div>

        <!-- 文件夹选择区域 -->
        <div class="folder-area">
          <h4>选择文件夹</h4>
          <div class="folder-list">
            <div v-for="folder in backupPanel.folders" :key="folder" class="folder-item"
              :class="{ selected: backupPanel.selectedFolder === folder }" @click="backupPanel.selectedFolder = folder">
              📁 {{ folder }}
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.app-container {
  display: flex;
  flex-direction: column;
  height: 100vh;
  background: #f5f5f5;
}

.top-panel {
  background: white;
  padding: 15px;
  border-bottom: 1px solid #ddd;
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

.bottom-panel {
  display: flex;
  flex: 1;
  gap: 0;
  overflow: hidden;
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