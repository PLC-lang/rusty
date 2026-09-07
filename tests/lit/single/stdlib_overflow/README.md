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

The stdlib ADD/SUB/MUL date and time functions wrap on overflow. TIME and the other
date/time types are signed 64-bit on this branch, so wrapping happens mod 2^64 at the
i64 range bounds and deficits below zero stay plain negative values. These tests run
to completion and check the wrapped values:

- `add_time_overflow.st` - ADD/ADD_TIME past the TIME range wraps mod 2^64
- `sub_time_overflow.st` - SUB/SUB_TIME below the range minimum wraps mod 2^64, SUB_DATE_DATE yields the signed difference
- `mul_time_signed_overflow.st` - MUL with a signed integer wraps mod 2^64
- `mul_time_unsigned_overflow.st` - MUL with an unsigned integer wraps mod 2^64
- `dt_tod_overflow.st` - ADD_DT_TIME past the DT range wraps mod 2^64, SUB_DT_TIME/SUB_TOD_TOD below zero yield signed values
- `concat_invalid_inputs.st` - CONCAT_DATE/CONCAT_TOD yield 0 for unrepresentable inputs and clamp to the DATE range
- `lreal_to_string_huge_negative.st` - LREAL_TO_STRING picks scientific notation by magnitude instead of overflowing the buffer
- `dt_to_date_first_day.st` - DATE_AND_TIME_TO_DATE saturates at the DATE range minimum on the first representable day

### Float Factors (saturating)

MUL and DIV with a REAL/LREAL factor cannot wrap meaningfully; the functions follow float semantics mapped to the TIME range. Oversized and infinite results (including division by zero) saturate at the TIME range, and a NaN factor or NaN result (zero divided by zero) yields zero:

- `mul_time_real_overflow.st` - Multiplying TIME by an oversized or NaN REAL
- `mul_time_lreal_overflow.st` - Multiplying TIME by an oversized or NaN LREAL
- `div_time_by_real_zero.st` - Dividing TIME by zero (REAL)
- `div_time_by_lreal_zero.st` - Dividing TIME by zero (LREAL)

### String Parameters (clamping)

Out-of-range string function parameters clamp or leave the base string unchanged, matching CODESYS:

- `string_param_clamping.st` - LEFT/MID/INSERT/DELETE/REPLACE with out-of-range lengths and positions
- `right_string_substring_too_long.st` / `right_wstring_substring_too_long.st` - RIGHT with a length above the string length clamps to the whole string

### Invalid String Content (lossy decoding)

Strings can carry bytes that are not valid UTF-8/UTF-16 (pointer writes, comms buffers, C interop). The string functions decode them lossily (invalid parts become U+FFFD) instead of aborting the process; CONCAT copies raw code units unchanged:

- `lossy_string_decoding.st` - LEN/FIND/CONCAT/STRING_TO_WSTRING on a `16#FF`-patched STRING, LEN/LEFT on a WSTRING holding an unpaired surrogate

### Panic Conditions (`XFAIL: *`)

The remaining conditions panic at runtime by design (integer division by zero is treated as a programming error). Their tests carry `XFAIL: *`, so a non-zero exit code is treated as success:

- `div_time_by_zero.st` - Dividing TIME by zero (LINT)

### FFI Return Width Tests

- `sub32_ffi_return_byte.st` / `sub32_ffi_return_signed.st` / `sub32_ffi_return_word.st` - sub-32-bit FFI return handling

## Running the Tests

To run all stdlib overflow tests:
```bash
lit -v -DLIB=output -DCOMPILER=target/debug/plc tests/lit/single/stdlib_overflow/
```

Or run the entire lit test suite:
```bash
./scripts/build.sh --lit
```
