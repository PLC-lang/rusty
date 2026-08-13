# Standard Library Edge-Case Tests

This directory contains integration tests for stdlib functions on inputs that
used to panic (overflow, underflow, division by zero, out-of-range string
arguments). The functions now handle these defensively — a misbehaving PLC
program must not bring down the runtime process — so every test asserts a
concrete result instead of expecting a crash.

## Behavior under test

### TIME arithmetic (32-bit, milliseconds)
- `add_time_overflow.st` / `sub_time_overflow.st` — wraps modulo 2^32
- `mul_time_signed_overflow.st` / `mul_time_unsigned_overflow.st` — wraps in
  modular 64-bit arithmetic, truncated to the 32-bit TIME result
- `mul_time_real_overflow.st` / `mul_time_lreal_overflow.st` — saturates at the
  LINT range
- `div_time_by_zero.st` / `div_time_by_real_zero.st` /
  `div_time_by_lreal_zero.st` — yields zero

The wrapping semantics match both CODESYS and the native code the compiler
emits for the corresponding operators on TIME values.

### String functions
- `right_string_substring_too_long.st` / `right_wstring_substring_too_long.st`
  — over-long substring requests are clamped to the whole string

## History

These tests originally documented the panicking behavior via `XFAIL: *` (a
panic cannot unwind across the FFI boundary in the Rust integration tests, see
the git history of this directory). Since the standard library no longer
panics, they are regular positive tests now.

## Running the tests

```bash
lit -v tests/lit/single/stdlib_overflow/
```

Or run the entire lit test suite:

```bash
./scripts/build.sh --build --lit
```
