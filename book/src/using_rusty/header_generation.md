# Header Generation

In addition to providing the ability to [call out to external code](../libraries/external_functions.md) (code not written in ST), rusty also provides a generator for header files to simplify the process of implementing the external code.

## Usage

Header generation can be invoked via two possible commands. Both of these commands will short-circuit the code-generation phase of the build and instead output the result of the compilation as a header for the specified language.

### --generate-headers

**Please Note:** At present, the only langauge supported for header generation using this command is `c`.

This is the command whereby the arguments can be supplied inline and extends the existing top-level commands.

**For example:**
`plc --header-ouput "tmp\my_dir" -o "my_header" --generate-headers "**\*.pli"`
> This will ingest every `.pli` file in the relative directory and output them into a single header file `my_header.h` in the `tmp\my_dir` directory.

#### Subcommands (special usage)
- `--header-ouput`: The output folder where generated headers will be placed.
- `-o` : Write output to `output-file`. (All headers will be combined into this one file).
- `-i` : Include source files for external functions. These source files can include other external libraries that will be used by the headers but not generated into separate header files.

### generate headers

This command uses the [build configuration](build_configuration.md) `plc.json` file to determine the options for ingestion of content.

#### Subcommands

- `--header-language`: The language used to generate the header file. Currently supported language(s) are: C
- `--header-output`: The output folder where generated headers will be placed.
- `--header-prefix`: The prefix for the generated header file(s). Will default to the project name if not
            supplied.

**For example:** the following `plc.json` file coupled with the command `plc generate "plc.json" headers` will ingest a .pli file name `colour_tracker.pli` and output a file `colour_tracker.h` in the same directory.
```json
{
	"name" : "HeaderGeneratorProject",
	"files" : [
		"colour_tracker.pli"
	],
	"compile_type" : "Shared",
	"output" : "prog.so",
	"libraries" : [
	]
}
```

## A generated header example in C
**Input file:** message_printer.pli
```ST
TYPE T_Message : STRING[255];
END_TYPE

FUNCTION PrintMessage
VAR_INPUT
    message: T_Message;
END_VAR
END_FUNCTION
```

**Output header:** message_printer.h
```C
// ---------------------------------------------------- //
// This file is auto-generated                          //
// Manual changes made to this file will be overwritten //
// ---------------------------------------------------- //

#ifndef MESSAGE_PRINTER
#define MESSAGE_PRINTER

#include <stdint.h>
#include <math.h>
#include <stdbool.h>
#include <dependencies.plc.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef char T_Message[255];

void PrintMessage(T_Message* message);

#ifdef __cplusplus
}
#endif /* __cplusplus */

#endif /* !MESSAGE_PRINTER */
```
