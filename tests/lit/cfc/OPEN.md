# Open points — CFC integration tests

This directory holds full-project, runtime-verified integration tests for the CFC
(`plc_cfc`) frontend: a `.cfc` unit under test plus an ST harness that drives it and
checks its output via `%RUN | %CHECK`.

The features below are **not yet supported** by `plc_cfc`. Each is parked on an open
design/implementation question. Once a feature is designed, agreed, and implemented,
it needs its own integration test added here.

## Jumps & labels (GOTO)

Not implemented. The IDE emits a `Jump` (`targetNetworkLabel="…"`) but does **not**
serialize the label it targets, and the standard schema only offers a network-level
label — incompatible with CFC's single-network model. A vendor `Label` FBD object was
proposed (see the design notes and `assets/`), but the work is parked until the IDE
actually emits the label.

→ Add jump/label integration tests once the label encoding lands.

## Builtin operator blocks (XOR, AND, OR, ADD, MUL, …)

CFC can drop an IEC builtin operator as a block (e.g. an `XOR` block with two inputs).
`plc_cfc` does not model these yet: they are builtins, not user POUs, so the current
"call the POU" lowering doesn't apply — a builtin-operator block should lower to the
operator expression / an intrinsic rather than a POU call, and we haven't defined that.

**Stopgap used by the tests here:** instead of a builtin-operator block, the test CFC
files call small user-defined ST wrapper functions (`MyAdd`, `MyXor`, …) whose body is
the native ST operator (`MyAdd := a + b;`). These are *concrete-typed*, not generic:
rusty generics require hand-written `NAME__TYPE` specializations (see `ABS` in
`libs/stdlib/iec61131-st/numerical_functions.st`) and variadics require `{external}`
builtins with no usable ST body — neither buys us a clean single wrapper, and our
graphs use known types and binary operators anyway.

→ TODO: define the builtin-operator-block lowering, then replace these wrappers with
real operator blocks and add integration tests covering the common operators.

## Validations

`plc_cfc` has **no validations** yet. The frontend lowers a `.cfc` to AST and relies
entirely on the shared downstream ST validation; it does not yet surface CFC-specific
diagnostics. Known gaps where the old `plc_xml` crate had proper error codes (now lost):

- cyclic connector/continuation chains — currently *panic* (`Resolver::resolve_alias`)
    instead of a diagnostic (cf. `E085`).
- unconnected / dangling pins, missing required connections.
- unnamed or malformed control objects.
- (later) jumps targeting a non-existent label, duplicate labels.

→ TODO: define a CFC validation/error-reporting story and the diagnostics it emits.

**Open question:** should validation behaviour be tested via lit at all? Validations
produce *diagnostics*, not runtime output — they're a poor fit for the
`%COMPILE && %RUN | %CHECK` (compile-and-run) shape these CFC integration tests use.
They likely belong in Rust-level diagnostic/snapshot tests (as `plc_xml`'s
`validation_tests` were) rather than here. Recorded in this file regardless, so all
open CFC topics live in one place — but the test *home* for validations is TBD.