<template>
  <ul class="tree-list">
    <li v-for="node in nodes" :key="node.name">
      <span>{{ node.name }}</span>
      <ResultTree v-if="node.children.length" :nodes="node.children" />
    </li>
  </ul>
</template>

<script lang="ts">
import { defineComponent, PropType } from "vue";

interface TreeNode {
  name: string;
  children: TreeNode[];
}

export default defineComponent({
  name: "ResultTree",
  props: {
    nodes: {
      type: Array as PropType<TreeNode[]>,
      required: true,
    },
  },
});
</script>

<style scoped>
.tree-list {
  list-style: none;
  padding-left: 16px;
  margin: 0;
}

.tree-list li {
  position: relative;
  padding: 2px 0 2px 12px;
}

.tree-list li::before {
  content: "";
  position: absolute;
  left: 0;
  top: 11px;
  width: 8px;
  height: 1px;
  background: #bbb;
}

.tree-list span {
  display: inline-block;
}
</style>
