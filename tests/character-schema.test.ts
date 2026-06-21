import Ajv2020 from "ajv/dist/2020";
import addFormats from "ajv-formats";
import { describe, expect, it } from "vitest";
import characterSchema from "../schemas/character.schema.json";
import commonSchema from "../schemas/common.schema.json";
import { createToyCharacterProfile } from "../src/domain/character";

const ajv = new Ajv2020({ allErrors: true, strict: true });
addFormats(ajv);
ajv.addSchema(commonSchema);
const validateProfile = ajv.compile(characterSchema);

describe("character schema", () => {
  it("publishes character and relationship contracts", () => {
    for (const name of [
      "CharacterStableProfile",
      "CharacterDynamicState",
      "CharacterArc",
      "CharacterTimeline",
      "RelationshipEdge",
      "RelationshipTemperature",
      "FocusRelationshipView",
      "InteractionPattern",
    ]) {
      expect(characterSchema.$defs).toHaveProperty(name);
    }
  });

  it("validates a public-safe character stable profile fixture", () => {
    const valid = validateProfile(createToyCharacterProfile("2026-06-05T00:00:00.000Z"));

    expect(validateProfile.errors).toBeNull();
    expect(valid).toBe(true);
  });
});
