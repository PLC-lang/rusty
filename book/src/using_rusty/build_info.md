# Embedded Compiler Version

By default, every artifact RuSTy compiles carries a small piece of metadata that records *which version of `plc` produced it*. The metadata follows the same convention `clang`, `rustc`, and `gcc` use for their own producer identification — it lives in the module's `!llvm.ident` named metadata, which the linker preserves into the ELF `.comment` section of the final binary.

## What gets embedded

A single line of the form:

```
plc version <version> (<commit date>, <short hash>)
```

For example:

```
plc version 0.5.0 (Mon May 11 10:32:29 2026 +0000, be1de6f175)
```

The same string is also reported by `plc --version`, so a deployed `.o`, `.so`, or executable can be matched back to the exact `plc` build that produced it without access to the source tree or build logs.

## Inspecting a binary

Any of these work on Linux:

```bash
readelf -p .comment my_program
llvm-objdump --section=.comment my_program
strings my_program | grep '^plc version'
```

A real link typically shows several producers stacked in `.comment` — one per toolchain involved in the build. For example:

```
plc version 0.5.0 (Mon May 11 10:32:29 2026 +0000, be1de6f175)
Linker: Ubuntu LLD 18.1.3
GCC: (Ubuntu 13.3.0-6ubuntu2~24.04.1) 13.3.0
```

The `plc version …` line is the one this feature controls. The others come from the linker driver and any C runtime objects that got pulled in at link time.

## Strippability

The `.comment` section survives `strip --strip-debug` and `strip --strip-all` by default — it is `PROGBITS` non-`ALLOC` data that GNU `strip` preserves. Removing it explicitly takes:

```bash
objcopy --remove-section=.comment my_program
# or
strip --remove-section=.comment my_program
```

If your deployment pipeline strips the comment section as a deliberate policy, the producer information is removed; without that explicit step it persists.

## Disabling for reproducible builds

The embedded date and hash refer to the `plc` binary itself, so the string changes whenever `plc` is upgraded. Pipelines that need byte-identical artifacts across `plc` upgrades can suppress the emission with:

```bash
plc --fno-ident src/main.st
```

With `--fno-ident`:

- No `!llvm.ident` named metadata is emitted into the module.
- No `plc version …` line appears in `.comment` of the compiled artifact.
- All other producer information (linker, C runtime) is unaffected — those come from tools outside `plc`.

## Implementation detail

The string is injected at module construction time via inkwell's `Module::add_global_metadata` with the named metadata key `"llvm.ident"`. The same mechanism is used by `clang` and `rustc`. Library consumers of `plc` as a Rust crate can override the string by setting `CompileOptions::build_info` to any `Option<String>` they wish — `None` disables emission entirely, matching `--fno-ident`.
