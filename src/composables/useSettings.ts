// src/composables/useSettings.ts
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import type { SettingsData, FolderPanel } from "@/types";

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

let settingsLoaded = false;

async function loadSettings() {
  if (settingsLoaded) return;
  settingsLoaded = true;

  try {
    const data = await invoke<SettingsData>("load_settings");
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

async function saveSettings() {
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
    await invoke("save_settings", { settings: data });
    alert("设置已保存到 setting.toml");
  } catch (err) {
    alert("保存设置失败: " + err);
  }
}

onMounted(() => {
  loadSettings();
});

export function useSettings() {
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
