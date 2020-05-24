# RuSTy

Structured text compiler written in Rust

[![Rust Build](https://github.com/ghaith/ruSTy/workflows/Rust%20on%20Docker/badge.svg)](https://github.com/ghaith/ruSTy/actions)


```
                    IR-CodeGen
     Project        Programs             Global
     start          Expressions          Variables
       |              |                    |
       |              |                    |
----I-------------I-------------I-------------I-------------I-------------I-------------I----
 nov-'19       jan-'20       mar-'20       may-'20      june-'20       sep-'20       nov-'20  
----I-------------I-------------I-------------I-------------I-------------I-------------I----
                  |                   |          |
                  |                   |          |
                Parsing            Control       Call Stmts
                POUs               Structures    for all
                Statements                       POUs
                Expressions
```




# Supported Language Concepts
## POUs
- :heavy_check_mark: Program
- :heavy_check_mark: Function
- :heavy_check_mark: FunctionBlock
- :x: Action

## Datatypes
- :x: IEC 61131-3 types
- :heavy_check_mark: Struct types
- :heavy_check_mark: Enum types
- :x: Alias types
- :x: Sub-ranges types
- :x: Array data types
- :x: Initial values

## Declarations
- :heavy_check_mark: VAR
- :heavy_check_mark: VAR_INPUT
- :x: VAR_OUTPUT
- :x: VAR_INOUT

## Statements
- :heavy_check_mark: Assignments
- :heavy_check_mark: Call statements
- :heavy_check_mark: Implicit call arguments
- :heavy_check_mark: Explicit call arguments

## Control Structures
- :heavy_check_mark: IF Statement
- :x: CASE Statement
- :heavy_check_mark: FOR Loops
- :heavy_check_mark: WHILE Loops
- :heavy_check_mark: REPEAT Loops

## Expressions
- :heavy_check_mark: Arithmetic Operators
- :heavy_check_mark: Relational Operators
- :heavy_check_mark: Logical Operators
- :heavy_check_mark: Bitwise Operators
