import { createToyBaseObject, type BaseObject, type IsoDateTime, type StableId } from "./common";
import type { ContextTaskType } from "./context-pack";

export interface Work extends BaseObject {
  type: "work";
  workKind: "novel" | "series" | "short" | "other";
  premise: string;
  readerContractRefs: StableId[];
  outline: WorkOutline;
}

export interface WorkOutline {
  id: StableId;
  workId: StableId;
  volumes: VolumeOutline[];
}

export interface VolumeOutline {
  id: StableId;
  workId: StableId;
  title: string;
  sequence: number;
  purpose: string;
  segments: StorySegment[];
}

export interface StorySegment {
  id: StableId;
  workId: StableId;
  parentId?: StableId;
  label: string;
  sequence: number;
  goal: string;
  pressure: string;
  chapters: ChapterPlan[];
}

export interface ChapterPlan {
  id: StableId;
  workId: StableId;
  sequence: number;
  title: string;
  chapterGoal: string;
  sceneRefs: StableId[];
}

export interface Beat {
  id: StableId;
  sceneId: StableId;
  sequence: number;
  beatKind: "action" | "dialogue" | "reveal" | "decision" | "reversal" | "transition" | "other";
  intent: string;
  expectedChange: string;
  characterRefs: StableId[];
  evidenceRefIds: StableId[];
}

export interface SceneCard extends BaseObject {
  type: "scene-card";
  chapterId: StableId;
  sceneGoal: string;
  povCharacterId?: StableId;
  povMode: string;
  knowledgeBoundaryRef: StableId;
  settingPressureRef?: StableId;
  settingPressureText?: string;
  entryState: string;
  exitState: string;
  characterStateRefs: StableId[];
  relationshipRefs: StableId[];
  foreshadowingRefs: StableId[];
  styleTargetRefs: StableId[];
  beats: Beat[];
}

export type StorySceneCard = SceneCard;

export interface ForeshadowingEntry {
  id: StableId;
  ledgerId: StableId;
  signal: string;
  plantedAtRef: StableId;
  intendedPayoff: string;
  currentState: "planted" | "partially-revealed" | "resolved" | "abandoned";
  visibility: "hidden" | "subtle" | "clear";
  riskNotes: string[];
}

export interface ForeshadowingLedger extends BaseObject {
  type: "foreshadowing-ledger";
  workId: StableId;
  entries: ForeshadowingEntry[];
  lastReviewedAt: IsoDateTime;
}

export interface RevisionLayer extends BaseObject {
  type: "revision-layer";
  targetRef: StableId;
  layerKind: "structure" | "character" | "scene" | "prose-style" | "continuity";
  objective: string;
  reviewItemRefs: StableId[];
}

export interface ContinuityIssue extends BaseObject {
  type: "continuity-issue";
  targetRef: StableId;
  issueKind: string;
  description: string;
  relatedRefs: StableId[];
  severity: "low" | "medium" | "high";
  proposedFix: string;
  confirmationState: "draft" | "accepted" | "rejected";
}

export interface DraftReviewItem extends BaseObject {
  type: "draft-review-item";
  targetRef: StableId;
  sourceDraftRef: StableId;
  severity: "low" | "medium" | "high";
  category: string;
  finding: string;
  recommendation: string;
  evidenceRefIds: StableId[];
  blockedReason?: string;
  conflictRefs?: StableId[];
}

export interface RevisionRequest extends BaseObject {
  type: "revision-request";
  targetRef: StableId;
  originalChunkRef: StableId;
  originalDraftRef: StableId;
  userFeedback: string;
  outputSchemaRef: string;
  outputSchemaVersion: number;
  contextHash: string;
  requestedRerunTaskType: ContextTaskType;
  requestStatus: "draft" | "ready-for-rerun" | "rerun-created" | "blocked" | "closed";
}

export interface ChangeSetEntry {
  targetRef: StableId;
  changeType: "create" | "update" | "remove";
  description: string;
}

export interface ChangeSet extends BaseObject {
  type: "change-set";
  summary: string;
  reason: string;
  sourceDraftRef: StableId;
  revisionRequestRefs: StableId[];
  targetRefs: StableId[];
  changes: ChangeSetEntry[];
  impactScope: string[];
  confirmationState: "pending-user-confirmation" | "confirmed" | "rejected";
}

export interface RevisionSuggestion {
  targetRef: StableId;
  suggestedTextRef: StableId;
  reason: string;
  evidenceRefIds: StableId[];
  riskNotes: string[];
}

export interface ProseQualityPass extends BaseObject {
  type: "prose-quality-pass";
  targetRef: StableId;
  qualityGoals: string[];
  beforeRef: StableId;
  suggestions: RevisionSuggestion[];
  decision: "pending-user-review" | "accepted" | "rejected";
}

export interface StyleRevisionPass extends BaseObject {
  type: "style-revision-pass";
  targetRef: StableId;
  styleProfileRefs: StableId[];
  voiceConstraints: string[];
  suggestions: RevisionSuggestion[];
  decision: "pending-user-review" | "accepted" | "rejected";
}

export function createToyWork(timestamp: IsoDateTime): Work {
  return {
    ...createToyBaseObject("work", "work-public-toy", "Public Toy Work", timestamp),
    workKind: "novel",
    premise: "A learner must choose which clue to preserve under a visible cost.",
    readerContractRefs: ["reader-contract-toy"],
    outline: {
      id: "outline-public-toy",
      workId: "work-public-toy",
      volumes: [],
    },
  };
}

export function createToySceneCard(timestamp: IsoDateTime): SceneCard {
  return {
    ...createToyBaseObject("scene-card", "scene-toy-gate", "Toy Gate Choice", timestamp),
    chapterId: "chapter-toy-one",
    sceneGoal: "Force a costed choice.",
    povMode: "third-person-limited",
    knowledgeBoundaryRef: "knowledge-boundary-toy",
    settingPressureText: "A token opens only one route.",
    entryState: "The clue path is unknown.",
    exitState: "One route is chosen and one is closed.",
    characterStateRefs: ["dynamic-state-gate-choice"],
    relationshipRefs: ["relationship-mentor-learner"],
    foreshadowingRefs: ["foreshadowing-token-cost"],
    styleTargetRefs: ["style-profile-toy"],
    beats: [
      {
        id: "beat-token-decision",
        sceneId: "scene-toy-gate",
        sequence: 1,
        beatKind: "decision",
        intent: "Make the resource cost explicit.",
        expectedChange: "The protagonist commits to one route.",
        characterRefs: ["character-practical-learner"],
        evidenceRefIds: ["evidence-toy-pressure"],
      },
    ],
  };
}

export function createToyRevisionRequest(timestamp: IsoDateTime): RevisionRequest {
  return {
    ...createToyBaseObject(
      "revision-request",
      "revision-request-toy",
      "Toy revision request",
      timestamp,
    ),
    targetRef: "evidence-toy-pressure",
    originalChunkRef: "source-chunk-toy",
    originalDraftRef: "draft-output-fake-toy",
    userFeedback: "Clarify the cost before the choice.",
    outputSchemaRef: "toy-draft-output",
    outputSchemaVersion: 1,
    contextHash: "0000000000000000000000000000000000000000000000000000000000000000",
    requestedRerunTaskType: "source-breakdown",
    requestStatus: "ready-for-rerun",
  };
}

export function createToyProseQualityPass(timestamp: IsoDateTime): ProseQualityPass {
  return {
    ...createToyBaseObject(
      "prose-quality-pass",
      "prose-quality-pass-toy",
      "Toy prose quality pass",
      timestamp,
    ),
    targetRef: "scene-toy-gate",
    qualityGoals: ["clarity", "cause-before-effect"],
    beforeRef: "draft-before-toy",
    suggestions: [
      {
        targetRef: "draft-before-toy",
        suggestedTextRef: "draft-after-toy",
        reason: "Improve clarity before the costed choice.",
        evidenceRefIds: ["evidence-toy-pressure"],
        riskNotes: ["Do not imitate source phrasing."],
      },
    ],
    decision: "pending-user-review",
  };
}

export function createToyChangeSet(timestamp: IsoDateTime): ChangeSet {
  return {
    ...createToyBaseObject("change-set", "change-set-toy", "Toy change set", timestamp),
    summary: "Clarify the toy gate cost.",
    reason: "User accepted a revision suggestion.",
    sourceDraftRef: "draft-output-fake-toy",
    revisionRequestRefs: ["revision-request-toy"],
    targetRefs: ["scene-toy-gate"],
    changes: [
      {
        targetRef: "scene-toy-gate",
        changeType: "update",
        description: "Make the cost explicit before the choice.",
      },
    ],
    impactScope: ["scene"],
    confirmationState: "pending-user-confirmation",
  };
}
