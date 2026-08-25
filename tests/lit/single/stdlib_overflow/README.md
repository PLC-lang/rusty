# Standard Library Overflow Tests

This directory contains integration tests for stdlib functions on overflow, underflow, and division-by-zero inputs.

## Background

These tests were originally part of the Rust integration test suite in `libs/stdlib/tests/date_time_numeric_functions_tests.rs`, but panic behavior could not be tested there because:

1. The Rust test infrastructure compiles PLC code into a shared library and loads it dynamically using `libloading`
2. When a panic occurs in dynamically loaded code, Rust's panic unwinding mechanism cannot cross the FFI boundary
3. Even with `#[should_panic]` or `std::panic::catch_unwind()`, the panic causes an abort with the error:
   ```
   fatal runtime error: Rust cannot catch foreign exceptions, aborting
   ```

## Test Coverage

### TIME Arithmetic (wrapping)

The stdlib ADD/SUB/MUL date and time functions wrap on overflow, exactly like the CODESYS operators. These tests run to completion and check the wrapped values:

- `add_time_overflow.st` - ADD/ADD_TIME past the TIME range wraps mod 2^32
- `sub_time_overflow.st` - SUB/SUB_TIME below zero and SUB_DATE_DATE past the TIME range wrap mod 2^32
- `mul_time_signed_overflow.st` - MUL with a signed integer wraps mod 2^64
- `mul_time_unsigned_overflow.st` - MUL with an unsigned integer wraps mod 2^64
- `dt_tod_overflow.st` - ADD_DT_TIME/SUB_DT_TIME past the DT range and SUB_TOD_TOD below zero wrap mod 2^32

### Float Factors (saturating)

MUL and DIV with a REAL/LREAL factor cannot wrap meaningfully; the functions follow float semantics mapped to the TIME range. Oversized and infinite results (including division by zero) saturate at the TIME range, and a NaN factor or NaN result (zero divided by zero) yields zero:

- `mul_time_real_overflow.st` - Multiplying TIME by an oversized or NaN REAL
- `mul_time_lreal_overflow.st` - Multiplying TIME by an oversized or NaN LREAL
- `div_time_by_real_zero.st` - Dividing TIME by zero (REAL)
- `div_time_by_lreal_zero.st` - Dividing TIME by zero (LREAL)

### Panic Conditions (`XFAIL: *`)

The remaining conditions panic at runtime by design (integer division by zero is treated as a programming error). Their tests carry `XFAIL: *`, so a non-zero exit code is treated as success:

- `div_time_by_zero.st` - Dividing TIME by zero (LINT)
- `right_string_substring_too_long.st` / `right_wstring_substring_too_long.st` - RIGHT with a length above the string length

## Running the Tests

To run all stdlib overflow tests:
```bash
lit -v -DLIB=output -DCOMPILER=target/debug/plc tests/lit/single/stdlib_overflow/
```

Or run the entire lit test suite:
```bash
./scripts/build.sh --lit
```
