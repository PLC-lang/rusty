# Compiling

The compiler takes source files and produces one artifact. This chapter covers the options that a normal build needs. The [command line reference](../reference/command-line.md) lists all of them.


## Files in, artifact out

```bash
plc main.st                      # one file
plc main.st motor.st sensor.st   # several files
plc "src/**/*.st"                # everything below src
```

Quote a pattern, so that the compiler expands it and not the shell.

Without an option that says otherwise, the compiler links an executable and names it after the first input file, here `main.st.out`. `-o` gives it another name. A pattern has no first file, so the name then comes from the pattern text and you get a file called `*.st.out`. Always give `-o` with a pattern.


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

`plc build` reads `plc.json` from the current directory, or from the path that you give it. Everything lands in `build/`: the artifact, and one object file per source file under the path of the source, so `src/motor.st` becomes `build/src/motor.st.o`. `--build-location` moves that directory.

The [project file reference](../reference/project-file.md) describes every key, including the libraries.


## Choose what to produce

| Option | Artifact |
|---|---|
| none | An executable |
| `-c` | An object file, not linked |
| `--shared` | A shared object |
| `--ir` | LLVM intermediate representation, as text |
| `--xml-omron` | IEC 61131-10 XML format for Omron Sysmac Studio |

In a project file, the key `compile_type` does the same. Use `Static`, `Object`, `Shared`, `Relocatable`, `Bitcode`, or `IR`.

> [!NOTE]
> `Static` and `--static` mean "link the units into one executable". They do not produce a fully static binary. The system libraries, the C library included, stay dynamic.


## Library namespaces in the Omron XML

Sysmac Studio keeps library types in a namespace and refers to them with a backslash, as in `Common\ServoDev`. `--xml-omron` writes that qualified form when the file that declares the type carries the [`{namespace}` attribute](../language/source-files.md#library-namespaces).

```iecst
{namespace := 'Common'}

{external}
TYPE ServoDev : STRUCT
    Position: LREAL;
END_STRUCT END_TYPE
```

A type in another file that refers to `ServoDev` then exports as `Common\ServoDev`, and an array exports as `ARRAY[0..9] OF Common\LogEntry`. Sysmac Studio resolves the member against the library and keeps it. Without the attribute the export says `ServoDev`, Sysmac Studio finds no such type in the global namespace, and it discards the member when you import the file.

Declarations that the namespaced file owns go into a `<NamespaceDecl>` element inside `<GlobalNamespace>`:

```xml
<Types>
  <GlobalNamespace>
    <DataTypeDecl name="TransferArmMod">
      <UserDefinedTypeSpec xsi:type="StructTypeSpec">
        <Member name="RotationalServo">
          <Type>
            <TypeName><![CDATA[Common\ServoDev]]></TypeName>
          </Type>
        </Member>
      </UserDefinedTypeSpec>
    </DataTypeDecl>
    <NamespaceDecl name="Common">
      <DataTypeDecl name="ServoDev">
        ...
      </DataTypeDecl>
    </NamespaceDecl>
  </GlobalNamespace>
</Types>
```

An `{external}` type declares the name but contributes no declaration, so a library of external types qualifies the references and writes no `<NamespaceDecl>` element. An empty namespace never reaches the file.


## POU comments in the Omron XML

The body that `--xml-omron` writes is the source text between the first statement and the last, so a comment reaches the `<ST>` element only when statements surround it. A comment above the POU, or one in the declaration section, falls outside that range.

The comment that stands immediately above a POU becomes its `<Documentation>` instead:

```iecst
(*
    Moves the transfer arm to the upper, middle or lower position
*)
FUNCTION_BLOCK TransferArmLiftSubSequence
```

```xml
<FunctionBlock name="TransferArmLiftSubSequence">
  <Documentation xsi:type="SimpleText"><![CDATA[Moves the transfer arm to the upper, middle or lower position]]></Documentation>
  <AddData>
    ...
```

A `(* *)` block and a run of `//` lines both work. The compiler removes the comment markers and the indentation that every line shares, so the relative indentation of a list stays. A comment that holds no text writes no element.

A variable takes its comment the same way, which fills the comment column of the Sysmac variable table. A comment on the same line documents that variable, and a comment on the line above documents the variable below it:

```iecst
VAR_GLOBAL
    DO_CameraCapture: BOOL;                 // Camera capture output
    DI_PlateEjectCylinderRetracted: BOOL;   (* Eject cylinder retracted limit *)
    (* Extends the plate eject cylinder *)
    DO_PlateEjectCylinderExtend: BOOL;
END_VAR
```

This applies to every variable that the export writes, global or local, and a variable with no comment writes no element.


## Optimization

```bash
plc main.st -O aggressive
```

The four levels are `none`, `less`, `default`, and `aggressive`, and they are the levels of LLVM from `-O0` to `-O3`. The default is `default`. Use `none` while you debug, because the generated code then follows the source closely. The level changes the machine code only. The text that `--ir` writes is the same at every level.


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

Each unit becomes its own module, and the linker joins them. `--single-module` builds one module for the whole project instead, which is slower but sometimes necessary for a tool that reads the result. A `plc build` of the project above then writes one object file, `build/src/main.st.o`, in place of three.


## Which compiler built an artifact

Every artifact carries the version of the compiler that produced it. In a linked artifact it sits in the `.comment` section, next to the lines of the linker and of the C runtime:

```bash
readelf -p .comment app
```

```
String dump of section '.comment':
  [     1]  Linker: Ubuntu LLD 21.1.8
  [    1b]  plc version 1.1.0-dev (Thu Sep 10 12:08:58 2026 +0200, 6f6e1d7f2db)
  [    5f]  GCC: (Ubuntu 15.2.0-16ubuntu1) 15.2.0
```

The version, the date, and the commit are the ones of the compiler that you used, and `plc --version` prints the same three. A deployed binary can therefore be matched to the compiler that built it. A pipeline that needs identical artifacts across compiler updates suppresses the line with `--fno-ident`.


## What's next

The compiler produced object files. The [next chapter](linking.md) joins them with libraries into the final artifact.
