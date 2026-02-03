# Test Reproduction Guide - Lit Failures

This guide provides specific commands to reproduce each type of failure with minimal examples.

---

## Quick Start - Reproduce the Two Main Bugs

### Bug #1: String Array Panic (21 tests affected)

```bash
cd /home/ghaith/git/rusty

# Minimal reproduction
$PWD/target/debug/plc \
  -o /tmp/bug1_test.out \
  -liec61131std \
  -L$PWD/output/lib \
  -i "$PWD/output/include/*.st" \
  -i $PWD/tests/lit/util/printf.pli \
  --linker=cc \
  tests/lit/single/init/global_variables.st 2>&1 | tail -30
```

**Expected Error**:
```
thread '<unnamed>' panicked at src/codegen/generators/expression_generator.rs:2443:38:
Found PointerValue(PointerValue { ptr_value: Value { name: "utf08_literal_1", ... } }) 
but expected the ArrayValue variant
```

### Bug #2: Array Initialization Type Mismatch (61 tests affected)

```bash
cd /home/ghaith/git/rusty

# Minimal reproduction
$PWD/target/debug/plc \
  -o /tmp/bug2_test.out \
  -liec61131std \
  -L$PWD/output/lib \
  -i "$PWD/output/include/*.st" \
  -i $PWD/tests/lit/util/printf.pli \
  --linker=cc \
  tests/lit/single/property/simple.st 2>&1 | tail -30
```

**Expected Error**:
```
initializing an array should be memcpy-able or memset-able 
at: /home/ghaith/git/rusty/tests/lit/single/property/simple.st:20:8:{20:8-20:16}: .
```

---

## Reproduce by Category

### 1. Initialization Tests (init/) - 18 failures

#### Test: global_variables.st (Bug #1 - String Array)
```bash
$PWD/target/debug/plc -o /tmp/init1.out -liec61131std -L$PWD/output/lib \
  -i "$PWD/output/include/*.st" -i $PWD/tests/lit/util/printf.pli --linker=cc \
  tests/lit/single/init/global_variables.st 2>&1 | grep -A 5 "panicked"
```

**Test Source**:
```st
VAR_GLOBAL
    arr : ARRAY[0..3] OF STRING := ['a', 'b', 'c', 'd'];
    alias AT arr : ARRAY[0..3] OF STRING;
END_VAR

FUNCTION main: DINT
    printf('%s, %s, %s, %s$N', REF(alias[0]), REF(alias[1]), REF(alias[2]), REF(alias[3]));
END_FUNCTION
```

#### Test: structs.st (Bug #1 - String Array)
```bash
$PWD/target/debug/plc -o /tmp/init2.out -liec61131std -L$PWD/output/lib \
  -i "$PWD/output/include/*.st" -i $PWD/tests/lit/util/printf.pli --linker=cc \
  tests/lit/single/init/structs.st 2>&1 | grep -A 5 "panicked"
```

#### Test: config_variables.st (Different error)
```bash
$PWD/target/debug/plc -o /tmp/init3.out -liec61131std -L$PWD/output/lib \
  -i "$PWD/output/include/*.st" -i $PWD/tests/lit/util/printf.pli --linker=cc \
  tests/lit/single/init/config_variables.st 2>&1 | tail -20
```

#### Run all init tests:
```bash
lit -v tests/lit/single/init/ -DLIB=$PWD/output -DCOMPILER=$PWD/target/debug/plc 2>&1 | grep -E "PASS|FAIL"
```

---

### 2. Object-Oriented Programming (oop/) - 24 failures

#### Test: fb_direct_calls.st (Bug #2 - Array Init Type)
```bash
$PWD/target/debug/plc -o /tmp/oop1.out -liec61131std -L$PWD/output/lib \
  -i "$PWD/output/include/*.st" -i $PWD/tests/lit/util/printf.pli --linker=cc \
  tests/lit/single/oop/fb_direct_calls.st 2>&1 | grep -B 2 -A 5 "memcpy"
```

**Pattern**: Function block with member variables fails during initialization.

#### Test: method_var_output.st (Bug #2)
```bash
$PWD/target/debug/plc -o /tmp/oop2.out -liec61131std -L$PWD/output/lib \
  -i "$PWD/output/include/*.st" -i $PWD/tests/lit/util/printf.pli --linker=cc \
  tests/lit/single/oop/method_var_output.st 2>&1 | tail -20
```

#### Test: super_basic_access.st (Bug #2)
```bash
$PWD/target/debug/plc -o /tmp/oop3.out -liec61131std -L$PWD/output/lib \
  -i "$PWD/output/include/*.st" -i $PWD/tests/lit/util/printf.pli --linker=cc \
  tests/lit/single/oop/super_basic_access.st 2>&1 | tail -20
```

#### Run all OOP tests:
```bash
lit -v tests/lit/single/oop/ -DLIB=$PWD/output -DCOMPILER=$PWD/target/debug/plc 2>&1 | tail -10
```

---

### 3. Polymorphism (polymorphism/) - 21 failures

#### Test: basic_inheritance.st (Bug #2)
```bash
$PWD/target/debug/plc -o /tmp/poly1.out -liec61131std -L$PWD/output/lib \
  -i "$PWD/output/include/*.st" -i $PWD/tests/lit/util/printf.pli --linker=cc \
  tests/lit/single/polymorphism/basic_inheritance.st 2>&1 | tail -20
```

**Pattern**: Inheritance chains with function blocks.

#### Test: fnptr/method_call.st (Bug #2)
```bash
$PWD/target/debug/plc -o /tmp/poly2.out -liec61131std -L$PWD/output/lib \
  -i "$PWD/output/include/*.st" -i $PWD/tests/lit/util/printf.pli --linker=cc \
  tests/lit/single/polymorphism/fnptr/method_call.st 2>&1 | tail -20
```

#### Test: function_block_call.st (Bug #2)
```bash
$PWD/target/debug/plc -o /tmp/poly3.out -liec61131std -L$PWD/output/lib \
  -i "$PWD/output/include/*.st" -i $PWD/tests/lit/util/printf.pli --linker=cc \
  tests/lit/single/polymorphism/function_block_call.st 2>&1 | tail -20
```

#### Run all polymorphism tests:
```bash
lit -v tests/lit/single/polymorphism/ -DLIB=$PWD/output -DCOMPILER=$PWD/target/debug/plc 2>&1 | tail -10
```

---

### 4. Properties (property/) - 16 failures

#### Test: simple.st (Bug #2 - SIMPLEST EXAMPLE)
```bash
$PWD/target/debug/plc -o /tmp/prop1.out -liec61131std -L$PWD/output/lib \
  -i "$PWD/output/include/*.st" -i $PWD/tests/lit/util/printf.pli --linker=cc \
  tests/lit/single/property/simple.st 2>&1 | tail -20
```

**Test Source** (minimal):
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
        instance : fb;  // <- Initialization fails here
    END_VAR
    printf('%d$N', instance.foo);
END_FUNCTION
```

#### Test: complex.st (Bug #2)
```bash
$PWD/target/debug/plc -o /tmp/prop2.out -liec61131std -L$PWD/output/lib \
  -i "$PWD/output/include/*.st" -i $PWD/tests/lit/util/printf.pli --linker=cc \
  tests/lit/single/property/complex.st 2>&1 | tail -20
```

#### Run all property tests:
```bash
lit -v tests/lit/single/property/ -DLIB=$PWD/output -DCOMPILER=$PWD/target/debug/plc 2>&1 | tail -10
```

---

### 5. Standard Library Overflow (stdlib_overflow/) - 9 failures ✅ EXPECTED

These are **XFAIL** tests that are supposed to fail at runtime. They're working correctly!

```bash
# Should compile but fail at runtime (this is expected)
lit -v tests/lit/single/stdlib_overflow/ -DLIB=$PWD/output -DCOMPILER=$PWD/target/debug/plc 2>&1 | grep -E "XFAIL|PASS|FAIL"
```

**Expected output**:
```
XFAIL: ... :: single/stdlib_overflow/add_time_overflow.st
XFAIL: ... :: single/stdlib_overflow/div_time_by_zero.st
...
```

All should show **XFAIL** (expected failure) - this is correct behavior.

---

### 6. Complex Return Types (complex_return_types/) - 3 failures

#### Test: array_of_string_return.st (Likely Bug #1)
```bash
$PWD/target/debug/plc -o /tmp/ret1.out -liec61131std -L$PWD/output/lib \
  -i "$PWD/output/include/*.st" -i $PWD/tests/lit/util/printf.pli --linker=cc \
  tests/lit/single/complex_return_types/array_of_string_return.st 2>&1 | tail -20
```

**Pattern**: Functions returning arrays of strings.

---

### 7. Self-Referential Types (types/) - 3 failures

#### Test: self_referential_struct_via_reference.st
```bash
$PWD/target/debug/plc -o /tmp/type1.out -liec61131std -L$PWD/output/lib \
  -i "$PWD/output/include/*.st" -i $PWD/tests/lit/util/printf.pli --linker=cc \
  tests/lit/single/types/self_referential_struct_via_reference.st 2>&1 | tail -20
```

**Pattern**: Recursive struct definitions via pointers.

---

## Minimal Reproduction Test Cases

### Minimal Bug #1 - Create your own test file

Create `test_string_array.st`:
```st
VAR_GLOBAL
    my_strings : ARRAY[0..2] OF STRING := ['hello', 'world', 'test'];
END_VAR

FUNCTION main: DINT
    main := 0;
END_FUNCTION
```

Compile:
```bash
$PWD/target/debug/plc -o /tmp/minimal1.out -liec61131std -L$PWD/output/lib \
  -i "$PWD/output/include/*.st" --linker=cc test_string_array.st 2>&1
```

### Minimal Bug #2 - Create your own test file

Create `test_fb_init.st`:
```st
FUNCTION_BLOCK MyFB
    VAR
        value : DINT := 42;
    END_VAR
END_FUNCTION_BLOCK

FUNCTION main: DINT
    VAR
        fb_instance : MyFB;
    END_VAR
    main := 0;
END_FUNCTION
```

Compile:
```bash
$PWD/target/debug/plc -o /tmp/minimal2.out -liec61131std -L$PWD/output/lib \
  -i "$PWD/output/include/*.st" --linker=cc test_fb_init.st 2>&1
```

---

## Run Tests by Priority

### High Priority (Critical Path)

```bash
# Phase 1 - Fix Bug #1, then run:
lit tests/lit/single/init/global_variables.st -DLIB=$PWD/output -DCOMPILER=$PWD/target/debug/plc
lit tests/lit/single/init/structs.st -DLIB=$PWD/output -DCOMPILER=$PWD/target/debug/plc
lit tests/lit/single/init/ -DLIB=$PWD/output -DCOMPILER=$PWD/target/debug/plc 2>&1 | tail -5

# Phase 2 - Fix Bug #2, then run:
lit tests/lit/single/property/simple.st -DLIB=$PWD/output -DCOMPILER=$PWD/target/debug/plc
lit tests/lit/single/oop/fb_direct_calls.st -DLIB=$PWD/output -DCOMPILER=$PWD/target/debug/plc
lit tests/lit/single/property/ -DLIB=$PWD/output -DCOMPILER=$PWD/target/debug/plc 2>&1 | tail -5
lit tests/lit/single/oop/ -DLIB=$PWD/output -DCOMPILER=$PWD/target/debug/plc 2>&1 | tail -5
lit tests/lit/single/polymorphism/ -DLIB=$PWD/output -DCOMPILER=$PWD/target/debug/plc 2>&1 | tail -5
```

### Medium Priority

```bash
# After both bugs fixed:
lit tests/lit/single/complex_return_types/ -DLIB=$PWD/output -DCOMPILER=$PWD/target/debug/plc 2>&1 | tail -5
```

### Low Priority

```bash
# After main bugs fixed:
lit tests/lit/single/types/ -DLIB=$PWD/output -DCOMPILER=$PWD/target/debug/plc 2>&1 | tail -5
lit tests/lit/single/pointer/ -DLIB=$PWD/output -DCOMPILER=$PWD/target/debug/plc 2>&1 | tail -5
lit tests/lit/single/builtin-named-arguments/ -DLIB=$PWD/output -DCOMPILER=$PWD/target/debug/plc 2>&1 | tail -5
```

---

## Full Test Suite Run

```bash
cd /home/ghaith/git/rusty

# Get summary
lit tests/lit/single/ -DLIB=$PWD/output -DCOMPILER=$PWD/target/debug/plc 2>&1 | grep -E "Testing:|Passed|Failed|Expectedly Failed"

# Get detailed results
lit -v tests/lit/single/ -DLIB=$PWD/output -DCOMPILER=$PWD/target/debug/plc > /tmp/lit_results.txt 2>&1

# Count by result type
grep "^PASS:" /tmp/lit_results.txt | wc -l
grep "^FAIL:" /tmp/lit_results.txt | wc -l
grep "^XFAIL:" /tmp/lit_results.txt | wc -l
```

---

## Debug Output - Add Logging

### For Bug #1 (expression_generator.rs:2440)

Add before line 2440:
```rust
eprintln!("=== DEBUG const_array ===");
eprintln!("llvm_type: {:?}", llvm_type);
eprintln!("v.len(): {}", v.len());
for (i, val) in v.iter().enumerate() {
    eprintln!("  [{}]: is_array={}, is_pointer={}, is_int={}, is_float={}", 
        i,
        val.is_array_value(),
        val.is_pointer_value(),
        val.is_int_value(),
        val.is_float_value()
    );
    if val.is_pointer_value() {
        eprintln!("    -> PointerValue: {}", val.print_to_string());
    }
}
```

### For Bug #2 (llvm.rs:380)

Add before line 380:
```rust
eprintln!("=== DEBUG generate_variable_initializer ===");
eprintln!("variable_to_initialize: {:?}", variable_to_initialize);
eprintln!("value checks:");
eprintln!("  is_pointer_value: {}", value.is_pointer_value());
eprintln!("  is_int_value: {}", value.is_int_value());
eprintln!("  is_array_value: {}", value.is_array_value());
eprintln!("  is_struct_value: {}", value.is_struct_value());
eprintln!("  is_float_value: {}", value.is_float_value());
eprintln!("  is_vector_value: {}", value.is_vector_value());
eprintln!("value: {}", value.print_to_string());
eprintln!("llvm_type: {:?}", llvm_type);
```

Then recompile and run:
```bash
cargo build
$PWD/target/debug/plc -o /tmp/debug.out -liec61131std -L$PWD/output/lib \
  -i "$PWD/output/include/*.st" -i $PWD/tests/lit/util/printf.pli --linker=cc \
  tests/lit/single/property/simple.st 2>&1 | grep -A 20 "DEBUG"
```

---

## Success Criteria

### After Bug #1 Fix
- `init/global_variables.st` - PASS
- `init/structs.st` - PASS
- ~18 init tests should pass
- Expected: ~72 tests passing (was 51)

### After Bug #2 Fix
- `property/simple.st` - PASS
- `oop/fb_direct_calls.st` - PASS
- ~61 more tests should pass
- Expected: ~133 tests passing (was 72)

### Final Target
- **>140 tests passing** (>90% success rate)
- Only XFAIL tests and edge cases failing
- All init/, oop/, polymorphism/, property/ categories mostly passing

---

## Quick Reference Card

```
┌──────────────────────────────────────────────────────────────────┐
│ REPRODUCTION QUICK REFERENCE                                     │
├──────────────────────────────────────────────────────────────────┤
│ Bug #1 (String Array):                                           │
│   plc ... tests/lit/single/init/global_variables.st             │
│   Error: "Found PointerValue but expected ArrayValue"           │
│                                                                  │
│ Bug #2 (Array Init):                                             │
│   plc ... tests/lit/single/property/simple.st                   │
│   Error: "memcpy-able or memset-able"                           │
│                                                                  │
│ Test Category:                                                   │
│   lit tests/lit/single/CATEGORY/ -DLIB=... -DCOMPILER=...       │
│                                                                  │
│ Full Suite:                                                      │
│   lit tests/lit/single/ -DLIB=$PWD/output -DCOMPILER=...        │
└──────────────────────────────────────────────────────────────────┘
```
