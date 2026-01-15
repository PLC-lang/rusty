# Architecture

## Overview

RuSTy is a compiler for IEC61131-3 languages. At the moment, ST and CFC ("FBD") are supported.
It utilizes the LLVM compiler infrastructure and contributes a [Structured Text](https://en.wikipedia.org/wiki/Structured_text) frontend that translates Structured Text into LLVM's language independent intermediate representation (IR).
[CFC](../cfc/cfc.md) uses a M2M-transformation and reuses most of the ST frontend for compilation.
The further optimization and native code generation is performed by the existing LLVM infrastructure, namely LLVM's common optimizer and the platform specific backend (see [here](https://www.aosabook.org/en/llvm.html)).

```ignore
    ┌──────────────────┐    ┌───────────────┐    ┌────────────────┐
    │                  │    │               │    │                │
    │      RuSTy       │    │  LLVM Common  │    │  LLVM Backend  │
    │                  ├───►│               ├───►│                │
    │  LLVM Frontend   │    │   Optimizer   │    │   (e.g Clang)  │
    │                  │    │               │    │                │
    └──────────────────┘    └───────────────┘    └────────────────┘
```

So RuSTy consists of the frontend part of the llvm compiler-infrastructure.
This means that this compiler can benefit from llvm's existing compiler-optimizations, as well as all backend target platforms available.

## Rusty Frontend Architecture

Ultimately the goal of a compiler frontend is to translate the original source code into the infrastructure's intermediate representation  (in this case we're talking about [LLVM IR](https://llvm.org/docs/LangRef.html)).
RuSTy treats this task as a compilation step of its own.
While a fully fledged compiler generates machine code as a last step, RuSTy generates LLVM IR assembly code.

## Structured Text

```ignore
      ┌────────┐                                                          ┌────────┐
      │ Source │                                                          │  LLVM  │
      │        │                                                          │   IR   │
      │ Files  │                                                          │        │
      └───┬────┘                                                          └────────┘
          │                                                                    ▲
          ▼                                                                    │
    ┌────────────┐   ┌────────────┐   ┌────────────┐   ┌────────────┐   ┌──────┴─────┐
    │            │   │            │   │            │   │            │   │            │
    │            │   │            │   │            │   │            │   │            │
    │   Parser   ├──►│   Indexer  ├──►│   Linker   ├──►│ Validation ├──►│   Codegen  │
    │            │   │            │   │            │   │            │   │            │
    │            │   │            │   │            │   │            │   │            │
    └────────────┘   └────────────┘   └────────────┘   └────────────┘   └────────────┘
```

## CFC/FBD

```ignore
         ┌────────┐                                                            ┌────────┐
         │ Source │                                                            │  LLVM  │
         │        │                                                            │   IR   │
         │ Files  │                                                            │        │
         └───┬────┘                                                            └────────┘
             │                                                                      ▲
             ▼                                                                      │
    ┌────────────────┐    ┌────────────┐   ┌────────────┐   ┌────────────┐   ┌──────┴─────┐
    │                │    │            │   │            │   │            │   │            │
    │ Model-to-Model │    │            │   │            │   │            │   │            │
    │ Transformation ├───►│   Indexer  ├──►│   Linker   ├──►│ Validation ├──►│   Codegen  │
    │                │    │            │   │            │   │            │   │            │
    │                │    │            │   │            │   │            │   │            │
    └────────────────┘    └────────────┘   └────────────┘   └────────────┘   └────────────┘
```
