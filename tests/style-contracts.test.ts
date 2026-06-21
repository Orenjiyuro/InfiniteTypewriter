import { describe, expect, it } from "vitest";
import { createToyNarrativeDNA, createToyStyleProfile } from "../src/domain/style";

describe("style TypeScript contracts", () => {
  it("keeps style observations evidence-backed and non-imitative", () => {
    const profile = createToyStyleProfile("2026-06-05T00:00:00.000Z");

    expect(profile.evidenceRefIds).toContain("evidence-toy-pressure");
    expect(profile.transferRuleIds).toContain("transfer-rule-toy");
    expect(profile.misuseRisks.length).toBeGreaterThan(0);
    expect(profile.doNotImitate).toContain("Do not copy source phrasing.");
  });

  it("keeps NarrativeDNA evidence-bound rather than a second fact model", () => {
    const dna = createToyNarrativeDNA("2026-06-05T00:00:00.000Z");

    expect(dna.schemaVersion).toBe(2);
    expect(dna.evidenceRefIds).toContain("evidence-toy-pressure");
    expect(dna.mechanismRefIds).toContain("reference-mechanism-toy");
    expect(dna.transferRuleRefs).toContain("transfer-rule-toy");
    expect(dna.doNotCopy).toContain("Do not copy source scenes.");
    expect(dna.currentWorkConflictChecks.length).toBeGreaterThan(0);
  });
});
