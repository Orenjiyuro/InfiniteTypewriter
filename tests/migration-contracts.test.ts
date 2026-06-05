import { describe, expect, it } from "vitest";
import {
  createToyMigrationDryRun,
  summarizeMigrationDryRun,
  type MigrationDryRun,
  type MigrationSource,
  type MigrationTarget,
} from "../src/features/migration";

describe("migration contracts", () => {
  it("creates a public-safe MigrationDryRun fixture without source file text", () => {
    const source: MigrationSource = {
      rootPath: "C:/Users/demo/old-local-notes",
      includePaths: ["breakdowns/scene-notes.md"],
    };
    const target: MigrationTarget = {
      libraryRootPath: "C:/Users/demo/InfiniteTypewriter",
    };

    const dryRun = createToyMigrationDryRun(
      source,
      target,
      "2026-06-05T00:00:00.000Z",
    );

    expect(dryRun).toMatchObject<MigrationDryRun>({
      kind: "migration-dry-run",
      generatedAt: "2026-06-05T00:00:00.000Z",
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
        generatedAt: "2026-06-05T00:00:00.000Z",
        totalFiles: 1,
        migratableFiles: 1,
        blockedFiles: 0,
        sourceTreeHash: "fnv1a64:toy-tree",
      },
    });

    expect(JSON.stringify(dryRun)).not.toContain("visible limit creates pressure");
  });

  it("summarizes migratable and blocked dry-run items", () => {
    const dryRun = createToyMigrationDryRun(
      {
        rootPath: "C:/Users/demo/old-local-notes",
        includePaths: ["breakdowns/scene-notes.md"],
      },
      {
        libraryRootPath: "C:/Users/demo/InfiniteTypewriter",
      },
      "2026-06-05T00:00:00.000Z",
    );

    expect(summarizeMigrationDryRun(dryRun)).toBe("1 migratable, 0 blocked");
  });
});
