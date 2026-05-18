# Signature Hash (deferred infrastructure)

> **Status:** preserved on `incremental_compilation-spec-infra-draft`.
> Not present on `master` or `incremental_compilation`.
> Reintroduce only when a downstream consumer drives the completeness
> requirements listed below.

## Purpose

A `SignatureHash` captures the externally observable shape of a POU,
global, or type — enough to answer "did this symbol's interface
change?" without doing a structural diff. Internal POU bodies and
source locations are deliberately *not* part of the hash: editing the
body of a function must not invalidate units that call it.

The intended consumer is an incremental rebuild driver: if a symbol's
signature hash is unchanged between rebuilds, units depending only on
its public surface don't need to be re-annotated.

## Where the code lives

`src/index/signature.rs` on `incremental_compilation-spec-infra-draft`.
Public surface:

```rust
pub struct SignatureHash(pub u64);
pub fn hash_pou(pou: &PouIndexEntry) -> SignatureHash;
pub fn hash_global(var: &VariableIndexEntry) -> SignatureHash;
pub fn hash_type(dt: &DataType) -> SignatureHash;
pub fn hash_implementation(imp: &ImplementationIndexEntry) -> SignatureHash;
```

## Known gaps — close before any downstream consumer ships

The current implementation has gaps that are inert today (no consumer)
but will silently produce stale annotations once consumed. Each must be
closed before a consumer lands.

### Function / Method parameter types not hashed

`hash_pou` for `PouIndexEntry::Function` and `PouIndexEntry::Method`
(`signature.rs:53-86`) hashes return type, generics, linkage, variadic
/ const flags — but **not** parameter types. Adding or changing a
`VAR_INPUT` produces the same hash and callers are not invalidated.

**Fix sketch.** Look the POU's members up in the index, filter to
input/output/in-out variables in declaration order, and hash each
variable's `type_name`, `declaration_kind`
(`VAR_INPUT` / `VAR_OUTPUT` / `VAR_IN_OUT`), `is_by_ref`, and ordinal
position.

### String size and Array dimensions dropped

```rust
// signature.rs:160-164
DataTypeInformation::Array { name, inner_type_name, dimensions } => {
    hash_str(h, name);
    hash_str(h, inner_type_name);
    h.write_u64(dimensions.len() as u64); // only count, not values
}

// signature.rs:185-190
DataTypeInformation::String { size: _, encoding } => {
    // ConstId reference, skipped
    hash_str(h, &format!("{encoding:?}"));
}
```

`STRING[64]` and `STRING[1024]` collide. The comment notes the
`ConstId` shifts across runs — fine, but the correct answer is to
hash the *resolved* constant value, not nothing. Same shape for
`SubRange` bounds (currently skipped entirely beyond the type name).

**Fix sketch.** Resolve each `ConstId` against the index's constant
table at hash time; hash the concrete `i64`/`u64` value. If
resolution fails, fall back to a sentinel that still differs from a
successful resolution.

### FB / Class / Program rely on an undocumented consumer contract

For `Program`, `FunctionBlock`, and `Class` the hash uses
`instance_struct_name` (a `String`) and assumes the consumer also
calls `hash_type(...)` on that struct to capture the member layout.
That contract is undocumented; future driver authors will not know to
walk both.

**Fix sketch.** Either (a) inline the struct's `hash_type` result into
the POU hash so callers only need `hash_pou`, or (b) document the
two-step lookup explicitly at every `hash_pou` call site and add a
helper `hash_pou_full(index, pou)` that does both.

### Struct member hash is incomplete

```rust
// signature.rs:151-158
DataTypeInformation::Struct { name, members, source, .. } => {
    hash_str(h, name);
    hash_str(h, &format!("{source:?}"));
    h.write_u64(members.len() as u64);
    for m in members {
        hash_str(h, m.get_name());
        hash_str(h, m.get_type_name());
    }
}
```

Missing: per-member declaration kind (`VAR_INPUT` vs `VAR_OUTPUT` vs
`VAR_IN_OUT`), `BY_REF`, and ordering effects beyond list length.
Reordering two members produces the same hash; flipping `VAR_INPUT`
to `VAR_OUTPUT` produces the same hash.

**Fix sketch.** For each member also hash `declaration_kind as u8`,
`is_by_ref as u8`, and a per-member ordinal counter.

### `format!("{x:?}")` for enum hashing is brittle

At `signature.rs:153, :168, :211, :127` enum values are converted to
strings via `Debug` and then hashed. Any rewording of a `Debug` impl
silently changes the hash and forces a project-wide re-annotate on the
next rebuild even when no source changed.

**Fix sketch.** Replace each `format!("{x:?}")` with either an
explicit `match` that writes a stable `u8` discriminant, or
`std::mem::discriminant(...)` hashed via its `u64` representation
(unstable across compiler versions — prefer the explicit match).

## Completeness gate

Before reintroducing this module on `master`:

1. All five gaps above closed.
2. A regression test per gap (e.g. `STRING[64]` vs `STRING[1024]`
   must hash differently; reordering struct members must change the
   hash; etc.).
3. A downstream consumer in the same PR that drives the requirements
   — speculative infrastructure must not land twice.
