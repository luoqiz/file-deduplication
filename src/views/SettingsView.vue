<!-- src/views/SettingsView.vue -->
<template>
  <div class="settings-view">
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
      <FolderConfigPanel v-model:selected-folder="sourcePanel.selectedFolder"
        v-model:selected-extensions="sourcePanel.selectedExtensions"
        v-model:custom-extension="sourcePanel.customExtension" :folders="sourcePanel.folders" title="源目录" />
    </div>

    <div class="setting-panel">
      <h3>备目录配置</h3>
      <FolderConfigPanel v-model:selected-folder="backupPanel.selectedFolder"
        v-model:selected-extensions="backupPanel.selectedExtensions"
        v-model:custom-extension="backupPanel.customExtension" :folders="backupPanel.folders" title="备目录" />
    </div>

    <div class="setting-panel">
      <button class="btn btn-secondary" @click="saveSettings">保存设置</button>
      <button class="btn btn-secondary" @click="loadSettings" style="margin-left: 10px">加载设置</button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useSettings } from "@/composables/useSettings";
import FolderConfigPanel from "@/components/FolderConfigPanel.vue";

const {
  operationMode,
  backupStrategy,
  dryRun,
  sourcePanel,
  backupPanel,
  saveSettings,
  loadSettings,
} = useSettings();
</script>

<style scoped>
.settings-view {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.setting-panel {
  background: #fff;
  border: 1px solid #ddd;
  border-radius: 6px;
  padding: 14px;
}

.setting-panel h3 {
  margin-top: 0;
}

.mode-selector,
.dry-run-selector {
  margin-bottom: 10px;
}

.mode-selector label,
.dry-run-selector label {
  margin-right: 16px;
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

.btn {
  padding: 8px 16px;
  border: 1px solid #ccc;
  border-radius: 4px;
  cursor: pointer;
  background: #fff;
}

.btn-secondary {
  background: #6c757d;
  color: white;
  border-color: #6c757d;
}
</style>