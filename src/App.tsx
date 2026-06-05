import { createEmptyLibraryManifest, type LibraryRoot } from "./domain/library";
import "./styles.css";

const demoRoot: LibraryRoot = {
  id: "local-demo",
  label: "Local Library",
  path: "Choose a local folder to initialize",
};

const demoManifest = createEmptyLibraryManifest(demoRoot, "2026-06-05T00:00:00.000Z");

export function App() {
  return (
    <main className="workspace-shell">
      <section className="workspace-panel" aria-labelledby="library-title">
        <p className="eyebrow">Architecture Scaffold</p>
        <h1 id="library-title">InfiniteTypewriter</h1>
        <dl className="manifest-summary">
          <div>
            <dt>Schema</dt>
            <dd>{demoManifest.schemaVersion}</dd>
          </div>
          <div>
            <dt>Sources</dt>
            <dd>{demoManifest.sources.length}</dd>
          </div>
          <div>
            <dt>Works</dt>
            <dd>{demoManifest.works.length}</dd>
          </div>
          <div>
            <dt>Analyses</dt>
            <dd>{demoManifest.analyses.length}</dd>
          </div>
        </dl>
      </section>
    </main>
  );
}
