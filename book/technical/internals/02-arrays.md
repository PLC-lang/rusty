# Arrays

A fixed array has an element type and constant bounds for each dimension. Codegen combines the dimensions into one flat block and computes an offset for each access. For example, `ARRAY[3..5] OF DINT` has three elements, so source index 3 maps to offset 0.

The example compares several dimensions with nested arrays and arrays of [structs](01-structs.md). It also shows initialization and parameter passing:

```iecst
VAR_GLOBAL CONSTANT
    MAX: DINT := 3;
END_VAR

TYPE Point:
    STRUCT
        x: DINT;
        y: DINT;
    END_STRUCT
END_TYPE

TYPE Data: ARRAY[0..9] OF DINT := [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]; END_TYPE

FUNCTION sum: DINT
    VAR_INPUT
        values: ARRAY[1..MAX] OF DINT;
    END_VAR
    VAR_IN_OUT
        target: ARRAY[0..1] OF DINT;
    END_VAR

    sum := values[1] + values[2] + values[3];
    target[0] := sum;
END_FUNCTION

PROGRAM main
    VAR
        a: Data;
        b: ARRAY[3..5] OF DINT := [3, 4, 5];
        neg: ARRAY[-2..2] OF INT;
        grid: ARRAY[0..1, 0..2] OF DINT;
        nested: ARRAY[0..1] OF ARRAY[0..2] OF DINT;
        points: ARRAY[0..1] OF Point := [(x := 1, y := 2), (x := 3)];
        rep: ARRAY[1..MAX] OF DINT := [(MAX)(7)];
        i: DINT;
        pair: ARRAY[0..1] OF DINT;
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

The parser records one range expression per dimension and a nested element type. Pre-processing gives each inline array a name, such as `__main_b` or `__sum_values`. A nested array needs two names: `__main_nested` refers to the inner array type `__main_nested_`. See [Index](../pipeline/02-index.md#pre-processing).

An array literal is a literal node whose elements are an expression list. The repetition `(MAX)(7)` becomes a call whose operator is the parenthesized constant, because the parser cannot tell the two apart.


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

Every bound is put into the constant store as an expression, even a plain literal, and constant evaluation at the end of the stage folds it (see [Index](../pipeline/02-index.md), Constant evaluation). `values: ARRAY[1..MAX]` therefore holds the expression `MAX` until the evaluator resolves it to 3. After indexing, the project has these array types, all with the nature `Any`:

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

A type-level initializer belongs to the type: `Data` stores the ID of its literal, and codegen computes the type's default value from it (a default-instance entry `__Data__init` is registered next to it, but no stage reads it; see [Initializers](07-initializers.md)). A variable-level initializer belongs to the variable entry: `main.b` and `main.points` store the ID of their literal, and their types have none.

Variable entries store type names. `main.a` uses `Data`, and the by-value input `sum.values` uses `__sum_values`. The in-out parameter `sum.target` instead stores the generated pointer type `__auto_pointer_to___sum_target`, which enables automatic dereferencing.


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

An array literal gets no annotation of its own, only a hint: the type of the place it is assigned to, `Data` for the literal in `a := [9, ...]`, `__main_pair` for `pair := [i, 2]`. The hint is pushed down. The element list receives the same array type, and every element receives the element type as its hint, so `9` is a `DINT` value hinted `DINT` and `i` is `main.i` hinted `DINT`.

For an array of structs, every parenthesized element is hinted with the struct type, `Point`, and its member assignments resolve against that struct, `Point.x` and `Point.y`:

```
        points: ARRAY[0..1] OF Point := [(x := 1, y := 2), (x := 3)];
                                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^   hint: "__main_points"
                                         ^^^^^^^^^^^^^^^^              hint: "Point"
                                          ^                            { kind: Variable, qualified_name: "Point.x", resulting_type: "DINT" }
```

The repetition `(MAX)(7)` is still a call here: its operator resolves to the constant `MAX` and the argument `7` is a `DINT`. The array lowerer sorts that out later. An array passed to a function is hinted with the type of the parameter: `rep` is hinted `__sum_values`, and `pair` is hinted `__auto_pointer_to___sum_target`, the pointer type of the in-out parameter.


## Lowering

Three participants touch arrays. The [init participant](../participants/06-init.md) moves every variable initializer into a constructor, so `self.b := [3, 4, 5]` and `self.points := [...]` become statements in `main__ctor`. It also gives every array type a constructor, an empty one, or a loop over the elements when the element type has one.

The [array lowerer](../participants/11-array.md) turns `(MAX)(7)` into the repetition `3(7)`. It splits `points` into assignments of struct elements and rewrites `pair := [i, 2]` as `pair[0] := i; pair[1] := 2`. Variable-to-variable copies and constant literals such as `a := [9, ...]` remain whole-array assignments.

A function that returns an array is rewritten by the [aggregate-return lowerer](../participants/09-aggregate-return.md) into a by-reference result parameter, the same way as for a string.


## Codegen

### Layout

An array type becomes an LLVM array of its element type, with the product of all dimension lengths as its length. `grid` has two dimensions of 2 and 3 elements and becomes `[6 x i32]`. `nested` is an array of arrays and stays nested, `[2 x [3 x i32]]`, because its inner type is an array type of its own. The program instance shows all of them, with the constant initializers in the static data and everything else zero:

```llvm
%main = type { [10 x i32], [3 x i32], [5 x i16], [6 x i32], [2 x [3 x i32]], [2 x %Point], [3 x i32], i32, [2 x i32] }

@main_instance = global %main {
    [10 x i32] [i32 0, i32 1, i32 2, i32 3, i32 4, i32 5, i32 6, i32 7, i32 8, i32 9],
    [3 x i32] [i32 3, i32 4, i32 5],
    [5 x i16] zeroinitializer, [6 x i32] zeroinitializer, [2 x [3 x i32]] zeroinitializer,
    [2 x %Point] zeroinitializer, [3 x i32] zeroinitializer, i32 0, [2 x i32] zeroinitializer }
```

`a` uses the default of `Data`; `b` uses its own literal. `points` and `rep` start as zeroed static data. Their initializers require lowering, so the constructors apply them later from private constants:

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

A repetition written with a literal count, `[3(7)]`, is folded into the static data as well; only the constant-name spelling stays zero there. The constructor writes both, because it writes every variable initializer.

### Element access

An access is one address computation, `getelementptr`, with a first index of zero to step into the array and a second index that is the flattened offset. For one dimension the offset is the index minus the lower bound. With constant indices LLVM folds that at build time, so `b[4]` on `ARRAY[3..5]` becomes index 1 and `neg[-2]` becomes index 0. For a variable index the subtraction is emitted, followed by the multiplication and addition the general formula below needs even for one dimension:

```llvm
  %tmpVar1 = getelementptr inbounds [3 x i32], ptr %b, i32 0, i32 1
  %tmpVar2 = getelementptr inbounds [5 x i16], ptr %neg, i32 0, i32 0

  %load_i = load i32, ptr %i, align 4
  %1 = sub i32 %load_i, -2
  %tmpVar4 = mul i32 1, %1
  %tmpVar5 = add i32 %tmpVar4, 0
  %tmpVar6 = getelementptr inbounds [5 x i16], ptr %neg, i32 0, i32 %tmpVar5
```

For several dimensions, subtract each lower bound and multiply by the number of elements in the following dimensions. Add the results. Thus `grid[1, 2]` on `ARRAY[0..1, 0..2]` has offset `1 * 3 + 2 * 1 = 5`. Nested arrays use one `getelementptr` per level. Struct elements add a member access after the array access. Each index is converted to `DINT` before the arithmetic.

```llvm
  %tmpVar3 = getelementptr inbounds [6 x i32], ptr %grid, i32 0, i32 5
  %tmpVar8 = getelementptr inbounds [2 x [3 x i32]], ptr %nested, i32 0, i32 1
  %tmpVar9 = getelementptr inbounds [3 x i32], ptr %tmpVar8, i32 0, i32 2
  %tmpVar12 = getelementptr inbounds [2 x %Point], ptr %points, i32 0, i32 1
  %y = getelementptr inbounds nuw %Point, ptr %tmpVar12, i32 0, i32 1
```

No bounds are checked at run time. A constant index outside the declared range is rejected by the validator; a variable index is not checked anywhere.

### Assignment

Assignment between compatible array variables copies the target type's size. A constant literal first becomes a private global. A supported literal with runtime elements is split into stores by the array lowerer. In the example, `a := [9, 8, ...]` uses a copy, while `pair := [i, 2]` uses two stores:

```llvm
  call void @llvm.memcpy.p0.p0.i64(ptr align 1 %a, ptr align 1 @.const_init, i64 ptrtoint (ptr getelementptr ([10 x i32], ptr null, i32 1) to i64), i1 false)
  %tmpVar14 = getelementptr inbounds [2 x i32], ptr %pair, i32 0, i32 0
  %load_i15 = load i32, ptr %i, align 4
  store i32 %load_i15, ptr %tmpVar14, align 4
  %tmpVar16 = getelementptr inbounds [2 x i32], ptr %pair, i32 0, i32 1
  store i32 2, ptr %tmpVar16, align 4
```

### Passing

An array is passed as a pointer in both directions. A by-value `VAR_INPUT` is copied into a local of the parameter's type at the start of the callee, so the callee works on its own copy; a `VAR_IN_OUT` is a pointer that is stored and dereferenced on every access:

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

`LOWER_BOUND` and `UPPER_BOUND` accept [variable-length arrays](05-variable-length-arrays.md), which carry bounds at run time. A fixed array call produces E037, for example `cannot assign 'ARRAY[-2..2] OF INT' to 'VARIABLE LENGTH ARRAY'`.


## Validation

The declaration is checked for bounds that are constant (E117), integer, and in ascending order (E097, `Invalid range 5..0`). An initializer with more elements than the array holds is rejected (E043). An access is checked for the number of dimensions (E045) and, for a constant index, against the declared range (E058, `Array access must be in the range 0..2`). Two arrays of different dimensions, or with different element types, are an invalid assignment (E037), and the same comparison of dimensions applies to an array argument.


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
