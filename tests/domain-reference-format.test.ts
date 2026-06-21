import { describe, expect, it } from "vitest";
import breakdownSchema from "../schemas/breakdown.schema.json";
import characterSchema from "../schemas/character.schema.json";
import commonSchema from "../schemas/common.schema.json";
import simulationSchema from "../schemas/simulation.schema.json";
import storySchema from "../schemas/story.schema.json";

const stableIdPattern = commonSchema.$defs.StableId.pattern;

function propertiesOf(definition: unknown): Record<string, unknown> {
  const candidate = definition as { properties?: Record<string, unknown>; allOf?: Array<{ properties?: Record<string, unknown> }> };
  if (candidate.properties) {
    return candidate.properties;
  }

  return Object.assign({}, ...candidate.allOf?.map((part) => part.properties ?? {}) ?? []);
}

describe("domain reference format contracts", () => {
  it("uses the shared StableId pattern for cross-schema references", () => {
    const workProperties = propertiesOf(storySchema.$defs.Work);
    const breakdownProjectProperties = propertiesOf(breakdownSchema.$defs.BreakdownProject);
    const breakdownScopeProperties = propertiesOf(breakdownSchema.$defs.BreakdownScope);
    const stableProfileProperties = propertiesOf(characterSchema.$defs.CharacterStableProfile);
    const simulationContextProperties = propertiesOf(simulationSchema.$defs.SimulationContext);
    const outlineAdjustmentProperties = propertiesOf(simulationSchema.$defs.OutlineAdjustmentSuggestion);

    const refs = [
      (workProperties.readerContractRefs as { items: unknown }).items,
      breakdownProjectProperties.sourceId,
      breakdownScopeProperties.breakdownProjectId,
      storySchema.$defs.Beat.properties.sceneId,
      (simulationContextProperties.characterRefs as { items: unknown }).items,
      (simulationContextProperties.relationshipRefs as { items: unknown }).items,
      breakdownSchema.$defs.CoverageClaim.properties.evidenceRefIds.items,
      outlineAdjustmentProperties.targetOutlineRef,
      stableProfileProperties.workId,
    ];

    for (const ref of refs) {
      expect(ref).toEqual({ "$ref": "common.schema.json#/$defs/StableId" });
    }
    expect(stableIdPattern).toBe("^[a-z][a-z0-9-]*$");
  });
});
