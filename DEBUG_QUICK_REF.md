# Quick Debugging Reference - Lit Test Failures

## TL;DR - Two Critical Bugs Block 60+ Tests

### Bug #1: String Array Panic (18+ tests)
**Location**: `src/codegen/generators/expression_generator.rs:2443`  
**Error**: `Found PointerValue(...) but expected the ArrayValue variant`  
**Example Test**: `tests/lit/single/init/global_variables.st`

### Bug #2: Array Init Type Mismatch (40+ tests)  
**Location**: `src/codegen/generators/llvm.rs:399`  
**Error**: `initializing an array should be memcpy-able or memset-able`  
**Example Test**: `tests/lit/single/oop/fb_direct_calls.st`

---

## Quick Test Commands

### Test a Single File
```bash
cd /home/ghaith/git/rusty

# Compile and see error
$PWD/target/debug/plc \
  -o /tmp/test.out \
  -liec61131std \
  -L$PWD/output/lib \
  -i "$PWD/output/include/*.st" \
  -i $PWD/tests/lit/util/printf.pli \
  --linker=cc \
  tests/lit/single/PATH/TO/TEST.st 2>&1 | tail -30
```

### Test Category
```bash
# Init tests (18 failures)
lit -v tests/lit/single/init/ -DLIB=$PWD/output -DCOMPILER=$PWD/target/debug/plc

# OOP tests (24 failures)
lit -v tests/lit/single/oop/ -DLIB=$PWD/output -DCOMPILER=$PWD/target/debug/plc

# Polymorphism (21 failures)
lit -v tests/lit/single/polymorphism/ -DLIB=$PWD/output -DCOMPILER=$PWD/target/debug/plc

# Property (16 failures)
lit -v tests/lit/single/property/ -DLIB=$PWD/output -DCOMPILER=$PWD/target/debug/plc
```

### Full Test Suite
```bash
lit tests/lit/single/ -DLIB=$PWD/output -DCOMPILER=$PWD/target/debug/plc 2>&1 | grep -E "(Testing:|Passed|Failed)"
```

---

## Example Failing Tests

### Example #1: String Array Init (Bug #1)

**File**: `tests/lit/single/init/global_variables.st`
```st
VAR_GLOBAL
    arr : ARRAY[0..3] OF STRING := ['a', 'b', 'c', 'd'];
    alias AT arr : ARRAY[0..3] OF STRING;
END_VAR

FUNCTION main: DINT
    printf('%s, %s, %s, %s$N', REF(alias[0]), REF(alias[1]), REF(alias[2]), REF(alias[3]));
END_FUNCTION
```

**Error**:
```
thread '<unnamed>' panicked at src/codegen/generators/expression_generator.rs:2443:38:
Found PointerValue(PointerValue { ptr_value: Value { name: "utf08_literal_1", ... } }) 
but expected the ArrayValue variant
```

**Root Cause**: String literals `'a'`, `'b'`, etc. are represented as pointers to global constants in LLVM. When building the array, the code expects `ArrayValue` but gets `PointerValue`.

**Test Command**:
```bash
$PWD/target/debug/plc -o /tmp/test1.out -liec61131std -L$PWD/output/lib \
  -i "$PWD/output/include/*.st" -i $PWD/tests/lit/util/printf.pli --linker=cc \
  tests/lit/single/init/global_variables.st 2>&1 | grep -A 5 "panicked"
```

---

### Example #2: Function Block Array Init (Bug #2)

**File**: `tests/lit/single/oop/fb_direct_calls.st` (simplified)
```st
FUNCTION_BLOCK FbA
    VAR
        multiplier2D: DINT := 2;
    END_VAR
    // methods...
END_FUNCTION_BLOCK

FUNCTION main
    VAR
        instanceA: FbA;  // <- Initialization of FB with members fails here
        instanceB: FbB;
        instanceC: FbC;
    END_VAR
    // ...
END_FUNCTION
```

**Error**:
```
initializing an array should be memcpy-able or memset-able 
at: tests/lit/single/oop/fb_direct_calls.st:76:8
```

**Root Cause**: When initializing the function block instances (which may contain arrays or structs), the generated initialization value is neither a `PointerValue` (for memcpy) nor an `IntValue` (for memset). The refactored init logic produces a different value type.

**Test Command**:
```bash
$PWD/target/debug/plc -o /tmp/test2.out -liec61131std -L$PWD/output/lib \
  -i "$PWD/output/include/*.st" -i $PWD/tests/lit/util/printf.pli --linker=cc \
  tests/lit/single/oop/fb_direct_calls.st 2>&1 | grep -A 3 "memcpy"
```

---

### Example #3: Property with Arrays (Bug #2)

**File**: `tests/lit/single/property/simple.st`
```st
FUNCTION_BLOCK fb
    VAR
        localPrivateVariable : DINT := 5;
    END_VAR

    PROPERTY foo : DINT
        GET
            foo := localPrivateVariable;
        END_GET
        SET
            localPrivateVariable := foo + 5;
        END_SET
    END_PROPERTY
END_FUNCTION_BLOCK

FUNCTION main
    VAR
        instance : fb;  // <- FB initialization fails
    END_VAR
    // ...
END_FUNCTION
```

**Error**:
```
initializing an array should be memcpy-able or memset-able
at: tests/lit/single/property/simple.st:20:8
```

**Test Command**:
```bash
$PWD/target/debug/plc -o /tmp/test3.out -liec61131std -L$PWD/output/lib \
  -i "$PWD/output/include/*.st" -i $PWD/tests/lit/util/printf.pli --linker=cc \
  tests/lit/single/property/simple.st 2>&1 | tail -10
```

---

## Code Locations to Investigate

### Bug #1: String Array Panic

**File**: `src/codegen/generators/expression_generator.rs:2440-2455`

Current code:
```rust
let array_value = match llvm_type {
    BasicTypeEnum::ArrayType(_) => llvm_type.into_array_type().const_array(
        v.iter().map(|it| it.into_array_value()).collect::<Vec<ArrayValue>>().as_slice(),
        //                  ^^^^^^^^^^^^^^^^^^^ PANICS when element is PointerValue (string)
    ),
    // ...
```

**Problem**: Assumes all array elements are `ArrayValue`, but string literals are `PointerValue`.

**Potential Fix**: Check the array's element type. If it's a pointer/string, use `into_pointer_value()`:
```rust
let array_value = match llvm_type {
    BasicTypeEnum::ArrayType(arr_type) => {
        let elem_type = arr_type.get_element_type();
        if elem_type.is_pointer_type() {
            // Array of pointers (e.g., strings)
            llvm_type.into_array_type().const_array(
                v.iter().map(|it| it.into_pointer_value()).collect::<Vec<PointerValue>>().as_slice(),
            )
        } else if elem_type.is_array_type() {
            // Array of arrays
            llvm_type.into_array_type().const_array(
                v.iter().map(|it| it.into_array_value()).collect::<Vec<ArrayValue>>().as_slice(),
            )
        } else {
            // Handle other cases...
        }
    },
    // ...
```

---

### Bug #2: Array Init Type Mismatch

**File**: `src/codegen/generators/llvm.rs:380-402`

Current code:
```rust
if value.is_pointer_value() {
    // mem-copy from a global constant variable
    self.builder.build_memcpy(...);
} else if value.is_int_value() {
    // mem-set the value (usually 0)
    self.builder.build_memset(...);
} else {
    Err(Diagnostic::codegen_error(
        "initializing an array should be memcpy-able or memset-able",
        location,
    ))?;  // <-- ERRORS HERE
};
```

**Problem**: The refactored initialization logic produces values that are neither `PointerValue` nor `IntValue`. Likely `ArrayValue`, `StructValue`, or other aggregate types.

**Investigation Steps**:
1. Add debug logging before the error:
```rust
else {
    eprintln!("DEBUG: Unexpected value type: {:?}", value);
    eprintln!("DEBUG: value.is_array_value(): {}", value.is_array_value());
    eprintln!("DEBUG: value.is_struct_value(): {}", value.is_struct_value());
    Err(Diagnostic::codegen_error(...
```

2. Check what produces this value - trace back through initialization chain

**Potential Fix Options**:

Option A: Handle additional value types
```rust
if value.is_pointer_value() {
    self.builder.build_memcpy(...);
} else if value.is_int_value() {
    self.builder.build_memset(...);
} else if value.is_array_value() || value.is_struct_value() {
    // Store the aggregate value directly
    self.builder.build_store(variable_to_initialize, value)?;
} else {
    Err(...)?;
}
```

Option B: Change initialization to always produce pointer/int values (create global constants)

---

## Debug Logging

Add to `src/codegen/generators/llvm.rs:390`:
```rust
eprintln!("=== DEBUG generate_variable_initializer ===");
eprintln!("variable: {:?}", variable_to_initialize);
eprintln!("value type: is_pointer={}, is_int={}, is_array={}, is_struct={}", 
    value.is_pointer_value(), 
    value.is_int_value(),
    value.is_array_value(),
    value.is_struct_value()
);
eprintln!("llvm_value: {}", value.print_to_string());
```

Add to `src/codegen/generators/expression_generator.rs:2440`:
```rust
eprintln!("=== DEBUG const_array ===");
eprintln!("llvm_type: {:?}", llvm_type);
eprintln!("v.len(): {}", v.len());
for (i, val) in v.iter().enumerate() {
    eprintln!("  [{}]: is_array={}, is_pointer={}, is_int={}", 
        i,
        val.is_array_value(),
        val.is_pointer_value(),
        val.is_int_value()
    );
}
```

---

## Failure Categories Summary

| Category | Failures | Primary Bug |
|----------|----------|-------------|
| **init/** | 18 | Bug #1 (string arrays) |
| **oop/** | 24 | Bug #2 (FB initialization) |
| **polymorphism/** | 21 | Bug #2 (FB initialization) |
| **property/** | 16 | Bug #2 (FB initialization) |
| **stdlib_overflow/** | 9 | ✅ XFAIL (expected) |
| **complex_return_types/** | 3 | Bug #1 (likely) |
| **types/** | 3 | Unknown |
| **misc** | 6 | Various |

**Total Impact**:
- Bug #1 fixes: ~21 tests (18 init + 3 complex_return)
- Bug #2 fixes: ~61 tests (24 oop + 21 poly + 16 prop)
- Together: ~82 of 90 failures

---

## Expected Results After Fixes

**Before fixes**: 51 passed, 90 failed, 11 XFAIL (58.4% failure rate)  
**After Bug #1 fix**: ~72 passed, ~69 failed (38% failure improvement)  
**After Bug #2 fix**: ~133 passed, ~8 failed (90% tests passing)  

Target: >140 tests passing (>90% success rate)

---

## Testing After Each Fix

### After Bug #1 Fix (String Arrays)
```bash
# Should now pass
lit tests/lit/single/init/global_variables.st -DLIB=$PWD/output -DCOMPILER=$PWD/target/debug/plc
lit tests/lit/single/init/structs.st -DLIB=$PWD/output -DCOMPILER=$PWD/target/debug/plc

# Check full init/ category
lit tests/lit/single/init/ -DLIB=$PWD/output -DCOMPILER=$PWD/target/debug/plc 2>&1 | grep -E "Passed|Failed"
```

### After Bug #2 Fix (Array Initialization)
```bash
# Should now pass
lit tests/lit/single/oop/fb_direct_calls.st -DLIB=$PWD/output -DCOMPILER=$PWD/target/debug/plc
lit tests/lit/single/property/simple.st -DLIB=$PWD/output -DCOMPILER=$PWD/target/debug/plc

# Check categories
lit tests/lit/single/oop/ -DLIB=$PWD/output -DCOMPILER=$PWD/target/debug/plc 2>&1 | tail -5
lit tests/lit/single/polymorphism/ -DLIB=$PWD/output -DCOMPILER=$PWD/target/debug/plc 2>&1 | tail -5
lit tests/lit/single/property/ -DLIB=$PWD/output -DCOMPILER=$PWD/target/debug/plc 2>&1 | tail -5
```

### Full Regression Test
```bash
lit tests/lit/single/ -DLIB=$PWD/output -DCOMPILER=$PWD/target/debug/plc 2>&1 | grep -E "Testing:|Passed|Failed"
```

---

## Need More Details on a Specific Test?

```bash
# Get full compilation output with line numbers
$PWD/target/debug/plc -o /tmp/debug.out -liec61131std -L$PWD/output/lib \
  -i "$PWD/output/include/*.st" -i $PWD/tests/lit/util/printf.pli --linker=cc \
  tests/lit/single/CATEGORY/TEST.st 2>&1 | tee /tmp/compile.log

# If it compiles, run with backtrace
RUST_BACKTRACE=full LD_LIBRARY_PATH=$PWD/output/lib /tmp/debug.out

# Run through lit with verbose output
lit -vv tests/lit/single/CATEGORY/TEST.st -DLIB=$PWD/output -DCOMPILER=$PWD/target/debug/plc
```
