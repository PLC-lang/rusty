# Libraries

RuSTy does not currently have support for importing _source based libraries_.

Source based libraries can, however, be compiled together with the application as normal files.

Precompiled libraries or system functions can be added using compilation flags or an entry in the `plc.json` file.

System functions can also be added using [External Function](libraries/external_functions.md) for each POU in that library.

## Library Structure

A library is defined by :

- A set of `st` interfaces, each interface represents a function that has been precompiled.

 > In a POU, the interface is the definition and variable section e.g:
>
 > ```iecst
 > (*Interface for program example *)
 > PROGRAM example
 > VAR_INPUT
 >  a,b,c : DINT
 > END_VAR 
 > (* End of interface *)
 >
 > (* Implementation *)
 > END_PROGRAM
 > ```
>
- A binary file for each architecture the library has been built for (`x86_64-linux-gnu`, `aarch64-linux-gnu`, ..)

## Linking libraries using the `rustyc` command line

To include a library when using the `rustyc` command line interface, the include files can be added using the `-i` flag.

Each [POU](pous.md), [Global Variable](variables.md), or [Datatype](datatypes.md) defined in the included files will be added to the project.
POUs and Global variables included with the `-i` are marked as external, the implementation part of a POU is ignored.

To link the library, two options are then available: [Shared](#shared-libraries) and [Static](#static-libraries) libraries.

### Shared Libraries

A shared library (i.e. extension `.so`) can be linked using the `-l` flag. </br>
For a library called `mylib`, when the flag `-lmylib` is passed, the linker will search for a file called `libmylib.so`.

> Note that the `lib<LibName>.so` format is required by the linker for unix like systems.

The library locations used by the linker are the default search locations of the linker (i.e. `/usr/lib`, `/lib`), additional paths can be provided using the `-L` flag (e.g `-L/opt/lib` will make the linker also search for files in /opt/lib).</br>
Additional library locations can be provided by supplying additional `-L` entries.</br>
Additionally, the environment variable `LD_LIBRARY_PATH` can be defined to append entries to the linker's search location. More information can be found [here](https://tldp.org/HOWTO/Program-Library-HOWTO/shared-libraries.html).

### Static Libraries

Static libraries compiled as object files can be linked by simply passing the object file (i.e. extension `.o`) as an input (simlar to other `.st` files).

Archive files (i.e. extension `.a`) can be linked similarly to [Shared Libraries](#shared-libraries) using the `-l` flag.
If the application is being compiled with the `--static` flag (or no shared library (`.so`) is found), the linker will use the archive file.</br>

> If neither a shared object (`.so`) or an archive file (`.a`) is found, compilation will fail.

### Command line example

To compile a file called `input.st` including a header and linking a library called `libiec.so` from `/lib` :

```sh
rustyc input.st -i iec/header.st -L/lib/ -liec
```

## Linking libraries using the Build Description File `plc.json`

Libraries can be added to a project managed with a [Build Description File](using_rusty/build_description_file.md#build-description-file-plcjson). </br>
To add a library to the project, the `"libraries"` section can be used.
A library entry requires a `name`, a `path`, the `package` behaviour, and a set of files to include (`include_path`).

### `name`

The name of the library to be linked. This will be used by the linker to find the library. </br>
A library with the name `mylib` must have an equivalant compiled file called `libmylib.so`.
> Note, archive files (ending with `.a`) are currently not supported.

### `path`

The location of the library to be linked. The path can be either absolute or relative to the project. </br>

### `package`

The packaging option for the library, i.e wether the library should be copied or is already available on the system.</br>
The value <a name="copy">`"Copy"`</a> indicates that the given library should be copied to the [Library Location](#library-location). </br>
The value <a name="system">`"System"`</a> indicates that the given library exists on the system and does not need to be copied.

### `include_path`

A list of files (can include globs) that should be included with the project.</br>
Each [POU](pous.md), [Global Variable](variables.md), or [Datatype](datatypes.md) defined in the included files will be added to the project.
POUs and Global variables included in the list are marked as external, the implementation part of a POU is ignored.

### Library Location

Libraries marked as `Copy` will be copied during the compilation to the defined [Library Location](using_rusty/build_description_file.md#--lib-location).
By default this is the same as the [Build Location](using_rusty/build_description_file.md#--build-location) unless overridden by the `--lib-location` parameter.

### Using environment variables

Since libraries can be compiled for multiple targets, the lib path can contain environment variables to disambiguate the compile location.
`$ARCH` can be used as placeholder in the path to indicate the the currently compiled target.
</br></br></br>

> During linking, if no `.so` file with name [`lib<name>.so`](#name) is found, the compilation will fail.

### Configuration Example (`plc.json`)

A configuration example for a `Copy` library called _mylib_ and a `System` library called _std_ :

```json
"libraries" : [
    {
        "name" : "mylib",
        "path" : "libs/$ARCH/",
        "package" : "Copy",
        "include_path" : [
            "simple_program.st"
        ]
    },
    {
        "name" : "std",
        "path" : "libs/$ARCH/",
        "package" : "System",
        "include_path" : [
            "include/*.st"
        ]
    }
]
```
