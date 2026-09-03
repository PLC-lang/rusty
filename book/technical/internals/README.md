# Internals

The [pipeline](../pipeline/README.md) chapters explain each stage. This part follows individual constructs across those stages, from declaration to LLVM IR. Start with POUs for the storage and calling rules, then follow the constructs relevant to your question.

| Chapter | The construct, in one sentence |
|---|---|
| [POUs](00-pous.md) | A stateful POU is a struct type plus a function that receives an instance pointer; a function keeps everything on the stack |
| [Structs](01-structs.md) | A struct is a memory layout: members in order, access by address computation, whole-struct operations are full-size copies |
| [Arrays](02-arrays.md) | An array is one flat zero-based block; the bounds survive only as constants in the index arithmetic |
| [Strings](03-strings.md) | A string is a fixed-capacity array with a terminator slot; assignment limits the copy to the target capacity |
| [Enums](04-enums.md) | An enum is its underlying integer plus one global constant per variant |
| [Variable-length arrays](05-variable-length-arrays.md) | A VLA is a struct pairing a pointer to the caller's array with its bounds; offsets are computed at run time |
| [Reference expressions](06-reference-expressions.md) | A reference is a chain of segments; each answers what it is for the resolver and what address it computes for codegen |
| [Initializers](07-initializers.md) | An initial value is folded into static data where it can be, and written again by a constructor or a body statement |
| [Annotated AST](08-annotated-ast.md) | An annotation is a side-table entry per node ID that says what an expression is; a hint says what it must become |

Construct chapters use the same sequence: Declaration, Index, Annotations, Lowering, Codegen, and Validation. Each ends with an At a glance table. The Annotated AST chapter is a reference organized by annotation kind. Examples show the relevant stage's view; later lowering can add members, parameters, and helper types.
