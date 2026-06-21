import Ajv2020 from "ajv/dist/2020";
import addFormats from "ajv-formats";
import { describe, expect, it } from "vitest";
import commonSchema from "../schemas/common.schema.json";
import providerJobSchema from "../schemas/provider-job.schema.json";
import { createToyFakeProviderJob } from "../src/domain/provider-job";

const ajv = new Ajv2020({ allErrors: true, strict: true });
addFormats(ajv);
ajv.addSchema(commonSchema);
const validateProviderJob = ajv.compile(providerJobSchema);

describe("provider job schema", () => {
  it("publishes the 9.8 provider job contract definitions", () => {
    for (const definitionName of [
      "ProviderConfig",
      "ProviderCapability",
      "ProviderJob",
      "ProviderJobEvent",
      "ProviderUsage",
      "ProviderJobStatus",
      "ProviderKind",
      "ProviderPermissionSet",
      "ProviderError",
      "ProviderCancellation",
    ]) {
      expect(providerJobSchema.$defs).toHaveProperty(definitionName);
    }
  });

  it("defines the required provider kinds and lifecycle statuses", () => {
    expect(providerJobSchema.$defs.ProviderKind.enum).toEqual([
      "api-openai-compatible",
      "api-anthropic",
      "api-gemini",
      "api-ollama",
      "cli-codex",
      "cli-claude-code",
      "fake-test",
    ]);
    expect(providerJobSchema.$defs.ProviderJobStatus.enum).toEqual([
      "queued",
      "building-context",
      "ready-for-provider",
      "running",
      "awaiting-approval",
      "succeeded",
      "failed",
      "cancelled",
    ]);
  });

  it("supports creative discussion and keeps prompt/output metadata per job", () => {
    expect(providerJobSchema.$defs.ContextTaskType.enum).toEqual([
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
    expect(providerJobSchema.$defs.ProviderOutputMode.enum).toContain("freeform-draft");
    expect(providerJobSchema.$defs.ProviderConfig.properties).not.toHaveProperty(
      "promptTemplateVersion",
    );
    expect(providerJobSchema.$defs.ProviderConfig.properties).not.toHaveProperty("outputSchemaRef");
    expect(providerJobSchema.$defs.ProviderConfig.properties).not.toHaveProperty(
      "outputSchemaVersion",
    );
  });

  it("validates a public-safe fake provider job fixture", () => {
    const valid = validateProviderJob(createToyFakeProviderJob("2026-06-06T00:00:00.000Z"));

    expect(validateProviderJob.errors).toBeNull();
    expect(valid).toBe(true);
  });
});
