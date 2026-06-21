import { createToyBaseObject, type BaseObject, type IsoDateTime, type StableId } from "./common";

export interface ActionCandidate {
  id: StableId;
  simulationContextId: StableId;
  actorId: StableId;
  action: string;
  motivation: string;
  expectedConsequence: string;
  supportingRefs: StableId[];
  riskFlags: StableId[];
}

export interface PuppeteeringRisk {
  id: StableId;
  actionCandidateId: StableId;
  riskKind: string;
  conflictingRefs: StableId[];
  severity: "low" | "medium" | "high";
  explanation: string;
  repairOptions: string[];
}

export interface SimulationContext extends BaseObject {
  type: "simulation-context";
  workId: StableId;
  taskRef: StableId;
  sceneOrSegmentRef: StableId;
  characterRefs: StableId[];
  stableProfileRefs: StableId[];
  dynamicStateRefs: StableId[];
  relationshipRefs: StableId[];
  constraints: string[];
  question: string;
  candidateActions: ActionCandidate[];
}

export interface SimulationResult extends BaseObject {
  type: "simulation-result";
  simulationContextId: StableId;
  candidateRefs: StableId[];
  recommendedDirection: string;
  requiredUserDecisions: string[];
  draftChangeSetRefs: StableId[];
  outlineAdjustmentSuggestionIds: StableId[];
}

export interface OutlineAdjustmentSuggestion {
  id: StableId;
  simulationResultId: StableId;
  targetOutlineRef: StableId;
  currentConstraint: string;
  suggestedChange: string;
  rationale: string;
  riskIfNotAdjusted: string;
}

export function createToySimulationContext(timestamp: IsoDateTime): SimulationContext {
  return {
    ...createToyBaseObject("simulation-context", "simulation-token-choice", "Token choice simulation", timestamp),
    workId: "work-public-toy",
    taskRef: "task-character-simulation-toy",
    sceneOrSegmentRef: "scene-toy-gate",
    characterRefs: ["character-practical-learner"],
    stableProfileRefs: ["character-practical-learner"],
    dynamicStateRefs: ["dynamic-state-gate-choice"],
    relationshipRefs: ["relationship-mentor-learner"],
    constraints: ["Token must be spent before either route opens."],
    question: "Which route can the character choose without violating boundaries?",
    candidateActions: [],
  };
}

export function createToyPuppeteeringRisk(_timestamp: IsoDateTime): PuppeteeringRisk {
  return {
    id: "risk-forced-token-spend",
    actionCandidateId: "action-spend-token-alone",
    riskKind: "boundary-violation",
    conflictingRefs: ["character-practical-learner"],
    severity: "high",
    explanation: "The action spends another person's resource without consent.",
    repairOptions: ["Ask the mentor first.", "Shift the token ownership."],
  };
}

export function createToySimulationResult(timestamp: IsoDateTime): SimulationResult {
  return {
    ...createToyBaseObject(
      "simulation-result",
      "simulation-result-token-choice",
      "Token choice result",
      timestamp,
    ),
    simulationContextId: "simulation-token-choice",
    candidateRefs: ["action-spend-token-alone"],
    recommendedDirection: "Let the character ask before spending the token.",
    requiredUserDecisions: ["Confirm whether the token belongs to both characters."],
    draftChangeSetRefs: [],
    outlineAdjustmentSuggestionIds: ["outline-adjustment-token-choice"],
  };
}
