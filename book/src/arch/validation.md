# Validation

The validation module implements the semantic validation step of the compiler.
The validator is a hand-written visitor that offers a callback when visiting the single AST-nodes to then perform the different validation tasks.

The validation rules are implemented in dedicated validator-structs:

| Validator           | Responsibilities                                                                                   |
| ------------------- | -------------------------------------------------------------------------------------------------- |
| global_validator    | Semantic rules on the level of declarations as a whole (e.g. name-conflicts)                       |
| pou_validator       | Semantic rules on the level of programs, function- and function-blocks.                            |
| recursive_validator | Semantic rules on the level of recursion (e.g. struct referencing itself)                          |
| stmt_validator      | Semantic rules on the level of statements (e.g. invalid type-casts).                               |
| variable_validator  | Semantic rules on the level of variable declarations (e.g. empty var-blocks, empty structs, etc.). |

## Diagnostics

Problems (semantic or syntactic) are represented as *Diagnostics* [^1].
Diagnostics carry information on the exact location inside the source-string (start- & end-offset), a custom message and a unique error-number to identify the problem.

There are 3 types of *Diagnostics*:

| Diagnostic   | Description |
| ------------ | ----------------- |
| SyntaxError  | A syntax error is a diagnostic that is created by the parser if it discovers a token-stream that does not match the language's grammar. |
| GeneralError | General errors are problems that occured during the compilation process, that cannot be linked to a malformed input (e.g. file-I/O problems, internal LLVM errors, etc.). |
| Improvement  | Problems that may not prevent successful compilation but are still considered a flaw in the source-code. (e.g. use proprietary *POINTER TO* instead of the norm-compliant *REF_TO*). |

[^1]: :(i): The diagnostics are subject to change since they don't elegantly represent the different types of problems (e.g. semantic problems).
