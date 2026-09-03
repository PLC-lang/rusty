# Command Line

Every option of `plc`, grouped by purpose. This page is for looking things up; [Building](../building/README.md) explains the options that a normal build uses.

```
plc [OPTIONS] <input-files>...
plc [OPTIONS] <input-files>... <SUBCOMMAND>
```

Most options are global, so they work with a subcommand as well. `plc -h` prints the same list.


## Subcommands

| Subcommand | Purpose |
|---|---|
| `build [plc.json]` | Build the project that the project file describes |
| `check [plc.json]` | Run the compiler up to validation, write nothing |
| `config schema` | Print the JSON schema of the project file |
| `config diagnostics` | Print the severity of every diagnostic code |
| `explain <code>` | Print the explanation of a diagnostic code |
| `generate [plc.json] headers` | Generate C headers for a project |

`config` prints JSON. `generate headers` accepts `--header-language`, where `c` is the implemented language, `--header-output`, and `--header-prefix`.


## Input

| Option | Effect |
|---|---|
| `<input-files>` | Paths or glob patterns. Quote a pattern so the compiler expands it |
| `-i`, `--include <file>` | Read declarations without compiling their bodies. Repeatable |
| `--encoding <name>` | Read the sources with this encoding instead of UTF-8 |

The extension decides how a file is read: `.cfc`, `.fbd`, and `.xml` are graphical sources, `.o`, `.so`, and `.exe` go to the linker, and everything else is Structured Text.


## Output

| Option | Artifact | Default name |
|---|---|---|
| none, `--static` | An executable | `<first input>.out` |
| `-c` | An object file, not linked | `<first input>.o` |
| `--relocatable` | One object file with every unit | `<first input>.o` |
| `--shared` | A shared object | `<first input>.so` |
| `--ir` | LLVM intermediate representation | `<first input>.ll` |
| `--bc` | LLVM bitcode | `<first input>.bc` |
| `--ast` | The syntax tree after parsing | standard output |
| `--ast-lowered` | The syntax tree after every rewrite | standard output |

| Option | Effect |
|---|---|
| `-o`, `--output <file>` | Name of the artifact |
| `--build-location <dir>` | Directory for the intermediate object files |
| `--check` | Produce nothing; report the diagnostics only |


## Code generation

| Option | Effect |
|---|---|
| `-O`, `--optimization <level>` | `none`, `less`, `default` (the default), `aggressive` |
| `--target <triple>` | Build for this LLVM target instead of the host |
| `--sysroot <dir>` | Root for the headers and libraries of that target |
| `-j`, `--threads <n>` | Use `n` threads. `0` or no value means every core |
| `--single-module` | Build one LLVM module for the whole project |
| `--fpic` | Force position-independent code |
| `--fno-pic` | Force code that is not position-independent |
| `--fno-ident` | Do not embed the compiler version in the artifact |


## Linking

| Option | Effect |
|---|---|
| `--linker <command>` | Use this linker, for example `cc` or `clang` |
| `--fuse-ld <name>` | Back end linker for a driver, for example `mold` |
| `--linker-arg <argument>` | Pass one argument to the linker. Repeatable |
| `-l`, `--library <name>` | Link `lib<name>`. Also `-l:libfoo.so.1` and a full path |
| `-L`, `--library-path <dir>` | Add a directory to the library search |
| `--script <file>` | Give the linker a linker script |
| `--no-linker-script` | Use no linker script, which is the default |
| `--nocrt` | Do not link the C runtime startup files |
| `--nolibc` | Do not link the default C libraries |
| `--allow-undefined-symbols` | Allow undefined symbols in a shared object |


## Debug information

| Option | Effect |
|---|---|
| `-g`, `--debug` | Generate source-level debug information |
| `--debug-variables` | Also for global variables |
| `--gdwarf <2..5>` | Debug information with a fixed DWARF version |
| `--gdwarf-variables <2..5>` | The same for global variables |
| `--file-prefix-map OLD=NEW` | Rewrite recorded paths. Repeatable. Alias `--debug-prefix-map` |
| `--debug-compilation-dir <dir>` | Set the compilation directory in the debug information |


## Diagnostics

| Option | Effect |
|---|---|
| `--error-config <file>` | Change the severity of diagnostic codes |
| `--error-format <format>` | `rich` (the default), `clang`, or `none` |
| `--log-level <level>` | `off`, `error`, `warn`, `info`, `debug`, `trace` |
| `-v`, `--verbose` | The same as `--log-level=debug` |


## Other outputs

| Option | Effect |
|---|---|
| `--generate-headers` | Write C headers instead of code |
| `--header-output <dir>` | Directory for the generated headers |
| `--hwmap-file[=<file>]` | Write the map of hardware-bound variables. The `=` is required |
| `--generate-external-constructors` | Also write constructors for `{external}` units |
| `--constructors-only` | Write the generated constructors and no bodies |
| `--online-change` | Emit the type information that a runtime needs to exchange code while it runs |
| `--got-layout-file <file>` | Read and write the table layout that an online change keeps stable |


## Deprecated

| Option | Use instead |
|---|---|
| `--pic` | `--shared --fpic` |
| `--no-pic` | `--shared --fno-pic` |
| `--hardware-conf <file>` | `--hwmap-file=<file>` |
