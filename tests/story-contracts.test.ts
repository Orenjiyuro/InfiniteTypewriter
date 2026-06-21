import { describe, expect, it } from "vitest";
import {
  createToyChangeSet,
  createToyProseQualityPass,
  createToyRevisionRequest,
  createToySceneCard,
  createToyWork,
  type StorySceneCard,
} from "../src/domain/story";

describe("story TypeScript contracts", () => {
  it("anchors Work as the domain root separate from the 9.1 index record", () => {
    const work = createToyWork("2026-06-05T00:00:00.000Z");

    expect(work.id).toBe("work-public-toy");
    expect(work.workKind).toBe("novel");
    expect(work.readerContractRefs).toContain("reader-contract-toy");
  });

  it("uses Beat as the scene-level narrative unit", () => {
    const scene: StorySceneCard = createToySceneCard("2026-06-05T00:00:00.000Z");

    expect(scene.beats[0]).toMatchObject({
      sceneId: scene.id,
      beatKind: "decision",
      sequence: 1,
    });
  });

  it("keeps revision requests chained to the original chunk draft feedback schema and context hash", () => {
    const request = createToyRevisionRequest("2026-06-05T00:00:00.000Z");

    expect(request.originalChunkRef).toBe("source-chunk-toy");
    expect(request.originalDraftRef).toBe("draft-output-fake-toy");
    expect(request.userFeedback).toContain("Clarify");
    expect(request.outputSchemaRef).toBe("toy-draft-output");
    expect(request.contextHash).toMatch(/^[a-f0-9]{64}$/);
    expect(request.requestedRerunTaskType).toBe("source-breakdown");
  });

  it("requires quality passes and change sets to preserve review and writeback gates", () => {
    const pass = createToyProseQualityPass("2026-06-05T00:00:00.000Z");
    const changeSet = createToyChangeSet("2026-06-05T00:00:00.000Z");

    expect(pass.beforeRef).toBe("draft-before-toy");
    expect(pass.suggestions[0].reason).toContain("clarity");
    expect(pass.decision).toBe("pending-user-review");
    expect(changeSet.revisionRequestRefs).toContain("revision-request-toy");
    expect(changeSet.confirmationState).toBe("pending-user-confirmation");
  });
});
