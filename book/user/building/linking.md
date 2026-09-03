# Linking and Libraries

The compiler does not link by itself. It writes object files and then calls a linker program with them, with the libraries that you named, and with the options below.


## Which linker runs

`--linker=<command>` names a linker with a `cc` compatible command line:

```bash
plc main.st -o app --linker=cc        # Linux
plc main.st -o app --linker=clang     # macOS, Windows
```

Without the option, the compiler takes the first of `cc`, `clang`, `ld.lld`, and `ld` that exists and supports the target. A compiler driver, `cc` or `clang`, is the better choice, because it knows the startup files and the default libraries of the platform. A bare linker cannot produce an executable that starts.

Two options adjust the driver: `--fuse-ld=<name>` selects its back end linker, for example `mold`, and `--linker-arg=<argument>` passes one argument through to the linker.


## Using a library

A library has two parts, and you need both. The declarations tell the compiler what exists, and the binary provides the code:

```bash
plc main.st -i "vendor/include/*.st" -L vendor/lib -l vendor -o app --linker=cc
```

`-i` reads a file of declarations. Everything in it is external: the compiler takes the interfaces and ignores the bodies. `-l` names a library, so `-lvendor` links `libvendor.so`, and `-L` adds a directory to search. Two more forms exist: `-l:libvendor.so.1` names an exact file, and `-l/opt/lib/libvendor.so.1` links that path.

An object file is an input like a source file:

```bash
plc main.st helper.o -o app --linker=cc
```

In a project file, the `libraries` key holds the same information and adds packaging, which the [project file reference](../reference/project-file.md) describes.


## The standard library

The functions of IEC 61131-3, the timers, the counters, and the string operations live in `iec61131std`. A release installs it, and a project that uses any of them links it:

```bash
plc main.st -i "/usr/share/plc/include/*.st" -l iec61131std -o app --linker=cc
```

Some language features call it as well, for example `**` and the comparison of text, so link it whenever you are not sure.


## Missing symbols

A shared object is linked with `--no-undefined`, so a symbol that nothing defines fails the build instead of failing later, when someone loads the library. `--allow-undefined-symbols` turns that off, for the case where the host program provides the symbols.


## Position-independent code

The compiler generates position-independent code for a shared object and the platform default for everything else. `--fpic` and `--fno-pic` force one of the two, and they exclude each other.

On x86_64 the generated code is position-independent anyway, and the visible effect is at link time: with `--fno-pic` the compiler passes `-no-pie` to the linker, which produces an executable that is not position-independent. On 32-bit x86 and on some ARM configurations the generated code differs as well. A shared object without position-independent code fails to link on targets that require it, exactly as with `gcc` and `clang`.


## Windows

Linking on Windows uses the Microsoft libraries, so the toolchain needs three things: the Windows SDK and MSVC, an `LIB` environment variable that holds the directories of `iec61131std.lib`, `ws2_32.lib`, `ntdll.lib`, `userenv.lib`, `libcmt.lib`, `oldnames.lib`, and `libucrt.lib`, and a restarted terminal so that the variable is visible.

A shared library also needs a file that lists the exported names:

```
EXPORTS
    main
```

```bash
plc hello_world.st -c -l iec61131std -l ws2_32 -l ntdll -l userenv -o hello_world.o
clang hello_world.o --shared -l iec61131std -l ws2_32 -l ntdll -l userenv ^
    -fuse-ld=lld-link "-Wl,/DEF:exports.def" -o hello_world.dll
```


## Bare metal

`--nocrt` leaves out the C runtime startup files, and `--nolibc` leaves out the default C libraries. Both are for targets that bring their own startup code. `--script <file>` gives the linker a linker script.


## What's next

When a build fails, the compiler tells you why. The [next chapter](diagnostics.md) explains how to read that and how to change it.
