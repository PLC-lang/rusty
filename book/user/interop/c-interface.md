# The C Interface

This chapter is the contract between Structured Text and C. It says which C type a declaration has, how a parameter is passed, and which symbols a library must provide.

The compiler can write these declarations for you, see [Generating Headers](headers.md).


## Types

| Structured Text | C | Size in bits |
|---|---|---|
| `BOOL` | `bool` | 8 |
| `BYTE` | `uint8_t` | 8 |
| `SINT` | `int8_t` | 8 |
| `USINT` | `uint8_t` | 8 |
| `WORD` | `uint16_t` | 16 |
| `INT` | `int16_t` | 16 |
| `UINT` | `uint16_t` | 16 |
| `DWORD` | `uint32_t` | 32 |
| `DINT` | `int32_t` | 32 |
| `UDINT` | `uint32_t` | 32 |
| `LWORD` | `uint64_t` | 64 |
| `LINT` | `int64_t` | 64 |
| `ULINT` | `uint64_t` | 64 |
| `REAL` | `float` | 32 |
| `LREAL` | `double` | 64 |
| `TIME`, `DATE`, `TIME_OF_DAY`, `DATE_AND_TIME` | `uint32_t` | 32 |
| `LTIME`, `LDATE`, `LTIME_OF_DAY`, `LDATE_AND_TIME` | `int64_t` | 64 |
| `CHAR` | `uint8_t` | 8 |
| `WCHAR` | `uint16_t` | 16 |
| `STRING[n]` | `char[n + 1]` | 8 * (n + 1) |
| `WSTRING[n]` | `uint16_t[n + 1]` | 16 * (n + 1) |
| `REF_TO T`, `POINTER TO T` | `T*` | 64 |
| `ARRAY[a..b] OF T` | `T[b - a + 1]` | |

A string holds one more element than its declared length, for the terminator. A pointer is 64 bits, which is the size of `LWORD` and not of `DWORD`.


## Functions

A `FUNCTION` becomes a C function. The table above gives the type of a parameter, and the block that declares it decides how the parameter arrives:

- A `VAR_INPUT` of an elementary type is passed by value.
- A `VAR_INPUT` of a string, array, or struct type is passed as a pointer to the caller's variable. A callee that the compiler generates copies the value on entry, so the caller sees no change. A callee that you write in C must do the same and not write through the pointer.
- A `VAR_INPUT {ref}`, a `VAR_IN_OUT`, and a `VAR_OUTPUT` are passed as a pointer to the caller's variable. A write through the pointer reaches the caller.

```iecst
TYPE Point:
    STRUCT
        x: DINT;
        y: DINT;
    END_STRUCT
END_TYPE

FUNCTION F1: DINT
    VAR_INPUT
        i: DINT;
        s: STRING[10];
        p: Point;
    END_VAR
    VAR_IN_OUT
        io: DINT;
    END_VAR
    VAR_OUTPUT
        o: DINT;
    END_VAR
END_FUNCTION
```

```c
typedef struct {
    int32_t x;
    int32_t y;
} Point;

int32_t F1(int32_t i, char* s, Point* p, int32_t* io, int32_t* o);
```

The parameters keep the order of the declaration blocks.


## Return values

An elementary return type is the return value of the C function.

An aggregate return type (string, array, or struct) is returned through a pointer that the caller provides, and that pointer is the **first** parameter. The C function then returns `void`.

```iecst
FUNCTION RetString: STRING[20]
    VAR_INPUT
        n: DINT;
    END_VAR
END_FUNCTION
```

```c
void RetString(char* RetString, int32_t n);
```


## Function blocks

A `FUNCTION_BLOCK` is a struct plus a function that takes a pointer to an instance. Every variable block becomes a member of the struct, in declaration order, and that includes the private `VAR` members. A `VAR_TEMP` block is not a member, because it lives only for the duration of one call.

```iecst
FUNCTION_BLOCK FB1
    VAR_INPUT
        i: DINT;
        s: STRING[10];
    END_VAR
    VAR_IN_OUT
        io: DINT;
    END_VAR
    VAR_OUTPUT
        o: DINT;
    END_VAR
    VAR
        priv: DINT;
    END_VAR
END_FUNCTION_BLOCK
```

```c
typedef struct {
    uint64_t* __vtable;
    int32_t i;
    char s[11];
    int32_t* io;
    int32_t o;
    int32_t priv;
} FB1_type;

void FB1(FB1_type* self);
```

> [!IMPORTANT]
> The first member of every function block struct is `__vtable`, the pointer to the method table. Structured Text has no `virtual` keyword, so every function block gets the member, whether it uses inheritance or not. A C struct without it has the wrong layout.

A `PROGRAM` has the same shape, but without the `__vtable` member, and the compiler creates its one instance as the global `<Program>_instance`. Do not use programs in a library.


## Struct layout

Layout and alignment follow the rules of C. In C, declare a normal struct. In another language, force the C layout, for example with `#[repr(C)]` in Rust:

```rust,noplayground
use std::ffi::c_char;

#[repr(C)]
pub struct MyStruct {
    x: i32,
    y: *mut i32,
    z: [c_char; 256],
}
```


## Initialization

The compiler writes a constructor function for every type and one for every source file. The constructor of a source file sets the globals of that file and calls the constructors of the types in it. It is registered in the constructor list of the binary, so it runs before the application starts and no manual call is necessary.

| Symbol | Purpose |
|---|---|
| `<TypeName>__ctor` | Initializes one instance of a struct or function block |
| `<FunctionBlock>__FB_INIT` | The `FB_INIT` method of a function block, called by the constructor of the type |
| `__unit_<file>_<hash>__ctor` | Initializes the globals of one source file, and calls the constructors above |

In the last symbol, `<file>` is the file name with every character that is not a letter, a digit, or an underscore replaced by an underscore, so `fb1.st` becomes `fb1_st`. The `<hash>` is eight hexadecimal characters that come from the full path, which keeps two files of the same name apart.

A function block that needs initialization in C implements the `FB_INIT` symbol:

```c
void myFunctionBlock__FB_INIT(myFunctionBlock_type* self) {
    self->a = 1;
    self->b = 2;
}
```

The declaration on the Structured Text side only states that the method exists:

```iecst
{external}
FUNCTION_BLOCK myFunctionBlock
    VAR
        a: DINT;
        b: DINT;
    END_VAR

    METHOD FB_INIT
    END_METHOD
END_FUNCTION_BLOCK
```

The C side must also define `myFunctionBlock`, the body of the function block, because the constructor of the type writes its address into the method table.


## Constructors for external code

The compiler writes no constructor for an `{external}` unit until you ask for it, so the `FB_INIT` above is never called. Two options control for which units the compiler writes constructors:

| Option | Use |
|---|---|
| `--constructors-only` | Write the generated constructors and no bodies. For building the constructor object of an external library. It implies `--generate-external-constructors` |
| `--generate-external-constructors` | Write constructors for `{external}` units as well. For the application that links such a library |

```bash
# 1. Build the constructor object of the library
plc --constructors-only -c -o libext_ctor.o my_lib.pli

# 2. Build and ship the shared library
gcc -shared -fPIC -o libext.so my_lib.c libext_ctor.o

# 3. Build the application, with constructors for the external declarations
plc -L. -lext -i my_lib.pli --generate-external-constructors app.st
```

For a library that mixes foreign code with Structured Text sources, compile the sources with constructors and archive the result:

```bash
plc iec61131-st/*.st -c --generate-external-constructors -o st.o
ar crs libst.a st.o
```


## What's next

You do not have to write these declarations by hand. The [next chapter](headers.md) makes the compiler generate them.
