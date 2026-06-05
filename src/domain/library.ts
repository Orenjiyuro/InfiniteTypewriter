export type IsoDateTime = string;

export interface LibraryRoot {
  id: string;
  label: string;
  path: string;
}

export interface SourceRecord {
  id: string;
  kind: "source";
  title: string;
  contentHash: string;
  createdAt: IsoDateTime;
  updatedAt: IsoDateTime;
}

export interface WorkRecord {
  id: string;
  kind: "work";
  title: string;
  createdAt: IsoDateTime;
  updatedAt: IsoDateTime;
}

export interface AnalysisRecord {
  id: string;
  kind: "analysis";
  sourceId: string;
  title: string;
  createdAt: IsoDateTime;
  updatedAt: IsoDateTime;
}

export interface LibraryManifest {
  schemaVersion: 1;
  root: LibraryRoot;
  createdAt: IsoDateTime;
  updatedAt: IsoDateTime;
  sources: SourceRecord[];
  works: WorkRecord[];
  analyses: AnalysisRecord[];
}

export function createEmptyLibraryManifest(
  root: LibraryRoot,
  timestamp: IsoDateTime = new Date().toISOString(),
): LibraryManifest {
  return {
    schemaVersion: 1,
    root,
    createdAt: timestamp,
    updatedAt: timestamp,
    sources: [],
    works: [],
    analyses: [],
  };
}
