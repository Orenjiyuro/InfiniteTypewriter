import { describe, expect, it } from "vitest";
import methodologySchema from "../schemas/methodology.schema.json";
import { createToyContextPack } from "../src/domain/methodology";

describe("methodology JSON schema", () => {
  it("publishes the 9.2 public methodology contracts", () => {
    for (const definitionName of [
      "EvidenceCard",
      "ReaderContract",
      "SceneCard",
      "KnowledgeBoundary",
      "SettingPressure",
      "ReferenceMechanism",
      "ContextPack",
      "DraftReviewItem",
      "ChangeSet",
    ]) {
      expect(methodologySchema.$defs).toHaveProperty(definitionName);
    }
  });

  it("requires evidence source coverage and confidence on EvidenceCard", () => {
    expect(methodologySchema.$defs.EvidenceCard.required).toEqual(
      expect.arrayContaining(["evidence", "source", "coverage", "confidence"]),
    );
  });

  it("uses ContextPack as the root schema", () => {
    expect(methodologySchema.$ref).toBe("#/$defs/ContextPack");
  });

  it("validates a public-safe ContextPack fixture against the JSON schema", () => {
    const result = validateAgainstSchema(
      createToyContextPack("2026-06-05T00:00:00.000Z"),
      methodologySchema,
    );

    expect(result).toEqual({ valid: true, errors: [] });
  });

  it("rejects EvidenceCard fixtures missing required evidence fields at schema level", () => {
    const contextPack = createToyContextPack("2026-06-05T00:00:00.000Z");
    const [evidenceCard] = contextPack.evidenceCards;

    for (const field of ["evidence", "source", "coverage", "confidence"] as const) {
      const invalidContextPack = {
        ...contextPack,
        evidenceCards: [{ ...evidenceCard, [field]: undefined }],
      };

      const result = validateAgainstSchema(invalidContextPack, methodologySchema);

      expect(result.valid, `${field} should be required`).toBe(false);
      expect(result.errors).toContain(`$.evidenceCards[0] missing required ${field}`);
    }
  });
});

interface SchemaValidationResult {
  valid: boolean;
  errors: string[];
}

type JsonSchemaNode = Record<string, unknown>;

function validateAgainstSchema(
  value: unknown,
  schema: JsonSchemaNode,
): SchemaValidationResult {
  const errors: string[] = [];
  validateNode(value, resolveRef(schema.$ref as string, schema), "$", schema, errors);

  return {
    valid: errors.length === 0,
    errors,
  };
}

function validateNode(
  value: unknown,
  schemaNode: JsonSchemaNode,
  path: string,
  rootSchema: JsonSchemaNode,
  errors: string[],
): void {
  if (schemaNode.$ref) {
    validateNode(value, resolveRef(schemaNode.$ref as string, rootSchema), path, rootSchema, errors);
    return;
  }

  if ("const" in schemaNode && value !== schemaNode.const) {
    errors.push(`${path} must equal ${String(schemaNode.const)}`);
  }

  if (Array.isArray(schemaNode.enum) && !schemaNode.enum.includes(value)) {
    errors.push(`${path} must be one of ${schemaNode.enum.join(", ")}`);
  }

  if (schemaNode.type === "string") {
    if (typeof value !== "string") {
      errors.push(`${path} must be string`);
      return;
    }
    if (typeof schemaNode.minLength === "number" && value.length < schemaNode.minLength) {
      errors.push(`${path} must have length at least ${schemaNode.minLength}`);
    }
    return;
  }

  if (schemaNode.type === "boolean") {
    if (typeof value !== "boolean") {
      errors.push(`${path} must be boolean`);
    }
    return;
  }

  if (schemaNode.type === "array") {
    if (!Array.isArray(value)) {
      errors.push(`${path} must be array`);
      return;
    }
    if (typeof schemaNode.minItems === "number" && value.length < schemaNode.minItems) {
      errors.push(`${path} must contain at least ${schemaNode.minItems} item(s)`);
    }
    const itemSchema = schemaNode.items as JsonSchemaNode | undefined;
    if (itemSchema) {
      value.forEach((item, index) => {
        validateNode(item, itemSchema, `${path}[${index}]`, rootSchema, errors);
      });
    }
    return;
  }

  if (schemaNode.type === "object") {
    if (!isRecord(value)) {
      errors.push(`${path} must be object`);
      return;
    }

    const required = Array.isArray(schemaNode.required)
      ? (schemaNode.required as string[])
      : [];
    for (const field of required) {
      if (!(field in value) || value[field] === undefined) {
        errors.push(`${path} missing required ${field}`);
      }
    }

    const properties = (schemaNode.properties ?? {}) as Record<string, JsonSchemaNode>;
    if (schemaNode.additionalProperties === false) {
      for (const field of Object.keys(value)) {
        if (!(field in properties)) {
          errors.push(`${path} has additional property ${field}`);
        }
      }
    }

    for (const [field, propertySchema] of Object.entries(properties)) {
      if (field in value && value[field] !== undefined) {
        validateNode(value[field], propertySchema, `${path}.${field}`, rootSchema, errors);
      }
    }
  }
}

function resolveRef(ref: string, rootSchema: JsonSchemaNode): JsonSchemaNode {
  const pathParts = ref.replace(/^#\//, "").split("/");
  let current: unknown = rootSchema;

  for (const pathPart of pathParts) {
    if (!isRecord(current)) {
      throw new Error(`Cannot resolve JSON schema ref ${ref}`);
    }
    current = current[pathPart];
  }

  if (!isRecord(current)) {
    throw new Error(`JSON schema ref ${ref} did not resolve to an object`);
  }

  return current;
}

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === "object" && value !== null && !Array.isArray(value);
}
