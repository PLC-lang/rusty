# Using RuSTy

`rustyc` offers a comprehensive help via the -h (--help) option. `rustyc` takes 
one output-format parameter and any number of input-files. The input files can also be
written as [glob patterns](https://en.wikipedia.org/wiki/Glob_(programming)).

`rustyc [OPTIONS] <input-files>... <--ir|--shared|--pic|--static|--bc>`

Note that you can only specify at most one output format. In the case that no output
format switch has been specified, the compiler will select `--static` by default.

Similarly, if you do not specify an output filename via the `-o` or `--output` options,
the output filename will consist of the first input filename, but with an appropriate
file extension depending on the output file format. A minimal invocation looks like this:

`rustyc input.st` ... this will take in the file input.st and compile it into a static object
that will be written to a file named input.o.

More examples:

- `rustyc --ir file1.st file2.st` will compile file1.st and file2.st.
- `rustyc --ir src/*.st` will compile all st files in the src-folder.
- `rustyc --ir "**/*.st"` will compile all st-files in the current folder and its subfolders recursively.

## Example: Building a hello world program
### Writing the code
We want to print something to the terminal, so we're going to declare external functions
for that and link with libc when we're done. This program can also be found at
`examples/hello_world.st` in the source tree of Rusty. 

* `_start` is our entry point to the program, because most linker scripts define it this way. 

* Since we don't have a `crt0` right now, we have to call the `exit()` function by ourselves after we're
done. Otherwise, the program will most likely crash (because it tries to return to a function that never
existed).

```iecst
@EXTERNAL FUNCTION puts : DINT
VAR_INPUT
    text : STRING;
END_VAR
END_FUNCTION

@EXTERNAL FUNCTION exit : DINT
VAR_INPUT
    status : DINT;
END_VAR
END_FUNCTION

FUNCTION _start : DINT
    puts('hello, world!');
    exit(0);
END_FUNCTION
```

### Compiling with rusty
Compiling with rusty is very easy. If you just want to build an object file, then do this:
```bash
rustyc -c hello_world.st -o hello_world.o
```

### Optimization
`rustyc` offers 4 levels of optimization which correspond to the levels established by llvm respectively [clang](https://clang.llvm.org/docs/CommandGuide/clang.html#code-generation-options) (`none` to `aggressive`, respectively `-O0` to `-O3`). 

To use an optimization, the flag `-O` or `--optimization` is required:

- `rustyc -c "**/*.st" -O none`
- `rustyc -c "**/*.st" -O less`
- `rustyc -c "**/*.st" -O default`
- `rustyc -c "**/*.st" -O aggressive`

By default `rustyc` will use `default` which corresponds to clang's `-O2`.

### Linking an executable
Instead, you can also compile this into an executable and run it:
```bash
rustyc hello_world.st -o hello_world -L/path/to/libs -lc
./hello_world
```

Please note that RuSTy will attempt to link the generated object file by default to generate
an executable if you didn't specify something else (option `-c`).
* The `-lc` flag tells the linker it should link against `libc`. Depending on the available libraries on your system,
the linker will prefer a dynamically linked library if available, and revert to a static one otherwise.
* You add library search pathes by providing additional `-L /path/...` options. By default, this will be
the current directory.