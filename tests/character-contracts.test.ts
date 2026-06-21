import { describe, expect, it } from "vitest";
import { createToyCharacterProfile, createToyRelationshipEdge } from "../src/domain/character";

describe("character TypeScript contracts", () => {
  it("requires evidence-backed stable profile fields", () => {
    const profile = createToyCharacterProfile("2026-06-05T00:00:00.000Z");

    expect(profile.coreDriveEvidenceRefIds).toContain("evidence-toy-pressure");
    expect(profile.boundaryEvidenceRefIds.length).toBeGreaterThan(0);
    expect(profile.voiceEvidenceRefIds.length).toBeGreaterThan(0);
  });

  it("represents relationship state separately from display projection", () => {
    const relationship = createToyRelationshipEdge("2026-06-05T00:00:00.000Z");

    expect(relationship.participantIds).toEqual(["character-practical-learner", "character-careful-mentor"]);
    expect(relationship.currentTemperature).toBe("warm");
  });
});
