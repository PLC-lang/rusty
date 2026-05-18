# Reverse Dependency Graph

The reverse-dependency graph maps each named symbol to the set of
compilation units that reference it. It is consumed by the lowering
framework to compute partial re-annotation closures: when a lowerer
changes a unit's signature, the units that must be re-annotated are
exactly the union of dependents of every changed signature name, plus
the mutated units themselves.

## Where the code lives

`compiler/plc_driver/src/pipelines.rs`. Public surface:

```rust
pub struct ReverseDependencyGraph { /* private */ }

impl ReverseDependencyGraph {
    pub fn dependents(&self, symbol: &str) -> Option<&HashSet<UnitId>>;
    pub fn is_empty_for(&self, symbol: &str) -> bool;
    pub fn len(&self) -> usize;
    pub fn is_empty(&self) -> bool;
}

impl AnnotatedProject {
    pub fn reverse_dependencies(&self) -> ReverseDependencyGraph;
}
```

Edges are built from each unit's `Dependency` set, which the type
annotator records in three flavors:

- `Dependency::Call(name)` — direct call sites by callee name.
- `Dependency::Variable(name)` — global variable references by
  qualified name.
- `Dependency::Datatype(name)` — type references by name.

`reverse_dependencies()` walks `self.units[i].dependencies()` and
indexes each entry by name → `UnitId::source(i)`.

## Consumers

- `AggregateTypeLowerer::post_annotate`
  (`compiler/plc_driver/src/pipelines/participant.rs`) captures the
  graph before its STRING-aggregate rewrites so it can ask "who
  used these signatures before the rewrite?".
- `AutoLowerer::post_annotate`
  (`compiler/plc_driver/src/pipelines/unit_lowerer.rs`) uses the
  graph for any `UnitLowerer` registered at post-annotate stage.
- `LoweringBookkeeper::apply_to_annotated`
  (`compiler/plc_driver/src/pipelines/bookkeeping.rs`) computes the
  re-annotation closure: `mutated_units ∪ dependents(changed_sig)`.

## Closure-coverage contract

The closure is correct only if every cross-unit symbol reference
produces a `Dependency` entry under the callee/referent's name. The
resolver paths that currently uphold this contract:

- [x] Direct function/method calls — `resolve_call` records
      `Dependency::Call` under the resolved callee's qualified name.
      Verified by `cross_unit_aggregate_signature.st` lit test.
- [x] Interface dispatch (vtable indirection) — records
      `Dependency::Datatype` for the interface type and
      `Dependency::Call` for each method invoked. Verified by
      `cross_unit_polymorphism_dispatch.st` lit test for the
      base→derived case.
- [x] Implementing-method signature change reaches the dispatcher —
      verified by `cross_unit_interface_dispatch_signature.st`
      (added with this PR; see Required regression tests below).
- [ ] Generic monomorphization — a unit that instantiates a generic
      function on another unit's type may not record a `Dependency`
      under the specialized name. Not yet verified; closure may
      under-invalidate. **If you land a feature that depends on
      cross-unit generics, add a regression test before relying on
      partial re-annotate.**

The first three are required for current consumers. The generic
monomorphization gap is documented as a known risk; no current
consumer exercises it across units.

## Required regression tests

Each new resolver path that contributes to the closure must come with
a lit test that demonstrates the path:

1. **Direct call across units.** A signature change on the callee
   invalidates the caller. Covered by
   `cross_unit_aggregate_signature.st`.

2. **Interface dispatch.** Unit A defines an interface; unit B
   implements it; unit C holds a pointer to the interface and
   dispatches through it. Changing the parameter type of B's
   implementation must invalidate C. Covered by
   `cross_unit_interface_dispatch_signature.st`.

3. **Base→derived vtable.** A method on a derived class is reached
   via the base's vtable. Covered by
   `cross_unit_polymorphism_dispatch.st`.

4. **Generic specialization** (open). A unit that instantiates a
   generic from another unit on a type from a third unit. Add when
   the first consumer crosses unit boundaries with generics.

## When the closure is wrong

A miss in the closure means a stale annotation survives into codegen.
Symptoms: a type mismatch between caller and callee that the
annotator should have caught, but which surfaces as an LLVM-level
crash or a silently miscompiled call. If you suspect a miss, you can
verify by running the same project with `PLC_TIMING=1` and comparing
the closure size against a fresh full-reannotate baseline — if the
miss is exercised, the partial closure will be strictly smaller than
the full one in a way that matters.
