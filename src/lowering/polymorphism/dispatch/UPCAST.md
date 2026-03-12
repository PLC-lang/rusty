# Interface-to-Interface Upcasting

## The Problem

When assigning a concrete POU instance to an interface reference, everything is statically known:

```
// refIA : IA, instance : FbA
refIA.data  := ADR(instance);
refIA.table := ADR(__itable_IA_FbA_instance);  // ✅ known at compile time
```

But for **interface-to-interface** assignments (upcasting), the concrete POU behind the source
reference is unknown at compile time:

```
// refIA : IA, refIB : IB (IB EXTENDS IA)
refIA := refIB;

// Lowered to:
refIA.data  := refIB.data;   // ✅ trivial
refIA.table := ???;           // ❌ which __itable_IA_*_instance?
```

`refIB.table` points to an `__itable_IB` (e.g. `__itable_IB_FbA_instance`), but `refIA` needs an
`__itable_IA` pointer. We don't know at compile time which POU is behind `refIB`, so we can't
statically pick the right `__itable_IA_<POU>_instance`.

## Solution: Ancestor Itable Pointers

Embed ancestor itable pointers directly inside each itable struct. For every ancestor interface
that a given interface extends, add a pointer field to the itable struct definition:

```
__itable_IA {
    foo: __FPOINTER IA.foo;
}

__itable_IB {
    __upcast_ia: POINTER TO __itable_IA;   // ← ancestor pointer
    foo: __FPOINTER IA.foo;
    bar: __FPOINTER IB.bar;
}
```

The itable instances populate these pointers to the matching itable for the same POU:

```
__itable_IA_FbA_instance: __itable_IA := (foo := ADR(FbA.foo))
__itable_IB_FbA_instance: __itable_IB := (
    __upcast_ia := ADR(__itable_IA_FbA_instance),
    foo := ADR(FbA.foo),
    bar := ADR(FbA.bar)
)
```

### Upcasting (O(1), compile-time resolvable)

The compiler knows the source type (`IB`) and target type (`IA`) from annotations, so it
statically emits a cast + field access:

```
// refIA := refIB   (IB extends IA)
refIA.data  := refIB.data;
refIA.table := __itable_IB#(refIB.table^).__upcast_ia;
```

This works regardless of which POU is behind `refIB` at runtime, because every `__itable_IB`
instance has its `__upcast_ia` correctly populated.

### Diamond / Multi-Parent Inheritance

For interfaces with multiple parents, each itable includes pointers to ALL ancestor itables:

```
INTERFACE IA ... END_INTERFACE
INTERFACE IB EXTENDS IA ... END_INTERFACE
INTERFACE IC EXTENDS IA ... END_INTERFACE
INTERFACE ID EXTENDS IB, IC ... END_INTERFACE

__itable_ID {
    __upcast_ia: POINTER TO __itable_IA;
    __upcast_ib: POINTER TO __itable_IB;
    __upcast_ic: POINTER TO __itable_IC;
    foo: __FPOINTER ...;
    bar: __FPOINTER ...;
    ...
}
```

All upcasts are O(1) field loads: `ID→IA`, `ID→IB`, `ID→IC`.

### Transitive Upcasting

Works because each itable knows its own ancestors:

```
refIB := refID;  // table = __itable_ID#(refID.table^).__upcast_ib → some __itable_IB instance
refIA := refIB;  // table = __itable_IB#(refIB.table^).__upcast_ia → some __itable_IA instance ✅
```

### Same-Type Assignment

When source and target have the same interface type, it's a plain struct copy:

```
// refIA_1 := refIA_2   (both IA)
refIA_1.data  := refIA_2.data;
refIA_1.table := refIA_2.table;   // same itable type, just copy the pointer
```

### Cost

One extra pointer per ancestor interface per itable struct. For typical PLC interface hierarchies
(2–3 levels deep), this is negligible.

## Design Trade-off: Embedded Pointers vs. Runtime Lookup

Our approach embeds ancestor itable pointers directly in the itable struct, making every upcast a
single pointer load at O(1) with zero runtime infrastructure. This is the same strategy Rust uses
for trait object upcasting (stabilized in Rust 1.76, RFC 3324): parent vtable pointers are stored
inside child vtables.

The alternative, used by Go, Java (JVM), and C# (CLR), is to resolve interface-to-interface
conversions at **runtime** — typically via hash table lookups or linear searches through interface
maps. Go caches the result after the first lookup for amortized O(1), but the first call pays a
search cost and the runtime must maintain a lookup cache.

| Approach                  | Used by        | Upcast cost     | Trade-off                        |
|---------------------------|----------------|-----------------|----------------------------------|
| Embedded ancestor pointers | Rust, **us**  | O(1) field load | Larger itables (one ptr/ancestor)|
| Runtime lookup/search      | Go, Java, C#  | O(1) amortized  | Needs runtime infrastructure     |

For a PLC compiler, the embedded-pointer approach is a natural fit:

- **Deterministic timing**: no first-call penalty or cache miss surprises, which matters in
  real-time control contexts.
- **No runtime infrastructure**: no hash tables, no caches, no allocator — just static data.
- **Negligible memory cost**: PLC interface hierarchies are typically shallow (2–3 levels), so
  each itable grows by only a handful of pointers.

C++ takes a third path for its multiple/virtual inheritance: vtables store **byte offsets** and use
**thunks** to adjust the `this` pointer. This is more complex and addresses problems specific to
concrete multiple inheritance (object layout, shared base sub-objects) that don't apply to our
interface-only model.

## Instanceof (deferred, not part of this PoC)

Instanceof requires runtime type identity ("is the concrete POU behind this fat pointer a subtype
of X?"), which is a fundamentally different question from itable lookup. The plan is to use
`__type_meta` for that purpose separately. The fat pointer would gain a third field:

```
__FATPOINTER {
    data:  POINTER TO __VOID;
    table: POINTER TO __VOID;
    meta:  POINTER TO __type_meta;   // for instanceof, added later
}
```

This is tracked separately and out of scope for this change.

## Summary

| Concern          | Mechanism                                          | Cost              |
|------------------|----------------------------------------------------|-------------------|
| Method dispatch  | `__itable_X#(ref.table^).method^(ref.data^)`       | O(1), existing    |
| Interface upcast | `__itable_X#(ref.table^).__upcast_target`           | O(1), field load  |
| `instanceof`     | `ref.meta^` → ancestry check                       | deferred          |
