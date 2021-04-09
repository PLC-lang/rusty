# RuSTy

Structured text compiler written in Rust

[![Rust Build](https://github.com/ghaith/ruSTy/workflows/Rust%20on%20Docker/badge.svg)](https://github.com/ghaith/ruSTy/actions)


```
                    IR-CodeGen
     Project        Programs             Global           IEC61131 Num and String types
     start          Expressions          Variables 
       |              |                    |                   |
       |              |                    |                   |
----I-------------I-------------I-------------I-------------I-------------I-------------I----
 nov-'19       jan-'20       mar-'20       may-'20      june-'20       dec-'20       jan-'21  
----I-------------I-------------I-------------I-------------I-------------I-------------I----
                  |                   |          |                        |
                  |                   |          |                        |
                Parsing            Control       Call Stmts           Array Access
                POUs               Structures    for all
                Statements                       POUs
                Expressions
```




# Supported Language Concepts
## POUs
- ✔ Program
- ✔ Function
- ✔ FunctionBlock
- ✔ Action

## Datatypes
- ✔ IEC 61131-3 numeric types
- ✔ Strings
- ✔ Struct types
- ✔ Enum types
- ✔ Array data types
- ✔ Alias types
- ✔ Sub-ranges types
- ✔ Sized String types
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
