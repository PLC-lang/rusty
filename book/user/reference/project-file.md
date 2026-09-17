# Project File

Every key of `plc.json`, for looking up. [Compiling](../building/compiling.md) explains how a project is built.

The file holds the inputs, the artifact kind, and the libraries of a build. It is a JSON file, and it is called `plc.json` by convention.

```bash
plc build                 # reads ./plc.json
plc build src/plc.json    # reads the given file
```

`plc check`, `plc config`, and `plc generate` take the same argument.


## Keys

| Key | Necessary | Default | Purpose |
|---|---|---|---|
| `name` | yes | | The name of the project, and the name of the artifact |
| `files` | yes | | The source files, as paths or glob patterns |
| `compile_type` | no | `Static` | What the build produces |
| `output` | no | `<name>` with the extension of the format | The name of the artifact |
| `libraries` | no | none | The libraries to include and to link |
| `version` | no | none | Free text, for your own use |
| `format_version` | no | none | Free text, for your own use |

Any other key is an error, and the message names the keys that the compiler accepts. Two of those keys are not in the table above, because they do nothing: `package_commands` is read and never used, and `format-version` is a second spelling of `format_version`.

```json
{
    "name": "motor",
    "files": [ "src/**/*.st" ],
    "compile_type": "Shared",
    "output": "libmotor.so"
}
```


## compile_type

| Value | Result |
|---|---|
| `Object` | One object file with all units, no link step |
| `Static` | An executable |
| `Shared` | A shared object |
| `Relocatable` | One object file, combined by a partial link |
| `Bitcode` | LLVM bitcode |
| `IR` | LLVM intermediate representation |

> [!WARNING]
> **Deprecated.** `PIC` and `NoPIC` are `Shared` with a fixed relocation model. Use `Shared` with `--fpic` or `--fno-pic`.


## libraries

A library entry adds the declarations of a precompiled library to the project, and links the library:

```json
"libraries": [
    {
        "name": "iec61131std",
        "path": "libs/",
        "link_path": "libiec61131std.so.1",
        "package": "Copy",
        "include_path": [ "include/*.st" ]
    }
]
```

| Key | Necessary | Purpose |
|---|---|---|
| `name` | yes | The name for the linker. `mylib` links `libmylib.so` |
| `path` | yes | The directory of the library, absolute or relative to the project |
| `package` | yes | How the library reaches the target system |
| `include_path` | yes | The declaration files of the library, resolved against `path`. Their bodies are ignored |
| `link_path` | no | An exact file to link instead of the name, for example `libmylib.so.1`. A relative value resolves against `path` |
| `architectures` | no | Accepted and never used |

`package` takes these values:

| Value | Meaning |
|---|---|
| `Copy`, `Local` | The library is copied to the library location of the build |
| `System` | The library is already on the target system |
| `Static` | The library is linked statically |


## Where the build writes

| Location | Default | Option |
|---|---|---|
| Intermediate objects and the artifact | `build`, next to the project file | `--build-location <dir>` |
| Copied libraries | the build location | `--lib-location <dir>` |

`--lib-location` exists on `build` only, and the directory must exist already. Outside `build`, the compiler writes intermediate objects to the temporary directory of the operating system unless `--build-location` is given, and `-o` always resolves against the current directory.


## Environment variables

A `$NAME` in any value is replaced with the value of the environment variable `NAME` before the file is read. A variable that is not set stays as written.

```bash
SYSROOT=/opt/toolchain plc build
```

```json
"libraries": [
    { "name": "vendor", "path": "$SYSROOT/lib", "package": "System", "include_path": [ "vendor.st" ] }
]
```


## Validation

The compiler validates the file against a JSON schema before the build starts. The schema is part of the compiler, and `plc config schema` prints it:

```bash
plc config schema > plc-json.schema
```

Give that file to your editor to get completion and validation while you write the project file. The schema is stricter than the compiler in one place: it marks `compile_type` as necessary, and the compiler takes the default instead.

The project file itself takes no `$schema` key, because the compiler rejects every key that it does not know.
