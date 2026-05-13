# Dependency tracker coverage

A coverage matrix for the per-unit `FxIndexSet<Dependency>` collected by
`TypeAnnotator` in `src/resolver.rs`. The resolver inserts at the sites
listed below; codegen already consumes the resulting set (see
`src/codegen.rs:160`, `data_type_generator.rs:58`,
`pou_generator.rs:60`, `variable_generator.rs:103`).

This document is the audit deliverable from the dependency-tracker
review. It is consulted by later phases to decide whether the existing
tracker is exhaustive enough to drive per-unit invalidation, and what
needs extending if not.

## Insertion sites

| Site | Variant inserted | When |
|---|---|---|
| `resolver.rs:187` | `Call(name)` | Function call resolution. Also extends with `get_datatype_dependencies(name)` and on the return type. |
| `resolver.rs:192` | `Call(qualified_name)` | Program call resolution. Plus `get_datatype_dependencies(qualified_name)`. |
| `resolver.rs:212` | `Variable(qualified_name)` | Reference to a global variable. Also adds the variable's datatype dependencies; for `AutoDerefType::{Alias, Reference}(inner)` it adds `Datatype(inner)`. |
| `resolver.rs:216` | `Datatype(resulting_type)` | Resolution of any `StatementAnnotation::Value`. Plus its transitive datatype deps. |
| `resolver.rs:1405` | `Datatype(pou.name)` | POU declaration visit. |
| `resolver.rs:1407` | `Datatype(super_class_name)` | POU's `EXTENDS` clause. |
| `resolver.rs:1440` | `Datatype(parent_pou_name)` | Method visit (records the enclosing POU). |
| `resolver.rs:1926` | (transitive) | Datatype declaration visit — extends with the datatype's full transitive closure. |
| `resolver.rs:1988` | `Datatype(name)` | Variable visit — variable's declared type. |
| `resolver.rs:2009` | (transitive) | Reference resolution — datatype deps of the referenced type. |
| `resolver.rs:1934` (`get_datatype_dependencies`) | `Datatype(name)` plus structural recursion | The transitive walker: structs recurse into member types, arrays / pointers into their element type, FB/class structs additionally include their `__vtable_*` type, methods include their parent. |

The set produced for a unit is the union of all the above firings.
Codegen also adds inserts elsewhere (e.g. `pou_generator` adds
`Datatype(name)` for declarations), but those are downstream of the
resolver — they live on `AnnotationMapImpl::new_index`, not on the
per-unit `Dependency` set.

## Known gaps

The following edges are *not* currently captured. Some are deliberate
(downstream phases handle them); others are tracked here for future
extension.

### 1. Generic instantiations are recorded into `new_index`, not into the requesting unit's deps

`register_generic_pou_entries` in `src/resolver/generics.rs:122` adds
synthesized POUs / pou types / data types into
`AnnotationMapImpl::new_index` (lines 132, 143, 205, 244) when it
encounters a generic call like `FB_Generic<INT>`. The resulting symbol
lives in the global index after the resolver's `new_index` is merged
(tagged `UnitId::SYNTHETIC`), but the requesting unit's
`Dependency::Call(...)` entry uses the *original* generic name, not
the instantiated name. Effect: when the user changes only the
instantiation site, the reverse-dependency graph does not see a fresh
edge to the synthesized symbol.

For the headline near-term win (eliminate redundant lowering re-passes)
this is not load-bearing — the participants don't ask "who uses the
instantiation." It becomes relevant in Phase 5 once we want to scope
re-annotation closures by signature-hash diff; we can either extend
the tracker here, or stamp the instantiation with deterministic
ownership (Phase 4) so the closure is computable from the original
generic alone.

### 2. Value dependencies on `VAR_GLOBAL CONSTANT`

When a unit reads a constant for an array dimension, string size, or
subrange bound, the dependency is recorded as `Variable(name)` (via
the path at `resolver.rs:212`) only when the constant is referenced
via a `StatementAnnotation::Variable`. Initializer expressions that
embed `ConstId` references via `ConstExpressions` don't go through the
normal annotation path and so don't appear in the unit's dep set.

Symptom: changing a `VAR_GLOBAL CONSTANT K : INT := 8` to `9` may not
invalidate units that used `ARRAY[1..K] OF INT`. For Phase 3 this is
acceptable because lowering re-pass elimination doesn't depend on
value-deps. For Phase 5 (incremental rebuilds) we'll need to extend.

### 3. Lowering-introduced dependencies

The resolver's `Dependency` set is what the *resolver* saw. Several
later passes inject calls or type references that aren't on the
resolver's radar — init-constructor calls, vtable lookups, generic
dispatch sites. These edges have to be added by each participant's
delta in Phase 3, not retro-fitted into the resolver.

### 4. Built-in functions

`Dependency::Call("ADD")` etc. for built-in operators *is* recorded
(they go through the same `StatementAnnotation::Function` path). The
target symbols live under `UnitId::BUILTIN` and are never invalidated,
so a missing edge here doesn't break incrementality even if it ever
appeared.

### 5. Labels are not tracked

Per-unit GOTO labels (`Index.labels`) aren't recorded as dependencies,
but they're also per-POU and don't cross unit boundaries, so this is
fine.

## Verdict

For the **near-term headline win** (eliminating redundant lowering
re-passes; Phases 3 and 4), the existing tracker is sufficient. The
participant deltas in Phase 3 will declare the units they mutated; the
reverse-dep graph closes over the rest using the by-name edges already
recorded.

For **incremental rebuilds driven by source-file edits** (Phase 5), at
least gaps (1) and (2) above must be filled. The extension is
mechanical — record a `Call` dep at the generic-instantiation site, and
record a `Variable` dep when a const expression embeds a `ConstId`
that points to a global constant — but it requires care to keep
diagnostics and codegen bit-identical.

## How to re-run this audit

```sh
grep -n "self\.dependencies\.\(insert\|extend\)\|get_datatype_dependencies" \
    src/resolver.rs
```

The grep output for the current tree is the authoritative source; the
table above is a human-readable summary as of the Phase 2 work. Any new
insertion site or removed line should be reflected here.
