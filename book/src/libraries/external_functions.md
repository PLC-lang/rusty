# External Functions

A `POU` (`PROGRAM`, `FUNCTION`, `FUNCTION_BLOCK`) can be marked as external,
which will cause the compiler to ignore its implementation.

```iecst
{external}
FUNCTION log : DINT
VAR_IN_OUT
  message : STRING[1024];
END_VAR
VAR_INPUT
  type : (Err,Warn,Info) := Info;
END_VAR
END_FUNCTION
```

At compilation time, the function `log` will be defined as an externally available function, and can be called from `ST` code.

> Note: At linking time, a `log` function with a compatible signature must be available on the system.

## Generic Functions

A `FUNCTION` can be made generic by declaring one or more type parameters with a
type-nature constraint, e.g. `<T: ANY_INT>`. The type parameters may be used as the type
of inputs, outputs and the return value:

```iecst
FUNCTION MAX <T: ANY_ELEMENTARY> : T
VAR_INPUT
    in1 : T;
    in2 : T;
END_VAR
END_FUNCTION
```

A generic function is never called directly. Every call is resolved to a concrete
*monomorphization* whose name is the generic function's name followed by `__<TYPE>` for
the resolved type argument. For example, `MAX(aDint, bDint)` resolves to `MAX__DINT`.

The compiler does not synthesize a monomorphization's body; the implementation is provided
by one of:

- an ordinary `FUNCTION` with the mangled name (an ST body),
- an implementation linked in from elsewhere (e.g. in C or Rust), or
- a compiler builtin.

When no ST body or builtin defines the mangled name, the compiler emits an `{external}`
declaration for it and the symbol is resolved at link time; if nothing provides it there,
the build fails with an undefined symbol. This applies regardless of the return type,
including aggregate returns such as `STRING`, `WSTRING`, arrays and structs.

A type argument that does not satisfy the generic's nature constraint (e.g. a `REAL` for a
`<T: ANY_INT>` parameter) is rejected at compile time with `E062`.

For example, the standard `TO_STRING <T: ANY> : STRING` conversion provides an
implementation for every supported source type (`TO_STRING__DINT`, `TO_STRING__REAL`, …).
Because `ANY` accepts every type, calling `TO_STRING` with a type that has no implementation
compiles and then fails at link time with an undefined symbol:

```iecst
TYPE Point : STRUCT x, y : DINT; END_STRUCT END_TYPE

FUNCTION main : DINT
VAR s : STRING; p : Point; END_VAR
    s := TO_STRING(p);   // undefined symbol at link time: TO_STRING__Point
END_FUNCTION
```

## Calling C functions

`ST` code can call into foreign functions natively.
To achieve this, the called function must be defined in a `C` compatible API, e.g. `extern "C"` blocks.

The interface of the function has to:

- either be included with the `-i` flag
- or be declared in `ST` using the `{external}` keyword

When including multiple header files/function interfaces, the `-i` flag must precede each individual file, e.g. `-i file1.st -i file2.st -i file3.st`. Alternatively, when including an entire folder with `-i '/liblocation/*.st'`, the path must be put in quotes, otherwise the command-line might parse the arguments in a way that is incompatible (i.e. does not precede each file with `-i`).

### Example

Given a `min` function defined in `C` as follows:

```C
int min(int a, int b) {
//...
}
```

an interface of that function in `ST` can be defined as:

```iecst
{external}
FUNCTION min : DINT
VAR_INPUT
  a : DINT;
  b : DINT;
END_VAR
END_FUNCTION
```

### Variadic arguments

Some foreign functions, especially ones defined in `C`, could be [variadic functions](https://en.cppreference.com/w/c/variadic).

These functions are usually defined with the last parameter `...`, and signify that a function can be called with unlimited parameters.

An example of a variadic function is `printf`.

Calling a variadic function is supported in `ST`. To mark an external function as variadic, you can add a parameter of type `...` to the `VAR_INPUT` block.

#### Variadic function example

Given the `printf` function defined as:

```C
int printf( const char *restrict format, ... );
```

the `ST` interface can be defined as:

```iecst
{external}
FUNCTION printf : DINT
VAR_INPUT {ref}
  format : STRING;
END_VAR
VAR_INPUT
  args : ...;
END_VAR
END_FUNCTION
```

#### Runnable example

With the `printf` function available on the system, there is no need to declare
the C function.

An `ST` program called `ExternalFunctions.st` with the following code can be declared:

```iecst
(*ExternalFunctions.st*)

(**
 * The printf function's interface, marked as external since
 * it is defined directly along other ST functions
 *)
{external}
FUNCTION printf : DINT
VAR_INPUT {ref}
    format : STRING;
END_VAR
VAR_INPUT
    args: ...;
END_VAR
END_FUNCTION


(**
* The main function of the program prints a demo to the standard out
* The function main is implemented at this location and thus not marked
* as {external}
*)
FUNCTION main : DINT
VAR
    tmp : DINT;
END_VAR
  tmp := 1;
  printf('Value %d, %d, %d$N', tmp, tmp * 10, tmp * 100);
  main := tmp;
END_FUNCTION
```

Compiling the previous code with the following command:

```sh
plc ExternalFunctions.st -o ExternalFunctions --linker=clang
```

will yield an executable called `ExternalFunctions`.

> We use clang to link the generated object file and generate an executable
> since the embedded linker cannot generate executable files.

The executable can then be started with `./ExternalFunctions`.

## Using Timers

Timers are used for measuring and actioning on real time delays or periods.

They are Function Blocks which may only be used in Programs or other Function Blocks. This is because Functions should ideally have no state side effects. Not only RuSTy supports this model of Structured Text but also other compiler environments, such as Sysmac Studio. You must use a `main` function as the entrypoint and ensure it does not reference a Function Block.

To use timers with the RuSTy compiler, include the header using the `-i ./stdlib/includes/timers.st` CLI argument. You must link with the Standard Library to execute this example program by passing the `-l iec61131std` CLI argument. The Standard Library can be retrieved as an Artifact from any of the RuSTy [Build Pipelines](https://github.com/PLC-lang/rusty/actions).

```iecst
{external}
FUNCTION_BLOCK TON
VAR_INPUT
    IN: BOOL;
    PT: TIME;
END_VAR
VAR_OUTPUT
    Q: BOOL;
    ET: TIME;
END_VAR
VAR
    __signal__ : BOOL; (* Value representing the internal signal *)
    __is_running__: BOOL; (* Internal flag to track timer on/off state *)
    __BUFFER__ : ARRAY[1..24] OF BYTE; (* Buffer used for internal implementation *)
END_VAR
END_FUNCTION_BLOCK

FUNCTION main : DINT
    MyTimerExample();
END_FUNCTION

PROGRAM MyTimerExample
    VAR
        timer: TON;
    END_VAR

    timer(IN:=TRUE, T#3s);

    IF timer.Q THEN //evaluates to true after 3 seconds
        timer(IN:=FALSE, T#3s);
    END_IF
END_PROGRAM
```
