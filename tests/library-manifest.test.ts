import { describe, expect, it } from "vitest";
import {
  createEmptyLibraryManifest,
  type AnalysisRecord,
  type ChangeSetRecord,
  type LibraryRoot,
  type RevisionRequestRecord,
  type SourceRecord,
  type WorkRecord,
} from "../src/domain/library";

describe("library manifest contract", () => {
  it("creates an empty manifest for a selected local root", () => {
    const root: LibraryRoot = {
      id: "root-local-demo",
      label: "Demo Library",
      path: "C:/Users/demo/InfiniteTypewriter",
    };

    const manifest = createEmptyLibraryManifest(
      root,
      "2026-06-05T00:00:00.000Z",
      "test-suite",
    );

    expect(manifest).toEqual({
      schemaVersion: 2,
      root,
      createdAt: "2026-06-05T00:00:00.000Z",
      updatedAt: "2026-06-05T00:00:00.000Z",
      lastWriterVersion: "test-suite",
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
    });
  });

  it("keeps source, work, and analysis records separated by kind", () => {
    const source: SourceRecord = {
      id: "source-toy",
      kind: "source",
      title: "Toy Source",
      contentHash: "fnv1a64:0000000000000000",
      createdAt: "2026-06-05T00:00:00.000Z",
      updatedAt: "2026-06-05T00:00:00.000Z",
    };
    const work: WorkRecord = {
      id: "work-toy",
      kind: "work",
      title: "Toy Work",
      createdAt: "2026-06-05T00:00:00.000Z",
      updatedAt: "2026-06-05T00:00:00.000Z",
    };
    const analysis: AnalysisRecord = {
      id: "analysis-toy",
      kind: "analysis",
      sourceId: "source-toy",
      title: "Toy Breakdown",
      createdAt: "2026-06-05T00:00:00.000Z",
      updatedAt: "2026-06-05T00:00:00.000Z",
    };

    const manifest = createEmptyLibraryManifest(
      {
        id: "root-local-demo",
        label: "Demo Library",
        path: "C:/Users/demo/InfiniteTypewriter",
      },
      "2026-06-05T00:00:00.000Z",
    );

    manifest.sources.push(source);
    manifest.works.push(work);
    manifest.analyses.push(analysis);

    expect(manifest.sources).toEqual([source]);
    expect(manifest.works).toEqual([work]);
    expect(manifest.analyses).toEqual([analysis]);
  });

  it("indexes revision requests and change sets as first-class manifest v2 records", () => {
    const manifest = createEmptyLibraryManifest(
      {
        id: "root-local-demo",
        label: "Demo Library",
        path: "C:/Users/demo/InfiniteTypewriter",
      },
      "2026-06-05T00:00:00.000Z",
    );
    const revisionRequest: RevisionRequestRecord = {
      id: "revision-request-toy",
      kind: "revision-request",
      targetRef: "evidence-toy-pressure",
      createdAt: "2026-06-05T00:00:00.000Z",
      updatedAt: "2026-06-05T00:00:00.000Z",
    };
    const changeSet: ChangeSetRecord = {
      id: "change-set-toy",
      kind: "change-set",
      targetRef: "scene-toy-gate",
      createdAt: "2026-06-05T00:00:00.000Z",
      updatedAt: "2026-06-05T00:00:00.000Z",
    };

    manifest.revisionRequests.push(revisionRequest);
    manifest.changeSets.push(changeSet);

    expect(manifest.schemaVersion).toBe(2);
    expect(manifest.revisionRequests).toEqual([revisionRequest]);
    expect(manifest.changeSets).toEqual([changeSet]);
    expect(manifest.migrationStatus.userConfirmationRequired).toBe(true);
    expect(manifest.exportPolicy.includeAiTaskAuditRecordsByDefault).toBe(false);
  });
});
