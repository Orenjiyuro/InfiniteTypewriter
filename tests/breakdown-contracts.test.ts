import { describe, expect, it } from "vitest";
import {
  createToyBreakdownRecipe,
  createToyEvidenceCard,
  createToyReferenceMechanism,
} from "../src/domain/breakdown";

describe("breakdown TypeScript contracts", () => {
  it("keeps concern kinds extensible without freezing a single breakdown format", () => {
    const recipe = createToyBreakdownRecipe("2026-06-05T00:00:00.000Z");

    expect(recipe.concerns.map((concern) => concern.kind)).toContain("character");
    expect(recipe.modules[0].fields[0]).toMatchObject({
      evidenceRequired: true,
      migrationStrategy: "manual-review",
    });
    expect(recipe.transferRules[0].prohibitedResidue.length).toBeGreaterThan(0);
  });

  it("formalizes evidence cards and reference mechanisms outside the seed bundle", () => {
    const evidence = createToyEvidenceCard("2026-06-05T00:00:00.000Z");
    const mechanism = createToyReferenceMechanism("2026-06-05T00:00:00.000Z");

    expect(evidence.type).toBe("evidence-card");
    expect(evidence.breakdownProjectId).toBe("breakdown-project-toy");
    expect(evidence.coverage.confidence).toBe("medium");
    expect(mechanism.type).toBe("reference-mechanism");
    expect(mechanism.sourceEvidenceRefs).toContain(evidence.id);
    expect(mechanism.applicableTaskTypes).toContain("draft-generation");
    expect(mechanism.prohibitedResidue.length).toBeGreaterThan(0);
  });
});
