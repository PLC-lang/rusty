# Build Configuration

In addition to the comprehensive help, `plc` offers a build subcommand that simplifies the build process. </br>
Instead of having numerous inline arguments, using the build subcommand along with a build description file makes passing the arguments easier. </br>
The build description file needs to be saved in the [json](https://en.wikipedia.org/wiki/JSON) format.

Usage:
`plc build`

Note that if `plc` cannot find the `plc.json` file, it will throw an error and request the path.
The default location for the build file is the current directory.

The command for building with an additional path looks like this:
`plc build src/plc.json`

## Build description file (plc.json)

For the build description file to work, it must be written in the [json](https://en.wikipedia.org/wiki/JavaScript_Object_Notation) format.
All the keys used in the build description file are described in the following sections.

### files

The keyword `files` is the equivalent to the `input` parameter, which adds all the `ST` files that need to be compiled.

The value of `files` is an array of strings, definied as follows:

```json
"files" : [
    "examples/hello_world.st",
    "examples/hw.st"
    "examples/*.gvl"
]
```

### libraries

To link several objects into one executable `plc` has the option to add libraries and automatically build and link them together.</br>
The `libraries` keyword is optional.

```json
"libraries" : [
    {
        "name" : "iec61131std",
        "path" : "path/to/lib/",
        "link_path" : "libiec61131std.so.1",
        "package" : "Copy",
        "include_path" : [
            "examples/hw.st",
            "examples/hello_world.st"
        ]
    }
]
```

### libraries.link_path

`link_path` is optional.

If present, RuSTy links the exact file instead of resolving by `name` (`-l<name>`).
This is useful for versioned filenames such as `libfoo.so.1`.

- Relative `link_path` values are resolved against the library's `path`.
- Absolute `link_path` values are used directly.
- Copy behavior is unchanged: libraries marked as `Copy` are still copied from the configured library directory.

### output

Similarly to specifying an output file via the `-o` or `--output` option using the command line, in the build file we use `"output" : "output.so"` to define the output file. The default location is the current build directory. (see [Build Location](#build-location)).

### compile_type

The following options can be used for the `compile_type` :

- `Static` specifies that linking/binding must be done at compile time.
- `Shared` (dynamic) specifies that linking/bingind must be done dynamically (at runtime).
- `PIC` Position Independent Code (Choosing this option implies that the linking will be done dynamically).
- `Relocatable` generates relocatable object code (for combining with other object code).
- `Bitcode` adds bitcode alongside machine code in executable file.
- `IR` intermediate `llvm` representation.

The compile format is specified in the build description file as follows:  `"compile_type" : "Shared"`.
The `compile_type` keyword is optional.

### package_commands

The `package_commands` keyword is optional.

> TODO

### Example

```json
{
    "files" : [
        "examples/hw.st",
        "examples/hello_world.st",
        "examples/ExternalFunctions.st",
        "examples/*.dt"
    ],
    "compile_type" : "Shared",
    "output" : "proj.so",
    "libraries" : [
        {
            "name" : "iec61131std",
            "path" : "path/to/lib",
            "package" : "Copy",
            "include_path" : [
                "examples/lib.st"
            ]
        },
        {
            "name" : "other_lib",
            "path" : "path/to/lib",
            "package" : "System",
            "include_path" : [
                "examples/hello_world.st"
            ]
        }
    ]
}
```

## Build Parameters

### `--build-location`

`--build-location` is a global `plc` option.</br>
It controls where intermediate build artifacts are written.

- With `plc build`, the default is `build` in the project root (the location of `plc.json`)
- With non-`build` commands, no default build directory is used unless `--build-location` is provided

When `--build-location` is not provided for non-`build` commands, RuSTy may place intermediate object files in the OS temporary directory. This is especially relevant for multi-file compilation, where intermediate objects are generated first and then passed to the linker to produce the final output artifact.

### `--lib-location`

`--lib-location` is available on the `build` subcommand.</br>
It controls where libraries marked with `Copy` are copied.

For `plc build`, if `--lib-location` is not set, RuSTy falls back to:
1. `--build-location` (if set)
2. `build`

### Additional linker options

All regular linker options from `plc` can also be used with the `build` subcommand, e.g.:

- `--linker=<cmd>`
- `--fuse-ld=<name>`
- `--linker-arg=<arg>` (repeatable)
- `--nocrt`
- `--nolibc`
- `--fpic` / `--fno-pic` (relocation model)

## Environment Variables

Environment variables can be used inside the build description file, the variables are evaluated before an entry is evaluated.

In addition to externally defined variables, the build exports variables that can be referenced in the description file:

### `PROJECT_ROOT`

The folder containing the `plc.json` file, i.e. the root of the project.

### `ARCH`

The target architecture currently being built, for a multi architecture build.
The value for `ARCH` will be updated for every target.

Example targets are:
`x86_64-pc-linux-gnu`, `x86_64-pc-windows-msvc`, `aarch64-pc-linux-musl`

### `BUILD_LOCATION`

`BUILD_LOCATION` is the folder where build artifacts are written.
For `plc build`, this is either [`--build-location`](#build-location) or the default `build` directory.
For non-`build` commands, it is only set when `--build-location` is provided.

### `LIB_LOCATION`

`LIB_LOCATION` is the folder where libraries marked with `Copy` are saved.
This is the value of [`--lib-location`](#lib-location), or the [build location](#build-location) fallback used by `plc build`.

### Usage

To reference an environment variable in the description file, reference the variables with a preceding `$`.

**Example:**

```json
{
 "name" : "mylib",
 "path" : "$ARCH/lib",
 "package" : "System",
 "include_path" : [
  "examples/hello_world.st"
 ]
}
```

## Validation

The build description file uses a [Json Schema](https://json-schema.org/) file located at `compiler/plc_project/schema/plc-json.schema` to validate the build description before build.
In order for the schema to be used, it has to be either in that location for source builds or copied next to the build binaries.
If the schema is not found, the schema based validation will be skipped.
