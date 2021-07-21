# Using RuSTy

`rustyc` offers a comprehensive help via the -h (--help) option. `rustyc` takes 
one output-format parameter and any number of input-files. The input files can also be
written as [glob patterns](https://en.wikipedia.org/wiki/Glob_(programming)).

`rustyc [OPTIONS] <input-files>... <--ir|--shared|--pic|--static|--bc>`

Note that you can only specify at most one output format. In the case that no output
format switch has been specified, the compiler will select `--static` by default.

Similarily, if you do not specify an output filename via the `-o` or `--output` options,
the output filename will consist of the first input filename, but with an appropriate
file extension depending on the output file format. A minimal invocation looks like this:

`rustyc input.st` ... this will take in the file input.st and compile it into a static object
that will be written to a file named input.o.

More examples:

- `rustyc --ir file1.st file2.st` will compile file1.st and file2.st.
- `rustyc --ir src/*.st` will compile all st files in the src-folder.
- `rustyc --ir "**/*.st"` will compile all st-files in the current folder and its subfolders recursively.

## Compiling a static object

## Compiling a linkable object

## Creating a shared library 

## Linking with an external application

## Writing a main
