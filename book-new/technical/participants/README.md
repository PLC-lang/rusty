# Participants

Participants are the plug-ins that rewrite the tree between the pipeline stages; the [Driver](../pipeline/00-driver.md) chapter introduces the model and its hooks. This part has one chapter per participant, in the order in which the driver registers them; a later participant sees the rewrites of every earlier one. The hook implementations are in `compiler/plc_driver/src/pipelines/participant.rs`; the Where column names the lowering logic they call.

| # | Chapter | Hooks | Rewrites | Why | Where |
|---|---|---|---|---|---|
| 1 | [CFC](00-cfc.md) | `post_index` | Graphical CFC sources into Structured Text bodies, and the types of their temporaries | A graph has no textual body; the later stages know only statements | `compiler/plc_cfc` |
| 2 | [Loop desugar](01-loop-desugar.md) | `pre_index` | `WHILE`, `REPEAT`, `FOR` into `WHILE TRUE` with explicit checks | Codegen knows one loop shape | `compiler/plc_lowering/src/loops.rs` |
| 3 | [Property](02-property.md) | `pre_index`, `post_annotate` | Accessor blocks into `__get_x` and `__set_x` methods, property uses into calls | Index, resolver, and codegen have no notion of a property | `src/lowering/property.rs` |
| 4 | [Polymorphism](03-polymorphism.md) | `post_index`, `post_annotate` | Method tables per function block, method calls into indirect calls, interface variables into fat pointers | Virtual dispatch needs tables and indirect calls | `src/lowering/polymorphism/` |
| 5 | [Control statements](04-control-statements.md) | `pre_index` | `ELSIF` into `ELSE` with a nested `IF` | Each condition needs its own statement list for moved-out calls | `compiler/plc_lowering/src/control_statement.rs` |
| 6 | [Reference to return](05-reference-to-return.md) | `pre_index`, `post_annotate` | `REFERENCE TO` returns into a leading reference parameter and caller-owned storage | Codegen returns by value and `REF=` accepts no call | `compiler/plc_lowering/src/reference_to_return.rs` |
| 7 | [Init](06-init.md) | `post_annotate` | Initializers into `__ctor` constructor functions, one per type and one per unit | Not every initial value is a constant | `compiler/plc_lowering/src/initializer.rs` |
| 8 | [Retain](07-retain.md) | `post_index` | Retained program variables into globals with alias pointers behind | A section is a property of a global symbol | `compiler/plc_lowering/src/retain.rs` |
| 9 | [Generic](08-generic.md) | `post_annotate` | Generic calls into calls of concrete implementations, declared when missing | Codegen cannot call a template | `src/lowering/generics.rs` |
| 10 | [Aggregate return](09-aggregate-return.md) | `post_annotate` | Aggregate returns into a `VAR_IN_OUT` result parameter, calls into statements with temporaries | Large results are passed by pointer | `src/lowering/calls.rs` |
| 11 | [Inheritance](10-inheritance.md) | `pre_index`, `post_annotate` | `EXTENDS` into an embedded `__Base` member and accesses through it | Codegen has no notion of inheritance | `compiler/plc_lowering/src/inheritance.rs` |
| 12 | [Array](11-array.md) | `post_annotate` | Array literal assignments with non-constant elements into element assignments | Codegen stores literals only as constants | `compiler/plc_lowering/src/array_lowering.rs` |

Within one hook the participants run in registration order, and a participant that rewrites the tree at `post_index` or `post_annotate` sends the project back through annotate (and through index, when it changed declarations) before the next one runs. Each chapter has an Interactions section that names the participants it depends on.
