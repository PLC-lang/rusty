# Using RuSTy

> The RuSTy compiler binary is called `plc`

`plc` offers a comprehensive help via the `-h` (`--help`) option.
`plc` takes one output-format parameter and any number of input-files.
The input files can also be written as [glob patterns](https://en.wikipedia.org/wiki/Glob_(programming)).

`plc [OPTIONS] <input-files>... <--ir|--shared|--static|--bc>`

Note that you can only specify at most one output format.
In the case that no output format switch has been specified, the compiler will select `--static` by default.

> **Deprecated:** `--pic` is equivalent to `--shared --fpic`. `--no-pic` is equivalent to `--shared --fno-pic`.
> Both flags will be removed in a future release.

Similarly, if you do not specify an output filename via the `-o` or `--output` options,
the output filename will consist of the first input filename, but with an appropriate
file extension depending on the output file format.

A minimal invocation looks like this:
`plc input.st` this will take in the file `input.st` and compile it into a static object that will be written to a file named `input.o`.

More examples:
- `plc --ir file1.st file2.st` will compile file1.st and file2.st.
- `plc --ir file1.cfc file2.st` will compile file1.cfc and file2.st.
- `plc --ir src/*.st` will compile all ST files in the src-folder.
- `plc --ir "**/*.st"` will compile all ST-files in the current folder and its subfolders recursively.

## Example: Building a hello world program

### Writing the code

We want to print something to the terminal, so we're going to declare external functions for that.
This example is available under `examples/hello_world.st` in the main RuSTy repository.

- `main` is our entry point to the program.
- To link the program, we are going to use the system's linker using the `--linker=cc` argument.
- On Windows and MacOS, replace this with `--linker=clang` as cc is usually not available.

```iecst
{external}
FUNCTION puts : DINT
VAR_INPUT {ref}
    text : STRING;
END_VAR
END_FUNCTION

FUNCTION main : DINT
    puts('hello, world!$N');
END_FUNCTION
```

### Compiling with RuSTy

The RuSTy command line interface is similar to that of other compilers.

If you just want to build an object file, then do this:

```bash
plc -c hello_world.st -o hello_world.o
```

### Optimization

`plc` offers 4 levels of optimization which correspond to the levels established by llvm respectively [clang](https://clang.llvm.org/docs/CommandGuide/clang.html#code-generation-options) (`none` to `aggressive`, respectively `-O0` to `-O3`).

To use an optimization, the flag `-O` or `--optimization` is required:

- `plc -c "**/*.st" -O none`
- `plc -c "**/*.st" -O less`
- `plc -c "**/*.st" -O default`
- `plc -c "**/*.st" -O aggressive`

By default `plc` will use `default` which corresponds to clang's `-O2`.

### Linking an executable

Instead, you can also compile this into an executable and run it:

```bash
plc hello_world.st -o hello_world --linker=cc
./hello_world
```

Please note that RuSTy will attempt to link the generated object file by default to generate an executable if you didn't specify something else (option `-c`).

- The `--linker=cc` flag tells RuSTy that it should link with the system's compiler driver instead of the built in linker. This provides support to create executables.
- When no `--linker` is specified, RuSTy resolves the internal linker in this order: `cc` → `clang` → `ld.lld` → `ld`. The first linker that is found and supports the target is used. Compiler drivers (`cc`, `clang`) are preferred for their correct platform setup (sysroot, CRT files, etc.).
- You can override the driver backend linker with `--fuse-ld=<name>` (for example `--fuse-ld=mold`).
- Additional libraries can be linked using the `-l` flag, additional library paths can be added with `-L`.
- You can pass raw linker arguments using `--linker-arg=<arg>` (repeatable). With compiler drivers these are forwarded via `-Xlinker`.
- You add library search paths by providing additional `-L /path/...` options. By default, this will be the current directory.
- The linker will prefer a dynamically linked library if available, and revert to a static one otherwise.
- For executable links with compiler drivers, startup/runtime defaults can be controlled explicitly with `--nocrt` and `--nolibc`.
- `-l` also supports exact filenames (`-l:libfoo.so.1`) and direct full paths (`-l/path/to/libfoo.so.1`).

### Relocation model (PIC / no-PIC)

By default, RuSTy generates position-independent code (PIC) when building shared libraries, and uses the
platform default relocation mode for object files and executables.

You can override this with:

- `--fpic` — Force PIC code generation. Required for shared libraries on most platforms.
- `--fno-pic` — Force non-PIC code generation. Produces code without the overhead of PIC
  relocations, which can be useful for bare-metal or static-only targets.

These flags apply to **all** output modes:

| Command | Effect |
|---|---|
| `plc -c --fpic file.st` | Compile to PIC object file |
| `plc -c --fno-pic file.st` | Compile to non-PIC object file |
| `plc --shared --fpic file.st` | Build PIC shared library (default for `--shared`) |
| `plc --shared --fno-pic file.st` | Build non-PIC shared library (may fail on some targets) |
| `plc --fno-pic file.st --linker=cc` | Build non-PIE executable (forwards `-no-pie` to linker) |

> **Note:** On x86_64, LLVM's code generator produces position-independent code by default regardless
> of relocation mode (the small code model uses RIP-relative addressing). The `--fpic` / `--fno-pic`
> flags are passed through to LLVM and **will** produce different code on targets where the distinction
> matters (e.g. 32-bit x86, certain ARM configurations).
>
> On x86_64 Linux, the primary observable effect is at **link time**: most toolchains default to PIE
> executables. When `--fno-pic` is used for executable output, RuSTy automatically passes `-no-pie`
> to the linker. Building a non-PIC shared library will fail on targets that require PIC for shared
> objects — this matches the behavior of `gcc`/`clang`.

> `--fpic` and `--fno-pic` are mutually exclusive and cannot be combined.

### Building for separate targets

RuSTy supports building for multiple targets by specifing the `--target` and optionally the `--sysroot` command.

- Multiple targets and sysroot can be specified for the compilation simply by adding additional `--target` and `--sysroot` entries.

### --target

To build and compile [structured text](https://en.wikipedia.org/wiki/Structured_text) for the rigth platform we need to specify the `target`.
As RuSTy is using [LLVM](https://en.wikipedia.org/wiki/LLVM) a target-tripple supported by LLVM needs to be selected.
The default `target` is the host machine's target.
So if a dev container on an `x86_64-docker` is used the target is `x86_64-linux-gnu`.

### --sysroot

`plc` use the `sysroot` option for linking purposes.
It is considered to be the root directory for the purpose of locating headers and libraries.

- If a target and sysroot are provided, the output will always be stored in a folder with the target name (e.g. an `x86_64-linux-gnu` target will have the output strored in a folder called `x86_64-linux-gnu`)
- `--sysroot` parameters have to always match target parameters, there can be no `sysroot` without a target.

## Parallel Compilation

By default, `plc` uses parallel compilation.

This option can be controlled with the `-j` or `--threads` flag. A value above `0` will indicate the number of threads to use for the compilation
Leaving the value unset, setting it to `0` or simply specifying `-j` sets the value to the maximum threads that can run for the current machine.
This is determined by the underlying parallelisation library [Rayon](https://crates.io/crates/rayon)

### Single module Compilation

With the introducton of parallel compilation, every unit is compiled into an object file independently and then linked together in a single module.
This behaviour might not always be desired and can be disabled using the `--single-module` flag.

> Note that the single module flag is currently much slower to produce as it requires first generating all modules and then merging them together.

## Configuration Options

`plc` supports different configuration options, these can be printed using the `config` subcommand

### `config schema`

Outputs the json schema used for the validation of the `plc.json` file

### `config diagnostics`

Ouputs a json file with the default error severity configuration for the project.
See [Error Configuration](./error_configuration.md) for more information.

### `config profile`

Outputs the resolved compatibility profile as JSON or TOML.
See [Compatibility Profiles](./compatibility_profiles.md) for more information.

## Project-wide initialization

RuSTy uses constructor functions for initialization. The compiler generates:

- **Type/POU constructors**: `<TypeName>__ctor` for structs and POUs
- **Global constructor**: `__unit_<name>__ctor` to initialize all globals and invoke type/POU constructors as needed

These constructors are registered in the global constructor list, so they run automatically when the binary loads. This includes calling [`FB_INIT`](../pous.md#function_block-initialization) where appropriate.

Manual calls are not required on any architecture.

## Native Windows Usage Example

- Ensure [`LLVM 21.1.7`](https://github.com/PLC-lang/llvm-package-windows/releases/tag/v21.1.7) is installed and it's `bin` folder is added to your `PATH` environment variable.

- Install `plc.zip` from the [Windows Build Pipeline](https://github.com/PLC-lang/rusty/actions/workflows/windows.yml).

    - Add it's location to the PATH environment variable. An AppData location is recommended.

- Install `stdlib.lib` from the same pipeline into the same folder.

- Install the `Windows SDK` and `MSVC`. You can use the Visual Studio Installer to do this or install them as standalone packages. 

- Create a `LIB` environment variable containing paths to `iec61131std.lib`, `ws2_32.lib`, `ntdll.lib`, `userenv.lib`, `libcmt.lib`, `oldnames.lib` and `libucrt.lib`.

    - Your environment variable should look something like this:

    ```
    C:/Users/<USERNAME HERE>/AppData/Local/rustycompiler;
    C:\Program Files (x86)\Windows Kits\10\Lib\10.0.26100.0\um\x64;
    C:\Program Files (x86)\Windows Kits\10\Lib\10.0.26100.0\ucrt\x64;
    C:\Program Files\Microsoft Visual Studio\2022\Community\VC\Tools\MSVC\14.43.34808\lib\x64;
    ```

- Restart your terminals to refresh the environment.

- Create an `exports.def` file in preparation `clang` usage.

```
EXPORTS
    main

```

This example allows linking with Rusty's Standard Library.
    
- Proceed with compilation:
    
    ```
    plc ./examples/hello_world.st -c -l iec61131std -l ws2_32 -l ntdll -l userenv -o ./hello_world.o
    clang ./hello_world.o --shared -l iec61131std -l ws2_32 -l ntdll -l userenv -fuse-ld=lld-link "-Wl,/DEF:exports.def" -o ./hello_world.dll
    ```
