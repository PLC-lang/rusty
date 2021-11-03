# RuSTy

Structured text compiler written in Rust

[![Rust Build](https://github.com/PLC-lang/ruSTy/workflows/Rust%20on%20Docker/badge.svg)](https://github.com/PLC-lang/ruSTy/actions)
[![codecov](https://codecov.io/gh/PLC-lang/rusty/branch/master/graph/badge.svg?token=7ZZ5XZYE9V)](https://codecov.io/gh/PLC-lang/rusty)
[![Lines of Code](https://tokei.rs/b1/github/PLC-lang/rusty)](https://github.com/XAMPPRocky/tokei)


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

For build, installation and usage reference as well as supported language constructs, check out
the [documentation](https://plc-lang.github.io/rusty/).

## About RuSTy
RuSTy is a structured text (ST) compiler written in Rust and based on the
LLVM compiler backend. We use the [_logos_](https://crates.io/crates/logos/0.8.0)
crate library to perform lexical analysis before the custom parser runs. RuSTy
puts out static or shared objects as well as LLVM IR or bitcode by the flip of
a command line flag. We are aiming towards an open-source industry-grade ST compiler
supporting at least the features in 2nd edition IEC 61131 standard. 

## Getting started
Start by [installing Rust](https://www.rust-lang.org/tools/install) and git on your machine.
Then, clone this repository and run `cargo build` to compile the code. Check out the
[documentation](https://plc-lang.github.io/rusty/) for further reference.
