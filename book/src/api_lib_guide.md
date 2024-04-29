### 1. Introduction

- **Purpose**: Provide plc library developers with information how an interface for an application written in `IEC61131-3` should be designed and why.
- **Scope**: This guideline applies to all developers writing libraries for
         use in IEC61131-3 applications.

### 2. API Guidelines

#### 2.1 `VAR_IN_OUT` instead of pointers

If your function takes a parameter for the purpose of reading and writing to it
use a `VAR_IN_OUT` instead of a pointer in the `VAR_INPUT`.

#### 2.2 `FUNCTION` and `FUNCTION_BLOCK`

`FUNCTION` and `FUNCTION_BLOCK` have similar properties, but they have fundamentally different representation in the compiler.
A `FUNCTION` is defined in a similar manner to a `C` function, it has no backing struct and values defined inside it will only survive for the duration of the function call.
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
To declare a `FUNCTION_BLOCK`, you have to declare the backing struct and pass it as a reference to the function block implementation.

```iecst
FUNCTION_BLOCK myFb
VAR_INPUT
  x  : DINT;
END_VAR
END_FUNCTION_BLOCK
```

```c
typedef struct {
  int32_t x;
} myFunctStr;

void myFb(myFunctStr*);
```



##### 2.2.1 Parameters

`FUNCTION` and `FUNCTION_BLOCK` may define input parameters. These are passed using the `VAR_INPUT` or `VAR_IN_OUT` blocks.
The differences between the two blocks is how the values are passed.
A `VAR_INPUT` variable is passed _by value_, while a `VAR_IN_OUT` variable is passed _by reference_.
In general, it is recommended to use a `VAR_IN_OUT` for data that needs to be both read and written, while `VAR_INPUT` should be reserved to read only values.

In `FUNCTIONS`, it is possible to still pass references to a `VAR_INPUT` block by using the `{ref}` pragma: `VAR_INPUT {ref}`.
This would treat all variables as references and could be useful for some more complex datatypes.

> **_NOTE:_** Internally non primitive datatypes are always passed as pointers, they are however copied into a temporary variable before getting passed in order to preserve the by value properties. Using `{ref}` will circumvent this behaviour.

Examples:

`FUNCTION`:

```iecst
FUNCTION myFunc : DINT
VAR_INPUT
  myInt  : DINT;
  myString : STRING(255);
END_VAR

VAR_INPUT {ref}
  myRefInt : DINT;
END_VAR

VAR_IN_OUT
  myInOutInt : DINT;
END_VAR
END_FUNCTION
```

```c
int32_t myFunc(int32_t myInt, char* myString, int32_t* myRefInt, int32_t* myInOutInt);
```

`FUNCTION_BLOCK`:


```iecst
FUNCTION_BLOCK myFb
VAR_INPUT
  myInt  : DINT;
  myString : STRING(255);
END_VAR

VAR_IN_OUT
  myInOutInt : DINT;
END_VAR
END_FUNCTION_BLOCK
```

```c

typedef struct {
  int32_t myInt;
  char myString[256];
  int32_t* myInOutInt;
} myFbStruct

void myFb(myFbStruct* myFbInstance);
```


##### 2.2.2 Private members

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
  int32_t current;
} CountStruct

void Count(CountStruct* countInst) {
  countInst->current = countInst->current + 1;
}
```



##### 2.2.3 Return values

A `FUNCTION` defines a return value in the signature, while a `FUNCTION_BLOCK` relies on `VAR_OUTPUT` definitions.

The return type for a function can also include complex datatypes, such as strings, arrays and structs.
Internally, complex return types are treated as reference parameters.

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
    myOut2: STRING(255);
  END_VAR
END_FUNCTION
```

The C interface would look like:

```c
typedef struct {
  int32_t x;
  int32_t* y;
  int32_t myOut;
  char myOut2[256];

} myFbStruct;

void myFb(myFbStruct* myFbInst);
```

##### When to use a `FUNCTION` vs. `FUNCTION_BLOCK`

A `FUNCTION` can be well integrated into the API because of its return value which
can be nested into expressions. They however don't keep data over subssequent
executions. If you need to store static data use a `FUNCTION_BLOCK` or use
`VAR_IN_OUT`.

##### Do not use `PROGRAM`s in your libraries

`PROGRAM`s have static instances. These are reserved for applications and should
not be used in libraries.

### 2.3 Datatypes

The IEC61131-3 Standard defines several datatypes with their intended uses.
To stay standard compliant, an API/Library should try and follow these guidelines.

#### 2.3.1 Type sizes

Datatypes are generally convertable to `C` equivalent. With M100 being a 64bit system, some sizes were also fixed to 64bit.

Below is a table of primitive types and how they can be used from C

| type            | c equivalent | size | comment                                                    |
| --------------- | ------------ | ---- | ---------------------------------------------------------- |
| BOOL            | bool         | 8    |                                                            |
| BYTE            | uint8_t      | 8    | intended to be used as bit sequence and not as a number    |
| SINT            | int8_t       | 8    |                                                            |
| USINT           | uint8_t      | 8    |                                                            |
| WORD            | uint16_t     | 16   |                                                            |
| INT             | int16_t      | 16   |                                                            |
| UINT            | uint16_t     | 16   |                                                            |
| DINT            | int32_t      | 32   |                                                            |
| DWORD           | uint32_t     | 32   |                                                            |
| UDINT           | uint32_t     | 32   |                                                            |
| LINT            | int64_t      | 64   |                                                            |
| LWORD           | uint64_t     | 64   |                                                            |
| ULINT           | uint64_t     | 64   |                                                            |
| REAL            | float_t      | 32   |                                                            |
| LREAL           | double_t     | 64   |                                                            |
| TIME            | time_t       | 64   | Note that all time and date types are 64 bit               |
| LTIME           | time_t       | 64   |                                                            |
| DATE            | time_t       | 64   |                                                            |
| LDATE           | time_t       | 64   |                                                            |
| DATE_AND_TIME   | time_t       | 64   |                                                            |
| LDATE_AND_TIME  | time_t       | 64   |                                                            |
| DT              | time_t       | 64   |                                                            |
| LDT             | time_t       | 64   |                                                            |
| TIME_OF_DAY     | time_t       | 64   |                                                            |
| LTIME_OF_DAY    | time_t       | 64   |                                                            |
| TOD             | time_t       | 64   |                                                            |
| LTOD            | time_t       | 64   |                                                            |
| POINTER TO type | \*type       | 64   | The Pointer size is equivalent to `LWORD` and not `DWORD`  |
| REF_TO type     | \*type       | 64   | Prefer this type to `POINTER TO` for standard compliance   |
| STRING          | uint8_t[]    | var  | UTF-8 String, null terminated. Default is 80 chars         |
| WSTRING         | uint16_t[]   | var  | UTF-16 (wide) String, null terminated. Default is 80 chars |

#### 2.3.2 Using Types in interfaces

When deciding on a type to use for a `FUNCTION`, `FUNCTION_BLOCK`, or `STRUCT` use a type that reflects the intention of the API.
- A bit sequence should be in a BIT type like `WORD` and not in a numeric type like `INT`.
- A variable representing a time should be stored in the appropriate time type and not an `LINT` or `LWORD`
- A pointer should be stored as a `REF_TO` and not as an `LWORD` where possible.
  - Note that void pointers are currently possible by using a `REF_TO __VOID` type. The syntax is however subject to change before release. (TODO: Does this even work)
- `(W)STRING`s and `ARRAY`s stored in `VAR`, `VAR_INPUT` and `VAR_OUTPUT` sections of `FUNCTION_BLOCK`s are stored in the `FUNCTION_BLOCK`, and are passed by value.
  - You can use a `VAR_IN_OUT` block to force a type to be passed as a pointer, keep in mind that `VAR_IN_OUT` is a read-write variable and changes to the parameter will change it for the caller.
  - `FUNCTION`s expecting an `ARRAY` parameter can use the `ARRAY[*]` syntax (Variable sized array). The same functionality will be available for `STRING` it is however not yet implemented.

#### 2.4 Struct alignment

Struct alignment in plc follows the default behaviour of c/cpp. This means when developing a libraryin c/cpp there is nothing to be done but when developing a library in `rust` the `#[repr(C)]` attribute to enable rust to use c/cpp behaviour should be used.

Example
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
  int32_t x;
  int32_t* y;
  int32_t myOut;
  char z[256];

} myStruct;
```

The `rust` struct would look like
```rust
#[repr(C)]
pub struct myStruct {
    x: i32,
    y: *mut i32,
    z: [c_char; 256],
}
```

#### 2.5 `FUNCTION_BLOCK` initialization
!todo


TODO: When writing a function think of signature