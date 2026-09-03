# Debug Information

The compiler writes DWARF debug information into the artifact, so that a debugger can show source lines, variables, and types.


## Generate debug information

| Option | Effect |
|---|---|
| `-g`, `--debug` | Source-level debug information |
| `--debug-variables` | Debug information for global variables as well |
| `--gdwarf <2..5>` | Source-level debug information with a fixed DWARF version |
| `--gdwarf-variables <2..5>` | The same for global variables |

```bash
plc -g main.st -o app --linker=cc
```

Use a fixed version when the debugger or the runtime on the target accepts one version only.


## Why paths matter

The debug information stores the path of every source file. For local work this is fine. For a shipped binary and for remote debugging you usually want two properties:

- no local paths of the build machine inside the artifact
- stable paths that an IDE or `gdb` can map to a local checkout

Without any option, the compiler writes compile units relative to the compilation directory where it can, and the compilation directory itself can stay absolute. This follows what `clang` does.


## Rewrite the paths

`--file-prefix-map OLD=NEW` rewrites every path that starts with `OLD`, so that it starts with `NEW`. Repeat the option for more mappings.

```bash
plc -g \
  --file-prefix-map /home/alice/work/MyApp=/src/MyApp \
  --file-prefix-map /home/alice/work/MyApp/build=/build/MyApp \
  app.st
```

- `OLD` resolves against the current directory, and is canonicalized where possible.
- `NEW` is used as written, and normalized for the platform.
- When two mappings match, the longest one wins.

`--debug-prefix-map` is another name for the same option. It exists so that build systems can use the same spelling as with GCC and Clang.

`--debug-compilation-dir <dir>` sets the compilation directory of the debug information.

```bash
plc -g \
  --file-prefix-map /home/alice/work/MyApp=/src/MyApp \
  --debug-compilation-dir /src/MyApp \
  app.st
```

The two options are independent. The prefix map rewrites the recorded source file paths, the compilation directory sets one field of the compile unit. With both, the debug information can hold two records for one file, one for the source and one for the compile unit:

```llvm
!2  = !DIFile(filename: "main.st", directory: "/SOURCE_ROOT/...")
!10 = !DIFile(filename: "/SOURCE_ROOT/.../main.st", directory: "/BUILD_ROOT")
```

This is how `clang` behaves with `-ffile-prefix-map` and `-fdebug-compilation-dir` together. A tool resolves whichever record it reads. For one canonical path, choose a prefix map that puts the source below the compilation directory.


## A convention for shipped builds

Use virtual roots that exist on no machine:

- source root `/src/<Product>`
- build root `/build/<Product>`

```bash
plc -g \
  --file-prefix-map /real/source/root=/src/MyApp \
  --file-prefix-map /real/build/root=/build/MyApp \
  --debug-compilation-dir /src/MyApp \
  ...
```

The same options work with the `build` subcommand:

```bash
plc build plc.json -g \
  --file-prefix-map /real/source/root=/src/MyApp \
  --debug-compilation-dir /src/MyApp
```

> [!NOTE]
> On Windows, prefer virtual roots to paths with a drive letter. The drive of the build machine and the drive of the developer machine are often different, and a virtual root keeps the mapping stable.


## Sources above the working directory

A build that runs in a subdirectory and compiles a file above it, for example `../main.st`, keeps that form:

```bash
cd examples/test
plc -g ../main.st \
  --file-prefix-map "$(pwd)=/root" \
  --debug-compilation-dir "$(pwd)"
```

The compile unit then holds the directory `/root` and the name `../main.st`, and no local path of the build machine.


## Map the paths in the debugger

```gdb
set substitute-path /src/MyApp /home/bob/dev/MyApp
set substitute-path /build/MyApp /home/bob/dev/MyApp/build
```

For a remote session:

```gdb
file /path/to/local/unstripped/binary
set substitute-path /src/MyApp /home/bob/dev/MyApp
target remote <host>:<port>
```


## Projects with C and Structured Text

If the project also builds C or C++ with `clang`, use the same virtual roots on both sides:

```text
-ffile-prefix-map=<real-source-root>=/src/MyApp
-ffile-prefix-map=<real-build-root>=/build/MyApp
```

One convention for both compilers keeps the source lookup consistent in Eclipse, in `gdb`, and in every debug adapter.


## What's next

That is the toolchain. The next part connects a project to code that is not written in Structured Text: [interoperability](../interop/README.md).
