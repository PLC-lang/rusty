# Writing a Lowering Pass

A *lowering pass* rewrites the AST between parse and codegen — for
example, the polymorphism lowerer adds vtable members to function
blocks, the aggregate-return lowerer rewrites STRING-returning
functions into VAR_IN_OUT-parameter form. There are two trait
surfaces a new pass can implement, and two helpers that make the
bookkeeping invisible. This chapter walks through which to pick and
how to wire it up.

## Decide which trait to implement

| Shape | Recommended trait | Why |
|---|---|---|
| Walks units one at a time, doesn't need to stash state on `self` between units | [`UnitLowerer`](#unitlowerer) | Per-unit transform, framework drives the walk and the invalidation |
| Walks units one at a time but needs a project-wide context first | [`UnitLowerer` with `prepare`](#two-pass-prepare-then-lower_unit) | Two-pass — gather, then transform |
| Has two distinct hooks that share state (e.g. one runs at post-index, the other at post-annotate) | [`UnitLowerer` paired with `PipelineParticipantMut` via `Rc<RefCell<...>>`](#multi-stage-shared-state) | Each slot in the participant chain sees the same inner instance |
| Holds visitor state on `self` during the walk in a way that doesn't fit "lend context, get `UnitChange`" | [`PipelineParticipantMut` + `LoweringBookkeeper`](#piplelineparticipantmut-with-loweringbookkeeper) | Manual destructure + the bookkeeper handles the rest |
| Needs full control over destructuring `ParsedProject` / `IndexedProject` / `AnnotatedProject` | `PipelineParticipantMut` directly | Escape hatch; rare |

Defaults to the top of the table — most new lowerers fit `UnitLowerer`.

## What `AutoLowerer` does for you

When you implement `UnitLowerer` and register it through
`AutoLowerer::new(inner, stage, ids)`, the framework:

1. Calls `inner.prepare(&units, &ctx)` once (no-op by default).
2. Walks every compilation unit and calls
   `inner.lower_unit(&mut unit, &ctx)`. Collects each `UnitChange`
   into a `LoweringBookkeeper`.
3. If the bookkeeper is empty (no `mutated` flag, no
   `changed_signatures`), returns the project untouched. No
   re-index. No re-annotate.
4. Otherwise:
   - Calls `Index::reindex_unit` for each unit marked mutated.
   - If any unit reported `introduces_constants`, runs
     `evaluate_constants` once over the patched index.
   - Computes the re-annotation closure: the set of mutated units
     plus every unit recorded as a dependent of any
     `changed_signatures` name in the pre-mutation reverse-dep
     graph.
   - Calls `AnnotatedProject::reannotate_units(&closure, ids)` in
     parallel for post-annotate stages.

None of that appears in your `lower_unit` body.

## `UnitLowerer`

The simplest shape. A POU-renamer that uppercases program names:

```rust
use ast::ast::CompilationUnit;
use plc_driver::pipelines::unit_lowerer::{
    LoweringContext, UnitChange, UnitLowerer,
};

pub struct UppercaseProgramNames;

impl UnitLowerer for UppercaseProgramNames {
    fn name(&self) -> &'static str { "UppercaseProgramNames" }

    fn lower_unit(
        &mut self,
        unit: &mut CompilationUnit,
        _ctx: &LoweringContext,
    ) -> UnitChange {
        let mut change = UnitChange::default();
        for pou in &mut unit.pous {
            if pou.kind.is_program() {
                pou.name = pou.name.to_uppercase();
                change.mutated = true;
                change.changed_signatures.push(pou.name.clone());
            }
        }
        change
    }
}
```

Register it:

```rust
use plc_driver::pipelines::unit_lowerer::{AutoLowerer, LoweringStage};

pipeline.register_mut_participant(Box::new(AutoLowerer::new(
    UppercaseProgramNames,
    LoweringStage::PostIndex,
    pipeline.context.provider(),
)));
```

### Picking the stage

`LoweringStage` determines when the lowerer fires:

- `PreIndex` — after parsing, before the initial index pass. No
  index available; `LoweringContext::index` is a dummy. Use when
  you transform the AST purely structurally.
- `PostIndex` — after the index is built. `LoweringContext::index`
  is the global index; `annotations` is `None`. Most "I need to
  see what types exist" lowerers go here.
- `PreAnnotate` — between indexing and annotation. Same shape as
  `PostIndex`.
- `PostAnnotate` — after annotation. `LoweringContext::annotations`
  is `Some(&AstAnnotations)`. Use when you need resolved type
  information to drive the rewrite. The framework runs the
  re-annotation closure for you.

### `UnitChange` fields

```rust
pub struct UnitChange {
    pub mutated: bool,
    pub changed_signatures: Vec<String>,
    pub introduces_constants: bool,
}
```

- `mutated`: did the unit's AST actually change? If false, the unit
  is not re-indexed.
- `changed_signatures`: names of POUs / globals / types declared in
  this unit whose public signature changed (return type added, new
  parameter, etc.). The framework looks these names up in the
  reverse-dep graph and adds every dependent unit to the
  re-annotation closure. Skip this when the change is purely
  internal (statement-level rewrites that don't affect callers).
- `introduces_constants`: set this when the rewrite added
  `ConstId`-backed expressions (string sizes, array dimensions,
  subrange bounds). Triggers `evaluate_constants` after the
  partial re-index. Cheap to over-flag (the const-eval pass is
  idempotent); silent footgun to forget.

Use the constructors when you can:

```rust
UnitChange::none()                    // identical to default()
UnitChange::mutated()                 // mutated = true, others default
```

## Two-pass: `prepare` then `lower_unit`

When the lowerer needs a project-wide context — e.g. "find every
POU returning `REFERENCE TO` before I can rewrite any of their call
sites" — implement `prepare`. It runs once, with immutable access
to every unit:

```rust
use rustc_hash::FxHashSet;

pub struct ReferenceToReturnUnit {
    reference_returners: FxHashSet<String>,
}

impl UnitLowerer for ReferenceToReturnUnit {
    fn name(&self) -> &'static str { "ReferenceToReturnLowerer" }

    fn prepare(&mut self, units: &[&CompilationUnit], _ctx: &LoweringContext) {
        self.reference_returners.clear();
        for unit in units {
            for pou in &unit.pous {
                if returns_reference_to(pou) {
                    self.reference_returners.insert(pou.name.clone());
                }
            }
        }
    }

    fn lower_unit(
        &mut self,
        unit: &mut CompilationUnit,
        _ctx: &LoweringContext,
    ) -> UnitChange {
        // …rewrite call sites that target a name in
        // self.reference_returners; report mutated / changed_signatures.
        UnitChange::default()
    }
}
```

`prepare` cannot mutate units (slice is `&[&CompilationUnit]`);
that's the compile-time enforcement that "gather happens
first." Stash whatever you need on `self`, read it in
`lower_unit`.

## Multi-stage shared state

Some lowerers need to do work at two pipeline stages that depend on
each other. The polymorphism lowerer emits vtable types at
`PostIndex` and rewrites method calls through those tables at
`PostAnnotate`. Each stage is a separate participant in the chain;
they must see the same inner state.

Wrap the inner lowerer in `Rc<RefCell<...>>` and register two
adapter slots that share the same handle:

```rust
pub type SharedMyLowerer = Rc<RefCell<MyLowerer>>;

pub fn shared_my_lowerer(ids: IdProvider) -> SharedMyLowerer {
    Rc::new(RefCell::new(MyLowerer::new(ids)))
}

// Stage 1: per-unit work, via UnitLowerer
pub struct MyLowererPhase1 { inner: SharedMyLowerer }
impl UnitLowerer for MyLowererPhase1 {
    fn name(&self) -> &'static str { "MyLowerer (phase 1)" }
    fn lower_unit(&mut self, unit: &mut CompilationUnit, ctx: &LoweringContext) -> UnitChange {
        let mutated = self.inner.borrow().phase1_one_unit(ctx.index, unit);
        if mutated { UnitChange::mutated() } else { UnitChange::none() }
    }
}

// Stage 2: project-wide work, via PipelineParticipantMut
pub struct MyLowererPhase2 { inner: SharedMyLowerer }
impl PipelineParticipantMut for MyLowererPhase2 {
    fn name(&self) -> &'static str { "MyLowerer (phase 2)" }
    fn post_annotate(&mut self, project: AnnotatedProject) -> AnnotatedProject {
        let mut inner = self.inner.borrow_mut();
        inner.phase2(/* … */);
        // … rebuild project …
        project
    }
}
```

Registration:

```rust
let shared = shared_my_lowerer(pipeline.context.provider());

pipeline.register_mut_participant(Box::new(AutoLowerer::new(
    MyLowererPhase1 { inner: shared.clone() },
    LoweringStage::PostIndex,
    pipeline.context.provider(),
)));
pipeline.register_mut_participant(Box::new(MyLowererPhase2 {
    inner: shared,  // last clone, moved
}));
```

The `Send` bound is off on `AutoLowerer` to allow `Rc<RefCell<...>>`:
the participant chain runs sequentially, so single-threaded
sharing is sound.

> **Why not two independent instances?** The first attempt at
> migrating the polymorphism lowerer constructed a fresh
> `MyLowerer` per slot. The two diverged on `ids` and on the
> vtable-instance type names they emitted; downstream
> `InitParticipant` then failed to find the vtable members it
> expected. The shared `Rc<RefCell<...>>` keeps the two slots'
> view of state identical.

## `PipelineParticipantMut` with `LoweringBookkeeper`

When the visitor pattern needs to stash state on `self` between
units in a way that doesn't fit `UnitLowerer`'s "lend context, get
`UnitChange`" model, implement `PipelineParticipantMut` directly
and use `LoweringBookkeeper` to drive the invalidation:

```rust
use plc_driver::pipelines::bookkeeping::LoweringBookkeeper;

impl PipelineParticipantMut for MyStatefulLowerer {
    fn post_annotate(&mut self, project: AnnotatedProject) -> AnnotatedProject {
        if !project_needs_my_lowering(&project.index) {
            return project;
        }

        // Capture reverse-dep graph BEFORE mutation.
        let reverse_deps = project.reverse_dependencies();

        // Destructure, stash state on self for the visitor's duration.
        let AnnotatedProject { mut units, index, annotations, diagnostics } = project;
        self.index = Some(index);
        self.annotation = Some(Box::new(annotations));

        // Walk units; describe what changed.
        let mut book = LoweringBookkeeper::new();
        for (idx, annotated_unit) in units.iter_mut().enumerate() {
            if self.visit_one_unit(&mut annotated_unit.unit) {
                book.mark_unit(idx);
                // … fill in book.signature_changed(...) for each
                //    POU/global/type whose signature changed in this unit
            }
        }
        if !book.is_empty() && self.may_introduce_constants() {
            book.mark_const_introduced();
        }

        // Reclaim state.
        let index = self.index.take().expect("index returned by visit");
        let annotations = recover_annotations(self.annotation.take().unwrap());

        let project = AnnotatedProject { units, index, annotations, diagnostics };
        book.apply_to_annotated(project, &reverse_deps, self.id_provider.clone())
    }
}
```

The bookkeeper hides the same invariants `AutoLowerer` hides — per-
unit `Index::reindex_unit`, `evaluate_constants`, closure
construction, parallel `AnnotatedProject::reannotate_units`. The
difference is the visitor's state pattern stays under your control.

`AggregateTypeLowerer` in the driver follows this shape today.

## Things the framework won't do for you

- **Decide whether your rewrite is sound.** If you flag a unit as
  not mutated but actually changed its AST, downstream
  participants may use a stale index. The framework trusts
  `UnitChange::mutated`.
- **Re-evaluate signatures.** If you change a POU signature but
  forget to push the name into `changed_signatures`, callers'
  annotations stay stale. The framework trusts your annotation.
- **Track inter-pass dependencies.** If two lowerers contradict
  each other (one undoes the other), the participant chain runs
  them in list order and you get the last writer's result.

## Debugging

Set `PLC_TIMING=1` to see every participant's wall-clock time
in the build trace. Wrappers forward their `name()` through, so
e.g. an `AutoLowerer<MyUnitLowerer>` shows up as
`post_index/MyUnitLowerer: 1.234ms` rather than as
`post_index/AutoLowerer`. See [Profiling Build Phases](phase_timing.md)
for the trace format.
