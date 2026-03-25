// src/composables/useFileOperations.ts
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import type { OrganizeResult } from "@/types";

export function useFileOperations() {
  const processing = ref(false);
  const result = ref<OrganizeResult | null>(null);

  async function selectMainFolder() {
    try {
      const folder = await invoke<string>("select_folder");
      if (folder) {
        return folder;
      }
    } catch (error) {
      console.error("选择文件夹失败:", error);
    }
    return null;
  }

  async function listFolder(path: string) {
    try {
      const folders = await invoke<string[]>("list_folder", { path });
      return folders;
    } catch (error) {
      console.error("列出文件夹失败:", error);
      return [];
    }
  }

  async function processFiles(params: {
    mainFolder: string;
    sourceFolder: string;
    backupFolder: string;
    sourceExtensions: string[];
    backupExtensions: string[];
    mode: string;
    backupStrategy: string;
    dryRun: boolean;
  }) {
    try {
      processing.value = true;
      result.value = null;
      const res = await invoke<OrganizeResult>("process_files", params);
      result.value = res;
      return res;
    } catch (error) {
      throw error;
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

  return {
    processing,
    result,
    selectMainFolder,
    listFolder,
    processFiles,
    clearResult,
    exportResult,
  };
}
