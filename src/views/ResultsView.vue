<!-- src/views/ResultsView.vue -->
<template>
  <div class="results-view">
    <div class="results-panel" v-if="result">
      <h3>处理结果</h3>
      <div class="result-summary">
        <p>已创建文件夹: {{ result.created_folders.length }}</p>
        <p>已处理文件: {{ result.moved_files.length }}</p>
        <p>已跳过: {{ result.skipped_files.length }}</p>
        <p v-if="dryRun" class="dry-run-notice">注意：这是预览模式的结果，未执行实际操作</p>
      </div>

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

      <div class="action-buttons">
        <button class="btn btn-secondary" @click="exportResult">导出结果</button>
        <button class="btn btn-secondary" @click="clearResult" style="margin-left:8px">清空结果</button>
      </div>
    </div>

    <div class="no-results" v-else>
      <h3>暂无结果</h3>
      <p>请先在主程序页面执行文件整理操作</p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useFileOperations } from "@/composables/useFileOperations";
import { useSettings } from "@/composables/useSettings";

const { result, exportResult, clearResult } = useFileOperations();
const { dryRun } = useSettings();
</script>

<style scoped>
.results-view {
  display: flex;
  flex-direction: column;
  gap: 16px;
  flex: 1;
  min-height: 0;
  overflow: auto;
}

.results-panel,
.no-results {
  background: #fff;
  border: 1px solid #ddd;
  border-radius: 6px;
  padding: 14px;
}

.results-panel h3,
.no-results h3 {
  margin-top: 0;
}

.result-summary {
  margin-bottom: 16px;
}

.result-list {
  max-height: 400px;
  overflow: auto;
  margin-bottom: 16px;
}

.result-list h4 {
  margin: 16px 0 8px;
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
  padding: 2px 0;
}

.action-buttons {
  display: flex;
  align-items: center;
  gap: 8px;
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

.dry-run-notice {
  color: #d32f2f;
  font-weight: bold;
  margin-top: 8px;
}

.no-results {
  text-align: center;
  color: #666;
}
</style>