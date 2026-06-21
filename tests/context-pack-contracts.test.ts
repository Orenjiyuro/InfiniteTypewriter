import { describe, expect, it } from "vitest";
import {
  createToyDraftGenerationContextPack,
  createToySourceBreakdownRevisionContextPack,
  CONTEXT_TASK_TYPES,
  type ContextExclusionReason,
} from "../src/domain/context-pack";

describe("context pack public TypeScript contract", () => {
  it("builds a toy draft-generation fixture with activations, inclusions, exclusions, and hash", () => {
    const pack = createToyDraftGenerationContextPack("2026-06-06T00:00:00.000Z");

    expect(pack.taskType).toBe("draft-generation");
    expect(pack.activations.map((activation) => activation.sourceObjectType)).toEqual([
      "SceneCard",
      "Beat",
      "CharacterDynamicState",
    ]);
    expect(pack.includedItems.map((item) => item.sourceRef)).toEqual([
      "scene-card-toy",
      "beat-toy",
      "character-state-toy",
    ]);
    expect(pack.contextHash.value).toMatch(/^[a-f0-9]{64}$/);
  });

  it("records budget and stale exclusions with approved reasons", () => {
    const pack = createToyDraftGenerationContextPack("2026-06-06T00:00:00.000Z");
    const reasons: ContextExclusionReason[] = pack.exclusions.map(
      (exclusion) => exclusion.reason,
    );

    expect(reasons).toContain("budget-insufficient");
    expect(reasons).toContain("stale-or-superseded");
    expect(pack.budget.overflowPolicy).toBe("fail-closed");
  });

  it("keeps preview packs provider-free until explicit 9.8 job creation", () => {
    const pack = createToyDraftGenerationContextPack("2026-06-06T00:00:00.000Z");
    const fieldNames = collectFieldNames(pack);

    expect(pack.createdForJobRef).toBeUndefined();
    expect(fieldNames).not.toEqual(
      expect.arrayContaining([
        "provider",
        "providerConfig",
        "model",
        "modelId",
        "apiKey",
        "api_key",
        "cliCommand",
        "promptTemplateBody",
      ]),
    );
  });

  it("defines the required task types including creative discussion", () => {
    expect(CONTEXT_TASK_TYPES).toEqual([
      "source-breakdown",
      "creative-discussion",
      "outline-generation",
      "character-simulation",
      "scene-planning",
      "draft-generation",
      "style-revision",
      "continuity-audit",
      "writeback-suggestion",
    ]);
  });

  it("keeps source-breakdown phases as descriptor parameters and supports revision rerun context", () => {
    const pack = createToySourceBreakdownRevisionContextPack("2026-06-06T00:00:00.000Z");

    expect(pack.taskType).toBe("source-breakdown");
    expect(pack.taskDescriptor?.sourceBreakdownStage).toBe("draft-review-rerun");
    expect(pack.taskDescriptor?.originalChunkRef).toBe("source-chunk-toy");
    expect(pack.taskDescriptor?.originalDraftRef).toBe("draft-output-fake-toy");
    expect(pack.taskDescriptor?.revisionRequestRef).toBe("revision-request-toy");
    expect(pack.taskDescriptor?.outputSchemaRef).toBe("toy-draft-output");
    expect(pack.taskDescriptor?.contextHash).toMatch(/^[a-f0-9]{64}$/);
  });
});

function collectFieldNames(value: unknown): string[] {
  if (Array.isArray(value)) {
    return value.flatMap(collectFieldNames);
  }
  if (value != null && typeof value === "object") {
    return Object.entries(value).flatMap(([key, nested]) => [key, ...collectFieldNames(nested)]);
  }
  return [];
}
