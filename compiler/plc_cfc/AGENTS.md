# plc_cfc — working conventions

This crate transpiles CFC/FBD diagrams (`.cfc` XML files exported by the IDE)
into ST ASTs. When implementing a new feature or fixing a behavior, follow the
fixture-first workflow below.

## Workflow

1. **Create a fixture** under `fixtures/valid/<scenario>/` (or
   `fixtures/invalid/` for diagnostics). The fixture must be *IDE-correct*: it
   should be copy-pasteable into the IDE to cross-check and debug. That means
   the XML must adhere to the schema the IDE exports — including `RelPosition`
   and `Size` elements, `connectionPointOutId`s, `EvaluationPriority` entries
   and the vendor `AddData` wrappers. Use the existing fixtures as templates.

   Each fixture folder contains:
   - `README.md` — an ASCII diagram of the network, a legend for any special
     markers (negation bubbles, priority badges, …), a file list, and the ST
     the network is expected to mean.
   - `mainProgram.cfc` (or similarly named `.cfc` files) — the diagram under
     test, with a `textDeclaration` carrying the POU declaration.
   - `.st` files for every POU the diagram calls, so the fixture compiles as a
     whole project.

2. **Add a unit test in `src/resolver.rs`** using the fixture (via
   `include_str!`), asserting the resolved objects: source count, which
   `Object` each connection ID maps to, and any flags relevant to the feature.

3. **Add a unit test in `src/transpiler.rs`** using the fixture, asserting the
   generated ST with an inline `assert_snapshot!`. Copy the fixture README's
   ASCII diagram into both tests as a comment.

4. **Add a lit test** under `tests/lit/cfc/<scenario>/`: the fixture's files
   plus a `main.st` entry point that runs the network and `printf`s the
   observable result, and a `.test` driver whose RUN line compiles and executes
   them. Put the `// CHECK: ...` lines at the printf call sites in `main.st`
   and point the driver at them (`... | %CHECK %S/main.st`). This is the only
   test tier that proves runtime behavior, not just the transpiled shape.

5. **Implement the feature** until the tests pass (`cargo test -p plc_cfc`).
   The lit suite needs lit/FileCheck and a stdlib build; if unavailable
   locally, emulate the RUN line by hand (`plc <files> tests/lit/util/*.pli
   --linker=clang -o <exe>`, run it, compare stdout against the CHECKs).

Steps 2 and 3 assume the fixture reaches those stages: put the test where the
behavior lives instead when it doesn't (deserialize errors → `model.rs`,
diagnostics → `validator.rs`).

**Before writing any of these tests, check whether an existing test already
covers the behavior** — if one does, skip yours. Similar-but-not-identical
tests are fine as long as they pin *different functionality* (e.g.
`negated_output` covers a function's negated return pin, and
`negated_output_fan_out` still earns its place by covering the output-argument
route and multiple consumers); a test that only re-exercises the same path
with different names or values is noise.

## Conventions

- Test names match the fixture folder name (`negated_input` fixture →
  `fn negated_input()` in both resolver and transpiler tests).
- ASCII diagrams use `o` for negation bubbles, `(n)` for the IDE's
  evaluation-priority badges, `--->` for wires and `<-->` for in-out pins.
- Result temporaries are named `__<callee>_res_<n>` and typed with placeholder
  types (`__return@<pou>`, `__output@<pou>@<pin>`) resolved by a later pass —
  see `src/placeholder.rs`.
