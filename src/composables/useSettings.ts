// src/composables/useSettings.ts
import { ref, onMounted } from "vue";
import type { SettingsData, FolderPanel } from "@/types";

const settingsKey = "file-deduplication-settings";

export function useSettings() {
  const selectedFolder = ref<string>("");
  const operationMode = ref<"copy" | "move">("copy");
  const backupStrategy = ref<"source" | "backup" | "largest" | "newest">(
    "source",
  );
  const dryRun = ref(false);

  const sourcePanel = ref<FolderPanel>({
    selectedFolder: "",
    selectedExtensions: new Set<string>(),
    customExtension: "",
    folders: [],
  });

  const backupPanel = ref<FolderPanel>({
    selectedFolder: "",
    selectedExtensions: new Set<string>(),
    customExtension: "",
    folders: [],
  });

  function loadSettings() {
    try {
      const json = localStorage.getItem(settingsKey);
      if (!json) return;
      const data = JSON.parse(json) as SettingsData;
      selectedFolder.value = data.selectedFolder || "";
      operationMode.value = data.operationMode || "copy";
      backupStrategy.value = data.backupStrategy || "source";
      dryRun.value = data.dryRun || false;
      sourcePanel.value.selectedFolder = data.sourceFolder || "";
      backupPanel.value.selectedFolder = data.backupFolder || "";
      sourcePanel.value.selectedExtensions = new Set(
        data.sourceExtensions || [],
      );
      backupPanel.value.selectedExtensions = new Set(
        data.backupExtensions || [],
      );
    } catch (err) {
      console.warn("加载设置失败", err);
    }
  }

  function saveSettings() {
    try {
      const data: SettingsData = {
        selectedFolder: selectedFolder.value,
        operationMode: operationMode.value,
        backupStrategy: backupStrategy.value,
        dryRun: dryRun.value,
        sourceFolder: sourcePanel.value.selectedFolder,
        backupFolder: backupPanel.value.selectedFolder,
        sourceExtensions: Array.from(sourcePanel.value.selectedExtensions),
        backupExtensions: Array.from(backupPanel.value.selectedExtensions),
      };
      localStorage.setItem(settingsKey, JSON.stringify(data));
      alert("设置已保存");
    } catch (err) {
      alert("保存设置失败: " + err);
    }
  }

  onMounted(() => {
    loadSettings();
  });

  return {
    selectedFolder,
    operationMode,
    backupStrategy,
    dryRun,
    sourcePanel,
    backupPanel,
    loadSettings,
    saveSettings,
  };
}
