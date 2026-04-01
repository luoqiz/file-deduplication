<!-- src/components/Sidebar.vue -->
<template>
  <aside class="app-sidebar">
    <button :class="['sidebar-item', { active: activePage === 'main' }]" @click="$emit('page-change', 'main')"
      title="主程序">
      📂
    </button>
    <button :class="['sidebar-item', { active: activePage === 'settings' }]" @click="$emit('page-change', 'settings')"
      title="设置">
      ⚙️
    </button>
    <button :class="['sidebar-item', { active: activePage === 'results' }]" @click="$emit('page-change', 'results')"
      title="结果">
      📊
    </button>
  </aside>
</template>

<script setup lang="ts">
interface Props {
  activePage: string;
}

defineProps<Props>();
defineEmits<{
  'page-change': [page: string];
}>();
</script>

<style scoped>
.app-sidebar {
  width: 60px;
  background: #f8f9fa;
  border-right: 1px solid #ddd;
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 16px 0;
  gap: 8px;
}

.sidebar-item {
  width: 40px;
  height: 40px;
  border: none;
  border-radius: 6px;
  background: transparent;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 18px;
  transition: all 0.2s;
}

.sidebar-item:hover {
  background: #e9ecef;
}

.sidebar-item.active {
  background: #007bff;
  color: white;
}

.sidebar-item.active:hover {
  background: #0056b3;
}
</style>