# plc_cfc

A CFC/FBD **XML → Structured Text (ST) AST transpiler** for the PLC compiler. It
reads a PLCopen-style `.cfc` document describing one POU whose body is an FBD
network and lowers it directly into a `CompilationUnit` AST.

## Pipeline

`parse_file` (in `lib.rs`) orchestrates `deserialize → resolve → validate →
transpile`:

- `model.rs` — raw XML deserialization types.
- `resolve.rs` — classify FBD elements, link each sink to its source, order by
  `EvaluationPriority`, and record which producer output-pins are consumed.
- `validation.rs` — one `validate_*` function per rule, aggregated by `validate`.
- `transpile.rs` — `transpile_*` AST builders at the top level; a `helper` mod
  holds generic plumbing (parsing an identifier or the interface).
- `lowering.rs` — function-output temporaries: their naming, the placeholder-type
  contract, generation, and the post-index `resolve` of those placeholders.

Transpilation builds the AST **directly** (not via generated ST text): the POU
interface is parsed from the textual declaration plus a synthesized `END_<kind>`
through the ST parser; body statements are AST nodes whose `SourceLocation` is a
`globalId` block location. Leaf expressions come from `parse_expression` on the
identifier fields.

Entry point (matches the driver's fn-pointer type; `linkage` is always
`Internal`, so it's ignored):

```rust
fn parse_file(
    source: &SourceCode,
    _: LinkageType,
    id_provider: IdProvider,
    diagnostician: &mut Diagnostician,
) -> Result<CompilationUnit, Diagnostic>
```

## Integration seam

- `compiler/plc_source/src/lib.rs` — `.cfc` / `.fbd` / `.xml` map to
  `SourceType::Xml`.
- `compiler/plc_driver/src/pipelines.rs` — `SourceType::Xml` dispatches to
  `plc_cfc::parse_file`.
- `compiler/plc_driver/src/pipelines/participant.rs` — `CfcTypeLowerer`, a
  `post_index` participant, resolves function-output placeholder types (see
  Functions). It lives in the driver because `plc_cfc` can't depend on the
  participant trait without a cycle; it re-indexes only when it rewrote
  something, so non-CFC builds pay nothing.

## Deserialization (`model.rs`)

- **quick-xml serde strips namespace prefixes** (v0.38, `serialize` feature):
  match on local names in `#[serde(rename = ...)]` (`Function`, `Network`), never
  `ppx:` / `xsi:`; attributes take `@`. Only names are stripped, not values, so
  an `xsi:type` discriminator like `ppx:DataSource` keeps its prefix.
- **Interleaved children need a `$value` enum.** quick-xml deserializes a struct
  field once, so two `Vec` fields for two element names fail with "duplicate
  field" when the elements interleave. Capture both in one `#[serde(rename =
  "$value")] Vec<Enum>` whose variants are named after the element local names;
  this also preserves document order (`Network::elements()` flattens it).
- **`.cfc` layout.** The root element encodes the POU type (`Function` /
  `FunctionBlock` / `Program`). ST declaration: `AddData > Data >
  textDeclaration > content`. Graphical body: `MainBody > BodyContent > Network`.

## Resolution (`resolve.rs`)

`resolve` classifies each element by its `@type`, links every consumer to the
concrete source feeding it, and orders the resulting statements by
`EvaluationPriority` (sinks, returns, and calls interleave by priority).

- **FBD elements.** `ppx:DataSource` (RHS value), `ppx:DataSink` (LHS target of
  one assignment, carries `EvaluationPriority`), `ppx:Return` (guarded early
  return), `ppx:Block` (a call to `@typeName`), `ppx:Unconnected` (placed but
  unwired → warning, emits nothing). Connector/continuation pairs arrive as
  `CommonObject` (`ppx:Connector` / `ppx:Continuation`), paired by `@label`.
- **Wiring.** A consumer's `Connection @refConnectionPointOutId` points at a
  producer's `ConnectionPointOut @connectionPointOutId` (unique; a producer may
  fan out to many consumers). `trace` walks a wire from a producer pin to its
  source, hopping transparently through continuation → matching connector → its
  input (cycle-guarded), and terminating on a block output pin (a real producer —
  which is what stops a feedback loop from chasing itself).
- **Connectors/continuations are pure routing** — never statements. Errors fire
  only when a *consumed* chain can't reach a source (never emit an incomplete `x
  :=`). An unread connector/continuation is quiet, like an unused source.
- **Identifiers** run through `parse_expression`; only reference/literal (incl.
  parenthesized) are allowed — calls and compound expressions are rejected
  (`E083`).

## Returns

A `ppx:Return` fires when its wired input is true; `AddData` carries `<negated
value=".."/>` (true → the condition is wrapped in `NOT`) plus its
`EvaluationPriority`. Lower to the compiler's native conditional return —
`AstFactory::create_return_statement(Some(cond), ..)` — which codegen turns into
a real branch; do **not** hand-build an `IF` wrapper. A return with no wired
condition is an error (`E085`). The condition *expression* of a connected return
is not validated here — with only the untyped model that is the main pipeline's
job. The AST serializer renders a conditional return as `IF <cond> THEN RETURN;
END_IF`.

## Blocks (stateful calls)

A `ppx:Block` is a call to its `@typeName`, and is both a consumer (its
`InputVariables` / `InOutVariables` pins are sinks) and a producer (its
`OutputVariables` pins are sources). Each pin carries `@parameterName`, an
inversion `@negated` (an *attribute*, unlike a return's `<negated>` element), and
a connection point.

- A block lowers to `Statement::Call` carrying its inputs then in_outs as `param
  := value` associations; it **always** emits (state must advance), even bare.
- Outputs are never `=>`-associated — a consumer reads a block output as a member
  of the instance. Two references come off a block: `instance()` (`@instanceName
  ?? owner`) is the **read base** (`inst.out`); `call_target()` is the **call
  operator** (`inst`, or `inst.act` for an action).
- A function block carries a caller-declared `instanceName`; a program has none
  and is reached by its bare type name (the singleton global). Distinct FB
  instances hold separate state.
- **Actions** ride the same path: a qualified `@typeName` (`owner.action`) splits
  on the last `.` into `owner` (instance fallback) and `action` (suffixed onto
  the call operator), binding the *parent's* members and dispatching to the named
  action — `inst.act(..)` reading `inst.out`, `P.act(..)` reading `P.out`.
- Because a read of a not-yet-called block just observes the persisted member,
  statements stay in raw `EvaluationPriority` order — no reordering, no
  temporaries. An unwired input emits no argument (keeps last cycle); a broken
  input reuses `E082` / `E086`.
- **Negation** wraps the pin's value/read in `NOT` (bitwise on integers). A
  negated in_out is emitted faithfully; the main pipeline rejects it (`E031`) — we
  don't pre-validate, the same stance as return conditions.

## Functions (stateless calls)

A `ppx:Block` with no `@instanceName` *and* an output pin named after the
(unqualified) `@typeName` — the return pin — is a stateless function
(`FbdObject::is_function`). Its outputs don't persist, so each consumed output is
read through a generated persistent temporary `__out_<param>_<globalId>` (a `VAR`
block on the POU): the return via `t := fn(..)`, other outputs via `param =>
temp` — never `inst.member`. Reads see the temp's persisted prior value, so
statements still stay in priority order.

- A function must supply **every** parameter (`E032` rejects a partial list), so
  an unwired input/in_out and an unread output are emitted as *empty arguments*
  (`p := ` / `p => `): an empty output discards into a throwaway temp, an empty
  input takes the callee's declared default, and an empty in_out is rejected
  downstream (`E031`) — the faithful-emit stance again.
- `lowering.rs` owns the temporaries. It types them with placeholders
  (`return@<fn>` / `output@<fn>@<pin>`, since the real type is unknown pre-index)
  and later `resolve`s each to the callee's type via the index
  (`find_return_variable` / `find_member` → `get_type_name`). **A VAR_OUTPUT is
  passed by reference**, so its indexed member type is an auto-deref pointer
  (`__auto_pointer_to_<T>`); resolution unwraps to the inner type
  (`get_inner_name`) or codegen fails synthesizing the pointer's ctor.
- A **void** function (no return pin) is currently indistinguishable from a
  program (TODO in `return_pin`).

**Methods** (`inst.m(..)`, a method with its own params) are not yet supported —
the IDE doesn't emit them.

## Diagnostics

Reused orphaned `plc_xml` codes (all Errors except `E084`):

| Code   | Meaning                                             |
|--------|-----------------------------------------------------|
| `E081` | duplicate connector label                           |
| `E082` | dangling continuation (no matching connector)       |
| `E083` | unsupported CFC expression                          |
| `E084` | unconnected element (Warning)                       |
| `E085` | disconnected return (no wired condition)            |
| `E086` | open connector (no input)                           |

No `plc_xml` codes remain unclaimed — mint fresh codes from here on.

**Block locations render as a note, not a snippet.** A `globalId` block location
has no text range, so the codespan reporter emits a note `= <file>: block <id>`
instead of pointing at a bogus line. Text (ST) diagnostics are unchanged.

## Fixtures & tests

Everything is grouped **by CFC concept** (concept-on-top), so a concept's whole
footprint lives together and new concepts slot in beside existing ones:

```
fixtures/<concept>/
    valid/<name>/       compiles cleanly (warnings allowed)
    invalid/<name>/     rejected with a diagnostic
    reference/          verbatim IDE exports as schema ground-truth
tests/lit/cfc/<concept>/<name>/     end-to-end lit tests
src/{resolve,transpile,validation}.rs → mod tests { mod <concept> { … } }
```

Each fixture folder holds a lean `README.md` (a `What:` line then an
`Illustrated:` ASCII diagram) plus the `.cfc`. Keep the diagram's `RelPosition`s
valid so fixtures import into the IDE unchanged.

Unit tests share one helper, `test_utils::transpile_project(fixture) ->
Result<serialized_ast, rendered_diagnostics>` (concept-first fixture path, e.g.
`"variables/valid/fan_out"`):

- transpile tests: `.unwrap()` → snapshot the POU.
- validation tests: `.unwrap_err()` → snapshot the diagnostics as the console
  renders them; `test_utils::diagnostics(..)` returns them even without an abort
  (warnings on otherwise-valid fixtures).
- resolve tests: `fixture_source` + a local rendering of the resolved network.

All are inline `insta` snapshots. `cargo insta accept` is version-skewed and does
nothing here, so fill snapshots by hand (a clean `cargo test` confirms) or read
the `.pending-snap` written by `INSTA_UPDATE=always cargo test`.

lit tests are the `multi/`-style project form: a `plc.json` listing the `.cfc` +
a `main.st` driver + `printf.pli`, and a `run.test` whose `RUN` builds and runs
it, piping to `%CHECK main.st` so the expected output lives at the call site next
to the `printf`.
