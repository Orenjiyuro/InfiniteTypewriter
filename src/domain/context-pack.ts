import type { BaseObject, IsoDateTime, StableId } from "./common";
import { createToyBaseObject } from "./common";

export const CONTEXT_TASK_TYPES = [
  "source-breakdown",
  "creative-discussion",
  "outline-generation",
  "character-simulation",
  "scene-planning",
  "draft-generation",
  "style-revision",
  "continuity-audit",
  "writeback-suggestion",
] as const;

export type ContextTaskType = (typeof CONTEXT_TASK_TYPES)[number];
export type SourceBreakdownStage =
  | "segment-extraction"
  | "parent-synthesis"
  | "cross-dimension-review"
  | "draft-review-rerun";
export interface ContextTaskDescriptor {
  taskType?: ContextTaskType;
  sourceBreakdownStage: SourceBreakdownStage;
  inputRefs?: StableId[];
  outputRefs?: StableId[];
  sourceChunkRefs?: StableId[];
  originalChunkRef?: StableId;
  originalDraftRef?: StableId;
  userFeedbackRef?: StableId;
  revisionRequestRef?: StableId;
  outputSchemaRef?: string;
  outputSchemaVersion?: number;
  contextHash?: string;
}
export type ActivationReason =
  | "target-match"
  | "direct-reference"
  | "context-hint-keyword"
  | "task-default"
  | "change-set-impact"
  | "continuity-dependency"
  | "user-selected"
  | "pinned-required";
export type EvidenceChainEntry = {
  ref: StableId;
  relation: string;
  note: string;
};
export type FreshnessState = "current" | "stale" | "superseded" | "unknown";
export type UserSelectionState = "auto" | "user-included" | "user-excluded" | "user-pinned";
export type ContextPriority = "required" | "high" | "normal" | "low";
export type RelevanceBand = "low" | "medium" | "high";
export type ConfidenceLevel = "low" | "medium" | "high";

export interface ContextActivation {
  id: StableId;
  contextPackId: StableId;
  sourceRef: StableId;
  sourceObjectType: string;
  sourceSchemaVersion: number;
  activationReason: ActivationReason;
  evidenceChain: EvidenceChainEntry[];
  relevance: RelevanceBand;
  confidence: ConfidenceLevel;
  priority: ContextPriority;
  freshnessState: FreshnessState;
  userSelection: UserSelectionState;
  selectedAt: IsoDateTime;
}

export interface TokenBudget {
  maxInputTokens: number;
  reservedOutputTokens: number;
  estimationMethod: string;
}

export interface FragmentBudget {
  maxFragments: number;
  maxFragmentChars: number;
}

export interface ObjectBudget {
  objectType: string;
  maxObjects: number;
}

export interface PriorityBands {
  required: number;
  high: number;
  normal: number;
  low: number;
}

export type TrimmingStrategy =
  | "drop-low-priority"
  | "summarize-eligible"
  | "latest-confirmed-first"
  | "closest-scope-first"
  | "user-pinned-first";
export type OverflowPolicy = "fail-closed" | "drop-with-exclusion-record";

export interface ContextBudget {
  id: StableId;
  contextPackId: StableId;
  tokenBudget: TokenBudget;
  fragmentBudget: FragmentBudget;
  objectBudget: ObjectBudget[];
  priorityBands: PriorityBands;
  trimmingStrategy: TrimmingStrategy;
  excludedObjectTypes: string[];
  excludedRefs: StableId[];
  overflowPolicy: OverflowPolicy;
}

export interface ManifestHashEntry {
  ref: StableId;
  hash: string;
}

export interface SchemaVersionEntry {
  schemaId: string;
  version: number;
}

export interface ContextHash {
  algorithm: "sha256";
  canonicalization: "json-canonical-v1";
  schemaVersionMap: SchemaVersionEntry[];
  libraryManifestHash: string;
  sourceHashRefs: ManifestHashEntry[];
  changeSetRefs: StableId[];
  changeSetHash: string;
  selectionPolicyVersion: string;
  promptTemplateVersion?: string;
  includedItemHashes: ManifestHashEntry[];
  activationHashes: ManifestHashEntry[];
  excludedItemHashes?: ManifestHashEntry[];
  value: string;
}

export type ContextExclusionReason =
  | "budget-insufficient"
  | "stale-or-superseded"
  | "user-excluded"
  | "permission-boundary"
  | "task-irrelevant"
  | "insufficient-evidence"
  | "duplicate-or-covered"
  | "schema-unsupported"
  | "source-boundary"
  | "not-in-selected-scope";

export type ContextExclusionRiskKind =
  | "privacy-boundary"
  | "copyright-boundary"
  | "stale-state"
  | "budget-overflow"
  | "irrelevant"
  | "low-confidence"
  | "user-choice"
  | "permission-denied"
  | "unsupported-schema";

export interface ContextExclusion {
  id: StableId;
  contextPackId: StableId;
  excludedRef: StableId;
  sourceObjectType: string;
  reason: ContextExclusionReason;
  riskKind: ContextExclusionRiskKind;
  explanation: string;
  candidateScore?: RelevanceBand;
  wouldFitIfBudget?: boolean;
  relatedActivationRef?: StableId;
  createdAt: IsoDateTime;
}

export interface ContextIncludedItem {
  sourceRef: StableId;
  sourceObjectType: string;
  sourceSchemaVersion: number;
  contentHash: string;
  activationRef: StableId;
}

export interface ContextPack extends BaseObject {
  type: "context-pack";
  taskType: ContextTaskType;
  taskDescriptor?: ContextTaskDescriptor;
  targetRef: StableId;
  activations: ContextActivation[];
  includedItems: ContextIncludedItem[];
  exclusions: ContextExclusion[];
  budget: ContextBudget;
  contextHash: ContextHash;
  createdForTaskRef: StableId;
  createdForJobRef?: StableId;
}

const HASH_A = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
const HASH_B = "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";
const HASH_C = "cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc";
const HASH_D = "dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd";
const HASH_E = "eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee";
const HASH_F = "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff";

export function createToyDraftGenerationContextPack(timestamp: IsoDateTime): ContextPack {
  const base = createToyBaseObject(
    "context-pack",
    "context-pack-draft-toy",
    "Toy Draft Generation Context",
    timestamp,
    "draft",
  );

  const activations: ContextActivation[] = [
    createActivation(
      "activation-scene-toy",
      "scene-card-toy",
      "SceneCard",
      "target-match",
      "required",
      "current",
      timestamp,
    ),
    createActivation(
      "activation-beat-toy",
      "beat-toy",
      "Beat",
      "direct-reference",
      "high",
      "current",
      timestamp,
    ),
    createActivation(
      "activation-character-state-toy",
      "character-state-toy",
      "CharacterDynamicState",
      "task-default",
      "high",
      "current",
      timestamp,
    ),
  ];

  return {
    ...base,
    taskType: "draft-generation",
    targetRef: "scene-card-toy",
    activations,
    includedItems: [
      includedItem("scene-card-toy", "SceneCard", HASH_A, "activation-scene-toy"),
      includedItem("beat-toy", "Beat", HASH_B, "activation-beat-toy"),
      includedItem(
        "character-state-toy",
        "CharacterDynamicState",
        HASH_C,
        "activation-character-state-toy",
      ),
    ],
    exclusions: [
      {
        id: "exclusion-style-budget-toy",
        contextPackId: "context-pack-draft-toy",
        excludedRef: "style-profile-extra-toy",
        sourceObjectType: "StyleProfile",
        reason: "budget-insufficient",
        riskKind: "budget-overflow",
        explanation: "Optional style profile exceeded the object budget for draft preview.",
        candidateScore: "medium",
        wouldFitIfBudget: true,
        createdAt: timestamp,
      },
      {
        id: "exclusion-state-stale-toy",
        contextPackId: "context-pack-draft-toy",
        excludedRef: "character-state-stale-toy",
        sourceObjectType: "CharacterDynamicState",
        reason: "stale-or-superseded",
        riskKind: "stale-state",
        explanation: "A confirmed change set superseded this older character state.",
        candidateScore: "high",
        wouldFitIfBudget: true,
        createdAt: timestamp,
      },
    ],
    budget: {
      id: "budget-draft-toy",
      contextPackId: "context-pack-draft-toy",
      tokenBudget: {
        maxInputTokens: 1200,
        reservedOutputTokens: 600,
        estimationMethod: "toy-char-estimate",
      },
      fragmentBudget: {
        maxFragments: 6,
        maxFragmentChars: 800,
      },
      objectBudget: [
        { objectType: "SceneCard", maxObjects: 1 },
        { objectType: "Beat", maxObjects: 4 },
        { objectType: "CharacterDynamicState", maxObjects: 1 },
      ],
      priorityBands: {
        required: 1,
        high: 3,
        normal: 2,
        low: 0,
      },
      trimmingStrategy: "drop-low-priority",
      excludedObjectTypes: [],
      excludedRefs: [],
      overflowPolicy: "fail-closed",
    },
    contextHash: {
      algorithm: "sha256",
      canonicalization: "json-canonical-v1",
      schemaVersionMap: [
        {
          schemaId: "https://infinite-typewriter.local/schemas/context-pack.schema.json",
          version: 1,
        },
      ],
      libraryManifestHash: HASH_D,
      sourceHashRefs: [{ ref: "source-toy", hash: HASH_E }],
      changeSetRefs: ["change-set-toy"],
      changeSetHash: HASH_F,
      selectionPolicyVersion: "context-pack-selection-v1",
      includedItemHashes: [
        { ref: "scene-card-toy", hash: HASH_A },
        { ref: "beat-toy", hash: HASH_B },
        { ref: "character-state-toy", hash: HASH_C },
      ],
      activationHashes: [
        { ref: "scene-card-toy", hash: HASH_A },
        { ref: "beat-toy", hash: HASH_B },
        { ref: "character-state-toy", hash: HASH_C },
      ],
      excludedItemHashes: [{ ref: "exclusion-manifest-toy", hash: HASH_D }],
      value: "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
    },
    createdForTaskRef: "task-draft-preview-toy",
  };
}

export function createToySourceBreakdownRevisionContextPack(timestamp: IsoDateTime): ContextPack {
  const base = createToyBaseObject(
    "context-pack",
    "context-pack-source-breakdown-rerun-toy",
    "Toy Source Breakdown Revision Context",
    timestamp,
    "draft",
  );
  const contextHash = "1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef";

  return {
    ...base,
    taskType: "source-breakdown",
    taskDescriptor: {
      taskType: "source-breakdown",
      sourceBreakdownStage: "draft-review-rerun",
      inputRefs: ["source-chunk-toy", "draft-output-fake-toy", "user-feedback-toy"],
      outputRefs: ["revision-suggestion-toy"],
      sourceChunkRefs: ["source-chunk-toy"],
      originalChunkRef: "source-chunk-toy",
      originalDraftRef: "draft-output-fake-toy",
      userFeedbackRef: "user-feedback-toy",
      revisionRequestRef: "revision-request-toy",
      outputSchemaRef: "toy-draft-output",
      outputSchemaVersion: 1,
      contextHash,
    },
    targetRef: "revision-request-toy",
    activations: [
      createActivation(
        "activation-revision-request-toy",
        "revision-request-toy",
        "RevisionRequest",
        "target-match",
        "required",
        "current",
        timestamp,
        "context-pack-source-breakdown-rerun-toy",
      ),
    ],
    includedItems: [
      includedItem("source-chunk-toy", "SourceChunk", HASH_A, "activation-revision-request-toy"),
      includedItem("draft-output-fake-toy", "DraftOutput", HASH_B, "activation-revision-request-toy"),
      includedItem("user-feedback-toy", "UserFeedback", HASH_C, "activation-revision-request-toy"),
    ],
    exclusions: [],
    budget: {
      id: "budget-source-breakdown-rerun-toy",
      contextPackId: "context-pack-source-breakdown-rerun-toy",
      tokenBudget: {
        maxInputTokens: 900,
        reservedOutputTokens: 300,
        estimationMethod: "toy-char-estimate",
      },
      fragmentBudget: {
        maxFragments: 4,
        maxFragmentChars: 700,
      },
      objectBudget: [
        { objectType: "RevisionRequest", maxObjects: 1 },
        { objectType: "SourceChunk", maxObjects: 1 },
        { objectType: "DraftOutput", maxObjects: 1 },
        { objectType: "UserFeedback", maxObjects: 1 },
      ],
      priorityBands: {
        required: 1,
        high: 3,
        normal: 0,
        low: 0,
      },
      trimmingStrategy: "drop-low-priority",
      excludedObjectTypes: [],
      excludedRefs: [],
      overflowPolicy: "fail-closed",
    },
    contextHash: {
      algorithm: "sha256",
      canonicalization: "json-canonical-v1",
      schemaVersionMap: [
        {
          schemaId: "https://infinite-typewriter.local/schemas/context-pack.schema.json",
          version: 1,
        },
      ],
      libraryManifestHash: HASH_D,
      sourceHashRefs: [{ ref: "source-toy", hash: HASH_E }],
      changeSetRefs: ["change-set-toy"],
      changeSetHash: HASH_F,
      selectionPolicyVersion: "context-pack-selection-v1",
      promptTemplateVersion: "source-breakdown-rerun-v1",
      includedItemHashes: [
        { ref: "source-chunk-toy", hash: HASH_A },
        { ref: "draft-output-fake-toy", hash: HASH_B },
        { ref: "user-feedback-toy", hash: HASH_C },
      ],
      activationHashes: [{ ref: "revision-request-toy", hash: HASH_A }],
      value: contextHash,
    },
    createdForTaskRef: "task-source-breakdown-rerun-toy",
  };
}

function createActivation(
  id: StableId,
  sourceRef: StableId,
  sourceObjectType: string,
  activationReason: ActivationReason,
  priority: ContextPriority,
  freshnessState: FreshnessState,
  timestamp: IsoDateTime,
  contextPackId = "context-pack-draft-toy",
): ContextActivation {
  return {
    id,
    contextPackId,
    sourceRef,
    sourceObjectType,
    sourceSchemaVersion: 1,
    activationReason,
    evidenceChain: [
      {
        ref: "scene-card-toy",
        relation: "draft-context",
        note: "Toy public fixture links the target scene to required draft context.",
      },
    ],
    relevance: priority === "required" ? "high" : "medium",
    confidence: "high",
    priority,
    freshnessState,
    userSelection: "auto",
    selectedAt: timestamp,
  };
}

function includedItem(
  sourceRef: StableId,
  sourceObjectType: string,
  contentHash: string,
  activationRef: StableId,
): ContextIncludedItem {
  return {
    sourceRef,
    sourceObjectType,
    sourceSchemaVersion: 1,
    contentHash,
    activationRef,
  };
}
