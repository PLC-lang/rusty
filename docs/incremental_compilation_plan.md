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

### Phase 2 — Status

_Complete._

**Landed**

- `src/index/signature.rs`: a new module that hashes the public
  surface of POUs, globals, types, and implementations into a
  `SignatureHash(u64)` (siphash13). Internal bodies, source locations,
  and `ConstId`-dependent values (e.g. `String { size }`) are
  deliberately excluded so editing a function body doesn't shift the
  signature of its callers. Re-exported as `plc::index::SignatureHash`.
- `AnnotatedProject::reverse_dependencies()` and a
  `ReverseDependencyGraph` helper in
  `compiler/plc_driver/src/pipelines.rs`. The graph inverts each
  unit's `Dependency::{Call,Datatype,Variable}` set into
  `symbol_name → Set<UnitId>` using the same `UnitId::source(i)`
  convention Phase 1 established.
- `AnnotatedUnit::dependencies()` accessor so the graph builder (and
  later phases) don't need to reach into private fields.
- Audit document at `docs/baselines/dependency_coverage.md`: lists
  every site in `src/resolver.rs` that inserts into
  `self.dependencies`, plus the known gaps and a verdict on whether
  the existing tracker is sufficient for the headline near-term win.
- Tests: four `SignatureHash` tests in `src/index/signature.rs` cover
  stability across runs and detect return-type / struct-field /
  global-type changes. Three `tests/reverse_deps.rs` tests in
  `plc_driver` cover caller→callee, struct-user, and unrelated-unit
  isolation.

**Deviations from the original plan**

- The plan called for extending the existing dep tracker to cover
  generic instantiations and const-expression value-deps. The audit
  produced a clear verdict that those gaps don't block Phases 3–4
  (lowering re-pass elimination doesn't need them) but will block
  Phase 5 (source-file-edit incremental rebuilds). Extensions are
  deferred to a Phase 5 prerequisite step rather than landing now —
  changing the dep set has subtle downstream effects on codegen
  (which already consumes the set) and would risk the bit-identical
  invariant if rushed.
- The signature hash is computed on demand (free functions in
  `signature` module) rather than stored on `Index`. Storing was not
  necessary for Phase 2 and would have added serde churn. Phase 3 /
  Phase 5 can introduce a `SignatureCache` if profiling shows
  recomputation is hot; for now it's a single pass per query.

**Gotchas / for the next implementer**

- `hash_pou` / `hash_global` / `hash_type` use `LinkageType as u8` —
  this assumes `LinkageType` stays `#[repr]`-friendly and that variant
  additions append (don't reorder). If `LinkageType` reorders, hashes
  shift silently. Add a defensive snapshot test if this becomes a
  worry.
- The reverse-dep graph uses `dep.get_name().to_string()` as the key,
  preserving whatever case the resolver recorded. Lookups must use
  the same casing — don't `.to_lowercase()` at the lookup site.
- `String { size: _ }` is intentionally skipped in
  `hash_data_type_information`: `size` is a `ConstId` that can shift
  between const-evaluation runs even when the source spec didn't
  change. Symptom if you re-include it: hashes thrash across rebuilds
  for no actual interface change.

**Carries into Phase 3**

- `ReverseDependencyGraph::dependents(symbol)` is the closure-building
  primitive Phase 3 will call when a participant reports a changed
  signature.
- Signature hashing is in place but **not yet stored or compared** —
  Phase 3 will compute and compare hashes during the delta-aware
  pipeline.
- The dep-tracker gaps documented in
  `docs/baselines/dependency_coverage.md` are a TODO for Phase 5; they
  don't block Phase 3.

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

### Phase 3 — Status

_Complete._ Headline win delivered: oscat `annotate (driver)` dropped
**from 605 ms to 390 ms (-35 %)** with no behavioural changes. Full
numbers in `docs/baselines/phase3_oscat_after.md`.

**Landed**

Each lowering participant that previously triggered a whole-project
re-index or re-annotate is now gated. Some report "did I change
anything?" from the lowerer itself; the others use an exact-predicate
precheck against the `Index`. Both produce the same coarse "skip the
re-pass when nothing to do" win.

Migrated participants:

- `ArrayLowerer::pre_annotate` —
  `array_lowering::lower_literal_arrays` now returns `bool`. Re-index
  skipped when no array literal was lowered. oscat saves ~12 ms.
- `RetainParticipant::post_index` — `lower_retain` returns `bool`,
  driven by an internal `changed` flag on `RetainLowerer`. The lowerer
  now borrows the index instead of consuming it, so the caller can
  reuse the unchanged `IndexedProject` when nothing was rewritten.
  oscat saves ~6 ms.
- `PolymorphismLowerer::post_index` — `PolymorphismLowerer::table()`,
  `TableGenerator::generate`, `VirtualTableGenerator::generate`, and
  `InterfaceTableGenerator::generate` now return `bool` (true if any
  vtable/itable definition or instance was emitted). Re-index skipped
  on no-class / no-FB / no-interface projects.
- `PolymorphismLowerer::post_annotate` — gated on an
  `Index`-precheck (`project_uses_polymorphism`) that's exact for the
  condition it checks (no classes / FBs / interfaces ⇒ no dispatch
  sites to rewrite). Threading a `changed` flag through the dispatch
  visitors is invasive and deferred to Phase 4.
- `AggregateTypeLowerer::post_annotate` — gated on
  `project_has_aggregate_returns`. Exact predicate.
- `InheritanceLowerer::post_annotate` — gated on
  `project_uses_inheritance` (any POU with `super_class` or
  `interfaces`). Exact predicate. **oscat saves ~115 ms** here.
- `PropertyLowerer::post_annotate` — gated on a new
  `Index::has_any_properties()` accessor. **oscat saves ~113 ms** here.

Supporting changes:

- `SymbolMap::is_empty()` added.
- `Index::has_any_properties()` added.

**Deviations from the original plan**

- The plan called for introducing a `PipelineDelta` return type on
  participant hooks. After discussion, deferred to Phase 4 or later.
  The participant hook signatures are unchanged in this phase. The
  bool-returning lowerer functions are an internal contract between
  each lowerer and its participant hook, not a trait-level API.
- Mixed strategy by design: "lowerer reports `bool`" for participants
  where the visitor naturally tracks it (Array, Retain, Polymorphism
  table); `Index`-precheck for participants where threading a
  `changed` flag would be invasive (Polymorphism dispatch, Aggregate,
  Inheritance, Property). Both paths land at the same outcome and the
  prechecks are exact predicates for their conditions.
- The original Phase 0 baseline doc incorrectly listed
  `LoopDesugarer::post_annotate` as an offender; it has no
  `post_annotate` impl at all. The actual seventh offender was
  `PropertyLowerer::post_annotate`, which is now also migrated.

**Gotchas / for the next implementer**

- The current win is "skip the re-pass when the lowerer is a no-op
  project-wide." The three big-cost participants on oscat
  (`PolymorphismLowerer::post_annotate`, `AggregateTypeLowerer`,
  `PolymorphismLowerer::post_index`) still re-index/re-annotate the
  whole project when even a single class / aggregate-returning
  function exists. Reducing those needs the lowerer to report *which
  units* it actually mutated, not just "yes/no." That's Phase 4+.
- The "exact-precheck" approach (Polymorphism dispatch, Aggregate,
  Inheritance, Property) relies on the index field staying in sync
  with what each lowerer actually touches. If anyone adds a new
  lowerer feature that activates on a different signal than the
  precheck checks (e.g. polymorphism that doesn't go through classes /
  FBs / interfaces), the precheck must be widened or moved to a
  lowerer-reports approach.
- `RetainLowerer` now takes `&Index` not `Index`. If anyone changes the
  visitor to need an owned index (e.g. for `transfer_constants`), the
  signature has to revert; check the call sites.

**Carries into Phase 4**

- The natural next step is per-unit deltas: `lower_retain`,
  `TableGenerator::generate`, etc. report a `Vec<UnitId>` describing
  the units they touched. The pipeline then re-indexes only those,
  and re-annotates the reverse-dependency closure built in Phase 2.
- The remaining ~390 ms `annotate` phase on oscat is dominated by the
  three participants that genuinely have work to do. They're the
  primary Phase 4 targets.

### Phase 4 — Cross-unit lowerer hygiene

_Mostly complete._ Two big participants migrated to per-unit
re-index + closure-scoped re-annotate. Numbers in
`docs/baselines/phase4_progress.md`.

**Landed**

- `Index::reindex_unit(unit_id, &mut unit, ids)` — pre-processes the
  unit then drops the unit's previous entries (via the side-table
  Phase 1 built) and re-imports them from the unit's current AST.
- `AnnotatedProject::reannotate_units(&[usize], ids)` — runs
  `TypeAnnotator::visit_unit` in parallel on the listed units against
  the current `self.index`, merges the resulting annotations into the
  existing `AstAnnotations`, replaces each unit's `dependencies` /
  `literals`, and folds `new_index` (generic instantiations) into the
  project index as `UnitId::SYNTHETIC`.
- `AnnotationMap::into_any_box` — small downcast hook so a
  `post_annotate` participant can take its annotations back from
  `Box<dyn AnnotationMap>` after the visitor finishes and rebuild a
  concrete `AstAnnotations` for partial re-annotation.
- `PolymorphismLowerer::table()` (and the inner vtable + itable
  generators) now return `Vec<usize>` of touched unit indices instead
  of `bool`. `PolymorphismLowerer::post_index` re-indexes only those
  units.
- `AggregateTypeLowerer` tracks per-unit mutations via a counter on
  itself; `visit_unit_tracked` returns whether a unit was modified.
  `AggregateTypeLowerer::post_annotate` builds the reverse-dep
  closure of mutated units (callers of POUs whose signature just
  changed), re-indexes the mutated units, runs `evaluate_constants`
  to settle new const-id-backed entries, and re-annotates the
  closure in parallel.
- `compute_reannotate_closure(mutated, units, &reverse_deps)` —
  shared helper that walks each mutated unit's `pous` and unions in
  the dependents recorded by the Phase-2 reverse-dep graph.
- Two new lit tests under `tests/lit/multi/`:
  - `incremental_polymorphism_p4/` — cross-unit vtable generation
    and dispatch through `POINTER TO Base`.
  - `cross_unit_aggregate_signature/` — a STRING-returning function
    declared in one unit and called from another, asserting that
    `AggregateTypeLowerer`'s per-unit re-pass refreshes the caller's
    annotations against the new VAR_IN_OUT signature.
- Multi-file oscat corpus (`.baseline/oscat-multi/`, 556 files one
  POU per file) and a reproducible splitter
  (`docs/baselines/oscat_multi_split.py`).

**Numbers from `docs/baselines/phase4_progress.md`**

| Phase                | annotate (driver) median |
|----------------------|-------------------------:|
| pre Phase 3          | 252 ms                   |
| post Phase 3         | 162 ms                   |
| post Phase 4 step 1  | 160 ms                   |
| post Phase 4 step 2  | **152 ms**               |

On the small polymorphism test the Phase 4 step 1 win is bigger
(~40 % off `index`) because the project has 3 units and the per-unit
re-index entirely avoids walking the unchanged ones; on oscat with
556 units the saving is smaller per absolute number but the
infrastructure now exists for Phase 5 to use the same
remove-and-re-import + reannotate-closure machinery on file edits.

**Deviations from the original plan**

- The plan called for migrating
  `PolymorphismLowerer::post_annotate` (dispatch lowering) too.
  Skipped in this pass — the dispatch visitors (`InterfaceDispatch`,
  `PolymorphicCallLowerer`) don't track per-unit mutation today, and
  threading a `changed` flag through them is more invasive than the
  AggregateTypeLowerer migration. The participant still gates on the
  `project_uses_polymorphism` precheck from Phase 3, so the
  whole-project re-pass only fires when polymorphism is actually
  present. A targeted migration of the dispatch visitors is a
  follow-up Phase 4.x.
- The plan also listed init-function synthesis and generic
  instantiation ownership. The init pass is currently per-unit and
  not on the hot path; the generic-instantiation ownership fix is a
  correctness item for Phase 5 (source-file-edit incremental
  rebuilds) rather than a perf item, so it's parked there.

**Gotchas / for the next implementer**

- The `post_annotate` partial re-annotate path calls
  `const_evaluator::evaluate_constants` because per-unit re-import
  adds entries with un-evaluated `ConstId`-backed expressions (e.g.
  `STRING[K+1]`). Without that call, downstream codegen panics in
  `typesystem::extract_int_constant`. Phase 5 will need the same
  step on every re-import path.
- The reverse-dep closure is conservative — it adds every dependent
  of every POU declared in a mutated unit. For oscat that ends up
  wide because most units call aggregate-returning helpers. Phase 5
  can tighten by hashing pre/post POU signatures and only invoking
  the closure when the signature actually changed.
- `AnnotationMap::into_any_box` is a load-bearing trait method. Any
  new `impl AnnotationMap for ...` must provide it or downcasting
  breaks at participant boundary.

**Carries into Phase 5**

- The `reindex_unit` + `reannotate_units` pair is the partial-rebuild
  primitive Phase 5 will drive from "this file changed."
- The closure builder (`compute_reannotate_closure`) is exactly what
  Phase 5 will use when a signature-hashed comparison reports a
  changed POU.
- The dispatch-visitor migration (`PolymorphismLowerer::post_annotate`)
  remains the biggest single perf opportunity left in `annotate`.

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

## Release strategy & PR chain

This whole effort lands **after v1**. The v1 cut-line is PR #1701
("properties now support the REFERENCE TO type") plus the rest of the
`1.0` label backlog. We let v1 finish first; this work rebases onto
post-v1 `master` once it does. The rebase touches one substantive hunk
(see "PR #1701 interaction" below); everything else is textual
adjacency.

The work is split into a chain of small PRs rather than one giant
branch. Each PR is reviewable on its own, and each one builds on the
previous (stacked diffs). Nothing is opened yet — this section is the
tracking ledger for what will be opened, in what order, against which
base branch.

### PR chain

| # | Branch / scope | Base | Commits on this branch | Notes |
|---|---|---|---|---|
| 1 | `incremental/phase-0-timing` | `master` (post-v1) | `docs: add incremental compilation plan` · `feat(driver): per-phase timing instrumentation` · `docs(book): phase-timing development page` | Plumbing only. No behaviour change. Baselines doc included so reviewers see the target. |
| 2 | `incremental/phase-1-unit-id` | PR #1 | `feat(index): per-unit symbol ownership` | Adds `UnitId`, side-table provenance, `Index::remove_unit`. Still no behaviour change; snapshots stay byte-identical via `#[serde(skip)]`. |
| 3 | `incremental/phase-2-signature-graph` | PR #2 | `feat(index): signature hashing and reverse-dependency graph` | Adds `SignatureHash`, `ReverseDependencyGraph`, dep-tracker audit doc. Still inert plumbing. |
| 4 | `incremental/phase-3-skip-no-op-re-passes` | PR #3 | `perf(driver): skip redundant lowering re-passes when nothing to do` | **Headline win starts here.** Precheck/bool gates on each lowering participant. oscat `annotate` drops from 252 ms to 162 ms in the docs. |
| 5 | `incremental/phase-4-polymorphism-table-per-unit` | PR #4 | `perf(driver): per-unit re-index after polymorphism table generation` · new `tests/lit/multi/incremental_polymorphism_p4/` | First per-unit reindex. Adds `Index::reindex_unit`. |
| 6 | `incremental/phase-4-aggregate-partial-reannotate` | PR #5 | `perf(driver): partial re-annotation after AggregateTypeLowerer` · new cross-unit aggregate lit test | Adds `AnnotatedProject::reannotate_units`, `AnnotationMap::into_any_box`, `compute_reannotate_closure`. oscat `annotate` drops to 152 ms. Largest single behavioural surface; expect the most review. |
| 7 | `incremental/phase-4-retain-array-per-unit` | PR #6 | `perf(driver): per-unit re-index for Retain + Array` | Smaller follow-up. Uniform pattern across the remaining bool-returning lowerers. |
| 8 | `incremental/phase-4.1-lowering-bookkeeper` | PR #7 | `feat(driver): LoweringBookkeeper helper for participant bookkeeping` _(landed locally)_ | Option A from the ergonomics brainstorm. No new trait; existing participants migrate one-by-one in follow-ups. AggregateTypeLowerer migrated in the same commit as the worked example. |
| 9 | `incremental/phase-4.2-unit-lowerer-trait` | PR #8 | `feat(driver): UnitLowerer trait + AutoLowerer adapter` _(landed locally)_ | Option B. New lowerers opt in. `PipelineParticipantMut` keeps working. `RetainParticipant`'s `post_index` migrated as the worked example. |
| 10 | `incremental/phase-4.3-prepare-pass` | PR #9 | `feat(driver): UnitLowerer::prepare for two-pass context-gather` _(landed locally)_ | Optional pre-pass for lowerers that scan all units before transforming any. Default no-op; existing impls unaffected. |
| 11 | `incremental/phase-4.4-array-migration` | PR #10 | `refactor(driver): migrate ArrayLowerer to UnitLowerer` _(landed locally)_ | Old direct `PipelineParticipantMut` impl removed; replaced by `ArrayUnitLowerer` registered through `AutoLowerer`. |
| ~~12~~ | ~~`incremental/phase-4.5-polymorphism-table-migration`~~ | ~~PR #11~~ | `refactor(driver): migrate PolymorphismLowerer table to UnitLowerer` **_(reverted — 13 initializer tests broke; see Phase 4.5 status below for the root-cause sketch)_** | Splits the polymorphism participant; needs follow-up to keep the table pass on the same `PolymorphismLowerer` instance as dispatch. |
| 12 | `incremental/phase-5-incremental-driver` | PR #11 _or later_ | _(future)_ `feat(driver): in-process incremental driver` | The LSP-ready core. Phase 5 of the plan above. |

Stacked-diff workflow:

- Each PR's *target* branch is the previous PR's *head* branch. GitHub
  auto-rebases the next one onto `master` when the previous lands.
- Once a PR is opened, comment on the dependents to mark them
  "blocked by #N" so reviewers see the chain.
- We rebase, don't squash, except inside a PR where multiple commits
  obviously belong together (`docs: ... + feat(driver): ... + docs(book): ...`
  on PR #1, for example).

### PR #1701 interaction

PR #1701 ("fix: properties now support the REFERENCE TO type") is
scoped for v1 and will land first. The conflict surface for this work
is one substantive hunk and several textual ones:

| File | Their change | Our change | Conflict severity |
|---|---|---|---|
| `compiler/plc_driver/src/pipelines.rs` `get_default_mut_participants` | reorders participant list | none of substance | trivial |
| `compiler/plc_driver/src/pipelines/participant.rs` `InitParticipant` | moves hook `pre_annotate` → `post_annotate` | none | trivial |
| `compiler/plc_driver/src/pipelines/participant.rs` `ArrayLowerer` | moves hook to `post_annotate`, body ends in `.index().annotate()` | keeps hook on `pre_annotate`, replaces body with per-unit reindex | **needs reconciliation in PR #7** |
| `compiler/plc_driver/src/pipelines/participant.rs` `ReferenceToReturnParticipant` | adds new `post_annotate` doing `.index().annotate()` | none | additive — leaves a follow-up opportunity to migrate the new participant onto the per-unit path |
| `compiler/plc_lowering/src/reference_to_return.rs` | +491 lines | none | clean |
| `src/lowering/polymorphism/dispatch/interface.rs` | +113 lines for property dispatch | none | clean for now; relevant when Phase 4 dispatch migration happens later |

**Rebase plan for the chain post-#1701:**

1. PR #1701 merges to `master`.
2. Rebase `incremental_compilation` onto new `master`. The only real
   conflict is on `ArrayLowerer`'s impl block — keep our per-unit
   reindex body but move it to the new `post_annotate` hook position
   they introduced. (Per-unit reindex composes with either hook; the
   behaviour is the same.)
3. Re-run workspace tests + lit; numbers in
   `docs/baselines/phase4_progress.md` may shift slightly with the
   new participant order. Re-capture if they do.
4. Split the rebased branch into the seven PRs above (each PR =
   `git cherry-pick`-style on a fresh branch).

**Optional follow-up after the chain lands**: a small PR migrating
`ReferenceToReturnParticipant::post_annotate` from `.index().annotate()`
to the per-unit + closure pattern. This wasn't part of #1701's scope
and is exactly the kind of "new participant immediately benefits from
the framework" win the ergonomics brainstorm is about.

### Phase 4.1 — Status

_Landed locally._

**Landed**

- `compiler/plc_driver/src/pipelines/bookkeeping.rs`: a new
  `LoweringBookkeeper` struct that accumulates per-unit mutation
  effects (mutated units, changed signatures, const-introduction
  flag) and drives the matching invalidation via
  `apply_to_indexed` / `apply_to_annotated`. The hidden invariants
  (per-unit reindex order, `evaluate_constants` placement, closure
  computation against the *pre-mutation* reverse-dep graph) all
  live here now.
- `AggregateTypeLowerer::post_annotate` migrated from its bespoke
  ~70-line body to ~15 lines that just describe what it changed
  and hand the project to the bookkeeper. The free function
  `compute_reannotate_closure` is removed (the bookkeeper absorbs
  it).
- 5 unit tests in `bookkeeping.rs` covering empty / mark / dedup /
  signature / const-flag.

No trait changes; no behavioural change. Workspace tests, lit
suite, and the multi-file oscat baseline are bit-identical to the
post-Phase-4 numbers (~150 ms median annotate, noise band ~±10 ms).

**Remaining participants (PolymorphismLowerer table, Retain, Array)
have NOT been migrated yet** — each is a tiny follow-up PR.
Migrating them is mechanical (the bookkeeper handles their case
trivially since they only mutate units, no signature changes).

### Phase 4.2 — Status

_Landed locally._

**Landed**

- `compiler/plc_driver/src/pipelines/unit_lowerer.rs`: new module with
  `LoweringStage`, `LoweringContext`, `UnitLowerer` trait, `UnitChange`
  struct, `AutoLowerer<T>` adapter implementing
  `PipelineParticipantMut`. The adapter walks units at the registered
  stage, calls `T::lower_unit` per unit, aggregates `UnitChange` into a
  `LoweringBookkeeper`, and dispatches the bookkeeping. `name()`
  forwards through so phase-timing labels stay readable.
- `RetainParticipant::lower_one_unit(&mut CompilationUnit, &Index) -> bool`
  added in `plc_lowering/src/retain.rs` for use by adapters.
- `RetainUnitLowerer` wrapper in `participant.rs` implements
  `UnitLowerer` by delegating to `RetainParticipant::lower_one_unit`.
  Registered via `AutoLowerer::new(...)` in `get_default_mut_participants`,
  replacing the previous `Box::new(RetainParticipant::new(...))`. The old
  `impl PipelineParticipantMut for RetainParticipant` is removed.
- 5 unit tests in `unit_lowerer.rs` covering `UnitChange` constructors,
  `absorb`'s handling of mutated / unmutated / signature-only changes.

`PipelineParticipantMut` is untouched. Existing participants
(`PolymorphismLowerer`, `AggregateTypeLowerer`, `InheritanceLowerer`,
`ArrayLowerer`, etc.) still use it. Workspace tests + lit suite are
bit-identical to Phase 4.1; oscat-multi annotate median sits at ~149 ms.

**What this enables**

- A new lowerer author writes a `UnitLowerer` impl (one trait method)
  and registers it via `AutoLowerer::new(impl, stage, ids)`. The
  framework drives the per-unit walk, runs `evaluate_constants` when
  flagged, computes the reverse-dep closure, partial-re-annotates the
  closure in parallel. None of that bookkeeping appears in the
  lowerer's source.
- New PRs in flight that need a new lowering pass can either implement
  `UnitLowerer` (concise, recommended) or `PipelineParticipantMut`
  (full control, still supported).

**Two-pass support landed in Phase 4.3 below.**

### Phase 4.3 — Status

_Landed locally._

**Landed**

- `UnitLowerer::prepare(&[&CompilationUnit], &LoweringContext)` —
  optional pre-pass for two-pass lowerers (gather context, then
  transform). Default no-op. `AutoLowerer` calls `prepare` once
  before walking units, building a `Vec<&CompilationUnit>` view at
  each stage so existing `Vec<AnnotatedUnit>` (post-annotate) and
  `Vec<CompilationUnit>` (pre-annotate) shapes both work without
  cloning units.
- One demo test (`PrepareCounter`) confirming the contract: prepare
  sees every unit before any `lower_unit` call.

The two-pass shape PR #1701's `ReferenceToReturnParticipant` uses
(`gather_context` then `lower_reference_to_return`) now has a
first-class home on `UnitLowerer` — once #1701 lands, its
participant impl could collapse onto this API.

### Phase 4.4 — Status

_Landed locally._

**Landed**

- `ArrayLowerer::lower_one_unit(&mut CompilationUnit, &Index) -> bool`
  added in `participant.rs`; calls the existing
  `array_lowering::lower_literal_arrays` free function on a single
  unit.
- `ArrayUnitLowerer` wrapper implements `UnitLowerer`. Registered as
  `AutoLowerer::new(ArrayUnitLowerer::new(...), PreAnnotate, ids)`.
- Old `impl PipelineParticipantMut for ArrayLowerer` removed.

The bookkeeping (per-unit re-index when any unit was mutated) moves
into `AutoLowerer`. Participant body went from ~15 lines to a 3-line
`lower_unit`.

### Phase 4.5 — Status

_Reverted (commit `85181cb1a3`)._ The attempted migration of
`PolymorphismLowerer::post_index` (vtable / itable table pass) into a
separate `UnitLowerer` registered alongside the surviving
`PolymorphismLowerer` participant (which kept its `post_annotate`
dispatch impl) passed `cargo check`, `cargo xtask lit`, and
`cargo test --workspace --test-threads=8` but broke **13 initializer
tests** in `plc_lowering/src/initializer.rs`: the
`__vtable__ctor(self.__vtable)` constructor call disappeared from
generated constructor bodies (e.g.
`alias_variable_in_function_block_wrapped_in_adr`,
`class_with_fb_init_calls_user_defined_constructor`,
`fb_without_fb_init_has_no_user_defined_call`).

Suspected root cause is an interaction between the per-unit table
pass and the later `InitParticipant`'s constructor-synthesis. The
table pass patches `__vtable` members into each unit's POUs and adds
vtable types into `user_types`; `Index::reindex_unit` then refreshes
the global index. By the time `InitParticipant` walks members and
emits per-member `__ctor` calls, either:

- the `IdProvider` shared with the rest of the pipeline lost a
  sequence (the wrapper builds a new `PolymorphismLowerer` whose
  state — including vtable-instance type names that hash the source
  path — diverged), or
- the per-unit `reindex_unit` doesn't make the patched `__vtable`
  member visible to `InitParticipant` in the form it expects.

For the next attempt: keep the table pass running through the
*same* `PolymorphismLowerer` instance (don't construct a fresh one
inside the wrapper), and add a focused snapshot test that asserts
the constructor body shape after splitting — exactly the shape the
13 reverted snapshots already capture. Until then,
`PolymorphismLowerer::post_index` stays on the original
`PipelineParticipantMut` path. The per-unit reindex landed in the
Phase 4 step 1 commit `4cca9fdeb5` is still in effect.

### Lowerer-API ergonomics work (post-chain)

The full brainstorm lives in `~/.claude/plans/phase4-extension-api-ergonomics.md`
(not committed; it's a working doc). The summary that's relevant for
this plan:

- **Option A** (`LoweringBookkeeper` helper): no trait changes,
  participants opt in. Reduces the per-participant boilerplate that
  Phases 3–4 added. Targeted as PR #8 above.
- **Option B** (`UnitLowerer` trait + `AutoLowerer<T>` adapter): new
  lowerers can write a tight per-unit transform and get all
  bookkeeping for free. `PipelineParticipantMut` keeps working.
  Targeted as PR #9 above.
- **Option C** (rewrite `PipelineParticipantMut` to return a delta):
  explicitly ruled out. Breaks every existing impl, every in-flight
  PR, and gives no benefit Option B doesn't.

The Option A/B distinction matters for PR coordination too: A is the
substrate B is built on, and a participant author can mix the two
freely. New PRs in flight at the time of these slices don't need to
do anything special.

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
