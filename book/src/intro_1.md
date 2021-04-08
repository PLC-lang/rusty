# RuSTy

RuSTy is an [ST](https://en.wikipedia.org/wiki/Structured_text) Compiler using LLVM

# Supported Language Concepts
## POUs
- ✔ Program
- ✔ Function
- ✔ FunctionBlock
- ✔ Action

## Datatypes
- ✔ IEC 61131-3 numeric types
- ✔ Strings
- ❌ Wide Strings
- ✔ Struct types
- ✔ Enum types
- ✔ Array data types
- ✔ Alias types
- ✔ Sub-ranges types
- ❌ Date and Time types
- ✔ Sized String types
- ❌ Sized Wide String types
- ✔ Initial values

## Declarations
- ✔ VAR
- ✔ VAR_INPUT
- ✔ VAR_OUTPUT
- ❌ VAR_INOUT

## Statements
- ✔ Assignments
- ✔ Call statements
- ✔ Implicit call arguments
- ✔ Explicit call arguments

## Control Structures
- ✔ IF Statement
- ✔ CASE Statement
- ✔ FOR Loops
- ✔ WHILE Loops
- ✔ REPEAT Loops

## Expressions
- ✔ Arithmetic Operators
- ✔ Relational Operators
- ✔ Logical Operators
- ✔ Bitwise Operators
