# Build description File

In addition to the comprehensive help, `rustyc` offers a build description file, that simplifies the build process. Instead of having numerous inline arguments, using the build description file makes passing the arguments easier and neater. The build description file needs to be safed as a [json](https://en.wikipedia.org/wiki/JSON) format.

`rustyc build [Config]`

Note that if `rustyc` cannot find the `plc.json` file, it will throw an error and request the path. The default location for the build file is the current directory. The command for building with an additional path looks like this:

`rustyc build src/plc.json`


# Plc.json

For the build description file to work, the build description file must be the [json](https://en.wikipedia.org/wiki/JavaScript_Object_Notation) format. All the keys used in the build description file are described in the following sections. 


## files

The key `files` is the equivalent to the `input` parameter, which adds all the `.st` files that needs to be compiled. The value of `files` is an array of strings, definied as followed:
```json
"files" : [
    "examples/hello_world.st",
    "examples/hw.st"
]
```


## libraries

To link several executables `rustyc` has the option to add libraries and automatically build and like them together. if no compile type has been selected `rustyc` will link the files on default.

```json
"libraries" : [
    {
        "name" : "iec61131std",
        "path" : "path/to/lib/",
        "include_path" : [
            "examples/hw.st",
            "examples/hello_world.st"
        ]
    }
]
```

## output

Similarly to specifying an output file via the `-o` or `--output` option, in the build file we use `"output" : "output.so"` to define the output file. The default location is likewise to the location for the build file, namely the current directory. 



## Optional Keys
### sysroot

`rustyc` is using the `sysroot` key for linking purposes. It is considered to be the root directory for the purpose of locating headers and libraries.


### target

To build and compile [structured text](https://en.wikipedia.org/wiki/Structured_text) for the rigth platform we need to specify the `target`. As `rustyc` is using [LLVM](https://en.wikipedia.org/wiki/LLVM) a target-tripple supported by LLVM needs to be selected. The default `target` is the host machine's target. So if a dev container on an `x86_64-docker` is used the target is `x86_64-linux-gnu`.


### compile_type
There are six options for choosing the `compile_type`. The valid options are:
<!-- TODO we should probably describe what each of those options do -->
- `Static` bindings have to be done at compile time
- `PIC` Position Independent Code
- `Shared` (dynamic) binginds will be done dynamically
- `Relocatable` generates Relocatable 
- `Bitcode` adds bitcode alongside machine code in executable file
- `IR` Intermediate Representation

To specify which of the above mentioned compile formats is wanted, it needs to be added to the build description file as followed: `"compile_type" : "Shared"`.

# Example
```json
{
    "files" : [
        "examples/hw.st",
        "examples/hello_world.st",
        "examples/ExternalFunctions.st"
    ],
    "compile_type" : "Shared",
    "output" : "proj.so",
    "libraries" : [
        {
            "name" : "iec61131std",
            "path" : "path/to/lib",
            "include_path" : [
                "examples/hw.st"
            ]
        },
        {
            "name" : "other_lib",
            "path" : "path/to/lib",
            "include_path" : [
                "examples/hello_world.st"
            ]
        }
    ]
}
```