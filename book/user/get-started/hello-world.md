# Hello, World

You write one file, compile it, and run it.


## The program

A program needs a place to start. The compiler starts at the function `main`.

Printing needs a function that writes to the terminal, and `puts` from the C library does that. The `{external}` attribute tells the compiler that the implementation is somewhere else and that the linker will find it.

Write this into `hello_world.st`:

```iecst
{external}
FUNCTION puts: DINT
    VAR_INPUT {ref}
        text: STRING;
    END_VAR
END_FUNCTION

FUNCTION main: DINT
    puts('hello, world!$N');
END_FUNCTION
```

Two details of the language show up already. A string literal stands between single quotation marks, and `$N` inside it is the escape for a new line.


## Compile and run

```bash
plc hello_world.st -o hello_world --linker=cc
./hello_world
```

```
hello, world!
```

The compiler translated the file into an object file and then called `cc` to link that object with the C library into an executable. `-o` names the result. `--linker=cc` names the program that links; use `--linker=clang` on macOS and on Windows, where `cc` usually does not exist.

Leave out `-o` and the executable is named after the input file, here `hello_world.st.out`. Add `-c` and the compiler stops after the object file.


## When something is wrong

Change the body to an assignment that cannot work:

```iecst
FUNCTION main: DINT
    VAR
        x: DINT;
    END_VAR

    x := 'text';
END_FUNCTION
```

```
error[E037]: Invalid assignment: cannot assign 'STRING' to 'DINT'
  ┌─ hello_world.st:6:5
  │
6 │     x := 'text';
  │     ^^^^^^^^^^^ Invalid assignment: cannot assign 'STRING' to 'DINT'

error: Compilation aborted due to critical errors.
Hint: You can use `plc explain <ErrorCode>` for more information
```

Every message has a code. `plc explain E037` prints what the code means, with an example. The compiler also returns a non-zero exit code, so a script sees the failure.

To check the file without producing anything, use `plc --check hello_world.st`.


## What's next

One file on the command line is enough for one program. Real code lives in several files and needs the same options on every build. The [next chapter](first-project.md) puts both into a project.
