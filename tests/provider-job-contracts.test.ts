import { describe, expect, it } from "vitest";
import {
  createToyCreativeDiscussionProviderJob,
  createToyFakeProviderJob,
} from "../src/domain/provider-job";

const TOY_ZERO_HASH = "0000000000000000000000000000000000000000000000000000000000000000";

describe("provider job public TypeScript contract", () => {
  it("creates a fake provider job with fail-closed defaults", () => {
    const job = createToyFakeProviderJob("2026-06-06T00:00:00.000Z");

    expect(job.status).toBe("ready-for-provider");
    expect(job.providerKind).toBe("fake-test");
    expect(job.contextHash.value).toBe(TOY_ZERO_HASH);
    expect(job.permissions.allowLongTermWrite).toBe(false);
    expect(job.permissions.allowFilesystemWrite).toBe(false);
    expect(job.permissions.allowShell).toBe(false);
    expect(job.permissions.allowNetwork).toBe(false);
    expect(job.permissions.allowTools).toBe(false);
    expect(job.permissions.requiresUserApproval).toBe(true);
  });

  it("records fake non-billable usage without raw provider material", () => {
    const job = createToyFakeProviderJob("2026-06-06T00:00:00.000Z");
    const fieldNames = collectFieldNames(job);

    expect(job.usage?.isBillable).toBe(false);
    expect(fieldNames).not.toEqual(
      expect.arrayContaining([
        "apiKey",
        "authorization",
        "secretValue",
        "rawPrompt",
        "rawResponse",
        "rawTranscript",
      ]),
    );
  });

  it("models creative discussion as a freeform draft-only provider job", () => {
    const job = createToyCreativeDiscussionProviderJob("2026-06-06T00:00:00.000Z");

    expect(job.taskType).toBe("creative-discussion");
    expect(job.outputMode).toBe("freeform-draft");
    expect(job.outputSchemaRef).toBe("freeform-discussion-draft-v1");
    expect(job.permissions.allowLongTermWrite).toBe(false);
    expect(job.draftOutputRef).toBe("discussion-draft-output-toy");
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
