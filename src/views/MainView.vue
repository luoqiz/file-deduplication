<!-- src/views/MainView.vue -->
<template>
  <div class="main-view">
    <div class="main-card">
      <h3>主程序</h3>
      <p>当前已选择文件夹: {{ selectedFolder || '未选择' }}</p>
      <button @click="handleConfirm" :disabled="confirmButtonDisabled" class="btn btn-confirm">
        执行整理
      </button>
      <p class="tip">完成前请先到"设置"页面检查源、备选项与后缀。</p>
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
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from "vue";
import { useSettings } from "@/composables/useSettings";
import { useFileOperations } from "@/composables/useFileOperations";

const {
  selectedFolder,
  operationMode,
  backupStrategy,
  sourcePanel,
  backupPanel,
  dryRun
} = useSettings();

const {
  processing,
  result,
  processFiles,
  clearResult,
  exportResult
} = useFileOperations();

const confirmButtonDisabled = computed(
  () => processing.value || !sourcePanel.value.selectedFolder || !backupPanel.value.selectedFolder
);

async function handleConfirm() {
  if (!sourcePanel.value.selectedFolder || !backupPanel.value.selectedFolder) {
    alert("请同时选择源目录和备目录");
    return;
  }

  try {
    const sourceExts = Array.from(sourcePanel.value.selectedExtensions);
    const backupExts = Array.from(backupPanel.value.selectedExtensions);

    const res = await processFiles({
      mainFolder: selectedFolder.value,
      sourceFolder: sourcePanel.value.selectedFolder,
      backupFolder: backupPanel.value.selectedFolder,
      sourceExtensions: sourceExts,
      backupExtensions: backupExts,
      mode: operationMode.value,
      backupStrategy: backupStrategy.value,
      dryRun: dryRun.value,
    });

    const action = dryRun.value ? "预览" : "整理";
    alert(`文件${action}完成！\n已处理：${res.moved_files.length}，跳过：${res.skipped_files.length}`);
  } catch (error) {
    alert("文件整理失败: " + error);
  }
}
</script>

<style scoped>
.main-view {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.main-card,
.status-panel {
  background: #fff;
  border: 1px solid #ddd;
  border-radius: 6px;
  padding: 14px;
}

.main-card h3 {
  margin-top: 0;
}

.btn {
  padding: 8px 16px;
  border: 1px solid #ccc;
  border-radius: 4px;
  cursor: pointer;
  background: #fff;
}

.btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.btn-confirm {
  background: #28a745;
  color: white;
  border-color: #28a745;
}

.btn-secondary {
  background: #6c757d;
  color: white;
  border-color: #6c757d;
}

.tip {
  color: #666;
  margin-top: 8px;
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
</style>