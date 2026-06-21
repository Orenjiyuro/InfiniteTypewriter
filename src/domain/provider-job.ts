import type { BaseObject, IsoDateTime, StableId } from "./common";
import { createToyBaseObject } from "./common";
import type { ContextHash, ContextTaskType } from "./context-pack";

export const PROVIDER_KINDS = [
  "api-openai-compatible",
  "api-anthropic",
  "api-gemini",
  "api-ollama",
  "cli-codex",
  "cli-claude-code",
  "fake-test",
] as const;

export type ProviderKind = (typeof PROVIDER_KINDS)[number];
export type ProviderAuthMode = "none" | "env-var" | "keychain-ref" | "manual-session" | "local-service";
export type ProviderCapabilityKind =
  | "text-generation"
  | "structured-output"
  | "streaming"
  | "tool-use"
  | "local-cli-agent"
  | "local-model"
  | "usage-reporting"
  | "cancellation"
  | "permission-gate";
export type ProviderSupportLevel = "native" | "emulated" | "unverified" | "unsupported";
export type ProviderJobStatus =
  | "queued"
  | "building-context"
  | "ready-for-provider"
  | "running"
  | "awaiting-approval"
  | "succeeded"
  | "failed"
  | "cancelled";

export type ProviderJobEventType =
  | "job-created"
  | "context-attached"
  | "prompt-built"
  | "provider-request-built"
  | "cli-command-built"
  | "provider-started"
  | "stdout-chunk"
  | "stderr-chunk"
  | "stream-chunk"
  | "approval-requested"
  | "approval-resolved"
  | "provider-finished"
  | "draft-created"
  | "usage-recorded"
  | "error-recorded"
  | "cancel-requested"
  | "cancelled"
  | "timeout";
export type ProviderActor = "user" | "system" | "provider" | "cli-process";
export type ProviderEventSeverity = "info" | "warning" | "error";
export type ProviderErrorClass =
  | "auth-error"
  | "permission-error"
  | "rate-limit"
  | "quota-or-billing"
  | "request-invalid"
  | "context-too-large"
  | "budget-blocked"
  | "provider-timeout"
  | "provider-overloaded"
  | "provider-internal"
  | "network-error"
  | "schema-output-invalid"
  | "safety-blocked"
  | "local-service-unavailable"
  | "model-not-found"
  | "secret-unresolved"
  | "advanced-agent-mode-disabled";
export type ProviderOutputMode = "structured-draft" | "freeform-draft" | "non-writeback-summary";

export interface ProviderConfig extends BaseObject {
  type: "provider-config";
  providerKind: ProviderKind;
  enabled: boolean;
  authMode: ProviderAuthMode;
  secretRef?: string;
  baseUrl?: string;
  defaultModel: string;
  defaultTimeoutMs: number;
  capabilityRefs: StableId[];
  redactionPolicyVersion: string;
  advancedAgentModeEnabled: boolean;
  failClosedDefaults: boolean;
}

export interface ProviderCapability {
  id: StableId;
  providerConfigId: StableId;
  capabilityKind: ProviderCapabilityKind;
  supported: boolean;
  supportLevel: ProviderSupportLevel;
  notes: string;
  verifiedAt?: IsoDateTime;
}

export interface ProviderPermissionSet {
  allowLongTermWrite: boolean;
  allowFilesystemWrite: boolean;
  allowShell: boolean;
  allowNetwork: boolean;
  allowTools: boolean;
  requiresUserApproval: boolean;
  cliPermissionMode?: string;
  workingDirectory?: string;
}

export interface ProviderUsage {
  providerKind: ProviderKind;
  model: string;
  inputTokens?: number;
  outputTokens?: number;
  cachedInputTokens?: number;
  reasoningTokens?: number;
  totalTokens?: number;
  providerUsageRawSummary?: string;
  localPromptEvalCount?: number;
  localEvalCount?: number;
  durationMs: number;
  requestId?: string;
  usageUnavailableReason?: string;
  usageRecordedAt: IsoDateTime;
  isBillable: boolean;
}

export interface ProviderError {
  errorClass: ProviderErrorClass;
  message: string;
  providerStatusCode?: number;
  providerRequestId?: string;
  retryable: boolean;
  redactionPolicyVersion: string;
}

export interface ProviderCancellation {
  requested: boolean;
  requestedAt?: IsoDateTime;
  requestedBy?: "user" | "system";
  reason?: string;
  gracefulTerminationAttempted: boolean;
  finalSignal?: string;
}

export interface ProviderJobEvent {
  id: StableId;
  jobId: StableId;
  sequence: number;
  eventType: ProviderJobEventType;
  at: IsoDateTime;
  actor: ProviderActor;
  statusBefore?: ProviderJobStatus;
  statusAfter?: ProviderJobStatus;
  summary: string;
  redactedPayloadRef?: StableId;
  rawPayloadRetained: boolean;
  severity: ProviderEventSeverity;
  errorClass?: ProviderErrorClass;
}

export interface ProviderJob extends Omit<BaseObject, "status" | "type"> {
  type: "provider-job";
  taskType: ContextTaskType;
  targetRef: StableId;
  contextPackRef: StableId;
  contextHash: ContextHash;
  libraryManifestHash: string;
  providerConfigRef: StableId;
  providerKind: ProviderKind;
  model: string;
  promptTemplateVersion: string;
  outputSchemaRef: string;
  outputSchemaVersion: number;
  outputMode?: ProviderOutputMode;
  permissions: ProviderPermissionSet;
  status: ProviderJobStatus;
  createdBy: "user" | "system";
  startedAt?: IsoDateTime;
  endedAt?: IsoDateTime;
  timeoutMs: number;
  cancellation: ProviderCancellation;
  inputSummary: string;
  outputSummary?: string;
  draftOutputRef?: StableId;
  usage?: ProviderUsage;
  error?: ProviderError;
  eventRefs: StableId[];
}

const TOY_ZERO_HASH = "0000000000000000000000000000000000000000000000000000000000000000";

export function createToyFakeProviderJob(timestamp: IsoDateTime): ProviderJob {
  const base = createToyBaseObject(
    "provider-job",
    "provider-job-fake-toy",
    "Toy Fake Provider Job",
    timestamp,
    "active",
  );

  return {
    ...base,
    taskType: "draft-generation",
    targetRef: "scene-card-toy",
    contextPackRef: "context-pack-draft-toy",
    contextHash: {
      algorithm: "sha256",
      canonicalization: "json-canonical-v1",
      schemaVersionMap: [
        {
          schemaId: "https://infinite-typewriter.local/schemas/context-pack.schema.json",
          version: 1,
        },
      ],
      libraryManifestHash: TOY_ZERO_HASH,
      sourceHashRefs: [{ ref: "source-toy", hash: TOY_ZERO_HASH }],
      changeSetRefs: ["change-set-toy"],
      changeSetHash: TOY_ZERO_HASH,
      selectionPolicyVersion: "context-pack-selection-v1",
      promptTemplateVersion: "draft-generation-v1",
      includedItemHashes: [{ ref: "scene-card-toy", hash: TOY_ZERO_HASH }],
      activationHashes: [{ ref: "activation-scene-toy", hash: TOY_ZERO_HASH }],
      excludedItemHashes: [{ ref: "exclusion-manifest-toy", hash: TOY_ZERO_HASH }],
      value: TOY_ZERO_HASH,
    },
    libraryManifestHash: TOY_ZERO_HASH,
    providerConfigRef: "provider-config-fake-toy",
    providerKind: "fake-test",
    model: "fake-deterministic-v1",
    promptTemplateVersion: "draft-generation-v1",
    outputSchemaRef: "toy-draft-output",
    outputSchemaVersion: 1,
    permissions: {
      allowLongTermWrite: false,
      allowFilesystemWrite: false,
      allowShell: false,
      allowNetwork: false,
      allowTools: false,
      requiresUserApproval: true,
    },
    status: "ready-for-provider",
    createdBy: "user",
    startedAt: timestamp,
    endedAt: timestamp,
    timeoutMs: 30_000,
    cancellation: {
      requested: false,
      gracefulTerminationAttempted: false,
    },
    inputSummary: "Toy draft-generation context pack with one selected scene manifest.",
    outputSummary: "Deterministic fake provider draft summary.",
    draftOutputRef: "draft-output-fake-toy",
    usage: {
      providerKind: "fake-test",
      model: "fake-deterministic-v1",
      inputTokens: 12,
      outputTokens: 8,
      totalTokens: 20,
      durationMs: 1,
      requestId: "fake-request-toy",
      usageRecordedAt: timestamp,
      isBillable: false,
    },
    eventRefs: ["event-job-created-toy", "event-usage-recorded-toy"],
  };
}

export function createToyCreativeDiscussionProviderJob(timestamp: IsoDateTime): ProviderJob {
  const base = createToyBaseObject(
    "provider-job",
    "provider-job-discussion-toy",
    "Toy Creative Discussion Job",
    timestamp,
    "active",
  );

  return {
    ...base,
    taskType: "creative-discussion",
    targetRef: "work-public-toy",
    contextPackRef: "context-pack-discussion-toy",
    contextHash: {
      algorithm: "sha256",
      canonicalization: "json-canonical-v1",
      schemaVersionMap: [
        {
          schemaId: "https://infinite-typewriter.local/schemas/context-pack.schema.json",
          version: 1,
        },
      ],
      libraryManifestHash: TOY_ZERO_HASH,
      sourceHashRefs: [],
      changeSetRefs: [],
      changeSetHash: TOY_ZERO_HASH,
      selectionPolicyVersion: "context-pack-selection-v1",
      promptTemplateVersion: "creative-discussion-v1",
      includedItemHashes: [],
      activationHashes: [],
      value: TOY_ZERO_HASH,
    },
    libraryManifestHash: TOY_ZERO_HASH,
    providerConfigRef: "provider-config-fake-toy",
    providerKind: "fake-test",
    model: "fake-deterministic-v1",
    promptTemplateVersion: "creative-discussion-v1",
    outputSchemaRef: "freeform-discussion-draft-v1",
    outputSchemaVersion: 1,
    outputMode: "freeform-draft",
    permissions: {
      allowLongTermWrite: false,
      allowFilesystemWrite: false,
      allowShell: false,
      allowNetwork: false,
      allowTools: false,
      requiresUserApproval: true,
    },
    status: "ready-for-provider",
    createdBy: "user",
    timeoutMs: 30_000,
    cancellation: {
      requested: false,
      gracefulTerminationAttempted: false,
    },
    inputSummary: "Toy creative discussion context.",
    draftOutputRef: "discussion-draft-output-toy",
    eventRefs: [],
  };
}
