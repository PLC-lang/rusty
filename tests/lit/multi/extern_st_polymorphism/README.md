# extern_st_polymorphism

Tests polymorphic dispatch when a function block is defined in a pure-ST library compiled to a
shared object, and the consumer declares it as `{external}` **without** `--generate-external-constructors`.

## Setup

- `lib.st` — Defines `INTERFACE IGreeter` and `FUNCTION_BLOCK FbA IMPLEMENTS IGreeter` with a
  `greet` method body. Compiled into `libpoly.so`.
- `app.st` — Redeclares the interface and declares `{external} FUNCTION_BLOCK FbA IMPLEMENTS IGreeter`.
  Exercises both dynamic vtable dispatch (via pointer variables) and interface-typed calls
  (itable dispatch).

## Build

1. `plc lib.st --shared -o libpoly.so`
2. `plc app.st -L... -lpoly --linker=cc` (no `--generate-external-constructors`)

## What this tests

- **Vtable dispatch**: the vtable generator is linkage-aware. For `{external}` POUs without the
  flag, vtable instances are emitted as extern declarations (`external global` in LLVM IR). The
  linker resolves them from the library.
- **Itable dispatch**: the itable generator is linkage-aware (mirroring the vtable generator).
  For `{external}` POUs without the flag, itable instances and struct constructors are emitted
  as extern declarations, resolved from the library at link time. This avoids duplicate symbol
  errors with both shared and static linking.
