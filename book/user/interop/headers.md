# Generating Headers

The compiler can write the declarations of a project as C headers. Use them when you implement a declared interface in C, or when C code calls into compiled Structured Text. Only C is supported. The generation replaces code generation: the compiler parses and validates the project, writes the headers, and stops before it produces an object file.


## From the command line

```bash
plc --generate-headers "**/*.pli" --header-output include
```

| Option | Purpose |
|---|---|
| `--generate-headers` | Generate headers instead of code |
| `--header-output <dir>` | The directory for the generated files, created when it is missing |
| `-o <name>` | Write one file, `<name>.h`, with the declarations of every input |
| `-i <file>` | Add declarations that the inputs need, without a header for them |

Without `-o`, every input file gets its own header, named after it. Give the name after `-o` without the `.h`, because the compiler always appends the extension. Without `--header-output`, a header lands beside its source, and a combined header beside the first input.


## From a project file

The `generate` subcommand reads the same [project file](../reference/project-file.md) as `plc build`, and writes one header per source file of the project:

```bash
plc generate plc.json headers
```

| Option | Purpose |
|---|---|
| `--header-output <dir>` | The directory for the generated files |
| `--header-language <lang>` | The language. `c` is the default and the only implemented value |
| `--header-prefix <name>` | Name every header `<name>.h` instead of naming it after its source |

> [!WARNING]
> `--header-prefix` is not a prefix. Each source file writes the same `<name>.h` and overwrites what the file before it wrote, so only the declarations of the last source survive, and nothing is reported. Use it for a project with one source file only. To get one header for a project with more, combine the sources with `--generate-headers` and `-o`.


## An example

The file `motor.pli` declares an alias, a function, and a function block:

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

The command at the top of this chapter writes `include/motor.h`:

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

The include guard is the path of the header in upper case, with each `/` and `.` turned into `_` and one `_` at the end. The C interface of a function block is its struct plus a function that takes a pointer to an instance. The first member of every function block struct is the pointer to its method table.

Every generated header includes `dependencies.plc.h`, and the compiler never writes that file. Put it on the include path yourself, empty or with the declarations the generator left out, or the C compiler stops at the include.


## What the generator skips

- Declarations marked `{external}`, because their implementation is elsewhere already.
- Declarations that came in with `-i`.
- The constructors that the compiler generates, and the types whose names start with `__`.

A file whose declarations are all external or included produces no header at all. A file that uses such a declaration still names it in its prototypes, so `dependencies.plc.h` is where the C side declares it.

The C interface of each construct, and the rules behind the type translation, are in the [C interface](c-interface.md) chapter.


## What's next

The last chapter here is about the interface itself: [how to design one](api-guidelines.md).
