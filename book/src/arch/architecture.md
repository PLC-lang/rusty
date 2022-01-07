# Architecture

## Overview

Rusty is a Compiler for Structured Text. It utilizes the llvm compiler infrastructurue and contributes a [Structured Text](https://en.wikipedia.org/wiki/Structured_text) Frontend that translates Structured Text into llvm's language independent intermediate representation (IR). The Further optimization and native code generation is performed by the existing LLVM infrastructure, namely llvm's common optimizer and the platform specific backend (see [here](https://www.aosabook.org/en/llvm.html)). 


```ignore
    ┌──────────────────┐    ┌───────────────┐    ┌────────────────┐
    │                  │    │               │    │                │
    │      Rusty       │    │  LLVM Common  │    │  LLVM Backend  │
    │                  ├───►│               ├───►│                │
    │  LLVM Frontend   │    │   Optimizer   │    │   (e.g Clang)  │
    │                  │    │               │    │                │
    └──────────────────┘    └───────────────┘    └────────────────┘
```

So Rusty consists of the frontend part of the llvm compiler-infrastructure. This means that this compiler can benefit from llvm's existing compiler-optimizations, as well as all backend target platforms available.

## Rusty Frontend Architecture

Ultimately the goal of a compiler frontend, is to translate the original source code into the infrastructure's intermediate representation  (in this case we're talking about [LLVM IR](https://llvm.org/docs/LangRef.html)). Rusty treats this task as a compilation step of its own. While a fully fledged compiler generates machine code as a last step, rusty generates LLVM IR assembly code.

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
