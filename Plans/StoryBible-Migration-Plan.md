# Story Bible Migration Plan — React Consolidation (Phase 2)

Generated: 2025-08-12 (follow-up to framework-mixing Phase 1)

Purpose
- Provide a prioritized, actionable task breakdown to migrate the Story Bible Svelte components to React/TypeScript.
- Keep migrations incremental and testable: migrate one component at a time, update imports, run quick smoke checks, then proceed.
- Preserve feature parity and Zustand-based state integration.

Scope
- Directory: `src/features/story-bible/`
- Components (Svelte) currently present (scan shows these primary files):
  - StoryBible.svelte
  - components/BraindumpEditor.svelte
  - components/CharactersManager.svelte
  - components/OutlineManager.svelte
  - components/ScenesManager.svelte
  - components/StyleExamplesManager.svelte
  - components/WorldBuildingManager.svelte
  - (plus smaller subcomponents referenced by the above)

Prioritization Criteria
1. Component criticality (core UX & feature surface)
2. Dependency surface (components that import many other Svelte components are higher risk)
3. Reusability / parent components (migrating parents early can simplify children)
4. Complexity (prefer easier ones first to build repeatable pattern)
5. Test coverage & usage frequency

Prioritized Task List (recommended order)
1. CharactersManager.svelte — High priority
   - Why: central to Story Bible; used broadly for character creation, often referenced elsewhere.
   - Estimated effort: 6–10h
2. WorldBuildingManager.svelte — High priority
   - Why: counterpart to characters; key world elements management.
   - Estimated effort: 6–10h
3. BraindumpEditor.svelte — Medium priority
   - Why: smaller editor-like UI; pattern for converting inputs and bindings.
   - Estimated effort: 3–6h
4. ScenesManager.svelte — Medium priority
   - Why: scene-centric operations that may reference characters/world items.
   - Estimated effort: 4–8h
5. OutlineManager.svelte — Medium priority
   - Why: outline flows and hierarchical structure; moderate complexity.
   - Estimated effort: 4–8h
6. StyleExamplesManager.svelte — Lower priority (but important)
   - Why: interacts with AdvancedAI/StyleManager patterns; reuse patterns from migrated templates.
   - Estimated effort: 3–6h
7. StoryBible.svelte (root) — Migrate after children
   - Why: root aggregates many child components — migrate last to simplify scope.
   - Estimated effort: 4–8h
8. Smaller subcomponents (Checkboxes, Inputs, Modals in that folder)
   - Consolidate by reusing centralized React UI components already present under `src/components/ui/` or `src/lib/components/*`.

Per-component migration checklist (repeatable)
- Create `<Component>.tsx` under same path (e.g., `src/features/story-bible/components/CharactersManager.tsx`).
- Convert Svelte script block logic to React:
  - Props => typed component props
  - Reactive statements => useEffect/useMemo/useState
  - Stores => hook into Zustand stores (e.g., useStoryBibleStore or useSeriesConsistencyStore patterns)
  - Events => callbacks (prop functions) or use context/store
- Convert markup to JSX/TSX using existing React UI components (prefer `src/components/ui` and other React components)
- Replace Svelte-specific syntaxes (bind:, on:, {#if}, {#each}) with React equivalents (.map, conditional rendering)
- Integrate CSS/Tailwind classes — keep existing classes where possible
- Update imports (other components referencing this Svelte component) to point to the new `.tsx`
- Run local dev build and smoke test the component route
- Add one or two basic Vitest tests to validate render & key interactions (optional but recommended)
- When validated, remove the `.svelte` source for that component only after ensuring no references remain

Repo-wide update & cleanup (final stage)
- After all Story Bible components are migrated and validated:
  1. Repo-wide search for `.svelte` imports:
     - Replace to point to `.tsx` React components where appropriate
     - For components intentionally left as Svelte (if any), keep references
  2. Remove migrated `.svelte` files
  3. Remove Svelte-specific dependencies/config:
     - Check package.json for svelte/svelte plugin entries
     - Check vite.config.ts for Svelte plugin references (none current per Phase 1)
     - Remove any Svelte build config if present
  4. Run full build and run automated tests

Acceptance criteria (for Phase 2 completion)
- All Story Bible components migrated to React/TSX
- No remaining `.svelte` files imported by the React app
- Full app builds without Svelte plugin or Svelte-related deps
- Smoke-tested Story Bible flows (open story bible, add/edit characters, add world elements, open editors)
- Unit tests for key migrated components added (minimum smoke tests)

Risks and mitigations
- Risk: Large components with intertwined logic may regress functionality.
  - Mitigation: Convert smaller pieces first; keep Svelte files until React component validated; write smoke tests.
- Risk: Store mismatch between Svelte stores and Zustand.
  - Mitigation: Migrate store usages to Zustand hooks gradually; ensure persisted state compatibility.
- Risk: Missing UI parity (styling differences).
  - Mitigation: Reuse existing React UI components and Tailwind utilities; keep same class names where possible.

Immediate next actions (I will perform now in ACT MODE)
1. Convert the highest-priority component: `CharactersManager.svelte` → `CharactersManager.tsx`
   - I will read the Svelte source to extract logic and markup.
   - I will create `src/features/story-bible/components/CharactersManager.tsx` with TypeScript React conversion.
   - I will update imports where that component is used (e.g., in StoryBible.svelte / other components).
2. After creating the React component, run a quick lint/build (where feasible in this environment) and save files.
3. Report back with the created file and any remaining manual edits required.

If this plan looks good, confirm and I will start by reading `src/features/story-bible/components/CharactersManager.svelte` and begin the migration.
