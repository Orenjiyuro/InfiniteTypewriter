import { describe, expect, it } from "vitest";
import librarySchema from "../schemas/library.schema.json";

describe("library JSON schema", () => {
  it("publishes the 9.1 public library record contracts", () => {
    expect(librarySchema.$defs).toHaveProperty("LibraryRoot");
    expect(librarySchema.$defs).toHaveProperty("LibraryManifest");
    expect(librarySchema.$defs).toHaveProperty("SourceRecord");
    expect(librarySchema.$defs).toHaveProperty("WorkRecord");
    expect(librarySchema.$defs).toHaveProperty("AnalysisRecord");
    expect(librarySchema.$defs).toHaveProperty("ReferenceBookRecord");
    expect(librarySchema.$defs).toHaveProperty("BreakdownProjectRecord");
    expect(librarySchema.$defs).toHaveProperty("ProviderRunRecord");
    expect(librarySchema.$defs).toHaveProperty("DraftRecord");
    expect(librarySchema.$defs).toHaveProperty("RevisionRequestRecord");
    expect(librarySchema.$defs).toHaveProperty("ChangeSetRecord");
    expect(librarySchema.$defs).toHaveProperty("TransferableAssetRecord");
    expect(librarySchema.$defs).toHaveProperty("CraftReferenceRecord");
    expect(librarySchema.$defs).toHaveProperty("BackupRestoreStatus");
    expect(librarySchema.$defs).toHaveProperty("MigrationStatus");
  });

  it("uses the library manifest as the root schema", () => {
    expect(librarySchema.$ref).toBe("#/$defs/LibraryManifest");
  });

  it("publishes library manifest v2 index and export safety policy", () => {
    expect(librarySchema.$defs.LibraryManifest.properties.schemaVersion.const).toBe(2);
    expect(librarySchema.$defs.LibraryManifest.required).toEqual(
      expect.arrayContaining([
        "referenceBooks",
        "breakdownProjects",
        "providerRuns",
        "drafts",
        "revisionRequests",
        "changeSets",
        "transferableAssets",
        "craftReferences",
        "backupRestoreStatus",
        "migrationStatus",
        "exportPolicy",
      ]),
    );
    expect(
      librarySchema.$defs.LibraryExportPolicy.properties.includeAiTaskAuditRecordsByDefault.const,
    ).toBe(false);
    expect(librarySchema.$defs.LibraryExportPolicy.properties.exportProviderSecrets.const).toBe(
      false,
    );
  });
});
