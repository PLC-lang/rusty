# Lit Test Failure Analysis Report

**Date**: Analysis of current refactor branch (initialization logic)  
**Total Tests**: 154  
**Status Overview**:
- ✅ **Passed**: 51 (33.12%)
- ⚠️ **Expectedly Failed (XFAIL)**: 11 (7.14%) - These are correct
- ❌ **Failed**: 90 (58.44%)
- 🔄 **Unexpectedly Passed**: 2 (1.30%)

---

## Executive Summary

The refactor to initialization logic has broken **core functionality** across multiple subsystems. Two critical root causes have been identified:

### 🔴 **Critical Issue #1: String Literal Array Initialization Panic**
**Location**: `src/codegen/generators/expression_generator.rs:2443`  
**Impact**: ~18+ init tests, multiple OOP/property tests

**Error**:
```
thread '<unnamed>' panicked at src/codegen/generators/expression_generator.rs:2443:38:
Found PointerValue(PointerValue { ... }) but expected the ArrayValue variant
```

**Root Cause**: When initializing arrays containing string literals (e.g., `['a', 'hello']`), the LLVM code generation expects `ArrayValue` but receives `PointerValue` because strings are represented as pointers to constant string data. The code at line 2443 attempts to convert array elements to `ArrayValue` but string constants are `PointerValue` objects.

**Affected Pattern**: Any global/struct initialization with string array literals.

---

### 🔴 **Critical Issue #2: Array Initialization Type Mismatch**
**Location**: `src/codegen/generators/llvm.rs:399`  
**Impact**: ~40+ OOP, polymorphism, and property tests

**Error**:
```
initializing an array should be memcpy-able or memset-able
```

**Root Cause**: The `generate_variable_initializer` function expects array initialization values to be either:
- `PointerValue` (for memcpy from global constant), or
- `IntValue` (for memset with a constant byte)

However, the refactored initialization logic is producing values that are neither, likely due to changes in how initialization expressions are evaluated/compiled. The function cannot handle the new value type being passed.

**Affected Pattern**: Complex struct/FB initialization, polymorphic types, property-containing FBs.

---

## Failure Categories

### 1. **Initialization Tests (init/)** - 18 failures ⚠️ HIGHEST PRIORITY
Core initialization functionality is broken. This is the subsystem that was refactored.

**Failures**:
- `config_variables.st` - Compiles successfully (no string arrays)
- `config_variables_dont_clash_with_globals.st` - Unknown error
- `function_locals.st` - Unknown error
- `global_nested_struct_with_default_and_override_refs.st` - Unknown error
- `global_nested_struct_with_mixed_ref_assignments.st` - Unknown error
- `global_struct_instances.st` - Unknown error
- `global_struct_with_partial_nested_initialization.st` - Unknown error
- `global_variables.st` - **STRING ARRAY PANIC** (string literal 'a')
- `local_struct_with_global_ref_initialization.st` - Unknown error
- `methods.st` - Unknown error
- `structs.st` - **STRING ARRAY PANIC** (string literal 'hello')
- `user_init.st` - Unknown error
- `user_init_for_stack_variable.st` - Unknown error

**Key Examples**:
```
Test: global_variables.st
Error: Panic at expression_generator.rs:2443 - String array literal handling
Pattern: Global variable with string array initializer

Test: structs.st  
Error: Panic at expression_generator.rs:2443 - String array literal handling
Pattern: Struct with string array field
```

---

### 2. **Object-Oriented Programming (oop/)** - 24 failures
Method calls, inheritance, super/this keywords all affected.

**Failures**:
- `aggregate_return_complex.st` - MEMCPY/MEMSET error
- `aggregate_return_types.st` - MEMCPY/MEMSET error
- `fb_access_members_from_base.st` - MEMCPY/MEMSET error
- `fb_complex_access_of_super_members.st` - MEMCPY/MEMSET error
- `fb_direct_calls.st` - **MEMCPY/MEMSET error** (line 76)
- `fb_method_callled.st` - MEMCPY/MEMSET error
- `fb_method_shadowing.st` - MEMCPY/MEMSET error
- `fb_method_with_explicit_return.st` - MEMCPY/MEMSET error
- `fb_with_in_and_out_extension.st` - MEMCPY/MEMSET error
- `fb_with_super_class_method_calls.st` - MEMCPY/MEMSET error
- `grandparent_access_through_super.st` - MEMCPY/MEMSET error
- `method_initial_values.st` - MEMCPY/MEMSET error
- `method_var_output.st` - MEMCPY/MEMSET error
- `parent_variable_access_in_method.st` - MEMCPY/MEMSET error
- `super_basic_access.st` - MEMCPY/MEMSET error
- `super_in_control_structures.st` - MEMCPY/MEMSET error
- `super_keyword_overridden_method.st` - MEMCPY/MEMSET error
- `super_multi_level_inheritance.st` - MEMCPY/MEMSET error
- `super_without_deref.st` - MEMCPY/MEMSET error
- `super_with_parenthesized_expressions.st` - MEMCPY/MEMSET error
- `this_basic_access.st` - MEMCPY/MEMSET error
- `this_in_control_structures.st` - MEMCPY/MEMSET error
- `this_without_deref.st` - MEMCPY/MEMSET error
- `this_with_parenthesized_expressions.st` - MEMCPY/MEMSET error

**Pattern**: Function blocks with complex initialization (arrays, structs, inherited members).

---

### 3. **Polymorphism (polymorphism/)** - 21 failures
Function pointers, method dispatch, inheritance chains.

**Failures**:
- `basic_inheritance.st` - **MEMCPY/MEMSET error** (line 78)
- `basic_override_method_call_in_method.st` - MEMCPY/MEMSET error
- `basic_override_no_method_call_in_method.st` - MEMCPY/MEMSET error
- `fnptr/function_block_call_explicit_arguments.st` - MEMCPY/MEMSET error
- `fnptr/function_block_call_implicit_arguments.st` - MEMCPY/MEMSET error
- `fnptr/function_block_call_local_variables.st` - MEMCPY/MEMSET error
- `fnptr/method_call.st` - MEMCPY/MEMSET error
- `fnptr/method_call_aggregate_return.st` - MEMCPY/MEMSET error
- `fnptr/method_call_aggregate_return_by_inout.st` - MEMCPY/MEMSET error
- `fnptr/method_call_aggregate_return_by_output.st` - MEMCPY/MEMSET error
- `fnptr/method_call_by_void_pointer_cast.st` - MEMCPY/MEMSET error
- `fnptr/method_call_inout_arguments.st` - MEMCPY/MEMSET error
- `fnptr/method_call_input_arguments.st` - MEMCPY/MEMSET error
- `fnptr/method_call_output_arguments.st` - MEMCPY/MEMSET error
- `fnptr/method_call_overridden.st` - MEMCPY/MEMSET error
- `fnptr/method_call_using_this.st` - MEMCPY/MEMSET error
- `fnptr/user_defined_polymorphism.st` - MEMCPY/MEMSET error
- `function_block_call.st` - MEMCPY/MEMSET error
- `inheritance_chain.st` - MEMCPY/MEMSET error
- `reference_to.st` - MEMCPY/MEMSET error
- `ref_to.st` - MEMCPY/MEMSET error
- `super_call.st` - MEMCPY/MEMSET error
- `this_call.st` - MEMCPY/MEMSET error
- `unordered_method_declaration.st` - MEMCPY/MEMSET error

**Pattern**: All related to FB/method initialization with arrays or complex types.

---

### 4. **Properties (property/)** - 16 failures
Getter/setter properties, especially in OOP contexts.

**Failures**:
- `called_inside_pou_where_defined.st` - MEMCPY/MEMSET error
- `complex.st` - MEMCPY/MEMSET error
- `getter_used_in_array_indexing.st` - MEMCPY/MEMSET error
- `modify_array_value_with_get_and_set.st` - MEMCPY/MEMSET error
- `multiply_properties_single_pou.st` - MEMCPY/MEMSET error
- `nested.st` - MEMCPY/MEMSET error
- `oop_extended_fb_calls_property_in_another_property.st` - MEMCPY/MEMSET error
- `oop_extended_function_block.st` - MEMCPY/MEMSET error
- `oop_inheritance.st` - MEMCPY/MEMSET error
- `oop_interface_extension.st` - MEMCPY/MEMSET error
- `oop_interface_extension_call_in_fb_body.st` - MEMCPY/MEMSET error
- `property_used_in_action.st` - MEMCPY/MEMSET error
- `property_with_local_variables.st` - MEMCPY/MEMSET error
- `recursion.st` - MEMCPY/MEMSET error
- `same_property_in_different_pous_called_in_method.st` - MEMCPY/MEMSET error
- `simple.st` - **MEMCPY/MEMSET error** (line 20)
- `struct_return_type.st` - MEMCPY/MEMSET error

**Pattern**: Properties in FBs with array members.

---

### 5. **Standard Library Overflow (stdlib_overflow/)** - 9 failures ✅ EXPECTED
These are **XFAIL** tests (expected to fail at runtime). They are working correctly!

**Failures** (Expected):
- `add_time_overflow.st` - XFAIL ✅
- `div_time_by_lreal_zero.st` - XFAIL ✅
- `div_time_by_real_zero.st` - XFAIL ✅
- `div_time_by_zero.st` - XFAIL ✅
- `mul_time_lreal_overflow.st` - XFAIL ✅
- `mul_time_real_overflow.st` - XFAIL ✅
- `mul_time_signed_overflow.st` - XFAIL ✅
- `mul_time_unsigned_overflow.st` - XFAIL ✅
- `sub_time_overflow.st` - XFAIL ✅

**Status**: These are intentionally marked XFAIL and behave correctly. No action needed.

---

### 6. **Complex Return Types** - 3 failures
String/array return values from functions.

**Failures**:
- `array_of_string_return.st` - Unknown error
- `nested_string_return_call_in_if_condition.st` - Unknown error
- `string_return_function_called_in_while_loop.st` - Unknown error

**Pattern**: Likely related to string array handling (Issue #1).

---

### 7. **Self-Referential Types** - 3 failures
Recursive struct definitions via pointers/references.

**Failures**:
- `self_referential_pointer_to.st` - Unknown error
- `self_referential_ref_to.st` - Unknown error
- `self_referential_struct_via_reference.st` - Unknown error

**Pattern**: Initialization of self-referential structures.

---

### 8. **Miscellaneous** - 6 failures

**Failures**:
- `builtin-named-arguments/main.st` - Unknown error
- `builtin-positional-arguments/main.st` - Unknown error
- `functions/segmented_variable_blocks_with_implicit_arguments_call.st` - Unknown error
- `pointer/referenceto_variable_referencing_struct.st` - Unknown error
- `pointer/value_behind_function_block_pointer_is_assigned_to_correctly.st` - Unknown error
- `variable/global_namespace_operator.st` - Unknown error
- `variadics/string_passed_as_pointer.st` - Unknown error

---

## Root Cause Analysis

### Critical Code Locations

#### 1. `src/codegen/generators/expression_generator.rs:2440-2455`
```rust
let array_value = match llvm_type {
    BasicTypeEnum::ArrayType(_) => llvm_type.into_array_type().const_array(
        v.iter().map(|it| it.into_array_value()).collect::<Vec<ArrayValue>>().as_slice(),
        //                  ^^^^^^^^^^^^^^^^^^^^ PANICS HERE when element is a string (PointerValue)
    ),
    BasicTypeEnum::FloatType(_) => llvm_type.into_float_type().const_array(...),
    BasicTypeEnum::IntType(_) => llvm_type.into_int_type().const_array(...),
    BasicTypeEnum::PointerType(_) => llvm_type.into_pointer_type().const_array(
        v.iter().map(|it| it.into_pointer_value()).collect::<Vec<PointerValue>>().as_slice(),
    ),
    // ...
}
```

**Problem**: When the array type is `ArrayType` (e.g., array of strings/chars), the code assumes each element in `v` is an `ArrayValue`. However, string literals are represented as `PointerValue` in LLVM. The `.into_array_value()` call panics.

**Fix Required**: Need to handle the case where array elements are pointers (strings) differently. Check the inner type of the array and use appropriate conversion.

---

#### 2. `src/codegen/generators/llvm.rs:380-402`
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
    ))?;  // ERRORS HERE
};
```

**Problem**: The initialization refactor changed how initial values are generated. Previously, array initializers were compiled to either:
- A pointer to a global constant (for complex init), or
- An integer 0 (for zero-initialization)

Now the code is producing a different value type (likely an aggregate/struct value or array value) that doesn't match either branch.

**Fix Required**: Need to handle additional value types (ArrayValue, StructValue, etc.) or change the initialization logic to produce the expected pointer/int values.

---

## Impact on Refactor

The initialization logic refactor has broken the contract between:
1. **Initialization expression generation** (how initial values are computed)
2. **Variable initialization** (how those values are stored into variables)

Before the refactor, these systems communicated via specific value types (pointers, ints). The refactor changed one side but not the other.

---

## Recommended Fix Priority

### 🔥 **Phase 1: Critical String Array Fix** (Blocks 18+ tests)
**Target**: `expression_generator.rs:2443`  
**Action**: 
1. Check if `llvm_type` is `ArrayType` and its inner type is a pointer/string type
2. If so, use `into_pointer_value()` instead of `into_array_value()`
3. Handle nested arrays of strings appropriately

**Expected Impact**: Fixes ~18 init tests, possibly some complex_return_types tests

---

### 🔥 **Phase 2: Array Initialization Type Mismatch** (Blocks 40+ tests)
**Target**: `llvm.rs:399` and initialization value generation  
**Action**:
1. Investigate what value types the new initialization logic produces
2. Add handling for ArrayValue, StructValue in `generate_variable_initializer`
3. OR modify initialization to produce pointer/int values as before
4. May need to generate temporary globals for complex initializers

**Expected Impact**: Fixes ~40 OOP, polymorphism, property tests

---

### 🟡 **Phase 3: Remaining Edge Cases** (Blocks 8+ tests)
**Target**: Various  
**Action**: Address complex_return_types, self-referential types, misc failures  
**Expected Impact**: Remaining ~8 failures

---

## Testing Strategy

### Step 1: Quick Validation
```bash
# Test string array panic fix
plc -o /tmp/test.out tests/lit/single/init/global_variables.st -liec61131std -L$PWD/output/lib ...

# Test memcpy/memset fix
plc -o /tmp/test.out tests/lit/single/oop/fb_direct_calls.st -liec61131std -L$PWD/output/lib ...
```

### Step 2: Category Testing
```bash
# After Phase 1 fix
lit tests/lit/single/init/ -DLIB=$PWD/output -DCOMPILER=$PWD/target/debug/plc

# After Phase 2 fix  
lit tests/lit/single/oop/ -DLIB=$PWD/output -DCOMPILER=$PWD/target/debug/plc
lit tests/lit/single/polymorphism/ -DLIB=$PWD/output -DCOMPILER=$PWD/target/debug/plc
lit tests/lit/single/property/ -DLIB=$PWD/output -DCOMPILER=$PWD/target/debug/plc
```

### Step 3: Full Regression
```bash
# After all fixes
lit tests/lit/single/ -DLIB=$PWD/output -DCOMPILER=$PWD/target/debug/plc
```

**Target**: > 90% pass rate (140+/154 tests, excluding 11 XFAIL)

---

## Debug Commands

### Get detailed error for specific test:
```bash
plc -o /tmp/test.out -liec61131std -L$PWD/output/lib \
    -i "$PWD/output/include/*.st" -i $PWD/tests/lit/util/printf.pli \
    --linker=cc tests/lit/single/PATH/TO/TEST.st 2>&1 | tail -30
```

### Run compiled test with backtrace:
```bash
RUST_BACKTRACE=full LD_LIBRARY_PATH=$PWD/output/lib /tmp/test.out
```

### Test specific lit subset:
```bash
lit -v tests/lit/single/init/ -DLIB=$PWD/output -DCOMPILER=$PWD/target/debug/plc
```

---

## Conclusion

The initialization refactor has introduced **two critical bugs** that together account for **~60 of the 90 failures**:

1. **String array literal handling** - Type mismatch in array element conversion
2. **Array initialization value type** - New initialization produces incompatible value types

Both issues stem from changes in how initialization expressions are evaluated and represented in LLVM IR. The fixes should focus on maintaining compatibility between the expression generator and the variable initialization logic.

**Estimated Fix Time**:
- Phase 1 (String arrays): 2-4 hours
- Phase 2 (Array init types): 4-8 hours
- Phase 3 (Edge cases): 2-4 hours
- **Total**: 8-16 hours of focused debugging

The remaining 30 failures are likely cascading effects or separate edge cases that may resolve once the core issues are fixed.