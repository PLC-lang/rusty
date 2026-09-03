# Arrays

An array in Structured Text has a fixed element type and one or more dimensions with a fixed lower and upper bound, `ARRAY[3..5] OF DINT` or `ARRAY[0..1, 0..2] OF DINT`. Internally an array is one flat block of elements that starts at index zero: every dimension is folded into a single length, every access subtracts the lower bound before it becomes an offset, and every assignment of a whole array is one bounded copy. The bounds survive in the generated code only as constants in the arithmetic.

This chapter follows one project through the compiler:

```iecst
VAR_GLOBAL CONSTANT
    MAX : DINT := 3;
END_VAR

TYPE Point : STRUCT
    x : DINT;
    y : DINT;
END_STRUCT END_TYPE

TYPE Data : ARRAY[0..9] OF DINT := [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]; END_TYPE

FUNCTION sum : DINT
    VAR_INPUT
        values : ARRAY[1..MAX] OF DINT;
    END_VAR
    VAR_IN_OUT
        target : ARRAY[0..1] OF DINT;
    END_VAR

    sum := values[1] + values[2] + values[3];
    target[0] := sum;
END_FUNCTION

PROGRAM main
    VAR
        a : Data;
        b : ARRAY[3..5] OF DINT := [3, 4, 5];
        neg : ARRAY[-2..2] OF INT;
        grid : ARRAY[0..1, 0..2] OF DINT;
        nested : ARRAY[0..1] OF ARRAY[0..2] OF DINT;
        points : ARRAY[0..1] OF Point := [(x := 1, y := 2), (x := 3)];
        rep : ARRAY[1..MAX] OF DINT := [(MAX)(7)];
        i : DINT;
        pair : ARRAY[0..1] OF DINT;
    END_VAR

    a[2] := b[4];
    neg[-2] := 1;
    grid[1, 2] := neg[i];
    nested[1][2] := grid[1, 2];
    points[1].y := points[0].x;
    a := [9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
    pair := [i, 2];
    i := sum(rep, pair);
END_PROGRAM
```


## Declaration

The parser produces an array type node with the bounds as a list of range expressions, one per dimension, and the element type as a nested type declaration. Only `Data` has a name at this point. Every inline array in a variable declaration is moved out by pre-processing at the start of the index stage and named after container and member, `__main_b`, `__main_grid`, `__sum_values` (see [Index](../pipeline/02-index.md), Pre-processing). An array of arrays is split twice: `nested` becomes `__main_nested`, whose element type is the second generated type `__main_nested_`. An array literal is a literal node whose elements are an expression list; the repetition `(MAX)(7)` is parsed as a call whose operator is the parenthesized constant, since the parser cannot tell the two apart.


## Index

The type index holds one record per array type. Trimmed to the array variant of the type information:

```rust
Array {
    /// The type of the elements, as a name; for an array of arrays the inner generated type
    inner_type_name: TypeId,

    /// One entry per dimension, each with a start and an end offset
    dimensions: Vec<Dimension>,
}

pub struct Dimension {
    /// The lower bound, as a literal or a constant expression
    pub start_offset: TypeSize,

    /// The upper bound, inclusive
    pub end_offset: TypeSize,
}
```

Every bound is put into the constant store as an expression, even a plain literal, and constant evaluation at the end of the stage folds it (see [Index](../pipeline/02-index.md), Constant evaluation). `values : ARRAY[1..MAX]` therefore holds the expression `MAX` until the evaluator resolves it to 3. After indexing, the project has these array types, all with the nature `Any`:

```
Data             inner: DINT              dims: [0..9]            initializer: ConstId -> [0, 1, ..., 9]
__sum_values     inner: DINT              dims: [1..MAX -> 3]
__sum_target     inner: DINT              dims: [0..1]
__main_b         inner: DINT              dims: [3..5]
__main_neg       inner: INT               dims: [-2..2]
__main_grid      inner: DINT              dims: [0..1], [0..2]
__main_nested    inner: __main_nested_    dims: [0..1]
__main_nested_   inner: DINT              dims: [0..2]
__main_points    inner: Point             dims: [0..1]
__main_rep       inner: DINT              dims: [1..MAX -> 3]
__main_pair      inner: DINT              dims: [0..1]
```

A type-level initializer belongs to the type: `Data` stores the id of its literal, and codegen computes the type's default value from it (a default-instance entry `__Data__init` is registered next to it, but no stage reads it; see [Initializers](07-initializers.md)). A variable-level initializer belongs to the variable entry: `main.b` and `main.points` store the id of their literal, and their types have none.

The variable entries know nothing about arrays. `main.a` stores the type name `Data`, `sum.values` stores `__sum_values` as a by-value input. A `VAR_IN_OUT` array does not keep its type: `sum.target` stores the auto-dereferencing pointer type `__auto_pointer_to___sum_target`, like every by-reference parameter.


## Annotations

An index access is a reference expression whose access part is the index expression, or an expression list for several dimensions, and whose base is the array. The resolver annotates the access with the element type of the base's array type; the index expressions themselves are ordinary values with no hint. A member access on an element continues from there. For the body of `main`:

```
    a[2] := b[4];
    ^^^^                     { kind: Value,    resulting_type: "DINT",  hint: None }
    ^                        { kind: Variable, qualified_name: "main.a",   resulting_type: "Data",         hint: None }
      ^                      { kind: Value,    resulting_type: "DINT",  hint: None }
            ^^^^             { kind: Value,    resulting_type: "DINT",  hint: "DINT" }

    grid[1, 2] := neg[i];
    ^^^^^^^^^^               { kind: Value,    resulting_type: "DINT",  hint: None }
    ^^^^                     { kind: Variable, qualified_name: "main.grid", resulting_type: "__main_grid", hint: None }
         ^^^^                no annotation; the list is only a container for the two index values
                  ^^^^^^     { kind: Value,    resulting_type: "INT",   hint: "DINT" }
                      ^      { kind: Variable, qualified_name: "main.i",   resulting_type: "DINT",         hint: None }

    nested[1][2] := grid[1, 2];
    ^^^^^^^^^^^^             { kind: Value,    resulting_type: "DINT",  hint: None }
    ^^^^^^^^^                { kind: Value,    resulting_type: "__main_nested_", hint: None }

    points[1].y := points[0].x;
              ^              { kind: Variable, qualified_name: "Point.y", resulting_type: "DINT", hint: None }
    ^^^^^^^^^                { kind: Value,    resulting_type: "Point", hint: None }
```

An array literal gets no annotation of its own, only a hint: the type of the place it is assigned to, `Data` for the literal in `a := [9, ...]`, `__main_pair` for `pair := [i, 2]`. The hint is pushed down: the element list receives the same array type, and every element receives the element type as its hint, so `9` is a `DINT` value hinted `DINT` and `i` is `main.i` hinted `DINT`. For an array of structs, each parenthesized element is hinted with the struct type, `Point`, and its member assignments resolve against that struct, `Point.x` and `Point.y`:

```
        points : ARRAY[0..1] OF Point := [(x := 1, y := 2), (x := 3)];
                                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^   hint: "__main_points"
                                          ^^^^^^^^^^^^^^^^^^            hint: "Point"
                                           ^                            { kind: Variable, qualified_name: "Point.x", resulting_type: "DINT" }
```

The repetition `(MAX)(7)` is still a call at this point: its operator resolves to the constant `MAX` and the argument `7` is a `DINT`; the array lowerer sorts this out later. Passing an array to a function hints the argument with the parameter's type: `rep` is hinted `__sum_values`, and `pair` is hinted `__auto_pointer_to___sum_target`, the pointer type of the in-out parameter.


## Lowering

Three participants touch arrays. The [init participant](../participants/06-init.md) moves every variable initializer into a constructor, so `self.b := [3, 4, 5]` and `self.points := [...]` become statements in `main__ctor`, and gives every array type an empty constructor, or a loop over the elements when the element type has one. The [array lowerer](../participants/11-array.md) then rewrites the assignments that codegen cannot store as one constant: `(MAX)(7)` becomes the repetition `3(7)`, the struct-literal elements of `points` become two element assignments, and `pair := [i, 2]` in the body becomes `pair[0] := i; pair[1] := 2`. A whole-array assignment between two variables, or from a literal with only constant elements such as `a := [9, ...]`, is left alone. Functions returning an array are rewritten by the [aggregate-return lowerer](../participants/09-aggregate-return.md) into a by-reference result parameter, the same way as for strings.


## Codegen

**Layout.** An array type becomes an LLVM array of its element type whose length is the product of all dimension lengths. `grid` has two dimensions of 2 and 3 elements and becomes `[6 x i32]`; `nested` is an array of arrays and stays nested, `[2 x [3 x i32]]`, because its inner type is an array type of its own. The program instance shows all of them, with the constant initializers written into the static data and everything else zero:

```llvm
%main = type { [10 x i32], [3 x i32], [5 x i16], [6 x i32], [2 x [3 x i32]], [2 x %Point], [3 x i32], i32, [2 x i32] }

@main_instance = global %main {
    [10 x i32] [i32 0, i32 1, i32 2, i32 3, i32 4, i32 5, i32 6, i32 7, i32 8, i32 9],
    [3 x i32] [i32 3, i32 4, i32 5],
    [5 x i16] zeroinitializer, [6 x i32] zeroinitializer, [2 x [3 x i32]] zeroinitializer,
    [2 x %Point] zeroinitializer, [3 x i32] zeroinitializer, i32 0, [2 x i32] zeroinitializer }
```

`a` takes the initializer of its type `Data`, `b` its own. `points` and `rep` are zero in the static data although both have initializers: their literals contain a struct literal or a repetition, which the array lowerer turned into constructor statements, and the constructor copies them at start-up from private constants:

```llvm
@.const_init.3 = private unnamed_addr constant %Point { i32 1, i32 2 }
@.const_init.5 = private unnamed_addr constant [3 x i32] [i32 7, i32 7, i32 7]

define void @main__ctor(ptr %0) {
  ...
  %tmpVar = getelementptr inbounds [2 x %Point], ptr %points9, i32 0, i32 0
  call void @llvm.memcpy.p0.p0.i64(ptr align 1 %tmpVar, ptr align 1 @.const_init.3, i64 ptrtoint (ptr getelementptr (%Point, ptr null, i32 1) to i64), i1 false)
  ...
  call void @llvm.memcpy.p0.p0.i64(ptr align 1 %rep15, ptr align 1 @.const_init.5, i64 ptrtoint (ptr getelementptr ([3 x i32], ptr null, i32 1) to i64), i1 false)
  ret void
}
```

A repetition written with a literal count, `[3(7)]`, stays static data; only the constant-name spelling takes the constructor path.

**Element access.** An access is one address computation, `getelementptr`, with a first index of zero to step into the array and a second index that is the flattened offset. For one dimension the offset is the index minus the lower bound; with constant indices LLVM folds this at build time, so `b[4]` on `ARRAY[3..5]` becomes index 1 and `neg[-2]` becomes index 0. For a variable index the subtraction is emitted, followed by the multiplication and addition that the general formula below needs even for one dimension:

```llvm
  %tmpVar1 = getelementptr inbounds [3 x i32], ptr %b, i32 0, i32 1
  %tmpVar2 = getelementptr inbounds [5 x i16], ptr %neg, i32 0, i32 0

  %load_i = load i32, ptr %i, align 4
  %1 = sub i32 %load_i, -2
  %tmpVar4 = mul i32 1, %1
  %tmpVar5 = add i32 %tmpVar4, 0
  %tmpVar6 = getelementptr inbounds [5 x i16], ptr %neg, i32 0, i32 %tmpVar5
```

For several dimensions the offset is the sum of each adjusted index times the number of elements to its right: on `ARRAY[0..1, 0..2]` the access `grid[1, 2]` is `1 * 3 + 2 * 1`, which folds to 5. An array of arrays is accessed in two steps, one `getelementptr` per level, `[2 x [3 x i32]]` first and `[3 x i32]` second; an element that is a struct continues with the member's field index. The index expression is cast to `DINT` before it enters the arithmetic.

```llvm
  %tmpVar3 = getelementptr inbounds [6 x i32], ptr %grid, i32 0, i32 5
  %tmpVar8 = getelementptr inbounds [2 x [3 x i32]], ptr %nested, i32 0, i32 1
  %tmpVar9 = getelementptr inbounds [3 x i32], ptr %tmpVar8, i32 0, i32 2
  %tmpVar12 = getelementptr inbounds [2 x %Point], ptr %points, i32 0, i32 1
  %y = getelementptr inbounds nuw %Point, ptr %tmpVar12, i32 0, i32 1
```

No bounds are checked at run time. A constant index outside the declared range is rejected by the validator; a variable index is not checked anywhere.

**Assignment.** A whole array is an aggregate, so `a := b` is a `memcpy` of the target type's size, like the struct and string cases. A literal with only constant elements is first materialized as a private constant and then copied, so `a := [9, 8, ...]` is one `memcpy` from `@.const_init`; a literal with a runtime element has been split into element stores by the array lowerer before codegen sees it:

```llvm
  call void @llvm.memcpy.p0.p0.i64(ptr align 1 %a, ptr align 1 @.const_init, i64 ptrtoint (ptr getelementptr ([10 x i32], ptr null, i32 1) to i64), i1 false)
  %tmpVar14 = getelementptr inbounds [2 x i32], ptr %pair, i32 0, i32 0
  %load_i15 = load i32, ptr %i, align 4
  store i32 %load_i15, ptr %tmpVar14, align 4
  %tmpVar16 = getelementptr inbounds [2 x i32], ptr %pair, i32 0, i32 1
  store i32 2, ptr %tmpVar16, align 4
```

**Passing.** An array is passed as a pointer in both directions. A by-value `VAR_INPUT` is copied into a local of the parameter's type at the start of the callee, so the callee works on its own copy; a `VAR_IN_OUT` is a pointer that is stored and dereferenced on every access:

```llvm
define i32 @sum(ptr %0, ptr %1) {
entry:
  %sum = alloca i32, align 4
  %values = alloca [3 x i32], align 4
  call void @llvm.memcpy.p0.p0.i64(ptr align 1 %values, ptr align 1 %0, i64 ptrtoint (ptr getelementptr ([3 x i32], ptr null, i32 1) to i64), i1 false)
  %target = alloca ptr, align 8
  store ptr %1, ptr %target, align 8
  ...
  %deref = load ptr, ptr %target, align 8
  %tmpVar7 = getelementptr inbounds [2 x i32], ptr %deref, i32 0, i32 0
```

The caller passes the addresses of its own variables in both cases, `call i32 @sum(ptr %rep, ptr %pair)`. The copy in the callee uses the parameter's size, which is also the argument's size, because the validator requires equal dimensions for an array argument.

`LOWER_BOUND` and `UPPER_BOUND` accept only [variable-length arrays](05-variable-length-arrays.md); on a fixed array the validator rejects the call with `cannot assign 'ARRAY[-2..2] OF INT' to 'VARIABLE LENGTH ARRAY'` (E037), because the bounds of a fixed array have no representation at run time.


## Validation

The declaration is checked for bounds that are constant (E117), integer, and in ascending order (E097, `Invalid range 5..0`). An initializer with more elements than the array holds is rejected (E043). An access is checked for the number of dimensions (E045) and, for a constant index, for the declared range (E058, `Array access must be in the range 0..2`). Assigning arrays of different dimensions, or with different element types, is an invalid assignment (E037), and the same comparison of dimensions applies to array arguments.

> **Developer Note**
>
> A repetition whose count is a constant name works only in the parenthesized spelling `[(MAX)(7)]`. The bare spelling `[MAX(7)]` is parsed as a call and is neither rewritten by the array lowerer nor rejected by the validator; codegen aborts on it. Logged in `bugs.md`.


## At a glance

| Structured Text | Index | Annotation | LLVM |
|---|---|---|---|
| `ARRAY[a..b] OF T` | pre-processed type `__<pou>_<var>`, one dimension, bounds in the constant store | that type's name | `[b-a+1 x T]` |
| `ARRAY[a..b, c..d] OF T` | one type, two dimensions | that type's name | `[(b-a+1)*(d-c+1) x T]` |
| `ARRAY[..] OF ARRAY[..] OF T` | two types, the outer's inner type is the inner's name | | `[n x [m x T]]` |
| `arr[i]` | | `Value` of the element type; `i` has no hint | `getelementptr arr, 0, i - lower` |
| `arr[i, j]` | | `Value` of the element type | `getelementptr arr, 0, (i - lower_i) * len_j + (j - lower_j)` |
| `[1, 2, 3]` | | no annotation, hinted with the target array type; elements hinted with the element type | private constant plus `memcpy`, or one `store` per element after lowering |
| `a := b` | | `b` hinted with the type of `a` | `memcpy` of the type's size |
| `f(arr)` by value | `f.arr` of the array type | `arr` hinted with the parameter type | `ptr`, copied into a local of the parameter's size |
| `VAR_IN_OUT arr` | `__auto_pointer_to_<type>` | hinted with the pointer type | `ptr`, dereferenced on every access |
