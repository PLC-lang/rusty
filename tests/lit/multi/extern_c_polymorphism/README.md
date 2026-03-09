# extern_c_polymorphism

Tests polymorphic dispatch (vtable + itable) when a function block is implemented in C and
declared as `{external}` in ST. The compiler generates the vtable and itable plumbing via
`--generate-external-constructors` because C has no knowledge of these constructs.

## Setup

- `foo.c` — C implementation of `ExternalFb`: provides the FB body, `FB_INIT`, and `greet` method.
- `header.pli` — ST declaration of `ExternalFb` as `{external} FUNCTION_BLOCK ... IMPLEMENTS IGreeter`.
  Included via `-i`, giving it `Include` linkage.
- `main.st` — Defines `INTERFACE IGreeter` and a `LocalFb` (pure ST) implementing the same interface.
  Exercises both dynamic vtable dispatch (via pointer variables) and interface-typed calls
  (itable dispatch) for both the external and local FBs.

## Build

1. `gcc -shared -fPIC -o libfoo_cpoly.so foo.c`
2. `plc main.st -i header.pli -L... -lfoo_cpoly --generate-external-constructors --linker=cc`

## What this tests

- **Vtable dispatch**: calls through pointer variables like `pExt^.greet()` go through the vtable,
  which the compiler generates and initializes via `--generate-external-constructors`. The vtable
  generator is linkage-aware and correctly handles `Include`/`External` POUs.
- **Itable dispatch**: interface calls like `iface := ext; iface.greet()` go through the itable.
  The itable generator is linkage-aware (mirroring the vtable generator) and with
  `--generate-external-constructors`, generates internal itable instances for `{external}` POUs.
  The initializer module emits the unit constructor for Include units when the flag is set,
  ensuring the itable instances are properly initialized with function pointers.
