import { describe, expect, it } from "vitest";
import {
  createToyMethodologySeedBundle,
  validateEvidenceCard,
  type EvidenceCard,
  type MethodologySceneCard,
} from "../src/domain/methodology";

describe("methodology contracts", () => {
  it("round-trips a public-safe methodology seed bundle fixture", () => {
    const bundle = createToyMethodologySeedBundle("2026-06-05T00:00:00.000Z");
    const scene: MethodologySceneCard = bundle.sceneCards[0];

    const roundTripped = JSON.parse(JSON.stringify(bundle));

    expect(roundTripped).toEqual(bundle);
    expect(scene.kind).toBe("scene-card");
    expect(roundTripped.evidenceCards[0].source.title).toBe("Public Toy Source");
  });

  it("rejects EvidenceCard values missing evidence source coverage or confidence", () => {
    const incompleteCard: Partial<EvidenceCard> = {
      id: "evidence-missing-required",
      kind: "evidence-card",
      observedTechnique: "A choice is constrained by a public, invented resource limit.",
      sourceFunction: "Demonstrates pressure without copying a source event.",
      transferableMechanism: "Make a constraint visible before the character chooses.",
      forbiddenCopy: ["Do not copy named events or dialogue."],
      createdAt: "2026-06-05T00:00:00.000Z",
      updatedAt: "2026-06-05T00:00:00.000Z",
    };

    expect(validateEvidenceCard(incompleteCard)).toEqual({
      valid: false,
      missing: ["evidence", "source", "coverage", "confidence"],
    });
  });

  it("rejects EvidenceCard values with empty or incomplete evidence fields", () => {
    const [card] = createToyMethodologySeedBundle("2026-06-05T00:00:00.000Z").evidenceCards;

    expect(validateEvidenceCard({ ...card, evidence: [] })).toEqual({
      valid: false,
      missing: ["evidence"],
    });
    expect(validateEvidenceCard({ ...card, source: { ...card.source, locator: "" } })).toEqual({
      valid: false,
      missing: ["source.locator"],
    });
    expect(validateEvidenceCard({ ...card, coverage: { ...card.coverage, scope: "" } })).toEqual({
      valid: false,
      missing: ["coverage.scope"],
    });
    expect(
      validateEvidenceCard({
        ...card,
        confidence: { ...card.confidence, rationale: "" },
      }),
    ).toEqual({
      valid: false,
      missing: ["confidence.rationale"],
    });
  });
});
