# Using RuSTy

> The RuSTy compiler binary is called `plc`

`plc` offers a comprehensive help via the `-h` (`--help`) option.
`plc` takes one output-format parameter and any number of input-files.
The input files can also be written as [glob patterns](https://en.wikipedia.org/wiki/Glob_(programming)).

`plc [OPTIONS] <input-files>... <--ir|--shared|--pic|--static|--bc>`

Note that you can only specify at most one output format.
In the case that no output format switch has been specified, the compiler will select `--static` by default.

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

- The `--linker=cc` flag tells RuSTy that it should link with the system's compiler driver  instead of the built in linker. This provides support to create executables.
- Additional libraries can be linked using the `-l` flag, additional library paths can be added with `-L`
- You add library search paths by providing additional `-L /path/...` options. By default, this will be the current directory.
- The linker will prefer a dynamically linked library if available, and revert to a static one otherwise.

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

## Project-wide initialization

When your code is compiled, the compiler creates a special initialization function with the naming pattern `__init___<projectname>`. This function is responsible for calling all implicit and user-defined initialization code, including all [`FB_INIT`](../pous.md#function_block-initialization) methods.

`<projectname>` is either taken directly from the `plc.json`'s `name` field or derived from the first input file (replacing `.`/`-` with `_`) when compiling without a `plc.json` (e.g. `plc prog.st ...` would yield `__init___prog_st`).

This function is added to the global constructor list, therefore loading the binary will automatically call the `__init___<projectname>` function (and therefore your `<FunctionBlockName>__FB_INIT` function) when an instance of your function block is created, before any other methods are called. This allows you to set default values or perform required setup for your function block.

> **IMPORTANT:** The global constructor initialization is currently only supported for `x86` ISAs. To make sure initialization code runs reliably regardless of target-architecture, ensure your runtime calls this function before starting main task execution.
If you're using the executable without a runtime, you **must** ensure that `__init___<projectname>` is called before any other code runs. Failure to do so will result in uninitialized function blocks and pointers, which can lead to undefined behavior and/or crashes.

Example of ensuring initialization when using C (crt0):

```c
int main() {
    // Call the project initialization function first
    __init___myproject();
    
    // Now it's safe to start cyclic execution
    for (;;) {
        mainProg();
    }
    
    return 0;
}
```

## Native Windows Usage Example

- Ensure [`LLVM 14.0.6`](https://github.com/PLC-lang/llvm-package-windows/releases/tag/v14.0.6) is installed and it's `bin` folder is added to your `PATH` environment variable.

- Download `plc.zip` from the [Windows Build Pipeline](https://github.com/PLC-lang/rusty/actions/workflows/windows.yml).

    - Add it's location to the PATH environment variable. An AppData location is recommended.

- Download `stdlib.lib` from the same pipeline and install it to the same folder.

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

- Proceed with compilation:
    
    ```
    plc ./examples/hello_world.st -c -l iec61131std -l ws2_32 -l ntdll -l userenv -o ./hello_world.o
    clang ./hello_world.o --shared -l iec61131std -l ws2_32 -l ntdll -l userenv -fuse-ld=lld-link "-Wl,/DEF:exports.def" -o ./hello_world.dll
    ```
