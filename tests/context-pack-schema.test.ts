import Ajv2020 from "ajv/dist/2020";
import addFormats from "ajv-formats";
import { describe, expect, it } from "vitest";
import commonSchema from "../schemas/common.schema.json";
import contextPackSchema from "../schemas/context-pack.schema.json";
import methodologySchema from "../schemas/methodology.schema.json";
import { createToyDraftGenerationContextPack } from "../src/domain/context-pack";

const ajv = new Ajv2020({ allErrors: true, strict: true });
addFormats(ajv);
ajv.addSchema(commonSchema);
const validateContextPack = ajv.compile(contextPackSchema);

describe("context pack schema", () => {
  it("publishes the 9.7 canonical context pack contract definitions", () => {
    for (const definitionName of [
      "ContextPack",
      "ContextTaskType",
      "ContextActivation",
      "ContextBudget",
      "ContextHash",
      "ContextExclusion",
      "ContextIncludedItem",
      "ContextTaskDescriptor",
    ]) {
      expect(contextPackSchema.$defs).toHaveProperty(definitionName);
    }
  });

  it("uses kebab-case task type values", () => {
    expect(contextPackSchema.$defs.ContextTaskType.enum).toEqual([
      "source-breakdown",
      "creative-discussion",
      "outline-generation",
      "character-simulation",
      "scene-planning",
      "draft-generation",
      "style-revision",
      "continuity-audit",
      "writeback-suggestion",
    ]);
  });

  it("defines source-breakdown stages without adding them as task types", () => {
    expect(contextPackSchema.$defs.SourceBreakdownStage.enum).toEqual([
      "segment-extraction",
      "parent-synthesis",
      "cross-dimension-review",
      "draft-review-rerun",
    ]);
    expect(contextPackSchema.$defs.ContextTaskType.enum).not.toEqual(
      expect.arrayContaining([
        "segment-extraction",
        "parent-synthesis",
        "cross-dimension-review",
        "draft-review-rerun",
      ]),
    );
  });

  it("requires canonical hash metadata", () => {
    expect(contextPackSchema.$defs.ContextHash.required).toEqual(
      expect.arrayContaining(["algorithm", "canonicalization", "activationHashes", "value"]),
    );
    expect(contextPackSchema.$defs.ContextHash.properties.canonicalization.enum).toEqual([
      "json-canonical-v1",
    ]);
  });

  it("validates a public-safe draft-generation context pack fixture", () => {
    const valid = validateContextPack(
      createToyDraftGenerationContextPack("2026-06-06T00:00:00.000Z"),
    );

    expect(validateContextPack.errors).toBeNull();
    expect(valid).toBe(true);
  });

  it("keeps context-pack.schema.json as the only active public ContextPack contract", () => {
    expect(contextPackSchema.$ref).toBe("#/$defs/ContextPack");
    expect(methodologySchema.$defs).not.toHaveProperty("ContextPack");
  });
});
