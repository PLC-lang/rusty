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

`plc_cfc` validates an FBD network's structure before lowering (`plc_cfc::validation`),
reporting these reclaimed `plc_xml` error codes and aborting the unit on any of them:

- `E082` — a wire referencing a node nothing produces.
- `E084` — an unconnected data sink.
- `E085` — a cyclic connector/continuation chain (previously a panic).
- `E086` — a continuation with no matching connector.
- `E087` — an unnamed connector/continuation.

Everything else still relies on the shared downstream ST validation (e.g. unknown POU,
type mismatches, unconnected in-out → `E031`), which already points back at the diagram.
Remaining gaps:

- jumps & labels — `E081`/`E083` are parked with the jump feature itself (see above).
- multiple networks in one body — not yet rejected (CFC assumes exactly one).

**Test home (resolved):** these validations are tested as Rust unit tests in
`plc_cfc/src/validation.rs`, not via lit — they produce *diagnostics*, not runtime
output, so they don't fit the `%COMPILE && %RUN | %CHECK` shape used here. Recorded in
this file regardless, so all open CFC topics stay in one place.