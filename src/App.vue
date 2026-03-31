<template>
  <div class="app-container">
    <Sidebar :active-page="activePage" @page-change="handlePageChange" />
    <main class="app-main">
      <MainView v-show="activePage === 'main'" />
      <SettingsView v-show="activePage === 'settings'" />
      <ResultsView v-show="activePage === 'results'" />
    </main>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";
import Sidebar from "@/components/Sidebar.vue";
import MainView from "@/views/MainView.vue";
import SettingsView from "@/views/SettingsView.vue";
import ResultsView from "@/views/ResultsView.vue";

const activePage = ref<"main" | "settings" | "results">("main");

function handlePageChange(page: string) {
  activePage.value = page as "main" | "settings" | "results";
}
</script>

<style>
html,
body,
#app {
  height: 100%;
  margin: 0;
}

body {
  overflow: hidden;
}

.app-container {
  display: flex;
  height: 100vh;
  overflow: hidden;
}

.app-main {
  flex: 1;
  display: flex;
  flex-direction: column;
  min-height: 0;
  overflow: hidden;
  background: #f8f9fa;
}

.app-main > * {
  flex: 1;
  min-height: 0;
  overflow: hidden;
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
</style>