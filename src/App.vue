<template>
  <div class="app-container">
   <Sidebar :active-page="activePage" @page-change="handlePageChange" />
    <main class="app-main">
      <header class="main-header">
       <button @click="handleSelectMainFolder" class="btn btn-primary">选择文件夹</button>
        <span v-if="selectedFolder" class="folder-path">{{ selectedFolder }}</span>
      </header>

    <MainView v-if="activePage === 'main'" />
      <SettingsView v-if="activePage === 'settings'" />
      <ResultsView v-if="activePage === 'results'" />
    </main>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { useSettings } from "@/composables/useSettings";
import { useFileOperations } from "@/composables/useFileOperations";
import Sidebar from "@/components/Sidebar.vue";
import MainView from "@/views/MainView.vue";
import SettingsView from "@/views/SettingsView.vue";
import ResultsView from "@/views/ResultsView.vue";

const activePage = ref<"main" | "settings" | "results">("main");

const { selectedFolder, sourcePanel, backupPanel } = useSettings();
const { selectMainFolder, listFolder } = useFileOperations();

async function handleSelectMainFolder() {
  const folder = await selectMainFolder();
  if (folder) {
    selectedFolder.value = folder;
    const folders = await listFolder(folder);
    sourcePanel.value.folders = folders;
    backupPanel.value.folders = folders;
  }
}

function handlePageChange(page: string) {
  activePage.value = page as "main" | "settings" | "results";
}
</script>

<style>
.app-container {
  display: flex;
  height: 100vh;
  overflow: hidden;
}

.app-main {
  flex: 1;
  display: flex;
  flex-direction: column;
  background: #f8f9fa;
}

.main-header {
  background: #fff;
  border-bottom: 1px solid #ddd;
  padding: 12px;
  display: flex;
  align-items: center;
  gap: 10px;
}

.btn {
  padding: 8px 16px;
  border: 1px solid #ccc;
  border-radius: 4px;
  cursor: pointer;
}

.btn-primary {
  background: #007bff;
  color: white;
  border-color: #007bff;
}

.folder-path {
  color: #666;
  font-size: 14px;
}
</style>