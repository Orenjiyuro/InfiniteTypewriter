import Ajv2020 from "ajv/dist/2020";
import addFormats from "ajv-formats";
import { describe, expect, it } from "vitest";
import commonSchema from "../schemas/common.schema.json";

const ajv = new Ajv2020({ allErrors: true, strict: true });
addFormats(ajv);
const validateStableId = ajv.compile(commonSchema.$defs.StableId);

describe("common domain schema", () => {
  it("publishes 9.6 common contract primitives", () => {
    for (const definitionName of [
      "BaseObject",
      "AuditTrailEntry",
      "ContextHints",
      "StableId",
    ]) {
      expect(commonSchema.$defs).toHaveProperty(definitionName);
    }
  });

  it("rejects unstable id shapes", () => {
    expect(validateStableId("valid-id-1")).toBe(true);

    for (const id of ["Invalid", "has space", "../path", ""]) {
      expect(validateStableId(id), `${id} should be rejected`).toBe(false);
    }
  });
});
