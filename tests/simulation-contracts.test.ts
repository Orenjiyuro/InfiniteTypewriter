import { describe, expect, it } from "vitest";
import {
  createToyPuppeteeringRisk,
  createToySimulationContext,
  createToySimulationResult,
} from "../src/domain/simulation";

describe("simulation TypeScript contracts", () => {
  it("expresses stable, dynamic, relationship, and constraint inputs", () => {
    const context = createToySimulationContext("2026-06-05T00:00:00.000Z");

    expect(context.stableProfileRefs).toContain("character-practical-learner");
    expect(context.dynamicStateRefs).toContain("dynamic-state-gate-choice");
    expect(context.relationshipRefs).toContain("relationship-mentor-learner");
    expect(context.constraints).toContain("Token must be spent before either route opens.");
  });

  it("keeps outline adjustment suggestions as structured references", () => {
    const result = createToySimulationResult("2026-06-05T00:00:00.000Z");
    const risk = createToyPuppeteeringRisk("2026-06-05T00:00:00.000Z");

    expect(risk.conflictingRefs).toContain("character-practical-learner");
    expect(result.outlineAdjustmentSuggestionIds).toContain("outline-adjustment-token-choice");
  });
});
