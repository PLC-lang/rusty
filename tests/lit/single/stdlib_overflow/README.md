# Standard Library Overflow Tests

This directory contains integration tests for stdlib functions that are expected to panic on overflow or division by zero conditions.

## Background

These tests were originally part of the Rust integration test suite in `libs/stdlib/tests/date_time_numeric_functions_tests.rs`, but they could not be properly tested there because:

1. The Rust test infrastructure compiles PLC code into a shared library and loads it dynamically using `libloading`
2. When a panic occurs in dynamically loaded code, Rust's panic unwinding mechanism cannot cross the FFI boundary
3. Even with `#[should_panic]` or `std::panic::catch_unwind()`, the panic causes an abort with the error:
   ```
   fatal runtime error: Rust cannot catch foreign exceptions, aborting
   ```

## Solution

These tests have been moved to the lit test framework where they can be marked with `XFAIL: *` to indicate they are expected to fail at runtime. This properly documents the expected panic behavior without breaking the test suite.

## Test Coverage

The following overflow/panic conditions are tested:

### TIME Arithmetic Overflow
- `add_time_overflow.st` - Adding to max TIME value
- `sub_time_overflow.st` - Subtracting from min TIME value

### TIME Multiplication Overflow
- `mul_time_signed_overflow.st` - Multiplying TIME by max LINT
- `mul_time_unsigned_overflow.st` - Multiplying TIME by max ULINT
- `mul_time_real_overflow.st` - Multiplying TIME by max REAL
- `mul_time_lreal_overflow.st` - Multiplying TIME by large LREAL

### Division by Zero
- `div_time_by_zero.st` - Dividing TIME by zero (LINT)
- `div_time_by_real_zero.st` - Dividing TIME by zero (REAL)
- `div_time_by_lreal_zero.st` - Dividing TIME by zero (LREAL)

## Test Format

All tests follow this pattern:

```st
// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// XFAIL: *
// Test description

PROGRAM main
VAR
    a : TIME;
END_VAR
    // Operation that causes overflow/panic
    a := <operation>;
    // CHECK: Should not reach here
    printf('Should not reach here$N');
END_PROGRAM
```

The `XFAIL: *` directive tells lit that this test is expected to fail (panic), so a non-zero exit code is treated as success.

## Running the Tests

To run all stdlib overflow tests:
```bash
lit -v tests/lit/single/stdlib_overflow/
```

Or run the entire lit test suite:
```bash
./scripts/build.sh --build --lit
```

## Expected Output

When running these tests, you should see:
```
XFAIL: <unnamed> :: single/stdlib_overflow/add_time_overflow.st
XFAIL: <unnamed> :: single/stdlib_overflow/sub_time_overflow.st
...

Total Discovered Tests: 9
  Expectedly Failed: 9 (100.00%)
```

This indicates all tests behaved as expected (they panicked).