# Internals

After these chapters you can follow one language construct from its declaration to the LLVM IR that carries it. The pipeline part explains one stage at a time; these chapters take the same work from the other side, one construct at a time.

POUs, structs, arrays, strings, enumerations, variable-length arrays, reference expressions, and initial values each get a subchapter that follows the same sequence: declaration, index, annotations, lowering, code generation, and validation. One more subchapter is a reference of every annotation that the resolver can attach to a node, organized by kind.

Start with POUs for the storage and calling rules, then read the subchapter that your question is about.
