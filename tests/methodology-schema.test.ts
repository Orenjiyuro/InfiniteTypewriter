import Ajv2020 from "ajv/dist/2020";
import addFormats from "ajv-formats";
import { describe, expect, it } from "vitest";
import methodologySchema from "../schemas/methodology.schema.json";
import { createToyContextPack } from "../src/domain/methodology";

const ajv = new Ajv2020({ allErrors: true, strict: true });
addFormats(ajv);
const validateContextPack = ajv.compile(methodologySchema);

describe("methodology JSON schema", () => {
  it("publishes the 9.2 public methodology contracts", () => {
    for (const definitionName of [
      "EvidenceCard",
      "ReaderContract",
      "SceneCard",
      "KnowledgeBoundary",
      "SettingPressure",
      "ReferenceMechanism",
      "ContextPack",
      "DraftReviewItem",
      "ChangeSet",
    ]) {
      expect(methodologySchema.$defs).toHaveProperty(definitionName);
    }
  });

  it("requires evidence source coverage and confidence on EvidenceCard", () => {
    expect(methodologySchema.$defs.EvidenceCard.required).toEqual(
      expect.arrayContaining(["evidence", "source", "coverage", "confidence"]),
    );
  });

  it("uses ContextPack as the root schema", () => {
    expect(methodologySchema.$ref).toBe("#/$defs/ContextPack");
  });

  it("validates a public-safe ContextPack fixture against the JSON schema", () => {
    const valid = validateContextPack(createToyContextPack("2026-06-05T00:00:00.000Z"));

    expect(validateContextPack.errors).toBeNull();
    expect(valid).toBe(true);
  });

  it("rejects EvidenceCard fixtures missing required evidence fields at schema level", () => {
    const contextPack = createToyContextPack("2026-06-05T00:00:00.000Z");
    const [evidenceCard] = contextPack.evidenceCards;

    for (const field of ["evidence", "source", "coverage", "confidence"] as const) {
      const invalidContextPack = {
        ...contextPack,
        evidenceCards: [{ ...evidenceCard, [field]: undefined }],
      };

      const valid = validateContextPack(invalidContextPack);

      expect(valid, `${field} should be required`).toBe(false);
      expect(validateContextPack.errors).toContainEqual(
        expect.objectContaining({
          instancePath: "/evidenceCards/0",
          keyword: "required",
          params: { missingProperty: field },
        }),
      );
    }
  });
});
