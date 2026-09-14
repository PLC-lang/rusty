# Internals

The pipeline chapter explains one stage at a time. This chapter walks you through the same work from the other side, one language construct at a time, from its declaration to the LLVM IR that carries it.

POUs, structs, arrays, strings, enumerations, variable-length arrays, reference expressions, and initial values each get a subchapter that follows the same sequence: declaration, index, annotations, lowering, code generation, and validation. One more subchapter is a reference of every annotation that the resolver can attach to a node, organized by kind.

Start with POUs for the storage and calling rules, then read the subchapter that your question is about.
