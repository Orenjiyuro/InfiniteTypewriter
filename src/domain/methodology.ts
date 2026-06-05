export type IsoDateTime = string;
export type ConfidenceLevel = "low" | "medium" | "high";
export type DraftReviewSeverity = "low" | "medium" | "high";
export type ChangeType = "create" | "update" | "remove";

export interface SourceReference {
  sourceId: string;
  title: string;
  locator: string;
}

export interface EvidenceLocator {
  locator: string;
  summary: string;
}

export interface EvidenceCoverage {
  scope: string;
  notes: string;
}

export interface ConfidenceRating {
  level: ConfidenceLevel;
  rationale: string;
}

export interface EvidenceCard {
  id: string;
  kind: "evidence-card";
  source: SourceReference;
  evidence: EvidenceLocator[];
  observedTechnique: string;
  sourceFunction: string;
  transferableMechanism: string;
  forbiddenCopy: string[];
  coverage: EvidenceCoverage;
  confidence: ConfidenceRating;
  createdAt: IsoDateTime;
  updatedAt: IsoDateTime;
}

export interface ReaderContract {
  id: string;
  kind: "reader-contract";
  promise: string;
  progress: string;
  payoff: string;
  tone: string;
  emotionalTarget: string;
  evidenceCardIds: string[];
}

export interface KnowledgeBoundary {
  id: string;
  kind: "knowledge-boundary";
  pov: string;
  narratorKnows: string[];
  readerKnows: string[];
  withheldInformation: string[];
  unreliable: boolean;
}

export interface SettingPressure {
  id: string;
  kind: "setting-pressure";
  settingElement: string;
  restriction: string;
  resource: string;
  cost: string;
  choiceImpact: string;
}

export interface ReferenceMechanism {
  id: string;
  kind: "reference-mechanism";
  evidenceCardId: string;
  mechanism: string;
  transferRules: string[];
  forbiddenCopies: string[];
}

export interface SceneCard {
  id: string;
  kind: "scene-card";
  workId: string;
  title: string;
  pov: string;
  knowledgeBoundaryId: string;
  settingPressureIds: string[];
  readerContractId: string;
  characterGoals: string[];
  conflictAgenda: string;
  revealedInformation: string[];
  hiddenInformation: string[];
  stateChange: string;
  turn: string;
}

export interface DraftReviewItem {
  id: string;
  kind: "draft-review-item";
  targetId: string;
  severity: DraftReviewSeverity;
  category: string;
  finding: string;
  recommendation: string;
  evidenceCardIds: string[];
}

export interface ChangeSetEntry {
  targetId: string;
  changeType: ChangeType;
  description: string;
}

export interface ChangeSet {
  id: string;
  kind: "change-set";
  createdAt: IsoDateTime;
  summary: string;
  changes: ChangeSetEntry[];
  affectedIds: string[];
}

export interface ContextPack {
  id: string;
  kind: "context-pack";
  createdAt: IsoDateTime;
  updatedAt: IsoDateTime;
  purpose: string;
  evidenceCards: EvidenceCard[];
  readerContracts: ReaderContract[];
  knowledgeBoundaries: KnowledgeBoundary[];
  settingPressures: SettingPressure[];
  referenceMechanisms: ReferenceMechanism[];
  sceneCards: SceneCard[];
  draftReviewItems: DraftReviewItem[];
  changeSets: ChangeSet[];
}

export interface EvidenceCardValidationResult {
  valid: boolean;
  missing: string[];
}

const REQUIRED_EVIDENCE_CARD_FIELDS = [
  "evidence",
  "source",
  "coverage",
  "confidence",
] as const;

export function validateEvidenceCard(
  value: Partial<EvidenceCard>,
): EvidenceCardValidationResult {
  const missing: string[] = REQUIRED_EVIDENCE_CARD_FIELDS.filter(
    (field) => value[field] == null,
  );

  if (value.evidence != null && value.evidence.length === 0) {
    missing.push("evidence");
  }
  if (value.evidence != null) {
    value.evidence.forEach((evidence, index) => {
      if (!hasText(evidence.locator)) {
        missing.push(`evidence[${index}].locator`);
      }
      if (!hasText(evidence.summary)) {
        missing.push(`evidence[${index}].summary`);
      }
    });
  }
  if (value.source != null) {
    if (!hasText(value.source.sourceId)) {
      missing.push("source.sourceId");
    }
    if (!hasText(value.source.title)) {
      missing.push("source.title");
    }
    if (!hasText(value.source.locator)) {
      missing.push("source.locator");
    }
  }
  if (value.coverage != null) {
    if (!hasText(value.coverage.scope)) {
      missing.push("coverage.scope");
    }
    if (!hasText(value.coverage.notes)) {
      missing.push("coverage.notes");
    }
  }
  if (value.confidence != null) {
    if (value.confidence.level == null) {
      missing.push("confidence.level");
    }
    if (!hasText(value.confidence.rationale)) {
      missing.push("confidence.rationale");
    }
  }

  return {
    valid: missing.length === 0,
    missing: [...missing],
  };
}

function hasText(value: string | undefined): boolean {
  return typeof value === "string" && value.length > 0;
}

export function createToyContextPack(timestamp: IsoDateTime): ContextPack {
  const evidenceCard: EvidenceCard = {
    id: "evidence-toy-pressure",
    kind: "evidence-card",
    source: {
      sourceId: "source-public-toy",
      title: "Public Toy Source",
      locator: "chapter-1-scene-1",
    },
    evidence: [
      {
        locator: "chapter-1-scene-1",
        summary:
          "Invented public example: a gate opens only after a limited token is spent.",
      },
    ],
    observedTechnique: "A visible resource limit turns a simple route into a choice.",
    sourceFunction:
      "Creates pressure without needing copied events, names, or dialogue.",
    transferableMechanism:
      "Expose a scarce resource before the character commits to a route.",
    forbiddenCopy: [
      "Do not copy source events, names, dialogue, or relationship chains.",
    ],
    coverage: {
      scope: "single-scene",
      notes: "Covers only the pressure mechanism in a toy scene.",
    },
    confidence: {
      level: "medium",
      rationale: "The example is narrow and intentionally public-safe.",
    },
    createdAt: timestamp,
    updatedAt: timestamp,
  };

  return {
    id: "context-pack-toy",
    kind: "context-pack",
    createdAt: timestamp,
    updatedAt: timestamp,
    purpose: "Toy original planning context",
    evidenceCards: [evidenceCard],
    readerContracts: [
      {
        id: "reader-contract-toy",
        kind: "reader-contract",
        promise: "A practical problem will reveal character priorities.",
        progress: "Each choice shows a cost.",
        payoff: "The final route resolves the immediate constraint.",
        tone: "focused",
        emotionalTarget: "curiosity",
        evidenceCardIds: [evidenceCard.id],
      },
    ],
    knowledgeBoundaries: [
      {
        id: "knowledge-boundary-toy",
        kind: "knowledge-boundary",
        pov: "third-person-limited",
        narratorKnows: ["The current room condition."],
        readerKnows: ["A token is required."],
        withheldInformation: ["The later consequence of spending the token."],
        unreliable: false,
      },
    ],
    settingPressures: [
      {
        id: "setting-pressure-toy",
        kind: "setting-pressure",
        settingElement: "Token gate",
        restriction: "Only one path can be opened.",
        resource: "One-use token",
        cost: "The alternate path closes.",
        choiceImpact: "The protagonist must choose which clue to preserve.",
      },
    ],
    referenceMechanisms: [
      {
        id: "reference-mechanism-toy",
        kind: "reference-mechanism",
        evidenceCardId: evidenceCard.id,
        mechanism: "Scarcity before commitment",
        transferRules: ["Keep the scarcity abstract.", "Invent new setting details."],
        forbiddenCopies: ["Names", "Dialogue", "Event sequence"],
      },
    ],
    sceneCards: [
      {
        id: "scene-card-toy",
        kind: "scene-card",
        workId: "work-public-toy",
        title: "Toy Gate Choice",
        pov: "third-person-limited",
        knowledgeBoundaryId: "knowledge-boundary-toy",
        settingPressureIds: ["setting-pressure-toy"],
        readerContractId: "reader-contract-toy",
        characterGoals: ["Preserve the most useful clue."],
        conflictAgenda: "Resource scarcity forces prioritization.",
        revealedInformation: ["The gate consumes the token."],
        hiddenInformation: ["The closed path still matters later."],
        stateChange: "The protagonist commits to one path.",
        turn: "A tool becomes a cost.",
      },
    ],
    draftReviewItems: [
      {
        id: "draft-review-toy",
        kind: "draft-review-item",
        targetId: "scene-card-toy",
        severity: "medium",
        category: "continuity",
        finding: "Clarify that the token is consumed.",
        recommendation: "Add one original sentence before the gate opens.",
        evidenceCardIds: [evidenceCard.id],
      },
    ],
    changeSets: [
      {
        id: "change-set-toy",
        kind: "change-set",
        createdAt: timestamp,
        summary: "Clarified the toy gate cost.",
        changes: [
          {
            targetId: "scene-card-toy",
            changeType: "update",
            description: "Make the cost explicit before the choice.",
          },
        ],
        affectedIds: ["scene-card-toy", "setting-pressure-toy"],
      },
    ],
  };
}
