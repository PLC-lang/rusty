# Using RuSTy

`rustyc` offers a comprehensive help via the -h (--help) option. `rustyc` takes 
one output-format parameter and any number of input-files.

The input files can also be written as [glob patterns](https://en.wikipedia.org/wiki/Glob_(programming)).

`rustyc [OPTIONS] <input-files>... <--ir|--shared|--pic|--static|--bc>`

Examples:

- `rustyc --ir file1.st file2.st` will compile file1.st and file2.st.
- `rustyc --ir src/*.st` will compile all st files in the src-folder.
- `rustyc --ir "**/*.st"` will compile all st-files in the current folder and its subfolders recursively.

## Compiling a static object

## Compiling a linkable object

## Creating a shared library 

## Linking with an external application

## Writing a main
