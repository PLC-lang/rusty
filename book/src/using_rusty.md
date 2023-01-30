# Using RuSTy

`rustyc` offers a comprehensive help via the -h (--help) option.
`rustyc` takes one output-format parameter and any number of input-files.
The input files can also be written as [glob patterns](https://en.wikipedia.org/wiki/Glob_(programming)).

`rustyc [OPTIONS] <input-files>... <--ir|--shared|--pic|--static|--bc>`

Note that you can only specify at most one output format.
In the case that no output format switch has been specified, the compiler will select `--static` by default.

Similarly, if you do not specify an output filename via the `-o` or `--output` options,
the output filename will consist of the first input filename, but with an appropriate
file extension depending on the output file format.

A minimal invocation looks like this :
`rustyc input.st` ... this will take in the file input.st and compile it into a static object that will be written to a file named input.o.

More examples :

- `rustyc --ir file1.st file2.st` will compile file1.st and file2.st.
- `rustyc --ir src/*.st` will compile all st files in the src-folder.
- `rustyc --ir "**/*.st"` will compile all st-files in the current folder and its subfolders recursively.

## Example: Building a hello world program

### Writing the code

We want to print something to the terminal, so we're going to declare external functions for that.
This example is available under `examples/hello_world.st` in the main RuSTy repository.

- `main` is our entry point to the program.
- To link the program, we are going to use the system's linker using the `--linker=cc` argument.
- On windows, replace this with --linker=clang as cc is usually not available.

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

If you just want to build an object file, then do this :

```bash
rustyc -c hello_world.st -o hello_world.o
```

### Optimization

`rustyc` offers 4 levels of optimization which correspond to the levels established by llvm respectively [clang](https://clang.llvm.org/docs/CommandGuide/clang.html#code-generation-options) (`none` to `aggressive`, respectively `-O0` to `-O3`).

To use an optimization, the flag `-O` or `--optimization` is required :

- `rustyc -c "**/*.st" -O none`
- `rustyc -c "**/*.st" -O less`
- `rustyc -c "**/*.st" -O default`
- `rustyc -c "**/*.st" -O aggressive`

By default `rustyc` will use `default` which corresponds to clang's `-O2`.

### Linking an executable

Instead, you can also compile this into an executable and run it :

```bash
rustyc hello_world.st -o hello_world --linker=cc
./hello_world
```

Please note that RuSTy will attempt to link the generated object file by default to generate an executable if you didn't specify something else (option `-c`).

- The `--linker=cc` flag tells RuSTy that it should link with the system's compiler driver  instead of the built in linker. This provides support to create executables.
- Additional libraries can be linked using the `-l` flag, additial library pathes can be added with `-L`
- You add library search pathes by providing additional `-L /path/...` options. By default, this will be the current directory.
- The linker will prefer a dynamically linked library if available, and revert to a static one otherwise.

### Building for separate targets

RuSTy supports building for multiple targets by specifing the `--target` and optionally the `--sysroot` command.

- Multiple targets and sysroot can be specified for the compilation simply by adding additional `--target` and `--sysroot` entries.

### --target

To build and compile [structured text](https://en.wikipedia.org/wiki/Structured_text) for the rigth platform we need to specify the `target`.
As `rustyc` is using [LLVM](https://en.wikipedia.org/wiki/LLVM) a target-tripple supported by LLVM needs to be selected.
The default `target` is the host machine's target.
So if a dev container on an `x86_64-docker` is used the target is `x86_64-linux-gnu`.

### --sysroot

`rustyc` use the `sysroot` option for linking purposes.
It is considered to be the root directory for the purpose of locating headers and libraries.

- If a target and sysroot are provided, the output will always be stored in a folder with the target name (e.g. an `x86_64-linux-gnu` target will have the output strored in a folder called `x86_64-linux-gnu`)
- `--sysroot` parameters have to always match target parameters, there can be no `sysroot` without a target.
