import Ajv2020 from "ajv/dist/2020";
import addFormats from "ajv-formats";
import { describe, expect, it } from "vitest";
import methodologySchema from "../schemas/methodology.schema.json";
import { createToyMethodologySeedBundle } from "../src/domain/methodology";

const ajv = new Ajv2020({ allErrors: true, strict: true });
addFormats(ajv);
const validateMethodologySeedBundle = ajv.compile(methodologySchema);

describe("methodology JSON schema", () => {
  it("publishes the 9.2 public methodology contracts", () => {
    for (const definitionName of [
      "EvidenceCard",
      "ReaderContract",
      "SceneCard",
      "KnowledgeBoundary",
      "SettingPressure",
      "ReferenceMechanism",
      "MethodologySeedBundle",
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

  it("does not publish the retired 9.2 bundle as an active ContextPack contract", () => {
    expect(methodologySchema.$ref).toBe("#/$defs/MethodologySeedBundle");
    expect(methodologySchema.$defs).not.toHaveProperty("ContextPack");
  });

  it("validates a public-safe methodology seed bundle fixture against the JSON schema", () => {
    const valid = validateMethodologySeedBundle(
      createToyMethodologySeedBundle("2026-06-05T00:00:00.000Z"),
    );

    expect(validateMethodologySeedBundle.errors).toBeNull();
    expect(valid).toBe(true);
  });

  it("rejects EvidenceCard fixtures missing required evidence fields at schema level", () => {
    const bundle = createToyMethodologySeedBundle("2026-06-05T00:00:00.000Z");
    const [evidenceCard] = bundle.evidenceCards;

    for (const field of ["evidence", "source", "coverage", "confidence"] as const) {
      const invalidBundle = {
        ...bundle,
        evidenceCards: [{ ...evidenceCard, [field]: undefined }],
      };

      const valid = validateMethodologySeedBundle(invalidBundle);

      expect(valid, `${field} should be required`).toBe(false);
      expect(validateMethodologySeedBundle.errors).toContainEqual(
        expect.objectContaining({
          instancePath: "/evidenceCards/0",
          keyword: "required",
          params: { missingProperty: field },
        }),
      );
    }
  });
});
