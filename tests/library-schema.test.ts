import { describe, expect, it } from "vitest";
import librarySchema from "../schemas/library.schema.json";

describe("library JSON schema", () => {
  it("publishes the 9.1 public library record contracts", () => {
    expect(librarySchema.$defs).toHaveProperty("LibraryRoot");
    expect(librarySchema.$defs).toHaveProperty("LibraryManifest");
    expect(librarySchema.$defs).toHaveProperty("SourceRecord");
    expect(librarySchema.$defs).toHaveProperty("WorkRecord");
    expect(librarySchema.$defs).toHaveProperty("AnalysisRecord");
  });

  it("uses the library manifest as the root schema", () => {
    expect(librarySchema.$ref).toBe("#/$defs/LibraryManifest");
  });
});
