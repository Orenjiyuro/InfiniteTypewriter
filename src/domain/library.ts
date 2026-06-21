export type IsoDateTime = string;

export interface LibraryRoot {
  id: string;
  label: string;
  path: string;
}

export interface SourceRecord {
  id: string;
  kind: "source";
  title: string;
  contentHash: string;
  createdAt: IsoDateTime;
  updatedAt: IsoDateTime;
}

export interface WorkRecord {
  id: string;
  kind: "work";
  title: string;
  createdAt: IsoDateTime;
  updatedAt: IsoDateTime;
}

export interface AnalysisRecord {
  id: string;
  kind: "analysis";
  sourceId: string;
  title: string;
  createdAt: IsoDateTime;
  updatedAt: IsoDateTime;
}

export interface ReferenceBookRecord {
  id: string;
  kind: "reference-book";
  title: string;
  sourceId: string;
  createdAt: IsoDateTime;
  updatedAt: IsoDateTime;
}

export interface BreakdownProjectRecord {
  id: string;
  kind: "breakdown-project";
  title: string;
  sourceId: string;
  createdAt: IsoDateTime;
  updatedAt: IsoDateTime;
}

export interface ProviderRunRecord {
  id: string;
  kind: "provider-run";
  providerJobRef: string;
  createdAt: IsoDateTime;
  updatedAt: IsoDateTime;
}

export interface DraftRecord {
  id: string;
  kind: "draft";
  targetRef: string;
  createdAt: IsoDateTime;
  updatedAt: IsoDateTime;
}

export interface RevisionRequestRecord {
  id: string;
  kind: "revision-request";
  targetRef: string;
  createdAt: IsoDateTime;
  updatedAt: IsoDateTime;
}

export interface ChangeSetRecord {
  id: string;
  kind: "change-set";
  targetRef: string;
  createdAt: IsoDateTime;
  updatedAt: IsoDateTime;
}

export interface TransferableAssetRecord {
  id: string;
  kind: "transferable-asset";
  mechanismRef: string;
  createdAt: IsoDateTime;
  updatedAt: IsoDateTime;
}

export interface CraftReferenceRecord {
  id: string;
  kind: "craft-reference";
  title: string;
  createdAt: IsoDateTime;
  updatedAt: IsoDateTime;
}

export interface BackupRestoreStatus {
  lastDryRunAt?: IsoDateTime;
  lastBackupAt?: IsoDateTime;
  lastRestoreDryRunAt?: IsoDateTime;
  userConfirmationRequired: boolean;
  status: "not-run" | "dry-run-ready" | "confirmed" | "blocked" | "applied";
}

export interface MigrationStatus {
  currentSchemaVersion: number;
  pendingMigration: boolean;
  lastDryRunAt?: IsoDateTime;
  userConfirmationRequired: boolean;
  blockedReasons: string[];
}

export interface LibraryExportPolicy {
  includeAiTaskAuditRecordsByDefault: false;
  exportProviderSecrets: false;
  providerConfigExportMode: "summary-only" | "none";
}

export interface LibraryManifest {
  schemaVersion: 2;
  root: LibraryRoot;
  createdAt: IsoDateTime;
  updatedAt: IsoDateTime;
  lastWriterVersion: string;
  sources: SourceRecord[];
  works: WorkRecord[];
  analyses: AnalysisRecord[];
  referenceBooks: ReferenceBookRecord[];
  breakdownProjects: BreakdownProjectRecord[];
  providerRuns: ProviderRunRecord[];
  drafts: DraftRecord[];
  revisionRequests: RevisionRequestRecord[];
  changeSets: ChangeSetRecord[];
  transferableAssets: TransferableAssetRecord[];
  craftReferences: CraftReferenceRecord[];
  backupRestoreStatus: BackupRestoreStatus;
  migrationStatus: MigrationStatus;
  exportPolicy: LibraryExportPolicy;
}

export function createEmptyLibraryManifest(
  root: LibraryRoot,
  timestamp: IsoDateTime = new Date().toISOString(),
  lastWriterVersion = "unknown-writer",
): LibraryManifest {
  return {
    schemaVersion: 2,
    root,
    createdAt: timestamp,
    updatedAt: timestamp,
    lastWriterVersion,
    sources: [],
    works: [],
    analyses: [],
    referenceBooks: [],
    breakdownProjects: [],
    providerRuns: [],
    drafts: [],
    revisionRequests: [],
    changeSets: [],
    transferableAssets: [],
    craftReferences: [],
    backupRestoreStatus: {
      userConfirmationRequired: true,
      status: "not-run",
    },
    migrationStatus: {
      currentSchemaVersion: 2,
      pendingMigration: false,
      userConfirmationRequired: true,
      blockedReasons: [],
    },
    exportPolicy: {
      includeAiTaskAuditRecordsByDefault: false,
      exportProviderSecrets: false,
      providerConfigExportMode: "summary-only",
    },
  };
}
