# Internals

The pipeline chapters explain the compiler stage by stage. The chapters in this folder cut the other way: each one takes a single language construct and follows it through the whole compiler, from its declaration in the source to the LLVM IR it becomes. They answer questions such as "what does a function block look like in memory" or "how is a string literal passed to a function", which no single stage chapter can answer on its own.

Each chapter has the same outline: what the construct is in Structured Text, how the parser represents it, what the index records for it, how the resolver types it, how the lowering participants rewrite it, and what codegen emits for it.

The topics mirror the architecture design records in the compiler's test suite (`src/tests/adr/`), which pin the same behavior down as snapshot tests.
