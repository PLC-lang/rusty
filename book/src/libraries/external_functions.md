# External Functions

A `POU` (`PROGRAM`, `FUNCTION`, `FUNCTION_BLOCK`) can be marked as external,
which will cause the compiler to ignore its implementation

```iecst
@EXTERNAL
FUNCTION log : DINT
VAR_INPUT
  message : STRING[1024];
  type : (Err,Warn,Info) := Info;
END_VAR
END_PROGRAM
```

At compilation time, the function `log` will be defined,
and can be called from `ST` code.

> Note : At linking time, a `log` function with a compatible signature must be found.

## Calling C functions

`ST` code can call into foreign functions natively.
To achive this, the called function must be defined in a `C` compatible API
e.g. `extern "C"` blocks

The interface of the function has to be declared in `ST` using the `@EXTERNAL` keyword.

### Example

Given a `min` function defined in `C` as follows

```C
int min(int a, int b) {
//...
}
```

An interface of that function in `ST` can be defined as

```iecst
@EXTERNAL
FUNCTION min : DINT
VAR_INPUT
  a : DINT;
  b : DINT;
END_VAR
```

### Variadic arguments

Some foreign functions, especially ones defined in `C`,
could be [variadic functions](https://en.cppreference.com/w/c/variadic).

These functions are usually defined with the last parameter `...`, and signify
that a function can be called with unlimited parameters.

An example of a variadic function is printf.

Calling a variadic function is supported in `ST`. To mark an external function
as variadic, you can add a parameter of type `...` to the `VAR_INPUT` block

#### Variadic Function Example

Given the `printf` function defined as

```C
int printf( const char *restrict format, ... );
```

The `ST` interface can be defined as

```iecst
@EXTERNAL
FUNCTION printf : DINT
VAR_INPUT
  format : STRING;
  args : ...;
END_VAR
END_FUNCTION
```

#### Full Example

With the printf function available on the system, there is no need to declare
the C function

Declare an `ST` program called `ExternalFunctions.st` with the following code:

```iecst
@EXTERNAL FUNCTION printf : DINT
VAR_INPUT
    format : STRING;
    args: ...;
END_VAR
END_FUNCTION


(**
* The main function of the program prints a demo to the standard out
*)
FUNCTION main : DINT
  main := 1;
  printf('Value %d, %d, %d', main, main * 10, main * 100);
END_FUNCTION
```

Compile the previous code with the following command

```sh
rustyc ExternalFunctions.st -o ExternalFunctions.o  --static \
  && clang ExternalFunctions.o -o ExternalFunctions
```

> Note: that we use clang to link the generated object file and generate an executable

You can then run the demo with `./ExternalFunctions`
