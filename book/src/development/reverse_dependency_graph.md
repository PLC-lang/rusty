# Reverse Dependency Graph (deferred infrastructure)

> **Status:** preserved on `incremental_compilation-spec-infra-draft`.
> Not present on `master` or `incremental_compilation`.
> Reintroduce only when a downstream consumer drives the closure
> requirements listed below.

## Purpose

For each named symbol, record the set of compilation units that
reference it. The intended consumer is an incremental rebuild driver:
when a unit's signature changes, the closure of units that must be
re-annotated is exactly the union of dependents of every changed
symbol.

## Where the code lived

`compiler/plc_driver/src/pipelines.rs` on
`incremental_compilation-spec-infra-draft`. Public surface:

```rust
pub struct ReverseDependencyGraph {
    edges: HashMap<String, HashSet<UnitId>>,
}

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

`reverse_dependencies()` walks `self.units[i].dependencies()` and
indexes each `Dependency` by name → `UnitId::source(i)`.

## Closure gap — close before any downstream consumer ships

Edges are built from `Dependency::{Datatype, Call, Variable}`. These
are recorded by the type annotator (`src/resolver*.rs`) when it
resolves a symbol reference. The assumption is that this is
exhaustive — every cross-unit reference produces a `Dependency` under
the callee's name.

**That assumption is unverified.** Implicit references that may not
record a `Dependency` under the referenced symbol's name:

- **Interface dispatch through a base pointer.** A call site holding
  an interface variable invokes a method by vtable slot, not by
  resolved name. The dispatching unit may record only the interface
  type as a `Dependency::Datatype`, not the implementing method.
  Changing the implementing method's signature would not invalidate
  the dispatcher.
- **Vtable indirection emitted by polymorphism lowering.** The
  polymorphism lowerer synthesizes `__vtable__ctor`,
  `__FATPOINTER`, and itable entries *after* annotation. References
  to these symbols may not appear in the resolver's `Dependency`
  set.
- **Generic monomorphizations registered into `new_index` after
  annotation.** A unit that triggers a generic specialization on a
  type from another unit may not record a `Dependency` under the
  specialized name.

## Required resolver-paths contract (to be enumerated)

Before reintroduction, the resolver paths that *must* record a
`Dependency` for the closure to be correct should be enumerated and
named here. Skeleton (each line is a contract whose source path
needs verification):

- [ ] `resolve_call` records `Dependency::Call` under the callee's
      qualified name for every direct call, **including** interface
      method calls (under the implementing method's name, not the
      interface's).
- [ ] `resolve_variable_reference` records `Dependency::Variable`
      under the global's qualified name for cross-unit global
      reads/writes.
- [ ] `resolve_datatype_reference` records `Dependency::Datatype`
      for every named type reference, **including** parameter types,
      return types, and field types reached transitively through
      structs.
- [ ] Polymorphism lowering registers synthesized vtable / itable /
      `__FATPOINTER` types as `Dependency::Datatype` against every
      unit that consumes them (the lowerer runs after annotation, so
      this must be done either by the lowerer registering edges
      directly or by re-annotating the synthesizing units).
- [ ] Generic monomorphization registers `Dependency::Call` and
      `Dependency::Datatype` for each generated specialization in
      the unit that triggered it.

Any path that resolves a cross-unit reference but does not appear
above is a closure gap.

## Required regression tests before consumption

1. **Interface dispatch + signature change.** Unit A defines an
   interface `IFoo` with method `do(x: INT)`. Unit B implements
   `IFoo` in a function block. Unit C holds a `POINTER TO IFoo` and
   calls `do(...)` through it. Changing the parameter type of B's
   `do` from `INT` to `DINT` must invalidate C — the dependency
   closure must include C even though C does not name B's
   implementation directly.

2. **Cross-unit `__vtable__ctor`.** Unit A defines an FB with a
   member whose ctor needs `__vtable__ctor(self.__vtable)`. Unit B
   consumes that FB as a member field. Changing A's vtable shape
   must invalidate B.

3. **Generic specialization.** Unit A defines a generic function
   `swap<T>`. Unit B calls `swap` on a struct from unit C. Changing
   C's struct must invalidate B (through the specialization), not
   just C.

## Completeness gate

Before reintroducing this module on `master`:

1. The contract checklist above filled in with verified resolver
   paths.
2. The three regression tests added (likely as lit tests under
   `tests/lit/`).
3. A downstream consumer in the same PR that drives the closure
   requirements.
