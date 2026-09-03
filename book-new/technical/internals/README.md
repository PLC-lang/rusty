# Internals

The pipeline chapters explain the compiler stage by stage. The chapters in this folder cut the other way: each takes a single language construct and follows it through the whole compiler, from its declaration in the source to the LLVM IR it becomes. They answer questions such as "what does a function block look like in memory" or "how is a string literal passed to a function", which no single stage chapter can answer on its own.

| Chapter | The construct, in one sentence |
|---|---|
| [POUs](00-pous.md) | A stateful POU is a struct type plus a function that receives an instance pointer; a function keeps everything on the stack |
| [Structs](01-structs.md) | A struct is a memory layout: members in order, access by address computation, whole-struct operations are full-size copies |
| [Arrays](02-arrays.md) | An array is one flat zero-based block; the bounds survive only as constants in the index arithmetic |
| [Strings](03-strings.md) | A string is a fixed-size character array with one slot for the terminator; every operation is a bounded copy |
| [Enums](04-enums.md) | An enum is its underlying integer plus one global constant per variant |
| [Variable-length arrays](05-variable-length-arrays.md) | A VLA is a struct pairing a pointer to the caller's array with its bounds; offsets are computed at run time |
| [Reference expressions](06-reference-expressions.md) | A reference is a chain of segments; each answers what it is for the resolver and what address it computes for codegen |
| [Initializers](07-initializers.md) | An initial value is folded into static data where it can be, and written again by a constructor or a body statement |
| [Annotated AST](08-annotated-ast.md) | An annotation is a side-table entry per node id that says what an expression is; a hint says what it must become |

Every chapter follows the construct through Declaration, Index, Annotations, Lowering, Codegen, and Validation, and closes with an At a glance table. The topics mirror the architecture design records in `src/tests/adr/`.
