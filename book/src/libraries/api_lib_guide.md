# API guidelines

## 1. Introduction

- **Purpose**: Provide plc library developers with information how an interface for an application written in `IEC61131-3` should be designed and why.
- **Scope**: This guideline applies to all developers writing libraries for
         use in IEC61131-3 applications.

## 2. API Guidelines

### 2.1 `VAR_IN_OUT` instead of pointers

If a function takes a parameter for the purpose of reading and writing to it
a `VAR_IN_OUT` can be used instead of a pointer in the `VAR_INPUT`.

### 2.2 `FUNCTION` and `FUNCTION_BLOCK`

`FUNCTION` and `FUNCTION_BLOCK` have similar properties, but they have fundamentally different representation in the compiler.
A `FUNCTION` is defined in a similar manner to a `C` function:
- It has no backing struct
- values defined inside it will only persist for the duration of the function call

Example:

```iecst
FUNCTION myFunc : DINT
VAR_INPUT
  x  : DINT;
END_VAR
END_FUNCTION
```
```c
int32_t myFunc(int32_t);
```

In contrast, a `FUNCTION_BLOCK` is backed by a struct and is globally accessible by a defined instance.
To declare a `FUNCTION_BLOCK`, a backing struct has to be declared and passed as a reference to the function block implementation.

> **_NOTE:_** Due to OOP polymorphism features, all function blocks **must** include a `__vtable` parameter as the first field in their C struct representation. This vtable (virtual table) pointer enables dynamic method dispatch and polymorphic behavior. Since structured text does not support a `virtual` keyword, all function blocks have this vtable parameter regardless of whether they use inheritance or polymorphism. When interfacing with C code, this parameter must be included in the struct definition.

```iecst
FUNCTION_BLOCK myFb
VAR_INPUT
  x  : DINT;
END_VAR
END_FUNCTION_BLOCK
```

```c
typedef struct {
  void* __vtable;
  int32_t x;
} myFunctStr;

void myFb(myFunctStr*);
```



#### 2.2.1 Parameters

`FUNCTION` and `FUNCTION_BLOCK` may define input parameters. These are passed using the `VAR_INPUT` or `VAR_IN_OUT` blocks.
The difference between the two blocks is how the values are passed.
A `VAR_INPUT` variable is passed _by value_, while a `VAR_IN_OUT` variable is passed _by reference_.
In general, it is recommended to use a `VAR_IN_OUT` for data that needs to be both read and written, while `VAR_INPUT` should be reserved to read only values.

> **_NOTE:_** In `FUNCTION`s complex datatypes are handled as pointers. They are however copied and changes in the function will have no effect on the actual variable.

Examples:

`FUNCTION`:

```iecst
FUNCTION myFunc : DINT
VAR_INPUT
  myInt  : DINT;
  myString : STRING[255];
END_VAR

VAR_INPUT
  myRefStr : STRING;
END_VAR

VAR_IN_OUT
  myInOutInt : DINT;
END_VAR
END_FUNCTION
```

```c
int32_t myFunc(int32_t myInt, char* myString, char* myRefStr, int32_t* myInOutInt);
```

`FUNCTION_BLOCK`:


```iecst
FUNCTION_BLOCK myFb
VAR_INPUT
  myInt  : DINT;
  myString : STRING[255];
END_VAR

VAR_IN_OUT
  myInOutInt : DINT;
END_VAR
END_FUNCTION_BLOCK
```

```c

typedef struct {
  void* __vtable;
  int32_t myInt;
  char myString[256];
  int32_t* myInOutInt;
} myFbStruct

void myFb(myFbStruct* myFbInstance);
```


#### 2.2.2 Private members

A `FUNCTION_BLOCK` often requires local (private) members to hold data across executions. These members have to be declared in the struct.
As a side effect, these variables are visible to the users.

For example:

```
FUNCTION_BLOCK Count
VAR
  current : DINT;
END_VAR
END_FUNCTION_BLOCK
```

```c

typedef struct {
  void* __vtable;
  int32_t current;
} CountStruct;

void Count(CountStruct* countInst) {
  countInst->current = countInst->current + 1;
}
```

#### 2.2.3 Return values

A `FUNCTION` defines a return value in the signature, while a `FUNCTION_BLOCK` relies on `VAR_OUTPUT` definitions.

Example:
```iecst
FUNCTION myFunc : DINT
  VAR_INPUT
    x : DINT;
  END_VAR
  VAR_IN_OUT
    y : DINT;
  END_VAR
END_FUNCTION
```

The C interface would look like:

```c
int32_t myFunc(int32_t x, int32_t* y);
```


The return type for a function can also include complex datatypes, such as strings, arrays and structs.
Internally, complex return types are treated as reference parameters (pointers).

For complex return types, the function signature expects the return value as the first parameter.

Example:

```iecst
FUNCTION myFunc : STRING
  VAR_INPUT
    x : DINT;
  END_VAR
  VAR_IN_OUT
    y : DINT;
  END_VAR
END_FUNCTION
```

The C interface would look like:

```c
void myFunc(char* out, int32_t x, int32_t* y);
```

A `FUNCTION_BLOCK` should use `VAR_OUTPUT` for return values. Avoid using a
pointer in the `VAR_INPUT` as a return value.

Example:

```iecst
FUNCTION_BLOCK myFb
  VAR_INPUT
    x : DINT;
  END_VAR
  VAR_IN_OUT
    y : DINT;
  END_VAR
  VAR_OUTPUT
    myOut: DINT;
    myOut2: STRING[255];
  END_VAR
END_FUNCTION
```

The C interface would look like:

```c
typedef struct {
  void* __vtable;
  int32_t x;
  int32_t* y;
  int32_t myOut;
  char myOut2[256];

} myFbStruct;

void myFb(myFbStruct* myFbInst);
```

### 2.2.4 When to use a `FUNCTION` vs. `FUNCTION_BLOCK`

A `FUNCTION` can be well integrated into the API because of its return value which
can be nested into expressions. They however don't keep data over subsequent
executions. If you need to store static data use a `FUNCTION_BLOCK` or use
`VAR_IN_OUT`.


> **_NOTE:_** Do not use `PROGRAM`s in your libraries
> `PROGRAM`s have static instances. These are reserved for applications and should
> not be used in libraries.

### 2.3 Datatypes

The IEC61131-3 Standard defines several datatypes with their intended uses.
To stay standard compliant, an API/Library should try and follow these guidelines.

### 2.3.1 Type sizes

Datatypes are generally convertable to `C` equivalent. With the compiler defaulting to 64bit, some sizes were also fixed to 64bit.

Below is a table of types and how they can be used from `C`

| type            | c equivalent | size | comment                                                                             |
| --------------- | ------------ | ---- | ----------------------------------------------------------                          |
| BOOL            | bool         | 8    |                                                                                     |
| BYTE            | uint8_t      | 8    | intended to be used as bit sequence and not as a number                             |
| SINT            | int8_t       | 8    |                                                                                     |
| USINT           | uint8_t      | 8    |                                                                                     |
| WORD            | uint16_t     | 16   |                                                                                     |
| INT             | int16_t      | 16   |                                                                                     |
| UINT            | uint16_t     | 16   |                                                                                     |
| DINT            | int32_t      | 32   |                                                                                     |
| DWORD           | uint32_t     | 32   |                                                                                     |
| UDINT           | uint32_t     | 32   |                                                                                     |
| LINT            | int64_t      | 64   |                                                                                     |
| LWORD           | uint64_t     | 64   |                                                                                     |
| ULINT           | uint64_t     | 64   |                                                                                     |
| REAL            | float_t      | 32   |                                                                                     |
| LREAL           | double_t     | 64   |                                                                                     |
| TIME            | time_t       | 64   | Note that all time and date types are 64 bit                                        |
| LTIME           | time_t       | 64   |                                                                                     |
| DATE            | time_t       | 64   |                                                                                     |
| LDATE           | time_t       | 64   |                                                                                     |
| DATE_AND_TIME   | time_t       | 64   |                                                                                     |
| LDATE_AND_TIME  | time_t       | 64   |                                                                                     |
| DT              | time_t       | 64   |                                                                                     |
| LDT             | time_t       | 64   |                                                                                     |
| TIME_OF_DAY     | time_t       | 64   |                                                                                     |
| LTIME_OF_DAY    | time_t       | 64   |                                                                                     |
| TOD             | time_t       | 64   |                                                                                     |
| LTOD            | time_t       | 64   |                                                                                     |
| POINTER TO type | \*type       | 64   | The Pointer size is equivalent to `LWORD` and not `DWORD`                           |
| REF_TO type     | \*type       | 64   | Prefer this type to `POINTER TO` for standard compliance                            |
| STRING          | uint8_t[]    | var  | UTF-8 String, null terminated. Default is 80 chars + 1 termination byte             |
| WSTRING         | uint16_t[]   | var  | UTF-16 (wide) String, null terminated. Default is 80 chars + 1 termination byte     |

### 2.3.2 Using Types in interfaces

When deciding on a type to use for a `FUNCTION`, `FUNCTION_BLOCK`, or `STRUCT` use a type that reflects the intention of the API:
- A bit sequence should be in a BIT type like `WORD` and not in a numeric type like `INT`.
- A variable representing a time should be stored in the appropriate time type and not an `LINT` or `LWORD`
- A pointer should be stored as a `REF_TO` and not as an `LWORD` where possible.
- `(W)STRING`s and `ARRAY`s stored in `VAR`, `VAR_INPUT`, and `VAR_OUTPUT` sections of `FUNCTION_BLOCK`s are stored in the `FUNCTION_BLOCK`, and are passed by value.
  - A `VAR_IN_OUT` block can be used to force a type to be passed as a pointer. Note that `VAR_IN_OUT` is a read-write variable and changes to the parameter will change it for the caller.
  - `FUNCTION`s expecting an `ARRAY` parameter can use the `ARRAY[*]` syntax (Variable sized array). The same functionality will be available for `STRING`. It is however not yet implemented.

### 2.4 Struct alignment

Struct alignment in plc follows the default behaviour of `C`.
When developing a library in `C` a normal struct can be declared.
In langugages other than `C` the struct has to be `C` compatible. For example in `rust` the `#[repr(C)]` can be used to make the struct `C` compatible.

> **_IMPORTANT:_** All `FUNCTION_BLOCK` structs must include a `__vtable` parameter as the first field. This is required for the OOP polymorphism implementation. The vtable is a `void*` pointer that points to the virtual function table for the function block instance. This affects struct layout and alignment when interfacing with external code.

Example:

```iecst
TYPE myStruct:
STRUCT
    x      : DINT;
    y      : REF_TO DINT;
    z      : ARRAY[0..255] OF BYTE;
END_STRUCT
END_TYPE
```

The `C` struct would look like:

```c

typedef struct {
  void* __vtable;
  int32_t x;
  int32_t* y;
  char z[256];

} myStruct;

```

The `rust` struct would look like
```rust
use std::ffi::{c_void, c_char};

#[repr(C)]
pub struct myStruct {
    __vtable: *mut c_void,
    x: i32,
    y: *mut i32,
    z: [c_char; 256],
}
```

### 2.5 `FUNCTION_BLOCK` initialization

When creating a library with `FUNCTION_BLOCK`s, you can implement initialization logic that runs when an instance is created. 

For more details on `FB_INIT` in IEC61131-3, refer to the [Program Organization Units (POUs)](../pous.md#function_block-initialization) documentation.

#### Interoperability with libraries written in other languages

When implementing a `FUNCTION_BLOCK` with initialization in C or other languages, you need to follow a specific naming convention for the initialization function.

For a C implementation:

1. Define a struct that matches your `FUNCTION_BLOCK` variables:

```c
typedef struct {
    void* __vtable;
    int a;
    int b;
    // Other members as needed
} myFunctionBlock;
```

2. Optionally create an initialization function following the naming pattern `<FunctionBlockName>__FB_INIT`:

```c
void myFunctionBlock__FB_INIT(myFunctionBlock* fb_instance) {
    // Initialize members here
    fb_instance->a = 1;
    fb_instance->b = 2;
    
    // ...perform any other needed initialization
}
```

3. In your IEC61131-3 declaration (e.g., in a header file [`*.pli`]), ensure your `FUNCTION_BLOCK` includes the `FB_INIT` method (if present):

```
{external}
FUNCTION_BLOCK myFunctionBlock
VAR
    a : DINT;
    b : DINT;
END_VAR
    METHOD FB_INIT
    END_METHOD
END_FUNCTION_BLOCK
```

Note that the `FB_INIT` method doesn't need implementation details in the IEC61131-3 declaration when using an external implementation - the declaration just signals that initialization is available.

#### Constructor flags for external libraries

ruSTy generates constructor functions (`__ctor`) for initialization of stateful types.

Use these flags depending on how you build and link external code:

- `--constructors-only`
  - Use this when building an external library and you only want the compiler-generated constructors.
  - This emits ctor symbols without emitting user-defined bodies. Link the resulting object into your C/C++ shared library.
  - This flag implies `--generate-external-constructors`.

- `--generate-external-constructors`
  - Use this when compiling the final PLC application and you want constructors for `{external}` units to be emitted alongside normal code generation.
  - This is the typical mode when your project includes external declarations (`.pli`) and you link a shared library that provides the implementations.

Example workflow:

```bash
# 1) Build constructor object for the external library
plc --constructors-only -c -o libext_ctor.o -i "stdlib/include/*.st" my_lib.pli

# 2) Build and ship your shared library
gcc -shared -fPIC -o libext.so my_lib.c libext_ctor.o

# 3) Build the PLC application, generating constructors for externals
plc -L. -lext -i my_lib.pli --generate-external-constructors app.st
```

Mixed Rust + ST example (stdlib build):

```bash
# Compile the ST sources into a single object with constructors
plc iec61131-st/*.st -c --generate-external-constructors -o st.o

# Archive for static linking
ar crs libst.a st.o
```

This mirrors `libs/stdlib/build.rs` and is the recommended approach when your library includes ST implementations (not just external declarations).

#### Project-wide initialization

See [Project-wide initialization](../using_rusty.md#project-wide-initialization)

