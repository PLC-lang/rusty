# Incremental compilation in `rusty` — implementation plan

This is the working tracking document for adding incremental compilation
support to the rusty PLC compiler. It is a living document: at the end of
every phase the implementer appends a **Status** subsection to that phase
capturing what landed, deviations from the original plan, gotchas, and the
open items carried into the next phase. A fresh context should be able to
read this doc and resume work without re-deriving state.

## Near-term headline win

**Lowering passes today trigger redundant full-project re-index and
re-annotate calls on every build, even when nothing changed.** The first
deliverable of this work is the elimination of those redundant passes. The
result is a measurable speedup on every CLI build with no LSP needed —
that is the headline win Phases 0–3 deliver, and it is independent of any
language-server effort.

The same machinery (delta-driven invalidation, per-unit Index provenance,
reverse-dep graph) is also the foundation for incremental rebuilds, which
will eventually power an LSP. The LSP shell itself is out of scope here:
this plan stops at the in-process incremental driver. LSP framework
selection (tower-lsp vs lsp-server vs other) is a separate, later
decision.

---

## Non-negotiables

These constraints govern every phase and every PR. They are stricter than
"don't break tests" because an incremental path that diverges from the
non-incremental path is worse than no incremental compilation at all.

1. **Bit-identical outputs.** Building a project with incremental mode on
   from a clean state must produce byte-identical `Index`, `AnnotationMap`,
   and `.o` outputs compared to the legacy non-incremental path. Verified
   via snapshot tests and an `objdump`/`cmp` regression harness on a fixed
   corpus.
2. **Identical test behaviour.** All existing `cargo test --workspace`
   suites — correctness, validation, and lit (`cargo xtask lit`) — pass
   identically whether incremental mode is on or off. CI runs both. A test
   that passes only in one mode is a bug, not a feature flag.
3. **No diagnostic drift.** The set of diagnostics, their codes, and their
   ordering match between modes.
4. **Default off until proven safe.** Incremental mode lives behind a flag
   until corpus + CI parity is demonstrated.

---

## Current pipeline

`BuildPipeline` (`compiler/plc_driver/src/pipelines.rs:63`) runs the
following stages:

1. **Parse** — one `CompilationUnit` per source file
   (`compiler/plc_ast/src/ast.rs:482`). Already per-unit; no shared state.
2. **Index** — `ParsedProject::index` (`pipelines.rs:688`). Per-unit
   indices are built in parallel via `indexer::index(&unit)`
   (`src/index/indexer.rs:22`), then merged sequentially into a single
   global `Index` via `Index::import` (`src/index.rs:1338`). Provenance
   (which unit produced which symbol) is dropped on merge.
3. **Annotate** — `IndexedProject::annotate` (`pipelines.rs:738`).
   `TypeAnnotator::visit_unit` runs in parallel; annotation maps are
   merged into one `AnnotationMapImpl`; the resolver's `new_index`
   (generic instantiations, synthetic types) is folded back into the
   project `Index`.
4. **Lower** — multiple `PipelineParticipantMut` passes
   (`compiler/plc_driver/src/pipelines/participant.rs`). Some are per-unit
   and pure; others rewrite globally and trigger a whole-project re-index
   or re-annotate (see "Obstacle: lowering re-passes" below).
5. **Codegen** — two modes (`pipelines.rs:506`): a `--single-module` path
   that merges all units into one LLVM module, and the default
   `par_iter()` path that emits one LLVM module + one `.o` per unit.
   The default path is already structurally per-unit.
6. **Link** — `src/linker.rs:37`. External linker invocation. Already
   designed to consume multiple `.o` inputs.

Summary: parse, codegen, and link are already per-unit. The
"everything-or-nothing" assumption lives in index + annotate, plus a few
non-local lowering passes.

---

## Obstacles to per-unit invalidation

### `AstId` is process-global and order-dependent

`AstId = usize` is allocated from a shared `AtomicUsize` via `IdProvider`
(`compiler/plc_ast/src/provider.rs:11`). Annotation side-tables in
`AnnotationMapImpl` (`src/resolver.rs:1166,1172`) key on `AstId`. Re-parsing
any unit shifts the IDs and orphans every cached annotation, even for
unchanged units. Annotation caching across edits requires either
content-stable IDs or per-unit-scoped IDs.

### `Index` merge drops unit provenance

`Index::import` (`src/index.rs:1338`) drains the per-unit `Index` into
flat global maps. There is no `unit_id` on `VariableIndexEntry`
(`src/index.rs:65`), `PouIndexEntry`, implementation, or type entries; no
reverse map from unit → symbols. Removing one unit's contributions
requires rebuilding the whole index.

### `ConstId` / `ConstExpressions` are session-global and re-keyed on rebuild

Constants live in `ConstExpressions` indexed by sequential `ConstId`s.
`transfer_constants` (`src/index.rs:1351`) re-keys constant references
during import. Any cached entry embedding a `ConstId` (initial values,
array dimensions, string sizes, subrange bounds) becomes stale on
re-index.

### Generic instantiations are recorded into the global index implicitly

When unit A calls `FB_Generic<INT>` and unit B calls `FB_Generic<REAL>`,
both instantiations land in `AnnotationMapImpl::new_index`
(`src/resolver.rs:1189`) and merge into the project `Index`. There is no
explicit "instantiation X was requested by unit Y" record. Recompiling A
alone cannot know whether `<REAL>` exists unless we make ownership
explicit or always re-run B.

### Lowering participants trigger full re-index and full re-annotate

The `PipelineParticipantMut` trait
(`compiler/plc_driver/src/pipelines/participant.rs:71`) lets each lowerer
mutate units, then re-run the upstream phase against the entire project
because the existing API has no smaller knob:

| Participant | Hook | What it re-runs |
|---|---|---|
| `PolymorphismLowerer` | `post_index` (`participant.rs:326`) | full re-index |
| `PolymorphismLowerer` | `post_annotate` (`participant.rs:334`) | full re-index + full re-annotate |
| `AggregateTypeLowerer` | `post_annotate` (`participant.rs:302`) | full re-index + full re-annotate |
| `InheritanceLowerer` | `post_annotate` (`participant.rs:282`) | full re-annotate |
| `RetainParticipant` | `post_index` (`participant.rs:355`) | full re-index |
| `ArrayLowerer` | `pre_annotate` (`participant.rs:263`) | full re-index |

A single full build today does roughly three or four redundant
whole-project re-indexes and two or three redundant whole-project
re-annotations. Any incremental story has to start by replacing this with
a delta API. Fixing it pays for itself in full-build wall-clock time
before any LSP code ships.

### Other lowering issues

- **Init-function synthesis.** Adding or removing a global variable
  changes the unit-init POU body and the set of constructor calls. The
  pass is cross-unit by nature; it needs explicit dependency tracking.
- **Per-unit constructor naming** (`src/lowering/helper.rs:376`,
  introduced in commit `be1de6f175`): names encode a hash of the source
  file's path. Different checkout paths produce different mangled names,
  so any on-disk cache must include the path in the key — or constructor
  naming has to move to a path-independent scheme.

### Const-folding is iterative without an explicit dependency graph

`evaluate_constants` (`src/resolver/const_evaluator.rs:74`) loops to a
fixed point. No per-constant dependency edges, so any change requires
re-running the whole loop and re-validating every unit that references
any constant.

### Global validation requires the merged index

`Validator::perform_global_validation` (`pipelines.rs:818`,
`src/validation/global.rs:66`) and `RecursiveValidator` are inherently
project-wide. Per-unit validation is already split out
(`pipelines.rs:823`), so only the global pass needs special handling.

### No content hashing / fingerprinting infrastructure

No `hash`, `fingerprint`, `mtime`, build-manifest, or `.plc-cache`
directory in the codebase. `Project`
(`compiler/plc_project/src/project.rs:67`) and `Object`
(`compiler/plc_project/src/object.rs:6`) hold metadata but do nothing
with it.

### No LSP code

Greenfield. No `tower-lsp` or `lsp-server` dependency, no language-server
crate, no editor extension.

---

## Existing infrastructure to reuse

| Component | Where | Why it helps |
|---|---|---|
| Per-unit parse | `pipelines.rs:632` | Already isolated per file. |
| Per-unit codegen (multi-module) | `pipelines.rs:515`, `par_iter` | One LLVM module + `.o` per unit. |
| Per-unit `Dependency` records | `src/resolver.rs:1002`, collected in `AnnotatedUnit` (`pipelines.rs:771`), consumed by codegen (`src/codegen.rs:160`) | By-**name** edges (Datatype/Call/Variable), stable across rebuilds. Inverting them gives the reverse-dependency graph. |
| Per-unit validation pass | `pipelines.rs:823` | Already separate from global validation. |
| Rayon | `plc_driver/Cargo.toml:26` | Parallel infra already in place. |
| Header generator | `compiler/plc_header_generator/` | Exposes per-unit interfaces. |
| `Project` / `LibraryInformation` / `ProjectConfig` | `compiler/plc_project/` | Build-manifest plumbing already exists. |
| `siphasher` | workspace dep | Available for content + signature hashing. |

---

## Approach: delta-driven invalidation

Keep the current pipeline shape. Build it around one central mechanism — a
**dirty-set delta** that flows through every phase — and reuse the same
mechanism for two purposes:

1. Inside the participant chain, so today's full builds stop doing N
   redundant re-passes.
2. For incremental rebuilds (the LSP and cached-CLI cases).

```rust
struct PipelineDelta {
    mutated_units: FxHashSet<UnitId>,
    added_symbols: FxHashSet<SymbolKey>,      // POUs / globals / types created
    removed_symbols: FxHashSet<SymbolKey>,    // anything dropped
    changed_signatures: FxHashSet<SymbolKey>, // public signature changed
}
```

The pipeline (`pipelines.rs:455`) consumes a delta:

1. Call `Index::remove_unit(u)` for each `u ∈ delta.mutated_units`.
2. Re-run `indexer::index(&unit)` for those units only, merge back.
3. Compute the reverse-dependency closure of `delta.changed_signatures`.
4. Re-annotate `mutated_units ∪ closure`, not the whole project.
5. Leave everything else untouched.

Cross-unit lowerers (vtables, init-functions) stay cross-unit; they just
have to declare the units and symbols they actually touched.

A query-driven (Salsa-style) rewrite was considered. It produces a cleaner
architecture and finer-grained invalidation, but it is a multi-quarter
rewrite of the resolver and index, and most of the prerequisite work
(content-hashed inputs, `Index` provenance) is shared with this approach.
The recommendation is to do the pragmatic approach first; revisit Salsa
once a working LSP exists and sub-unit query granularity becomes the
limiting factor.

### Cache scope

Build the **in-process cache first** (Phases 1–5) — gives the LSP latency
win at minimal complexity. Add an **on-disk layer** (Phase 6) once the
in-process design is proven; the structures already derive
`Serialize`/`Deserialize`, so the additional work is largely mechanical,
but the cache key must include compiler version + target + relevant
codegen flags + source path (while constructor names hash the path).

---

## Phased roadmap

Each phase is independently shippable behind a feature flag. CI runs both
modes; outputs must match bit-identically on the corpus.

Ordering note: the **delta-aware participant work (Phases 1–3) lands first**
and pays for itself in full-build wall-clock time before any LSP work
starts. The LSP-specific glue is Phase 5 onwards.

### Phase 0 — Instrumentation & baseline

Goal: measure where time goes today, especially the redundant re-passes.

- Add per-phase timing to `BuildPipeline` (parse / index / annotate /
  lower / codegen / link). Time each participant invocation and each
  implicit re-index/re-annotate it triggers, separately.
- Capture baselines on a small project and a large one.
- No behaviour change. Output: numbers that justify phase ordering and
  give us a target to beat at the end of Phase 3.

### Phase 1 — Content hashing, per-unit Index retention, unit provenance

- Add `SourceHash` to `SourceCode`.
- Keep per-unit `Index` results in `IndexedProject` alongside the merged
  index.
- Add `unit_id: UnitId` to `VariableIndexEntry` (`src/index.rs:65`),
  `PouIndexEntry`, implementation entries, and type entries. Preserve
  through `Index::import` (`src/index.rs:1338`).
- Add `Index::remove_unit(unit_id: UnitId)` that drops every entry tagged
  with that unit, including from `enum_global_variables`,
  `global_initializers`, and the type index.

No invalidation logic yet; just enough plumbing to know what came from
where and to take it out again.

### Phase 1 — Status

_Complete._

**Landed**

- `SourceCode::content_hash()` (`compiler/plc_source/src/lib.rs`): returns
  a `SourceHash` (siphash13 of the source bytes). Computed on demand;
  callers cache if they need stability.
- `UnitId` newtype + sentinels (`BUILTIN`, `SYNTHETIC`, `UNTAGGED`) in
  `src/index/unit_id.rs`, plus `SymbolKind`, `OwnedSymbol`, and a
  `UnitSymbolIndex` side-table.
- `Index` grows a `#[serde(skip)] unit_symbols: UnitSymbolIndex` field
  (skipped from serde so existing snapshots stay byte-identical).
- New `Index::import_with_unit(other, unit_id)` records every imported
  entry's `(kind, map_key, identifier)` triple in `unit_symbols`. The old
  `Index::import` is now a thin wrapper that passes `UnitId::UNTAGGED`,
  so existing callers keep working.
- New `Index::remove_unit(unit_id)` walks `unit_symbols[unit_id]` and
  drops only the contributions for that unit, even when the underlying
  `SymbolMap` key is shared with another unit. `SymbolMap::retain_at_key`
  added in `src/index/symbol.rs` to support precise per-entry removal.
- `pipelines.rs` threads UnitIds at the three merge sites:
  source-unit indices use `UnitId::source(idx)`, the built-in functions
  use `UnitId::BUILTIN`, and the resolver's `new_index` (generic
  instantiations + synthetic types) uses `UnitId::SYNTHETIC`.
- `test_utils::tests::do_index` and `index_unit_with_id` also use
  `import_with_unit`, so `Index::remove_unit(UnitId::source(0))` in tests
  drops exactly the test fixture's contributions.
- Three focused tests in `src/index/tests/unit_id_tests.rs` cover the
  global / POU / shared-enum-key cases; unit tests in `unit_id.rs` cover
  the `UnitId` constructors and sentinel disjointness.

**Deviations from the original plan**

- `IndexedProject` does **not** retain `unit_indices: Vec<Index>`.
  Walking through the Phase 3 / Phase 5 flow showed that an incremental
  rebuild calls `remove_unit(uid)` + `import_with_unit(fresh_per_unit_index,
  uid)` and never references the old per-unit index — retaining it would
  just duplicate memory. If a Phase 5 use case turns up that genuinely
  needs the old per-unit index, the field can be added back; `Index` is
  not yet `Clone`, so adding it later requires a derive too.
- `unit_id` is **not** stamped onto each entry struct (`VariableIndexEntry`,
  `PouIndexEntry`, etc.). Per-entry ownership is tracked via the
  side-table only, which keeps the diff localized and avoids snapshot
  churn. The side-table records `(map_key, identifier)` per entry, so
  the precision required for Phase 3 is already there.

**Gotchas / for the next implementer**

- `UnitId::source(0)` is what tests use after going through the test
  helpers. If a test sets up its own `Index` manually, it must pick a
  `UnitId` or the entries land under `UnitId::UNTAGGED` and
  `remove_unit(source(0))` won't see them.
- `Index::register_type` / `register_pou` / `register_pou_type` (used
  for built-in primitives) do **not** record into `unit_symbols`. That's
  intentional — primitives never need to be invalidated. If Phase 3
  participants call `register_*` directly to add new entries, they need
  to either route through `import_with_unit` or record into
  `unit_symbols` manually.
- `labels` and `config_variables` are not yet covered by the side-table;
  they don't surface in any cross-unit invalidation scenarios in Phases
  1–3. Revisit if a real use case appears.

**Carries into Phase 2**

- The `unit_symbols` side-table is the natural place to extend with
  "this symbol's `PublicSignatureHash`" once Phase 2 introduces it.
- The reverse-dep graph (Phase 2) will key on the same symbol names
  used here as `identifier`, so the cross-phase identifier convention is
  already locked in.

### Phase 2 — Audit/extend the dependency tracker; reverse-dep graph; signature hashing

The resolver already produces a per-unit `FxIndexSet<Dependency>`
(`src/resolver.rs:1287`, returned at `1382`), and codegen consumes it
(`src/codegen.rs:160`). Phase 2 makes that set exhaustive enough to drive
invalidation, then inverts it.

- **Audit.** Read every insertion into `self.dependencies` in
  `src/resolver.rs` and every path in `get_datatype_dependencies`
  (`src/resolver.rs:1934`). Produce a coverage matrix: for each
  `Dependency` variant, list the AST shapes that should produce one and
  confirm they do. Drive with red tests where coverage is missing.
- **Extend for known gaps.**
  - Generic instantiations (`src/resolver/generics.rs:90`): record a
    `Dependency::Call(<instantiated-name>)` on the requesting unit, not
    just on the generic.
  - Const-expression edges: a unit that uses a `VAR_GLOBAL CONSTANT` as
    an array size or string length depends on the value, not just the
    variable. Capture the value-dependency.
  - Lowering-introduced dependencies (init-constructor calls, vtable
    references, dispatch-table reads) are added by each lowerer's delta
    in Phase 3, not by the resolver.
  - Document the builtins case (`pipelines.rs:715`); builtins are stable
    so a missing dep there doesn't break incrementality.
- **Public-signature hash.** Compute a `PublicSignatureHash` per POU /
  global / type at the end of indexing: hash only externally observable
  shape (POU signature including parameters and return type, variable
  type, struct layout, type-information variant). Internal POU bodies are
  not included. Use `siphasher`.
- **Reverse map.** At the end of `annotate()`, invert each
  `AnnotatedUnit.dependencies` into `symbol_name → Set<unit_id>`. Expose
  on `AnnotatedProject`.
- **Tests.** Unit test: "unit X depends on POU foo" ⇒ reverse-map for
  `foo` contains X. Coverage test: every `Dependency` insertion site is
  exercised by at least one test. Snapshot test: dep-set for a
  representative project is stable across runs.

### Phase 3 — Delta-aware participant pipeline

This phase delivers a full-build speedup before any LSP work.

- Introduce `PipelineDelta` as described above.
- Change `PipelineParticipantMut`'s `pre_index` / `post_index` /
  `pre_annotate` / `post_annotate` (`participant.rs:71`) to return a
  delta in addition to (or instead of) the project. Migrate the six
  offenders (`participant.rs:263–363`):
  - `PolymorphismLowerer` (`post_index` and `post_annotate`)
  - `AggregateTypeLowerer` (`post_annotate`)
  - `InheritanceLowerer` (`post_annotate`)
  - `RetainParticipant` (`post_index`)
  - `ArrayLowerer` (`pre_annotate`)
- In `BuildPipeline::index` and `BuildPipeline::annotate`
  (`pipelines.rs:455`), consume the delta:
  1. `Index::remove_unit(u)` for each `u ∈ delta.mutated_units`.
  2. Re-run `indexer::index(&unit)` for those units only, merge back.
  3. Compute the reverse-dep closure of `delta.changed_signatures`.
  4. Re-annotate `delta.mutated_units ∪ closure`.
- **Verification gate.** Assert that the resulting `Index` and
  `AnnotationMap` are byte-identical to the old whole-project re-pass
  output on the corpus before flipping the default on. Snapshot diffs and
  binary `.o` comparison.

### Phase 4 — Cross-unit lowerer hygiene

Tighten the genuinely cross-unit passes so their deltas are honest and
minimal.

- **Polymorphism / vtable emission** (`src/lowering/polymorphism/`):
  emit a delta whose `mutated_units` only includes units that actually
  reference the changed vtable, not "all units" defensively.
- **Init-function synthesis** (`src/lowering/helper.rs`): isolate the
  global init-sequencing artefact and report a delta scoped to units
  whose unit-init body actually changed.
- **Generic instantiation registration** (`src/resolver/generics.rs:90`):
  make ownership explicit. Each instantiation is emitted from a single
  deterministic unit (first-declarer or alphabetically-first — to be
  decided in this phase), so adding an instantiation in unit X doesn't
  invalidate an existing instantiation owned by unit Y.

### Phase 5 — In-process incremental driver

Plug "this-file-just-changed" into the delta-aware pipeline.

- New struct `IncrementalDriver` owning parsed / indexed / annotated
  state. Public surface: `on_change(path, new_contents) → Diagnostics`.
- Implementation:
  1. Re-parse the changed unit; compute new content hash and per-unit
     `Index`.
  2. Compute `PipelineDelta` by comparing new vs cached
     `PublicSignatureHash`es.
  3. Drive the same delta-aware pipeline added in Phase 3.
  4. Re-codegen the affected closure only; reuse cached `.o` for the
     rest.
  5. Re-run global validation (single scan); per-unit validation on the
     rebuilt set only.
- Expose via a `--incremental` CLI flag for testing. Not the default.

### Phase 6 — On-disk cache (optional)

- Serialise per-unit cache entries (`AnnotatedUnit`, per-unit `Index`,
  `.o` path) keyed by a hash of `(content_hash, compiler_version,
  target_triple, relevant_codegen_flags, source_path)`. The
  source-path component is required while constructor names hash the path.
- Layout: `target/plc-cache/<key>/{unit.json, unit.o, meta.json}`.
- GC: simple LRU or "delete if older than N days." Surface via an xtask
  command.
- Lit + cargo tests: build → touch one file → rebuild → assert cache
  hits for the unchanged units.

### Out of scope — LSP shell

The LSP server itself is a separate effort downstream of this plan. The
in-process incremental driver from Phase 5 is the handoff surface; the
LSP shell wraps it. Framework selection (`tower-lsp` vs `lsp-server` vs
other) is explicitly deferred — no decision is made or implied by this
document.

---

## Verification

Run at every phase:

1. **Correctness baseline.** Every existing `cargo test --workspace` and
   `cargo xtask lit` case stays green under both modes.
2. **Delta-equivalence (gate for Phase 3).** With the delta-aware
   pipeline on, build a corpus of representative projects and assert
   that the resulting `Index`, `AnnotationMap`, and per-unit `.o` files
   are byte-identical to the legacy whole-project re-pass output.
3. **Cache-hit test (Phase 5).** Build a 3-unit project; touch a comment
   inside one POU; rebuild; assert the other two units' cache entries
   were reused. Implement as a lit test under `tests/lit/incremental/`.
4. **Signature-change invalidation test (Phase 5).** Change a POU
   signature; assert the consumer closure is recomputed and unrelated
   units are reused.
5. **Cross-project regression.** Build oscat (or the in-repo equivalent)
   with and without incremental; binary-diff resulting object files.
   Identical for an unchanged build.
6. **Wall-clock re-measure.** Re-measure Phase 0 baselines at the end of
   Phases 3, 5, and 6. Phase 3 alone should show a meaningful full-build
   speedup with no LSP needed.

Standard hygiene per phase: `cargo fmt --all` clean,
`cargo clippy --workspace -- -Dwarnings` clean before a phase is called
done; inline snapshots preferred over external `.snap` files;
diagnostics-only changes go to validator unit tests rather than lit.

---

## Open design questions

These are real decisions to settle as the relevant phase opens; none are
blockers right now.

- **Generic instantiation ownership rule.** Alphabetical-first unit,
  declaration-order-first, or always-emit-in-the-defining-unit? Decided
  in Phase 4.
- **Constructor naming and the on-disk cache key.** Either keep the
  source path in the cache key, or move constructor naming to a
  path-independent scheme. Decided before Phase 6 starts.

---

## Status log

A subsection per phase is appended here as work lands. The intent is that
a fresh context can read this and resume without re-deriving state.

### Phase 0 — Status

_Complete._

**Landed**

- `compiler/plc_driver/src/pipelines/timing.rs`: a small RAII timer
  module. Enabled by `PLC_INCR_TIMING=1` in the environment; silent
  otherwise. Indented stderr output reflects call nesting so participant
  re-passes show up as children of the participant that triggered them.
- `name()` method on both `PipelineParticipant` and
  `PipelineParticipantMut` (in
  `compiler/plc_driver/src/pipelines/participant.rs`) with a default that
  returns the implementing type's short name (generics stripped).
  Used as the timer label for each participant invocation.
- `BuildPipeline::parse/index/annotate/generate`,
  `ParsedProject::index`, and `IndexedProject::annotate` are wrapped in
  timers. Wrapping the inner two methods is what catches the implicit
  re-passes inside participants — they show up nested under the
  participant call that triggered them.
- Baseline numbers captured in
  `docs/baselines/phase0_oscat_baseline.md`. The single-file oscat
  `--check` takes ~615–605 ms in the annotate phase alone, of which
  ~470 ms is **four redundant whole-project re-annotates** triggered by
  participant `post_annotate` hooks. That ~470 ms is the headline win
  Phase 3 targets.

**Gotchas / for the next implementer**

- The timer is keyed on `PLC_INCR_TIMING=1`. CI does not set it; nothing
  ships in default output. If you add new timed scopes, prefer wrapping
  the *callee* (the inner method) rather than the call site — that way a
  re-entrant call from a participant gets timed without duplication.
- Timer lines print on `Drop`, so children appear *before* their parent's
  log line in the trace (standard flame-graph convention). Read the
  indent to see parent-child structure.
- The `name()` default uses `std::any::type_name::<Self>()`. It works on
  default trait methods because the body is monomorphized at each impl
  site. If a participant ever returns a confusing label (e.g. with a
  surviving generic suffix), override `name()` on that impl.
- `.baseline/` is now in `.gitignore`; oscat lives there locally and is
  not committed.

**Carries into Phase 1**

- The timer output for a clean full build is the reference. After Phase 3
  the equivalent oscat baseline must show the redundant re-pass lines
  either gone or shrunk to per-unit cost.
- No code changes from Phase 0 are load-bearing for Phase 1; the
  instrumentation is purely diagnostic. Phase 1 starts from a clean
  build.
