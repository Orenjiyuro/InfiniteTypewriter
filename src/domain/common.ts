export type IsoDateTime = string;
export type StableId = string;
export type ObjectStatus = "draft" | "active" | "archived" | "deprecated";
export type MetadataValue = string | number | boolean;

export interface AuditTrailEntry {
  at: IsoDateTime;
  action: "create" | "update" | "confirm" | "reject" | "archive" | "migrate";
  actor: "user" | "system" | "provider";
  reason: string;
  sourceRefIds: StableId[];
}

export interface ContextHints {
  activationKeywords?: string[];
  priority?: "low" | "normal" | "high";
  exclusionReason?: string;
  budgetHint?: "minimal" | "standard" | "full";
}

export interface BaseObject {
  schemaVersion: 1;
  id: StableId;
  type: string;
  label: string;
  createdAt: IsoDateTime;
  updatedAt: IsoDateTime;
  status: ObjectStatus;
  tags: string[];
  metadata: Record<string, MetadataValue>;
  auditTrail: AuditTrailEntry[];
  contextHints?: ContextHints;
}

export interface BaseObjectV2 extends Omit<BaseObject, "schemaVersion"> {
  schemaVersion: 2;
}

export function createToyBaseObject<TType extends string>(
  type: TType,
  id: StableId,
  label: string,
  timestamp: IsoDateTime,
  status: ObjectStatus = "draft",
): BaseObject & { type: TType } {
  return {
    schemaVersion: 1,
    id,
    type,
    label,
    createdAt: timestamp,
    updatedAt: timestamp,
    status,
    tags: ["toy"],
    metadata: {},
    auditTrail: [],
  };
}

export function createToyBaseObjectV2<TType extends string>(
  type: TType,
  id: StableId,
  label: string,
  timestamp: IsoDateTime,
  status: ObjectStatus = "draft",
): BaseObjectV2 & { type: TType } {
  return {
    schemaVersion: 2,
    id,
    type,
    label,
    createdAt: timestamp,
    updatedAt: timestamp,
    status,
    tags: ["toy"],
    metadata: {},
    auditTrail: [],
  };
}
