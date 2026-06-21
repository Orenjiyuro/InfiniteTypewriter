import Ajv2020 from "ajv/dist/2020";
import addFormats from "ajv-formats";
import { describe, expect, it } from "vitest";
import breakdownSchema from "../schemas/breakdown.schema.json";
import commonSchema from "../schemas/common.schema.json";
import contextPackSchema from "../schemas/context-pack.schema.json";
import { createToyBreakdownRecipe } from "../src/domain/breakdown";

const ajv = new Ajv2020({ allErrors: true, strict: true });
addFormats(ajv);
ajv.addSchema(commonSchema);
ajv.addSchema(contextPackSchema);
const validateRecipe = ajv.compile(breakdownSchema);

describe("breakdown schema", () => {
  it("publishes the 9.6 breakdown meta-model", () => {
    for (const name of [
      "BreakdownRecipe",
      "BreakdownProject",
      "BreakdownScope",
      "EvidenceCard",
      "ReferenceMechanism",
      "BreakdownConcern",
      "CoverageClaim",
      "FieldEvidenceRef",
      "TransferRule",
      "BookSpecificModule",
      "ViewProjection",
      "CompletionGateCheck",
    ]) {
      expect(breakdownSchema.$defs).toHaveProperty(name);
    }
  });

  it("validates a public-safe breakdown recipe fixture", () => {
    const valid = validateRecipe(createToyBreakdownRecipe("2026-06-05T00:00:00.000Z"));

    expect(validateRecipe.errors).toBeNull();
    expect(valid).toBe(true);
  });
});
