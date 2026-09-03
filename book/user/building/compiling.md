# Compiling

The compiler takes source files and produces one artifact. This chapter covers the options that a normal build needs. The [command line reference](../reference/command-line.md) lists all of them.


## Files in, artifact out

```bash
plc main.st                      # one file
plc main.st motor.st sensor.st   # several files
plc "src/**/*.st"                # everything below src
```

Quote a pattern, so that the compiler expands it and not the shell.

Without an option that says otherwise, the compiler links an executable and names it after the first input file, here `main.st.out`. `-o` gives it another name.


## Build a project

Once a build needs more than a file list, put it into `plc.json` next to the sources:

```json
{
    "name": "plant",
    "files": [ "src/**/*.st" ],
    "compile_type": "Static",
    "output": "plant"
}
```

```bash
plc build
```

`plc build` reads `plc.json` from the current directory, or from the path that you give it. Everything lands in `build/`: one object file per source file, and the artifact. `--build-location` moves that directory.

The [project file reference](../reference/project-file.md) describes every key, including the libraries.


## Choose what to produce

| Option | Artifact |
|---|---|
| none | An executable |
| `-c` | An object file, not linked |
| `--shared` | A shared object |
| `--ir` | LLVM intermediate representation, as text |

In a project file, the key `compile_type` does the same with the values `Static`, `Object`, `Shared`, `Relocatable`, `Bitcode`, and `IR`.

> [!NOTE]
> `Static` and `--static` mean "link the units into one executable". They do not produce a binary without dynamic libraries; the system libraries stay dynamic.


## Optimization

```bash
plc main.st -O aggressive
```

The four levels are `none`, `less`, `default`, and `aggressive`, and they are the levels of LLVM from `-O0` to `-O3`. The default is `default`. Use `none` while you debug, because the generated code then follows the source closely.


## Check without producing anything

```bash
plc --check "src/**/*.st"
plc check plc.json
```

Both run the compiler up to validation and report every diagnostic, which is what an editor or a pre-commit hook needs.


## Build for another machine

```bash
plc main.st --target aarch64-linux-gnu --sysroot /opt/toolchains/aarch64 -o app
```

`--target` takes a target triple that LLVM knows, and `--sysroot` tells the linker where the headers and libraries of that target are. The compiler builds for one target per run, so a build for two machines runs twice.


## Speed

The compiler uses every core of the machine. `-j 4` limits it to four threads.

Each unit becomes its own module, and the linker joins them. `--single-module` builds one module for the whole project instead, which is slower but sometimes necessary for a tool that reads the result.


## Which compiler built an artifact

Every artifact carries the version of the compiler that produced it, in the `.comment` section, next to the lines of the linker and of the C runtime:

```bash
readelf -p .comment app
```

```
plc version 1.1.0-dev (Thu Sep 10 12:08:58 2026 +0200, 6f6e1d7f2db)
```

`plc --version` prints the same version, date, and commit, so a deployed binary can be matched to the compiler that built it. A pipeline that needs identical artifacts across compiler updates suppresses the line with `--fno-ident`.


## What's next

The compiler produced object files. The [next chapter](linking.md) joins them with libraries into the final artifact.
