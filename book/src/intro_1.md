# RuSTy

RuSTy is a [structured text (ST)](https://en.wikipedia.org/wiki/Structured_text)
compiler written in Rust and based on the
LLVM compiler backend. We use the [_logos_](https://crates.io/crates/logos/0.8.0)
crate library to perform lexical analysis before the custom parser runs. RuSTy
puts out static or shared objects as well as LLVM IR or bitcode by the flip of
a command line flag. We are aiming towards an open-source industry-grade ST compiler
supporting at least the features in 2nd edition IEC 61131 standard. 

You might also want to refer to the [API documentation](https://ghaith.github.io/rusty/api/rusty/).

## Supported Language Concepts
### POUs
- ✔ Program
- ✔ Function
- ✔ FunctionBlock
- ✔ Action

### Datatypes
- ✔ IEC 61131-3 numeric types
- ✔ Strings
- ✔ Wide Strings
- ✔ Struct types
- ✔ Enum types
- ✔ Array data types
- ✔ Alias types
- ✔ Sub-ranges types
- ✔ Date and Time types
- ✔ Sized String types
- ✔ Sized Wide String types
- ✔ Initial values

### Declarations
- ✔ VAR
- ✔ VAR_INPUT
- ✔ VAR_OUTPUT
- ✔ VAR_IN_OUT

### Statements
- ✔ Assignments
- ✔ Call statements
- ✔ Implicit call arguments
- ✔ Explicit call arguments
- ✔ EXIT, CONTINUE statements

### Control Structures
- ✔ IF Statement
- ✔ CASE Statement
- ✔ FOR Loops
- ✔ WHILE Loops
- ✔ REPEAT Loops
- ✔ RETURN statement

### Expressions
- ✔ Arithmetic Operators
- ✔ Relational Operators
- ✔ Logical Operators
- ✔ Bitwise Operators
