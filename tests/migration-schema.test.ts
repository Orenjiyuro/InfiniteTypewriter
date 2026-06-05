import Ajv2020 from "ajv/dist/2020";
import addFormats from "ajv-formats";
import { describe, expect, it } from "vitest";
import migrationSchema from "../schemas/migration.schema.json";
import { createToyMigrationDryRun } from "../src/features/migration";

const ajv = new Ajv2020({ allErrors: true, strict: true });
addFormats(ajv);
const validateMigrationDryRun = ajv.compile(migrationSchema);

describe("migration JSON schema", () => {
  it("publishes the 9.3 migration contracts", () => {
    expect(migrationSchema.$defs).toHaveProperty("MigrationDryRun");
    expect(migrationSchema.$defs).toHaveProperty("MigrationSource");
    expect(migrationSchema.$defs).toHaveProperty("MigrationTarget");
    expect(migrationSchema.$defs).toHaveProperty("MigrationReport");
  });

  it("uses MigrationDryRun as the root schema", () => {
    expect(migrationSchema.$ref).toBe("#/$defs/MigrationDryRun");
  });

  it("validates a public-safe MigrationDryRun fixture", () => {
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

    const valid = validateMigrationDryRun(dryRun);

    expect(validateMigrationDryRun.errors).toBeNull();
    expect(valid).toBe(true);
  });
});
