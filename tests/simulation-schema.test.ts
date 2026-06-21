import Ajv2020 from "ajv/dist/2020";
import addFormats from "ajv-formats";
import { describe, expect, it } from "vitest";
import commonSchema from "../schemas/common.schema.json";
import simulationSchema from "../schemas/simulation.schema.json";
import { createToySimulationContext } from "../src/domain/simulation";

const ajv = new Ajv2020({ allErrors: true, strict: true });
addFormats(ajv);
ajv.addSchema(commonSchema);
const validateSimulationContext = ajv.compile(simulationSchema);

describe("simulation schema", () => {
  it("publishes character simulation contracts", () => {
    for (const name of [
      "SimulationContext",
      "ActionCandidate",
      "PuppeteeringRisk",
      "SimulationResult",
      "OutlineAdjustmentSuggestion",
    ]) {
      expect(simulationSchema.$defs).toHaveProperty(name);
    }
  });

  it("validates a public-safe simulation context fixture", () => {
    const valid = validateSimulationContext(
      createToySimulationContext("2026-06-05T00:00:00.000Z"),
    );

    expect(validateSimulationContext.errors).toBeNull();
    expect(valid).toBe(true);
  });
});
