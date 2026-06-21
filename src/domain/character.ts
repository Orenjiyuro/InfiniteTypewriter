import { createToyBaseObject, type BaseObject, type IsoDateTime, type StableId } from "./common";

export interface CharacterStableProfile extends BaseObject {
  type: "character-stable-profile";
  workId: StableId;
  displayName: string;
  role: string;
  coreDrive: string;
  coreDriveEvidenceRefIds: StableId[];
  fearOrWound: string;
  boundaries: string[];
  boundaryEvidenceRefIds: StableId[];
  competencies: string[];
  values: string[];
  voiceNotes: string[];
  voiceEvidenceRefIds: StableId[];
  actionFingerprints: string[];
}

export interface CharacterDynamicState extends BaseObject {
  type: "character-dynamic-state";
  characterId: StableId;
  scopeRef: StableId;
  timepoint: string;
  emotionalState: string;
  knowledgeState: string[];
  resources: string[];
  constraints: string[];
  evidenceRefIds: StableId[];
}

export interface CharacterArc extends BaseObject {
  type: "character-arc";
  characterId: StableId;
  arcGoal: string;
  arcGoalEvidenceRefIds: StableId[];
  startingBelief: string;
  pressurePattern: string;
  turningPoints: StableId[];
  currentStage: string;
}

export interface CharacterTimelineEntry {
  timepoint: string;
  eventRef: StableId;
  stateDelta: string;
  evidenceRef?: StableId;
  changeSetRef?: StableId;
}

export interface CharacterTimeline extends BaseObject {
  type: "character-timeline";
  characterId: StableId;
  entries: CharacterTimelineEntry[];
  lastVerifiedAt: IsoDateTime;
}

export type RelationshipTemperatureLevel = "cold" | "cool" | "neutral" | "warm" | "hot";

export interface RelationshipEdge extends BaseObject {
  type: "relationship-edge";
  workId: StableId;
  participantIds: StableId[];
  relationshipKind: string;
  currentTemperature: RelationshipTemperatureLevel;
  trust: number;
  tension: number;
  publicStatus: string;
  privateStatus: string;
  evidenceRefIds: StableId[];
}

export interface RelationshipTemperature extends BaseObject {
  type: "relationship-temperature";
  relationshipEdgeId: StableId;
  scopeRef: StableId;
  affinity: number;
  conflict: number;
  dependency: number;
  recentTriggers: string[];
}

export interface FocusRelationshipView extends BaseObject {
  type: "focus-relationship-view";
  relationshipEdgeId: StableId;
  taskType: string;
  includedStateRefs: StableId[];
  excludedStateRefs: StableId[];
  activationReason: string;
}

export interface InteractionPattern extends BaseObject {
  type: "interaction-pattern";
  participantRolePattern: string;
  trigger: string;
  typicalResponse: string;
  variationRules: string[];
  riskNotes: string[];
}

export function createToyCharacterProfile(timestamp: IsoDateTime): CharacterStableProfile {
  return {
    ...createToyBaseObject(
      "character-stable-profile",
      "character-practical-learner",
      "Practical Learner",
      timestamp,
      "active",
    ),
    workId: "work-public-toy",
    displayName: "Practical Learner",
    role: "protagonist",
    coreDrive: "Preserve the clue with the broadest future use.",
    coreDriveEvidenceRefIds: ["evidence-toy-pressure"],
    fearOrWound: "Worries that early certainty hides a later cost.",
    boundaries: ["Will not spend another person's resource without consent."],
    boundaryEvidenceRefIds: ["evidence-toy-pressure"],
    competencies: ["Careful observation", "Tool use"],
    values: ["Consent", "Practical truth"],
    voiceNotes: ["Short concrete sentences under pressure."],
    voiceEvidenceRefIds: ["evidence-toy-pressure"],
    actionFingerprints: ["Checks cost before acting."],
  };
}

export function createToyRelationshipEdge(timestamp: IsoDateTime): RelationshipEdge {
  return {
    ...createToyBaseObject(
      "relationship-edge",
      "relationship-mentor-learner",
      "Mentor and Learner",
      timestamp,
      "active",
    ),
    workId: "work-public-toy",
    participantIds: ["character-practical-learner", "character-careful-mentor"],
    relationshipKind: "mentor",
    currentTemperature: "warm",
    trust: 7,
    tension: 3,
    publicStatus: "Cooperative partners.",
    privateStatus: "The learner withholds uncertainty.",
    evidenceRefIds: ["evidence-toy-pressure"],
  };
}
