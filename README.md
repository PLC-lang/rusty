# RuSTy

Structured text compiler written in Rust

[![Rust Build](https://github.com/ghaith/ruSTy/workflows/Rust%20on%20Docker/badge.svg)](https://github.com/ghaith/ruSTy/actions)

# Supported Language Concepts
## POUs
- :heavy_check_mark: Program
- :heavy_check_mark: Function
- :heavy_check_mark: FunctionBlock
- :x: Action

## Datatypes
- :x: IEC 61131-3 types
- :x: Struct types
- :x: Enum types
- :x: Alias types
- :x: Sub-ranges types
- :x: Array data types

## Declarations
- :heavy_check_mark: VAR
- :heavy_check_mark: VAR_INPUT
- :x: VAR_OUTPUT
- :x: VAR_INOUT

## Statements
- :heavy_check_mark: Assignments
- :heavy_check_mark: Call statements
- :x: Implicit call arguments
- :x: Explicit call arguments

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
