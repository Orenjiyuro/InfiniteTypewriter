import { createToyBaseObject, type BaseObject, type IsoDateTime, type StableId } from "./common";
import type { ContextTaskType } from "./context-pack";

export type ConcernKind =
  | "outline"
  | "character"
  | "scene"
  | "setting"
  | "worldbuilding"
  | "pov"
  | "reader-contract"
  | "style"
  | "craft"
  | "relationship"
  | "foreshadowing"
  | "revision"
  | "custom";

export interface CoverageClaim {
  scope: string;
  unavailableScope: string[];
  evidenceRefIds: StableId[];
  confidence: "low" | "medium" | "high";
  rationale: string;
}

export interface FieldEvidenceRef {
  fieldPath: string;
  evidenceRefIds: StableId[];
  required: boolean;
}

export interface TransferRule {
  id: StableId;
  sourceFact: string;
  transferableMechanism: string;
  targetVariableSlots: string[];
  prohibitedResidue: string[];
  misuseRisks: string[];
}

export interface EvidenceCard extends BaseObject {
  type: "evidence-card";
  breakdownProjectId: StableId;
  breakdownScopeId: StableId;
  sourceLocator: string;
  observationKind: "observation" | "inference" | "assumption";
  observedTechnique: string;
  functionInSource: string;
  transferableMechanism: string;
  prohibitedCopying: string[];
  coverage: CoverageClaim;
  confidence: "low" | "medium" | "high";
  fieldEvidenceRefs: FieldEvidenceRef[];
}

export interface ReferenceMechanism extends BaseObject {
  type: "reference-mechanism";
  sourceEvidenceRefs: StableId[];
  mechanism: string;
  transferRuleIds: StableId[];
  variableSlots: string[];
  prohibitedResidue: string[];
  misuseRisks: string[];
  applicableTaskTypes: ContextTaskType[];
  applicableScope: string;
  currentWorkConflictChecks: string[];
}

export interface BreakdownConcern {
  id: StableId;
  kind: ConcernKind;
  label: string;
  requiredForRecipe: boolean;
  coverage: CoverageClaim;
  fieldEvidenceRefs: FieldEvidenceRef[];
  transferRuleIds: StableId[];
}

export interface BookSpecificModuleField {
  name: string;
  valueKind: "text" | "number" | "boolean" | "string-list" | "record-list";
  evidenceRequired: boolean;
  displayHint: "card" | "table" | "timeline" | "graph" | "hidden";
  migrationStrategy: "preserve" | "drop-if-unknown" | "manual-review";
}

export interface BookSpecificModule {
  id: StableId;
  label: string;
  appliesToConcernIds: StableId[];
  fields: BookSpecificModuleField[];
}

export interface ViewProjection {
  id: StableId;
  label: string;
  targetConcernId: StableId;
  sourceFieldPaths: string[];
  displayKind: "card" | "table" | "timeline" | "graph";
}

export interface CompletionGateCheck {
  id: StableId;
  targetRef: StableId;
  gateLevel: "setup" | "stage" | "volume" | "book";
  checkKind: string;
  blocking: boolean;
  passed: boolean;
  evidenceRefIds: StableId[];
  notes: string;
}

export interface BreakdownRecipe extends BaseObject {
  type: "breakdown-recipe";
  concerns: BreakdownConcern[];
  transferRules: TransferRule[];
  modules: BookSpecificModule[];
  viewProjections: ViewProjection[];
  completionGateChecks: CompletionGateCheck[];
}

export interface BreakdownProject extends BaseObject {
  type: "breakdown-project";
  sourceId: StableId;
  recipeId?: StableId;
  manualMethod?: string;
  objective: string;
  scopeIds: StableId[];
  outputRefIds: StableId[];
}

export interface BreakdownScope extends BaseObject {
  type: "breakdown-scope";
  breakdownProjectId: StableId;
  scopeLabel: string;
  locator: string;
  coverageIntent: string;
  exclusions: string[];
}

export function createToyCoverageClaim(): CoverageClaim {
  return {
    scope: "toy chapter",
    unavailableScope: ["later chapters"],
    evidenceRefIds: ["evidence-toy-pressure"],
    confidence: "medium",
    rationale: "The toy fixture covers one invented scene only.",
  };
}

export function createToyBreakdownRecipe(timestamp: IsoDateTime): BreakdownRecipe {
  return {
    ...createToyBaseObject("breakdown-recipe", "recipe-toy-longform", "Toy Longform Recipe", timestamp, "active"),
    concerns: [
      {
        id: "concern-character-toy",
        kind: "character",
        label: "Character profile",
        requiredForRecipe: true,
        coverage: createToyCoverageClaim(),
        fieldEvidenceRefs: [
          {
            fieldPath: "stableProfile.coreDrive",
            evidenceRefIds: ["evidence-toy-pressure"],
            required: true,
          },
        ],
        transferRuleIds: ["transfer-rule-toy"],
      },
    ],
    transferRules: [
      {
        id: "transfer-rule-toy",
        sourceFact: "Invented public pressure fact.",
        transferableMechanism: "A visible constraint reveals priority.",
        targetVariableSlots: ["constraint", "choice", "cost"],
        prohibitedResidue: ["No copied names", "No copied event sequence"],
        misuseRisks: ["Constraint appears after the choice."],
      },
    ],
    modules: [
      {
        id: "module-character-profile-v1",
        label: "Character profile v1",
        appliesToConcernIds: ["concern-character-toy"],
        fields: [
          {
            name: "coreDrive",
            valueKind: "text",
            evidenceRequired: true,
            displayHint: "card",
            migrationStrategy: "manual-review",
          },
        ],
      },
    ],
    viewProjections: [
      {
        id: "view-character-card-toy",
        label: "Character card",
        targetConcernId: "concern-character-toy",
        sourceFieldPaths: ["stableProfile.coreDrive"],
        displayKind: "card",
      },
    ],
    completionGateChecks: [
      {
        id: "gate-character-profile-toy",
        targetRef: "concern-character-toy",
        gateLevel: "setup",
        checkKind: "character-profile-evidence",
        blocking: true,
        passed: true,
        evidenceRefIds: ["evidence-toy-pressure"],
        notes: "Toy character concern has one evidence reference.",
      },
    ],
  };
}

export function createToyEvidenceCard(timestamp: IsoDateTime): EvidenceCard {
  return {
    ...createToyBaseObject(
      "evidence-card",
      "evidence-toy-pressure",
      "Toy pressure evidence",
      timestamp,
      "active",
    ),
    breakdownProjectId: "breakdown-project-toy",
    breakdownScopeId: "breakdown-scope-toy",
    sourceLocator: "chapter-1-scene-1",
    observationKind: "observation",
    observedTechnique: "A visible resource limit turns route choice into pressure.",
    functionInSource: "Creates pressure without copying events or names.",
    transferableMechanism: "Expose a scarce resource before the character commits.",
    prohibitedCopying: ["No names", "No dialogue", "No event sequence"],
    coverage: createToyCoverageClaim(),
    confidence: "medium",
    fieldEvidenceRefs: [
      {
        fieldPath: "observedTechnique",
        evidenceRefIds: ["evidence-toy-pressure"],
        required: true,
      },
    ],
  };
}

export function createToyReferenceMechanism(timestamp: IsoDateTime): ReferenceMechanism {
  return {
    ...createToyBaseObject(
      "reference-mechanism",
      "reference-mechanism-toy",
      "Toy scarcity before commitment",
      timestamp,
      "active",
    ),
    sourceEvidenceRefs: ["evidence-toy-pressure"],
    mechanism: "Scarcity before commitment",
    transferRuleIds: ["transfer-rule-toy"],
    variableSlots: ["constraint", "choice", "cost"],
    prohibitedResidue: ["No copied names", "No copied event sequence"],
    misuseRisks: ["Constraint appears after the choice."],
    applicableTaskTypes: ["scene-planning", "draft-generation"],
    applicableScope: "original scenes that need costed choices",
    currentWorkConflictChecks: [
      "Do not reuse if the current scene already resolved the cost.",
    ],
  };
}
