export type IsoDateTime = string;

export type MigrationItemStatus = "migratable" | "blocked";
export type MigrationTargetKind = "source" | "analysis" | "work" | "unsupported";

export interface MigrationSource {
  rootPath: string;
  includePaths: string[];
}

export interface MigrationTarget {
  libraryRootPath: string;
}

export interface MigrationItemTarget {
  kind: MigrationTargetKind;
  libraryRelativePath: string;
}

export interface MigrationItem {
  relativePath: string;
  contentHash: string;
  target: MigrationItemTarget;
  status: MigrationItemStatus;
  scope: string;
  blockedReason?: string;
}

export interface MigrationReport {
  kind: "migration-report";
  generatedAt: IsoDateTime;
  totalFiles: number;
  migratableFiles: number;
  blockedFiles: number;
  sourceTreeHash: string;
}

export interface MigrationDryRun {
  kind: "migration-dry-run";
  generatedAt: IsoDateTime;
  source: MigrationSource;
  target: MigrationTarget;
  items: MigrationItem[];
  report: MigrationReport;
}

export function createToyMigrationDryRun(
  source: MigrationSource,
  target: MigrationTarget,
  timestamp: IsoDateTime,
): MigrationDryRun {
  return {
    kind: "migration-dry-run",
    generatedAt: timestamp,
    source,
    target,
    items: [
      {
        relativePath: "breakdowns/scene-notes.md",
        contentHash: "fnv1a64:toy",
        target: {
          kind: "analysis",
          libraryRelativePath: "analyses/breakdowns/scene-notes.json",
        },
        status: "migratable",
        scope: "structured-mechanism",
      },
    ],
    report: {
      kind: "migration-report",
      generatedAt: timestamp,
      totalFiles: 1,
      migratableFiles: 1,
      blockedFiles: 0,
      sourceTreeHash: "fnv1a64:toy-tree",
    },
  };
}

export function summarizeMigrationDryRun(dryRun: MigrationDryRun): string {
  return `${dryRun.report.migratableFiles} migratable, ${dryRun.report.blockedFiles} blocked`;
}
