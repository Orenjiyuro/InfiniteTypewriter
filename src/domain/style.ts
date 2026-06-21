import {
  createToyBaseObject,
  createToyBaseObjectV2,
  type BaseObject,
  type BaseObjectV2,
  type IsoDateTime,
  type StableId,
} from "./common";
import type { ContextTaskType } from "./context-pack";

export interface StyleProfile extends BaseObject {
  type: "style-profile";
  ownerRef: StableId;
  scope: string;
  styleDimensions: string[];
  voiceConstraints: string[];
  doNotImitate: string[];
  evidenceRefIds: StableId[];
  transferRuleIds: StableId[];
  misuseRisks: string[];
  taskUseHints: string[];
}

export interface NarrativeDNA extends BaseObjectV2 {
  type: "narrative-dna";
  workId: StableId;
  readerExperienceGoal: string;
  promiseProgressPayoffPattern: string;
  tensionEngine: string;
  sceneRhythm: string;
  signatureMoves: string[];
  evidenceRefIds: StableId[];
  coverage: string;
  confidence: "low" | "medium" | "high";
  mechanismRefIds: StableId[];
  transferRuleRefs: StableId[];
  applicableTaskTypes: ContextTaskType[];
  applicableScope: string;
  misuseRisks: string[];
  doNotCopy: string[];
  prohibitedResidue: string[];
  currentWorkConflictChecks: string[];
}

export interface MicroTechnique extends BaseObject {
  type: "micro-technique";
  name: string;
  mechanism: string;
  useCase: string;
  constraints: string[];
  sourceEvidenceRefs: StableId[];
  transferRuleIds: StableId[];
}

export interface ImageryRecord extends BaseObject {
  type: "imagery-record";
  workId: StableId;
  imageLabel: string;
  meaning: string;
  firstUseRef: StableId;
  recurrenceRules: string[];
  currentState: string;
}

export interface BookMechanic extends BaseObject {
  type: "book-mechanic";
  ownerRef: StableId;
  mechanicName: string;
  mechanicPurpose: string;
  operatingRules: string[];
  failureModes: string[];
  evidenceRefIds: StableId[];
}

export function createToyStyleProfile(timestamp: IsoDateTime): StyleProfile {
  return {
    ...createToyBaseObject(
      "style-profile",
      "style-profile-toy",
      "Toy Practical Pressure Style",
      timestamp,
      "active",
    ),
    ownerRef: "work-public-toy",
    scope: "single invented scene",
    styleDimensions: ["plain pressure", "visible cost"],
    voiceConstraints: ["Use concrete cause and effect."],
    doNotImitate: ["Do not copy source phrasing."],
    evidenceRefIds: ["evidence-toy-pressure"],
    transferRuleIds: ["transfer-rule-toy"],
    misuseRisks: ["Pressure is stated after the action."],
    taskUseHints: ["scene-planning", "style-revision"],
  };
}

export function createToyNarrativeDNA(timestamp: IsoDateTime): NarrativeDNA {
  return {
    ...createToyBaseObjectV2(
      "narrative-dna",
      "narrative-dna-toy",
      "Toy Narrative DNA",
      timestamp,
      "active",
    ),
    workId: "work-public-toy",
    readerExperienceGoal: "Make choices feel visibly costly.",
    promiseProgressPayoffPattern: "Promise a cost, escalate it, pay it off with a choice.",
    tensionEngine: "Visible scarcity before commitment.",
    sceneRhythm: "Set cost, force choice, show consequence.",
    signatureMoves: ["cost-before-choice"],
    evidenceRefIds: ["evidence-toy-pressure"],
    coverage: "single invented scene",
    confidence: "medium",
    mechanismRefIds: ["reference-mechanism-toy"],
    transferRuleRefs: ["transfer-rule-toy"],
    applicableTaskTypes: ["outline-generation", "draft-generation"],
    applicableScope: "original scenes that need practical pressure",
    misuseRisks: ["Using scarcity after the choice weakens causality."],
    doNotCopy: ["Do not copy source scenes."],
    prohibitedResidue: ["No copied names", "No copied event sequence"],
    currentWorkConflictChecks: ["Skip if the current work already resolved the cost."],
  };
}
