# Build Command 

In addition to the comprehensive help, `rustyc` offers a build subcommand that simplifies the build process. </br>
Instead of having numerous inline arguments, using the build subcommand along with a build description file makes passing the arguments easier. </br>
The build description file needs to be saved in the [json](https://en.wikipedia.org/wiki/JSON) format.

Usage: 

`rustyc build`

Note that if `rustyc` cannot find the `plc.json` file, it will throw an error and request the path. The default location for the build file is the current directory. The command for building with an additional path looks like this:

`rustyc build src/plc.json`


## Build description file (plc.json)

For the build description file to work, it must be written in the [json](https://en.wikipedia.org/wiki/JavaScript_Object_Notation) format. All the keys used in the build description file are described in the following sections. 


### files

The keyword `files` is the equivalent to the `input` parameter, which adds all the `ST` files that need to be compiled. The value of `files` is an array of strings, definied as follows:
```json
"files" : [
    "examples/hello_world.st",
    "examples/hw.st"
    "examples/*.gvl"
]
```


### libraries

To link several executables `rustyc` has the option to add libraries and automatically build and link them together.</br>
The `libraries` keyword is optional.

```json
"libraries" : [
    {
        "name" : "iec61131std",
        "path" : "path/to/lib/",
        "package" : "Copy",
        "include_path" : [
            "examples/hw.st",
            "examples/hello_world.st"
        ]
    }
]
```


### output

Similarly to specifying an output file via the `-o` or `--output` option using the command line, in the build file we use `"output" : "output.so"` to define the output file. The default location is the current build directory. (see [Build Location](#build-location)) 


### compile_type

The following options can be used for the `compile_type`:
<!-- TODO we should probably describe what each of those options do -->
- `Static` specifies that linking/binding must be done at compile time
- `Shared` (dynamic) specifies that linking/bingind must be done dynamically (at runtime)
- `PIC` Position Independent Code (Choosing this option implies that the linking will be done dynamically)
- `Relocatable` generates relocatable object code (for combining with other object code)
- `Bitcode` adds bitcode alongside machine code in executable file
- `IR` intermediate `llvm` representation

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

The `build` subcommand exposes the following optional parameters: 

### `--build-location`

The build location is the location all build files will be copied to. </br>
By default the build location is the `build` folder in the root of the project.</br>
This can be overriden with the `--build-location` command line parameter.

### `--lib-location`

The lib location is where all libraries marked with `Copy` will be copied. </br>
By default it is the same as the `build-location`.</br>
This can be overriden with the `--lib-location` command line parameter.


## Environment Variables

Environment variables can be used inside the build description file, the variables are evaluated before an entry is evaluated.

In addition to externally defined variables, the build exports variables that can be referenced in the description file:

### `PROJECT_ROOT`

The folder containing the `plc.json` file, i.e. the root of the project.

### `ARCH`

The target architecture currently being built, for a multi architecture build. The value for `ARCH` will be updated for every target.

### `BUILD_LOCATION`

`BUILD_LOCATION` is the folder where the build will be saved. This is the value of either the [`--build-location`](#build-location) parameter or the default build location.

### `LIB_LOCATION`

`LIB_LOCATION` is the folder where the lib will be saved. This is the value of either the [`--lib-location`](#lib-location) parameter or the [build location](#build-location).

> TODO : Example

