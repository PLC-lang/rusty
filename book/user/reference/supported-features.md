# Supported Features

Every construct that the compiler accepts today, for looking up. Each entry was compiled to verify it, and the [Language Guide](../language/README.md) explains the ones that need more than a line.


## Languages

| Language | State |
|---|---|
| Structured Text (ST) | Supported |
| Continuous Function Chart (CFC / FBD), as PLCopen XML | Supported |
| Sequential Function Chart (SFC), Ladder Diagram (LD), Instruction List (IL) | Not supported |


## Program organization units

| Construct | State |
|---|---|
| `PROGRAM` | Supported |
| `FUNCTION`, with a return type or without | Supported |
| `FUNCTION_BLOCK` | Supported |
| `CLASS` | Supported |
| `INTERFACE` | Supported |
| `METHOD` | Supported |
| `ACTION`, `ACTIONS` | Supported |
| `PROPERTY_GET`, `PROPERTY_SET` | Supported |
| `FB_INIT` as the constructor of a function block | Supported, without parameters and without a return type |


## Object orientation

| Construct | State |
|---|---|
| `EXTENDS` | Supported |
| `IMPLEMENTS` | Supported |
| `OVERRIDE` | Supported |
| `ABSTRACT`, `FINAL` | Supported |
| `PUBLIC`, `PRIVATE`, `PROTECTED`, `INTERNAL` | Supported. A member without a modifier is private |
| `SUPER^` | Supported |
| `THIS^` | Supported in a `FUNCTION_BLOCK` and its methods and actions only, not in a `CLASS` |
| Method calls through an interface variable | Supported |


## Variable blocks

| Block | State |
|---|---|
| `VAR`, `VAR_TEMP` | Supported |
| `VAR_INPUT`, `VAR_INPUT {ref}` | Supported |
| `VAR_OUTPUT`, `VAR_IN_OUT` | Supported |
| `VAR_GLOBAL` | Supported |
| `VAR_CONFIG` | Supported, for binding hardware addresses to instance members |
| `CONSTANT` | Supported on every block |
| `RETAIN`, `NON_RETAIN` | Supported |
| `VAR_EXTERNAL` | Parsed, but without effect. The compiler reports `E106` |


## Data types

| Type | State |
|---|---|
| `BOOL` | Supported |
| `BYTE`, `WORD`, `DWORD`, `LWORD` | Supported |
| `SINT`, `USINT`, `INT`, `UINT`, `DINT`, `UDINT`, `LINT`, `ULINT` | Supported |
| `REAL`, `LREAL` | Supported |
| `TIME`, `DATE`, `TIME_OF_DAY`, `DATE_AND_TIME` | Supported, 32 bits |
| `LTIME`, `LDATE`, `LTIME_OF_DAY`, `LDATE_AND_TIME` | Supported, 64 bits |
| `STRING`, `WSTRING`, with and without a length | Supported |
| `STRUCT` | Supported |
| Enumerations, with and without explicit values | Supported |
| Subranges, for example `INT (0..100)` | Supported |
| Alias types | Supported |
| `ARRAY`, also with more than one dimension | Supported |
| `ARRAY[*]`, the variable-length array | Supported in a parameter. It is always passed by reference |
| `REF_TO`, `POINTER TO` | Supported |
| `REFERENCE TO` with `REF=` | Supported |
| Initial values on every type above | Supported |


## Statements

| Statement | State |
|---|---|
| Assignment | Supported |
| Call, with implicit and with explicit arguments | Supported |
| `IF`, `ELSIF`, `ELSE` | Supported |
| `CASE`, with value lists, ranges, and `ELSE` | Supported |
| `FOR`, with `BY` | Supported |
| `WHILE`, `REPEAT` | Supported |
| `EXIT`, `CONTINUE`, `RETURN` | Supported |


## Expressions

| Element | State |
|---|---|
| `+`, `-`, `*`, `/`, `MOD` | Supported |
| `**` | Supported, and it needs the standard library, because it calls `EXPT` |
| `=`, `<>`, `<`, `>`, `<=`, `>=` | Supported |
| `AND`, `OR`, `XOR`, `NOT` | Supported. They evaluate both sides |
| `AND_THEN`, `OR_ELSE` | Supported. They stop at the first decisive side |
| `&` as another spelling of `AND` | Supported |
| Direct bit access `%X`, `%B`, `%W`, `%D`, with a constant or a variable | Supported |
| Dereference with `^` | Supported |
| Typed literals, for example `DINT#16#2A` and `REAL#1.5` | Supported |
| Literals with base `2#`, `8#`, `16#`, and digit separators | Supported |
| `TIME`, `DATE`, `TIME_OF_DAY`, `DATE_AND_TIME` literals | Supported |
| `NULL`, `TRUE`, `FALSE` | Supported |


## Generics and builtins

| Element | State |
|---|---|
| Generic functions with a nature constraint, for example `<T: ANY_INT>` | Supported |
| `REF`, `ADR` | Supported |
| `MUX`, `SEL`, `MOVE` | Supported |
| `SIZEOF` | Supported. The result is `ULINT` |
| `LOWER_BOUND`, `UPPER_BOUND` | Supported |
| `SHL`, `SHR` | Supported |


## Not supported

| Element | Comment |
|---|---|
| `CONFIGURATION`, `RESOURCE`, `TASK` | The configuration elements of the standard are not part of the language here. Use `VAR_CONFIG` for hardware binding |
| `NAMESPACE` | Not part of the language here |
| Source libraries | Compile the sources with the application instead |
| Header generation for languages other than C | `--header-language` accepts only `c` |
| More than one target per invocation | Run the compiler again for another target |
