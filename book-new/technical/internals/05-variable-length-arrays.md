# Variable-Length Arrays

A fixed array such as `ARRAY[0..2] OF DINT` has its bounds in its type, so a function that takes it accepts exactly that one shape. A variable-length array (VLA) replaces every bound with `*`: a parameter `ARRAY[*] OF DINT` accepts any one-dimensional array of `DINT`, and `ARRAY[*, *] OF INT` any two-dimensional array of `INT`. VLAs exist only as parameters. Internally a VLA is a small struct that pairs a pointer to the caller's array with that array's bounds, and every element access computes its offset from those bounds at run time.

This chapter follows one project through the compiler:

```iecst
FUNCTION sum : DINT
    VAR_IN_OUT
        values : ARRAY[*] OF DINT;
    END_VAR
    VAR
        i : DINT;
    END_VAR

    FOR i := LOWER_BOUND(values, 1) TO UPPER_BOUND(values, 1) DO
        sum := sum + values[i];
    END_FOR
END_FUNCTION

FUNCTION fill : DINT
    VAR_INPUT {ref}
        grid : ARRAY[*, *] OF INT;
    END_VAR

    grid[0, 1] := 7;
END_FUNCTION

PROGRAM main
    VAR
        small : ARRAY[0..2] OF DINT := [1, 2, 3];
        large : ARRAY[10..19] OF DINT;
        table : ARRAY[0..1, 0..2] OF INT;
        total : DINT;
    END_VAR

    total := sum(small);
    total := total + sum(large);
    fill(table);
END_PROGRAM
```


## Declaration

The parser produces an ordinary array type node with a flag that marks it as variable-length. The bounds are a placeholder node per `*`, so `ARRAY[*]` has one and `ARRAY[*, *]` has a list of two; the element type is a reference like in any array. Because the type is written inline, pre-processing moves it out into a named type scoped to the POU, `__sum_values` and `__fill_grid` here, exactly as it does for a fixed array (see [Arrays](02-arrays.md) and [Index](../pipeline/02-index.md), Pre-processing). The `{ref}` on `fill`'s input block is the parser's by-reference marker; a VLA parameter is always passed by reference, and the Validation section below lists what happens when the marker is missing.


## Index

A VLA does not become an array in the index. The indexer registers it as a struct with two members and a source marker that says what the struct stands for:

```rust
Struct {
    /// The pre-processed type name, __sum_values
    name: String,

    /// Two members: a pointer to the array and the bounds array
    members: Vec<VariableIndexEntry>,

    /// Marks the struct as a VLA of one element type with a fixed number of dimensions
    source: StructSource::Internal(VariableLengthArray { inner_type_name, ndims }),
}
```

For `values : ARRAY[*] OF DINT` the indexer creates four types. The struct itself, `__sum_values`, has the nature `__VLA`, which the resolver and the validator test for. Its first member, `struct_vla_dint_1`, is a pointer of type `__ptr_to___sum_values_vla_1_dint` to `__sum_values_vla_1_dint`, an array of `DINT` with one dimension whose bounds are marked undetermined; this array type exists only so that a reference to the VLA can be hinted with an array type, and codegen never lays it out. The second member, `dimensions`, is `__bounds___sum_values_vla_1_dint`, a fixed array of `DINT` with two entries per dimension, the lower and the upper bound:

```
__sum_values                       Struct, nature __VLA, source VariableLengthArray { DINT, 1 dim }
  .struct_vla_dint_1               __ptr_to___sum_values_vla_1_dint
  .dimensions                      __bounds___sum_values_vla_1_dint       ARRAY[0..1] OF DINT
__sum_values_vla_1_dint            Array of DINT, bounds Undetermined
__fill_grid                        Struct, nature __VLA, source VariableLengthArray { INT, 2 dims }
  .struct_vla_int_2                __ptr_to___fill_grid_vla_2_int
  .dimensions                      __bounds___fill_grid_vla_2_int         ARRAY[0..1, 0..1] OF DINT
```

The parameter entries do not point at the struct directly. Like every by-reference parameter, `sum.values` and `fill.grid` store an auto-dereferencing pointer type, `__auto_pointer_to___sum_values`, so a body can write `values[i]` without a `^` (see [Index](../pipeline/02-index.md), Indexing a unit). The names of the helper types are built from the struct name, the dimension count, and the element type, so two VLA parameters never share them, even when their shapes agree.


## Annotations

Inside the callee, a reference to the VLA is annotated as the variable it is, of the struct type, and then hinted with the array type behind the pointer member. The hint is what tells validation and codegen that `values` can be indexed although its type is a struct. The index expression itself is annotated with the element type:

```
    sum := sum + values[i];
                 ^^^^^^^^^        { kind: Value,                                        resulting_type: "DINT" }
                 ^^^^^^           { kind: Variable, qualified_name: "sum.values",       resulting_type: "__sum_values",  hint: Variable "__sum_values_vla_1_dint" }
                        ^         { kind: Variable, qualified_name: "sum.i",            resulting_type: "DINT",          hint: None }

    grid[0, 1] := 7;
    ^^^^^^^^^^                    { kind: Value,                                        resulting_type: "INT" }
    ^^^^                          { kind: Variable, qualified_name: "fill.grid",        resulting_type: "__fill_grid",   hint: Variable "__fill_grid_vla_2_int" }
                  ^               { kind: Value,                                        resulting_type: "DINT",          hint: "INT" }
```

At the call site the argument is an ordinary fixed array, and it receives the argument hint of the parameter, the auto-dereferencing pointer to the VLA struct. The mismatch between the annotation (a fixed array) and the hint (a pointer to a VLA struct) is the signal codegen acts on:

```
    total := sum(small);
                 ^^^^^            { kind: Variable, qualified_name: "main.small",       resulting_type: "__main_small",  hint: Argument { resulting_type: "__auto_pointer_to___sum_values", position: 0 } }
```

`LOWER_BOUND` and `UPPER_BOUND` are built-in generic functions declared as `FUNCTION LOWER_BOUND<U: __ANY_VLA, T: ANY_INT> : DINT`. Their annotation hints the first argument with its own VLA type when it is one, and with the reserved placeholder type `__VLA` when it is not, so that the argument fails the type check with a readable name; the second argument keeps its integer type. The call is a `DINT` value.


## Lowering

No participant rewrites VLAs. The [loop desugarer](../participants/01-loop-desugar.md) turns the `FOR` of the example into a `WHILE TRUE` loop, which is why the bounds calls appear once before the loop and once per iteration in the IR below.


## Codegen

**Layout.** The struct is laid out as declared: a pointer and an array of `i32` with two entries per dimension. The callee receives a pointer to it:

```llvm
%__sum_values = type { ptr, [2 x i32] }
%__fill_grid = type { ptr, [4 x i32] }

define i32 @sum(ptr %0)
define i32 @fill(ptr %0)
```

**Passing.** The caller owns the struct. For every argument whose hint is a VLA and whose own type is a fixed array, codegen allocates the struct on the caller's stack, stores the address of the array's first element into the pointer field, and stores the bounds of the array's type, known at compile time, into the dimensions field. The bounds are written in declaration order, lower then upper per dimension. For `sum(large)` and `fill(table)`:

```llvm
  %outer_arr_gep2 = getelementptr inbounds [10 x i32], ptr %large, i32 0, i32 0
  %vla_struct3 = alloca %__sum_values, align 8
  %vla_array_gep4 = getelementptr inbounds nuw %__sum_values, ptr %vla_struct3, i32 0, i32 0
  %vla_dimensions_gep5 = getelementptr inbounds nuw %__sum_values, ptr %vla_struct3, i32 0, i32 1
  store [2 x i32] [i32 10, i32 19], ptr %vla_dimensions_gep5, align 4
  store ptr %outer_arr_gep2, ptr %vla_array_gep4, align 8
  ...
  %call7 = call i32 @sum(ptr %vla_struct_ptr6)

  store [4 x i32] [i32 0, i32 1, i32 0, i32 2], ptr %vla_dimensions_gep12, align 4
  ...
  %call14 = call i32 @fill(ptr %vla_struct_ptr13)
```

The same call with `small` stores `[i32 0, i32 2]`. The callee only ever sees the struct, so `sum` compiles once and runs on both arrays.

> **Developer Note**
>
> Every call allocates a fresh struct, and the value is copied once more into a second stack slot before the call (`%vla_struct` then `%vla_struct_ptr`). A loop that calls `sum` a thousand times performs a thousand allocations; the optimizer removes most of them. The wrap is done by the argument generator for functions and methods only. A call to a function block instance, `inst(io := arr)`, stores the bare array address into the instance member, and the body then reads it as a VLA struct; see `bugs.md`.

**Element access.** An access `values[i]` cannot use a constant offset, because the lower bound is a run-time value. Codegen loads the data pointer and the bounds from the struct, subtracts the lower bound from the index, and indexes the data pointer with the result:

```llvm
  %vla_arr_gep = getelementptr inbounds nuw %__sum_values, ptr %deref17, i32 0, i32 0
  %vla_arr_ptr = load ptr, ptr %vla_arr_gep, align 8
  %dim_arr = getelementptr inbounds nuw %__sum_values, ptr %deref17, i32 0, i32 1
  %start_idx_ptr0 = getelementptr inbounds [2 x i32], ptr %dim_arr, i32 0, i32 0
  %start_idx_value0 = load i32, ptr %start_idx_ptr0, align 4
  %tmpVar19 = sub i32 %load_i18, %start_idx_value0
  %arr_val = getelementptr inbounds i32, ptr %vla_arr_ptr, i32 %tmpVar19
  %load_tmpVar = load i32, ptr %arr_val, align 4
```

With `large`, whose bounds are `10..19`, `values[10]` becomes element zero. For more than one dimension the access mirrors the compile-time formula for fixed arrays (see [Arrays](02-arrays.md)), evaluated at run time: the length of every dimension is `upper - lower + 1`, each index is normalized by its lower bound, and the flat offset is the sum of every normalized index times the product of the lengths of the dimensions after it. For `grid[0, 1]` codegen loads all four bounds, computes `len_dim1`, multiplies the first normalized index by it, and adds the second:

```llvm
  %1 = sub i32 %end_idx_value0, %start_idx_value0
  %len_dim0 = add i32 1, %1
  %2 = sub i32 %end_idx_value1, %start_idx_value1
  %len_dim1 = add i32 1, %2
  %adj_access0 = sub i32 0, %start_idx_value0
  %adj_access1 = sub i32 1, %start_idx_value1
  %multiply = mul i32 %adj_access0, %accessor_factor
  %multiply3 = mul i32 %adj_access1, 1
  %accessor = load i32, ptr %accum1, align 4
  %arr_val = getelementptr inbounds i16, ptr %vla_arr_ptr, i32 %accessor
  store i16 7, ptr %arr_val, align 2
```

The intermediate products and sums go through small stack slots named `accum`, which is why the IR is longer than the formula.

**Bounds.** `LOWER_BOUND(values, 1)` reads one entry of the dimensions array. Dimension `n` occupies entries `2(n-1)` and `2(n-1)+1`, so the lower bound of dimension 1 is entry 0 and the upper bound entry 1; a literal dimension gives a constant entry, an expression gives the same arithmetic at run time:

```llvm
  %dim = getelementptr inbounds nuw %__sum_values, ptr %deref, i32 0, i32 1
  %1 = getelementptr inbounds [2 x i32], ptr %dim, i32 0, i32 0
  %2 = load i32, ptr %1, align 4
```

No bounds check is generated for an element access; an index outside the passed array reads or writes past it, as it does for a fixed array.


## Validation

The validator keeps VLAs to the places where a struct of caller-owned storage makes sense. A VLA is accepted as `VAR_INPUT {ref}`, `VAR_OUTPUT`, or `VAR_IN_OUT` of a function or method, and as `VAR_IN_OUT` of a function block. It is rejected as a global variable, anywhere in a program, and as a local or other block of a function block (E044, "Variable Length Arrays are not allowed to be defined as ... variables inside a ..."). A by-value `VAR_INPUT` without `{ref}` in a function is a warning (E047) and is treated as by-reference.

At a call, the argument must match the parameter in element type and dimension count; `sum(ints)` with an `INT` array and `sum(grid)` with a two-dimensional array are invalid assignments (E037), reported with the array types in the message. Inside a body, a VLA cannot be assigned to another VLA (E037). An access must use as many indices as the VLA has dimensions (E045), and a literal dimension passed to `LOWER_BOUND` or `UPPER_BOUND` must be between one and the dimension count (E046). A fixed array passed to a bound function fails the generic constraint and is reported against the placeholder type as `cannot assign 'ARRAY[0..2] OF DINT' to 'VARIABLE LENGTH ARRAY'` (E037).

> **Developer Note**
>
> Two of the rejected declarations never reach the validator: a global VLA and a VLA in a by-reference input block of a function block make the resolver panic while it hints the constructor call that the init participant generated for them. Reading a function block's VLA parameter from the caller, `inst.io[0]`, panics the same way on valid input. A VLA as a function return type is not checked at all: the declaration alone passes, and a function that uses it panics in the same place. See `bugs.md`.


## At a glance

| Structured Text | Index | Annotation | LLVM |
|---|---|---|---|
| `values : ARRAY[*] OF DINT` (parameter) | struct `__<pou>_<var>` with a pointer and a bounds member, nature `__VLA`; the parameter holds an auto-dereferencing pointer to it | `Variable` of the struct, hinted with the placeholder array `__<pou>_<var>_vla_1_dint` | `{ ptr, [2 x i32] }`, passed as `ptr` |
| `ARRAY[*, *] OF INT` | same, with two dimensions in the source marker | same | `{ ptr, [4 x i32] }` |
| `f(small)` with `small : ARRAY[0..2] OF DINT` | | argument hinted with the pointer to the VLA struct | `alloca` of the struct, store data pointer and `[0, 2]`, pass its address |
| `values[i]` | | `Value` of the element type | load pointer and lower bound, `sub`, `getelementptr` |
| `grid[a, b]` | | `Value` of the element type | run-time lengths, normalized indices, multiply and accumulate |
| `LOWER_BOUND(values, 1)` | built-in generic `<U: __ANY_VLA, T: ANY_INT>` | `Value DINT`; the VLA argument hinted with its own type | load entry `2(n-1)` of the bounds array; `UPPER_BOUND` entry `2(n-1)+1` |
