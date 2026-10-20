# extern_st_interface_conversion

Tests that the itable pointers in a vtable have the same layout when a library and an application
are compiled separately. The library defines `Dog IMPLEMENTS ISpeaker`; the application declares it
as `{external}` and extends it with `Puppy`, which overrides `say` and adds `ITagged`.

## Build

1. `plc lib.st --shared -o libifaceconv.so`
2. `plc app.st -L... -lifaceconv --linker=cc` (no `--generate-external-constructors`)

## What this tests

- **Library reads an application vtable**: `Dog.describe` (library) converts `THIS^` to `ISpeaker`
  and reads the itable from the vtable of a `Puppy` instance, which the application built.
- **Application reads its own vtable through a base pointer**: `s := pd^` with `pd : POINTER TO Dog`
  selects the itable of `Puppy`, and `speak` (library) dispatches through it.
