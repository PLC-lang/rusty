# Debug Path Remapping

When RuSTy emits debug information, it stores source file paths in DWARF.

For local development this is usually fine, but for shipped binaries and remote debugging you often want two properties:

- no developer-local checkout paths inside the artifact
- stable, predictable paths that can be mapped by an IDE or `gdb`

RuSTy supports this with debug path remapping flags.

## Recommended shipped-build convention

For remote debugging, use stable virtual roots such as:

- source root: `/src/<Product>`
- build root: `/build/<Product>`

Example for an application called `MyApp`:

- `/src/MyApp`
- `/build/MyApp`

Your debugger frontend can then map those virtual paths to local folders with `set substitute-path`.

## Default behavior

If no remapping flags are used:

- RuSTy keeps using resolved/canonicalized filesystem paths internally for file loading
- DWARF compile units are emitted **relative to the compilation directory when possible**
- the DWARF compilation directory itself may stay absolute

This is intentionally closer to clang-style debug path behavior than always embedding absolute source paths in `DW_AT_name`.

## `--file-prefix-map OLD=NEW`

Repeatable flag.

Remaps debug/source paths whose resolved path starts with `OLD` so that they instead start with `NEW`.

Typical usage:

```bash
plc -g \
  --file-prefix-map /home/alice/work/MyApp=/src/MyApp \
  --file-prefix-map /home/alice/work/MyApp/build=/build/MyApp \
  app.st
```

### Notes

- `OLD` is resolved against the current working directory and canonicalized when possible.
- `NEW` is normalized for the host platform before being embedded in debug info.
- when multiple mappings match, the longest matching prefix wins.
- this affects DWARF file paths such as compile-unit and `DIFile` entries.

Use absolute virtual roots for predictable results.

## `--debug-prefix-map OLD=NEW`

Alias for `--file-prefix-map`.

This exists to make the CLI easier to align with GCC/Clang conventions in build systems and templates.

## `--debug-compilation-dir DIR`

Overrides the DWARF compilation directory (`DW_AT_comp_dir`).

Example:

```bash
plc -g \
  --file-prefix-map /home/alice/work/MyApp=/src/MyApp \
  --debug-compilation-dir /src/MyApp \
  app.st
```

This is useful when you want compile units to be emitted relative to a stable virtual root instead of the current working directory or project root.

## Recommended example

For a shipped build of `MyApp`:

```bash
plc -g \
  --file-prefix-map /real/source/root=/src/MyApp \
  --file-prefix-map /real/build/root=/build/MyApp \
  --debug-compilation-dir /src/MyApp \
  ...
```

## Parent-relative source paths

A common layout is invoking the compiler from a build/test subdirectory while compiling a parent file (for example `../main.st`).

Example:

```bash
cd examples/test
plc -g ../main.st \
  --file-prefix-map "$(pwd)=/root" \
  --debug-compilation-dir "$(pwd)"
```

With this setup, RuSTy emits compile-unit paths like:

- `DW_AT_comp_dir = /root`
- `DW_AT_name = ../main.st`

This avoids embedding host-local absolute paths in compile-unit filenames for parent-relative sources.
For this scenario, behavior aligns with clang when using `-ffile-prefix-map` together with `-fdebug-compilation-dir`.

## `plc build` usage

The same options can be passed to the `build` subcommand:

```bash
plc build plc.json \
  -g \
  --file-prefix-map /real/source/root=/src/MyApp \
  --file-prefix-map /real/build/root=/build/MyApp \
  --debug-compilation-dir /src/MyApp
```

## GDB usage

If DWARF contains `/src/MyApp` and `/build/MyApp`, a local debugger can map them like this:

```gdb
set substitute-path /src/MyApp /home/bob/dev/MyApp
set substitute-path /build/MyApp /home/bob/dev/MyApp/build
```

For a remote session:

```gdb
file /path/to/local/unstripped/binary
set substitute-path /src/MyApp /home/bob/dev/MyApp
set substitute-path /build/MyApp /home/bob/dev/MyApp/build
target remote <host>:<port>
```

### Windows note

For cross-machine debugging on Windows, prefer virtual roots like `/src/MyApp` and `/build/MyApp`
instead of drive-bound debug paths (`C:\...`).

Drive letters can differ between systems (for example build machine on `C:` and debugger user on `D:`),
while virtual roots keep `set substitute-path` stable across users and hosts.

## Mixed C/C++ and ST projects

If the project also builds C/C++ code with clang, use the same virtual roots on both sides.

Typical clang/CMake setup:

```text
-ffile-prefix-map=<real-source-root>=/src/MyApp
-ffile-prefix-map=<real-build-root>=/build/MyApp
```

Then pass the matching RuSTy flags shown above.

Using the same convention for both compilers keeps source lookup consistent in Eclipse, CLI `gdb`, and DAP frontends.

