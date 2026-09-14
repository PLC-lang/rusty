# Header Generation

The compiler can write the declarations of a project as C headers. Use them when you implement a declared interface in C, or when C code calls into compiled Structured Text. The generation replaces code generation: the compiler writes the headers and stops.

Only C is supported.


## From the command line

```bash
plc --generate-headers "**/*.pli" --header-output include
```

| Option | Purpose |
|---|---|
| `--generate-headers` | Generate headers instead of code |
| `--header-output <dir>` | The directory for the generated files |
| `-o <name>` | Write one file with the declarations of every input |
| `-i <file>` | Add declarations that the inputs need, without a header for them |

Without `-o`, every input file gets its own header, named after it.


## From a project file

```bash
plc generate plc.json headers
```

| Option | Purpose |
|---|---|
| `--header-output <dir>` | The directory for the generated files |
| `--header-language c` | The language. `c` is the only implemented value |
| `--header-prefix <name>` | A prefix for the generated file names |


## An example

```iecst
TYPE T_Message: STRING[255];
END_TYPE

FUNCTION PrintMessage: DINT
    VAR_INPUT
        message: T_Message;
    END_VAR
END_FUNCTION

FUNCTION_BLOCK Counter
    VAR_INPUT
        step: DINT;
    END_VAR
    VAR_OUTPUT
        value: DINT;
    END_VAR
END_FUNCTION_BLOCK
```

```c
// ---------------------------------------------------- //
// This file is auto-generated                          //
// Manual changes made to this file will be overwritten //
// ---------------------------------------------------- //

#ifndef INCLUDE_MOTOR_H_
#define INCLUDE_MOTOR_H_

#include <stdint.h>
#include <stdbool.h>
#include <math.h>
#include <time.h>
#include <dependencies.plc.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef char T_Message[256];

typedef struct {
    uint64_t* __vtable;
    int32_t step;
    int32_t value;
} Counter_type;

// message: maximum of 256 T_Message(s)
int32_t PrintMessage(T_Message* message);

void Counter(Counter_type* self);

#ifdef __cplusplus
}
#endif /* __cplusplus */

#endif /* !INCLUDE_MOTOR_H_ */
```

The include guard is the output path in upper case. The C interface of a function block is its struct plus a function that takes a pointer to an instance. The first member of every function block struct is the pointer to its method table.


## What the generator skips

- Declarations marked `{external}`, because their implementation is elsewhere already.
- Declarations that came in with `-i`.
- The constructors that the compiler generates, and every name that starts with `__`.

A file whose declarations are all external or included produces no header at all.

The C interface of each construct, and the rules behind the type translation, are in the [C interface](c-interface.md) chapter.


## What's next

The last chapter here is about the interface itself: [how to design one](api-guidelines.md).
