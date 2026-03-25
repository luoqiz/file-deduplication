// src/types/index.ts
export interface OrganizeResult {
  moved_files: string[];
  skipped_files: string[];
  created_folders: string[];
}

export type BackupStrategy = "source" | "backup" | "largest" | "newest";

export type OperationMode = "copy" | "move";

export interface SettingsData {
  selectedFolder: string;
  operationMode: OperationMode;
  backupStrategy: BackupStrategy;
  dryRun: boolean;
  sourceFolder: string;
  backupFolder: string;
  sourceExtensions: string[];
  backupExtensions: string[];
}

export interface FolderPanel {
  selectedFolder: string;
  selectedExtensions: Set<string>;
  customExtension: string;
  folders: string[];
}
