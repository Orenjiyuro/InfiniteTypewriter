import Ajv2020 from "ajv/dist/2020";
import addFormats from "ajv-formats";
import { describe, expect, it } from "vitest";
import commonSchema from "../schemas/common.schema.json";
import contextPackSchema from "../schemas/context-pack.schema.json";
import storySchema from "../schemas/story.schema.json";
import { createToyWork } from "../src/domain/story";

const ajv = new Ajv2020({ allErrors: true, strict: true });
addFormats(ajv);
ajv.addSchema(commonSchema);
ajv.addSchema(contextPackSchema);
const validateWork = ajv.compile(storySchema);

describe("story schema", () => {
  it("publishes story hierarchy, foreshadowing, and revision contracts", () => {
    for (const name of [
      "Work",
      "WorkOutline",
      "VolumeOutline",
      "StorySegment",
      "ChapterPlan",
      "SceneCard",
      "Beat",
      "ForeshadowingLedger",
      "RevisionLayer",
      "ContinuityIssue",
      "DraftReviewItem",
      "RevisionRequest",
      "ChangeSet",
      "ProseQualityPass",
      "StyleRevisionPass",
    ]) {
      expect(storySchema.$defs).toHaveProperty(name);
    }
  });

  it("validates a public-safe work fixture", () => {
    const valid = validateWork(createToyWork("2026-06-05T00:00:00.000Z"));

    expect(validateWork.errors).toBeNull();
    expect(valid).toBe(true);
  });
});
