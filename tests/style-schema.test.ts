import Ajv2020 from "ajv/dist/2020";
import addFormats from "ajv-formats";
import { describe, expect, it } from "vitest";
import commonSchema from "../schemas/common.schema.json";
import contextPackSchema from "../schemas/context-pack.schema.json";
import styleSchema from "../schemas/style.schema.json";
import { createToyNarrativeDNA, createToyStyleProfile } from "../src/domain/style";

const ajv = new Ajv2020({ allErrors: true, strict: true });
addFormats(ajv);
ajv.addSchema(commonSchema);
ajv.addSchema(commonSchema, "common.schema.json");
ajv.addSchema(contextPackSchema);
ajv.addSchema(contextPackSchema, "context-pack.schema.json");
const validateStyleProfile = ajv.compile(styleSchema);

describe("style schema", () => {
  it("publishes style and craft contracts", () => {
    for (const name of [
      "StyleProfile",
      "NarrativeDNA",
      "MicroTechnique",
      "ImageryRecord",
      "BookMechanic",
    ]) {
      expect(styleSchema.$defs).toHaveProperty(name);
    }
  });

  it("validates a public-safe style profile fixture", () => {
    const valid = validateStyleProfile(createToyStyleProfile("2026-06-05T00:00:00.000Z"));

    expect(validateStyleProfile.errors).toBeNull();
    expect(valid).toBe(true);
  });

  it("requires NarrativeDNA v2 evidence transfer and conflict guard fields", () => {
    expect(styleSchema.$defs.NarrativeDNA.required).toEqual(
      expect.arrayContaining([
        "evidenceRefIds",
        "coverage",
        "confidence",
        "mechanismRefIds",
        "transferRuleRefs",
        "applicableTaskTypes",
        "applicableScope",
        "misuseRisks",
        "doNotCopy",
        "prohibitedResidue",
        "currentWorkConflictChecks",
      ]),
    );

    const validateNarrativeDNA = ajv.compile({
      $defs: styleSchema.$defs,
      $ref: "#/$defs/NarrativeDNA",
    });
    const valid = validateNarrativeDNA(createToyNarrativeDNA("2026-06-05T00:00:00.000Z"));

    expect(validateNarrativeDNA.errors).toBeNull();
    expect(valid).toBe(true);
  });
});
