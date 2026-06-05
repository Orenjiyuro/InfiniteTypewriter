import { describe, expect, it } from "vitest";
import {
  createEmptyLibraryManifest,
  type AnalysisRecord,
  type LibraryRoot,
  type SourceRecord,
  type WorkRecord,
} from "../src/domain/library";

describe("library manifest contract", () => {
  it("creates an empty manifest for a selected local root", () => {
    const root: LibraryRoot = {
      id: "root-local-demo",
      label: "Demo Library",
      path: "C:/Users/demo/InfiniteTypewriter",
    };

    const manifest = createEmptyLibraryManifest(root, "2026-06-05T00:00:00.000Z");

    expect(manifest).toEqual({
      schemaVersion: 1,
      root,
      createdAt: "2026-06-05T00:00:00.000Z",
      updatedAt: "2026-06-05T00:00:00.000Z",
      sources: [],
      works: [],
      analyses: [],
    });
  });

  it("keeps source, work, and analysis records separated by kind", () => {
    const source: SourceRecord = {
      id: "source-toy",
      kind: "source",
      title: "Toy Source",
      contentHash: "sha256:source",
      createdAt: "2026-06-05T00:00:00.000Z",
      updatedAt: "2026-06-05T00:00:00.000Z",
    };
    const work: WorkRecord = {
      id: "work-toy",
      kind: "work",
      title: "Toy Work",
      createdAt: "2026-06-05T00:00:00.000Z",
      updatedAt: "2026-06-05T00:00:00.000Z",
    };
    const analysis: AnalysisRecord = {
      id: "analysis-toy",
      kind: "analysis",
      sourceId: "source-toy",
      title: "Toy Breakdown",
      createdAt: "2026-06-05T00:00:00.000Z",
      updatedAt: "2026-06-05T00:00:00.000Z",
    };

    const manifest = createEmptyLibraryManifest(
      {
        id: "root-local-demo",
        label: "Demo Library",
        path: "C:/Users/demo/InfiniteTypewriter",
      },
      "2026-06-05T00:00:00.000Z",
    );

    manifest.sources.push(source);
    manifest.works.push(work);
    manifest.analyses.push(analysis);

    expect(manifest.sources).toEqual([source]);
    expect(manifest.works).toEqual([work]);
    expect(manifest.analyses).toEqual([analysis]);
  });
});
