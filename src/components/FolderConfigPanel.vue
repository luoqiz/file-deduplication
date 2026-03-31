<!-- src/components/FolderConfigPanel.vue -->
<template>
  <div class="folder-config-panel">
    <div class="folder-area">
      <h4>{{ title }}</h4>
      <div class="folder-list">
        <div v-for="folder in folders" :key="folder" class="folder-item"
          :class="{ selected: selectedFolders.includes(folder) }" @click="toggleFolder(folder)">
          📁 {{ folder }}
        </div>
      </div>
    </div>

    <h4>选择后缀</h4>
    <div class="extension-list">
      <div v-for="ext in commonExtensions" :key="ext" class="extension-group">
        <label>
          <input type="checkbox" @change="toggleExtension(ext)" :checked="selectedExtensions.has(ext)" /> {{ ext }}
        </label>
      </div>
    </div>
    <div class="custom-extension">
      <input :value="customExtension" @input="handleCustomExtensionInput" type="text" placeholder="自定义后缀（如 .txt）" />
      <button @click="addCustomExtension" class="btn btn-small">添加</button>
    </div>
    <div v-if="selectedExtensions.size > 0" class="selected-extensions">
      已选择: <span v-for="ext of selectedExtensions" :key="ext" class="ext-tag">{{ ext }}</span>
    </div>
  </div>
</template>

<script setup lang="ts">

// 常见文件后缀列表
const commonExtensions = [
  ".jpg", ".jpeg", ".png", ".ARW",
  ".mp4", ".avi", ".mkv", ".mov", ".flv", ".wmv",
  ".mp3", ".flac", ".wav", ".aac",
];

interface Props {
  title: string;
  folders: string[];
  selectedFolders: string[];
  selectedExtensions: Set<string>;
  customExtension: string;
}

const props = defineProps<Props>();

const emit = defineEmits<{
  'update:selectedFolders': [value: string[]];
  'update:selectedExtensions': [value: Set<string>];
  'update:customExtension': [value: string];
}>();

function handleCustomExtensionInput(event: Event) {
  const target = event.target as HTMLInputElement;
  emit('update:customExtension', target.value);
}

function toggleFolder(folder: string) {
  const newFolders = [...props.selectedFolders];
  const index = newFolders.indexOf(folder);
  if (index >= 0) {
    newFolders.splice(index, 1);
  } else {
    newFolders.push(folder);
  }
  emit('update:selectedFolders', newFolders);
}

function toggleExtension(ext: string) {
  const newSet = new Set(props.selectedExtensions);
  if (newSet.has(ext)) {
    newSet.delete(ext);
  } else {
    newSet.add(ext);
  }
  emit('update:selectedExtensions', newSet);
}

function addCustomExtension() {
  const ext = props.customExtension.trim();
  if (ext && !ext.startsWith(".")) {
    const newSet = new Set(props.selectedExtensions);
    newSet.add("." + ext);
    emit('update:selectedExtensions', newSet);
  } else if (ext) {
    const newSet = new Set(props.selectedExtensions);
    newSet.add(ext);
    emit('update:selectedExtensions', newSet);
  }
  emit('update:customExtension', "");
}
</script>

<style scoped>
.folder-config-panel {
  display: flex;
  flex-direction: column;
  gap: 12px;
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

.extension-list {
  display: flex;
  flex-wrap: wrap;
  gap: 12px;
}

.extension-group {
  display: flex;
  align-items: center;
}

.custom-extension {
  display: flex;
  gap: 8px;
  align-items: center;
}

.custom-extension input {
  flex: 1;
  padding: 4px 6px;
  border: 1px solid #ccc;
  border-radius: 4px;
}

.btn {
  padding: 4px 8px;
  border: 1px solid #ccc;
  border-radius: 4px;
  cursor: pointer;
  background: #fff;
}

.btn-small {
  padding: 4px 8px;
  font-size: 12px;
}

.selected-extensions {
  margin-top: 8px;
  font-size: 14px;
}

.ext-tag {
  display: inline-block;
  background: #f0f0f0;
  padding: 2px 6px;
  margin: 0 4px;
  border-radius: 3px;
  font-size: 12px;
}
</style>