# Strings

`STRING` uses one-byte storage units; `WSTRING` uses two-byte units. Both have fixed capacity plus one slot for a zero terminator. `STRING[5]` has five data slots, while an unsized `STRING` has 80. Assignment copies data within the target capacity and truncates longer values without a diagnostic. Parameter passing has a separate copy path, described below.

Strings use the array storage described in [Arrays](02-arrays.md), with special rules for length and termination. The example shows literals, a shorter target, wide strings, and a function return:

```iecst
VAR_GLOBAL CONSTANT
    SIZE: DINT := 5;
END_VAR

FUNCTION greet: STRING
    VAR_INPUT
        who: STRING;
    END_VAR

    greet := who;
END_FUNCTION

PROGRAM main
    VAR
        text: STRING;
        short: STRING[SIZE] := 'hi';
        wide: WSTRING[10];
    END_VAR

    text := 'hello';
    short := text;
    wide := "world";
    text := greet('bob');
END_PROGRAM
```


## Declaration

The parser produces a string type node with two facts and no name: whether the type is wide, and the length expression if one was written. `text: STRING` is not such a node; it is a plain reference to the built-in type `STRING`.

Only `STRING[SIZE]` and `WSTRING[10]` are inline type definitions. Pre-processing at the start of the index stage moves them out into named types scoped to `main`, `__main_short` and `__main_wide`, and replaces the declaration with a reference to that name (see [Index](../pipeline/02-index.md), Pre-processing). A string literal is a literal node with the text and a wide flag: `'hello'` is narrow, `"world"` is wide.


## Index

The type index holds one record per string type. Trimmed to the string variant of the type information:

```rust
String {
    /// Capacity in characters plus one for the terminator, as a literal or a constant expression
    size: TypeSize,

    /// Utf8 for STRING, Utf16 for WSTRING
    encoding: StringEncoding,

    /// Whether the source wrote a length; false for plain STRING and WSTRING
    declared_with_length: bool,
}
```

The stored size includes the terminator. Built-in strings have size 81, and `WSTRING[10]` has size 11. For `STRING[SIZE]`, the index stores the expression `SIZE + 1`; constant evaluation resolves it to 6. The example therefore has four string types:

```
STRING          { size: 81,                encoding: Utf8,  declared_with_length: false }   built-in
WSTRING         { size: 81,                encoding: Utf16, declared_with_length: false }   built-in
__main_short    { size: SIZE + 1 -> 6,     encoding: Utf8,  declared_with_length: true }    from STRING[SIZE]
__main_wide     { size: 11,                encoding: Utf16, declared_with_length: true }    from WSTRING[10]
```

Variable entries store the string type name and any initializer ID. Here, `main.text`, `greet.who`, and the return variable use `STRING`; `main.short` uses `__main_short`. The type index supplies capacity and encoding.


## Annotations

Each string literal gets a type named for its encoding and length. For `'hello'`, the resolver creates `__STRING_5` with size 6 and later imports it into the global index. The target type becomes the hint used for copying. For the body of `main`:

```
    text := 'hello';
    ^^^^                     { kind: Variable, qualified_name: "main.text",  resulting_type: "STRING",       hint: None }
            ^^^^^^^          { kind: Value,                                  resulting_type: "__STRING_5",   hint: "STRING" }

    short := text;
    ^^^^^                    { kind: Variable, qualified_name: "main.short", resulting_type: "__main_short", hint: None }
             ^^^^            { kind: Variable, qualified_name: "main.text",  resulting_type: "STRING",       hint: "__main_short" }

    wide := "world";
    ^^^^                     { kind: Variable, qualified_name: "main.wide",  resulting_type: "__main_wide",  hint: None }
            ^^^^^^^          { kind: Value,                                  resulting_type: "__WSTRING_5",  hint: "__main_wide" }

    text := greet('bob');
    ^^^^                     { kind: Variable, qualified_name: "main.text",  resulting_type: "STRING",       hint: None }
            ^^^^^^^^^^^^     { kind: Value,                                  resulting_type: "STRING",       hint: "STRING" }
            ^^^^^            { kind: Function, qualified_name: "greet",      return_type: "STRING",          hint: None }
                  ^^^^^      { kind: Value,                                  resulting_type: "__STRING_3",   hint: Argument { resulting_type: "STRING", position: 0 } }
```

The resolver collects distinct literals from bodies so that codegen can emit private constants. The initializer `'hi'` enters this collection after the init participant moves it into a constructor body.

For a comparison such as `text = 'hello'`, the resolver attaches a replacement expression: `STRING_EQUAL(text, 'hello')`, or `WSTRING_EQUAL` for wide strings. It combines `_EQUAL`, `_LESS`, and `_GREATER` calls with `NOT` and `OR` for the other comparisons. The standard library supplies these functions; a missing one produces E073. See [ReplacementAst](08-annotated-ast.md#replacementast).


## Lowering

No participant rewrites strings themselves, but two rewrite the places they appear in. The [aggregate-return lowerer](../participants/09-aggregate-return.md) turns `greet` into a void function with a `VAR_IN_OUT greet: STRING` parameter and gives the call site a temporary, so by codegen the last statement of `main` reads `greet(__greet0, 'bob'); text := __greet0;`. The [init participant](../participants/06-init.md) moves the initializer `'hi'` out of the declaration into the constructor of `main`, as `self.short := 'hi'`. Codegen sees only assignments, calls, and pointer parameters.


## Codegen

### Layout

A string type becomes an array of its size, `i8` for `STRING` and `i16` for `WSTRING`. The program instance of the example is:

```llvm
%main = type { [81 x i8], [6 x i8], [11 x i16] }

@main_instance = global %main { [81 x i8] zeroinitializer, [6 x i8] c"hi\00\00\00\00", [11 x i16] zeroinitializer }
```

The constant initializer `'hi'` is written into the static data padded with zeros to the full size; the constructor copies it once more at start-up, like every constant initial value (see [Init](../participants/06-init.md)). Every literal collected by the resolver becomes a private constant sized to the literal, not to any target, with the terminator included:

```llvm
@utf08_literal_0 = private unnamed_addr constant [4 x i8] c"bob\00"
@utf08_literal_1 = private unnamed_addr constant [6 x i8] c"hello\00"
@utf16_literal_0 = private unnamed_addr constant [6 x i16] [i16 119, i16 111, i16 114, i16 108, i16 100, i16 0]
```

### Assignment

A string assignment uses `memcpy`. Its byte count is `min(target size - 1, source size)` multiplied by the storage-unit width. The copy leaves the target's final terminator slot unchanged. The three assignments in `main` copy 6, 5, and 12 bytes:

```llvm
  call void @llvm.memcpy.p0.p0.i32(ptr align 1 %text, ptr align 1 @utf08_literal_1, i32 6, i1 false)
  call void @llvm.memcpy.p0.p0.i32(ptr align 1 %short, ptr align 1 %text, i32 5, i1 false)
  call void @llvm.memcpy.p0.p0.i32(ptr align 2 %wide, ptr align 2 @utf16_literal_0, i32 12, i1 false)
```

`short := text` copies five bytes of a possibly 80-character value. If `text` holds `'hello'`, `short` receives `hello` and keeps its terminator in slot six; if it held a longer value, the value is cut without a diagnostic. A one-character literal assigned to a `CHAR` is the one exception to the copy rule: `c := 'x'` becomes `store i8 120`.

### Passing and returning

A string argument is passed as a pointer. In the callee, a by-value `VAR_INPUT` gets a local array of the parameter's size, which is zeroed and then filled from the pointer with a copy of the parameter's size minus one character. The result travels the other way, through the in-out pointer the aggregate-return lowerer added: the caller provides a zeroed temporary of the return type and copies it into the target afterwards:

```llvm
define void @greet(ptr %0, ptr %1) {
entry:
  %greet = alloca ptr, align 8
  store ptr %0, ptr %greet, align 8
  %who = alloca [81 x i8], align 4
  call void @llvm.memset.p0.i64(ptr align 1 %who, i8 0, i64 81, i1 false)
  call void @llvm.memcpy.p0.p0.i64(ptr align 1 %who, ptr align 1 %1, i64 80, i1 false)
  %deref = load ptr, ptr %greet, align 8
  call void @llvm.memcpy.p0.p0.i32(ptr align 1 %deref, ptr align 1 %who, i32 80, i1 false)
  ret void
}

define void @main(ptr %0) {
  ...
  %__greet0 = alloca [81 x i8], align 4
  call void @llvm.memset.p0.i64(ptr align 1 %__greet0, i8 0, i64 ptrtoint (ptr getelementptr ([81 x i8], ptr null, i32 1) to i64), i1 false)
  call void @greet(ptr %__greet0, ptr @utf08_literal_0)
  call void @llvm.memcpy.p0.p0.i32(ptr align 1 %text, ptr align 1 %__greet0, i32 80, i1 false)
  ...
}
```

This is also the C calling convention for the standard library: a C function that takes or returns a `STRING` takes `char*` parameters, with the result buffer first.

### Comparison

The hidden call the resolver attached is what codegen emits. Both operands are pointers, the literal directly from its constant, and the result is the `BOOL` the library function returns:

```llvm
  %call = call i8 @STRING_EQUAL(ptr %text, ptr @utf08_literal_1)
  store i8 %call, ptr %same, align 1
```

Everything else that works on strings, such as `LEN`, `CONCAT`, `LEFT`, or `FIND`, is a library function declared in Structured Text and reached through the [generic lowerer](../participants/08-generic.md); codegen has no string instructions beyond copy.


## Validation

The validator rejects assignments between `STRING` and `WSTRING` (E037), and literals longer than one character assigned to `CHAR` (E065 and E037). Ordinary string assignments do not require equal lengths: `short := 'far too long'` stores `far t` without a diagnostic. Interface method signatures do require matching string lengths and encodings (E118).


## At a glance

| Structured Text | Index | Annotation | LLVM |
|---|---|---|---|
| `STRING` | built-in, size 81, Utf8 | `STRING` | `[81 x i8]` |
| `STRING[n]` | pre-processed type `__<pou>_<var>`, size `n + 1`, Utf8 | that type's name | `[n+1 x i8]` |
| `WSTRING[n]` | pre-processed type, size `n + 1`, Utf16 | that type's name | `[n+1 x i16]` |
| `'abc'` | none until the resolver runs | `__STRING_3`, size 4, hinted to the target | `private constant [4 x i8] c"abc\00"` |
| `"abc"` | none until the resolver runs | `__WSTRING_3`, size 4, hinted to the target | `private constant [4 x i16]` |
| `a := b` | | `b` hinted to the type of `a` | `memcpy` of `min(size(a) - 1, size(b))` characters |
| `f(s)` | `f.s` of type `STRING` | `s` hinted to `STRING` | `ptr`, copied into a local of the parameter's size |
| `a = b` | | replaced by a call to `STRING_EQUAL` | `call i8 @STRING_EQUAL(ptr, ptr)` |
