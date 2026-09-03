# Strings

Structured Text has two string types. `STRING` holds single-byte characters and `WSTRING` holds two-byte characters. Both have a fixed capacity: `STRING[5]` holds up to five characters, and `STRING` without a length holds up to 80. Internally a string is a character array with one extra slot for a terminating zero, and every operation on it is a copy that stops at the smaller of the two capacities. Nothing is ever allocated at run time, and a value that is too long is cut, never reported.

This chapter follows one project through the compiler:

```iecst
VAR_GLOBAL CONSTANT
    SIZE : DINT := 5;
END_VAR

FUNCTION greet : STRING
    VAR_INPUT
        who : STRING;
    END_VAR

    greet := who;
END_FUNCTION

PROGRAM main
    VAR
        text : STRING;
        short : STRING[SIZE] := 'hi';
        wide : WSTRING[10];
    END_VAR

    text := 'hello';
    short := text;
    wide := "world";
    text := greet('bob');
END_PROGRAM
```


## Declaration

The parser produces a string type node with three facts: whether it is wide, the length expression if one was written, and no name. `text : STRING` is not such a node; it is a plain reference to the built-in type `STRING`. Only `STRING[SIZE]` and `WSTRING[10]` are inline type definitions, and pre-processing at the start of the index stage moves them out into named types, `__main_short` and `__main_wide`, scoped to `main`, and replaces the declaration with a reference to that name (see [Index](../pipeline/02-index.md), Pre-processing). A string literal is a literal node with the text and a wide flag: `'hello'` is narrow, `"world"` is wide.


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

The size is always the declared length plus one. For the built-in `STRING` and `WSTRING` it is the literal 81. For `WSTRING[10]` the indexer computes 11 directly. For `STRING[SIZE]` the length is an expression, so the indexer builds the expression `SIZE + 1`, puts it into the constant store, and records the id; constant evaluation at the end of the stage folds it to 6 (see [Index](../pipeline/02-index.md), Constant evaluation). After indexing, the project has four string types:

```
STRING          { size: 81,                encoding: Utf8,  declared_with_length: false }   built-in
WSTRING         { size: 81,                encoding: Utf16, declared_with_length: false }   built-in
__main_short    { size: SIZE + 1 -> 6,     encoding: Utf8,  declared_with_length: true }    from STRING[SIZE]
__main_wide     { size: 11,                encoding: Utf16, declared_with_length: true }    from WSTRING[10]
```

The variable entries know nothing about strings. `main.text` stores the type name `STRING`, `main.short` stores `__main_short` and the id of its initializer `'hi'` in the constant store, and `greet.who` and the return member `greet.greet` store `STRING`. Whether a variable is a string, and how big it is, is always one more lookup in the type index.


## Annotations

Every string literal gets a type of its own. The resolver names it after the encoding and the length, `__STRING_5` for `'hello'`, registers it in its own small index with size 6, and imports it into the global index when the unit is done (see [Resolver](../pipeline/03-resolver.md), Literals and generated types). The hint on the literal is the type of the target, which is how codegen later knows how many characters to copy. For the body of `main`:

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

Two more things happen here. Every literal that stands in a body is collected into the unit's set of string literals, once per distinct text, so that codegen can emit each as one global constant; `'hi'` in the initializer is not collected until the init participant moves it into a constructor body. And a comparison such as `text = 'hello'` is not annotated as a comparison at all: the resolver replaces it with a call to `STRING_EQUAL(text, 'hello')`, or `WSTRING_EQUAL` for wide strings, and `<>`, `<=`, and `>=` become `NOT`, `OR` combinations of the `_EQUAL`, `_LESS`, and `_GREATER` calls. The compare functions are not part of the compiler; the standard library declares them, and a project that compares strings without it gets `Missing compare function` (E073).


## Lowering

No participant rewrites strings themselves, but two rewrite the places they appear in. The [aggregate-return lowerer](../participants/09-aggregate-return.md) turns `greet` into a void function with a `VAR_IN_OUT greet : STRING` parameter and gives the call site a temporary, so by codegen the last statement of `main` reads `greet(__greet0, 'bob'); text := __greet0;`. The [init participant](../participants/06-init.md) moves the initializer `'hi'` out of the declaration into the constructor of `main` as `self.short := 'hi'`. Codegen sees only assignments, calls, and pointer parameters.


## Codegen

**Layout.** A string type becomes an array of its size, `i8` for `STRING` and `i16` for `WSTRING`. The program instance of the example is:

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

**Assignment.** A string is an aggregate, so an assignment is a `memcpy`, never a `store`. The length is the smaller of the target's size minus one and the source's size, in characters, times the character width; the target's own terminator slot is never written, so the result is always terminated. The three assignments of `main` copy 6, 5, and 12 bytes:

```llvm
  call void @llvm.memcpy.p0.p0.i32(ptr align 1 %text, ptr align 1 @utf08_literal_1, i32 6, i1 false)
  call void @llvm.memcpy.p0.p0.i32(ptr align 1 %short, ptr align 1 %text, i32 5, i1 false)
  call void @llvm.memcpy.p0.p0.i32(ptr align 2 %wide, ptr align 2 @utf16_literal_0, i32 12, i1 false)
```

`short := text` copies five bytes of a possibly 80-character value. If `text` holds `'hello'`, `short` receives `hello` and keeps its terminator in slot six; if it held a longer value, the value is cut without a diagnostic. A one-character literal assigned to a `CHAR` is the one exception to the copy rule: `c := 'x'` becomes `store i8 120`.

**Passing and returning.** A string argument is passed as a pointer. In the callee, a by-value `VAR_INPUT` gets a local array of the parameter's size, which is zeroed and then filled with a copy of the parameter's size minus one character from the pointer. The result travels the other way through the in-out pointer the aggregate-return lowerer added; the caller provides a zeroed temporary of the return type and copies it into the target afterwards:

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

> **Developer Note**
>
> The copy into the callee's local uses the parameter's size, not the argument's. For `greet('bob')` the callee copies 80 bytes out of a 4-byte constant, and for a `STRING[5]` variable passed to a `STRING` parameter it copies 80 bytes out of 6. The bytes after the terminator are ignored by every string operation, so the value is right, but the read runs past the end of the argument. Logged in `bugs.md`.

**Comparison.** The hidden call the resolver attached is what codegen emits. Both operands are pointers, the literal directly from its constant, and the result is the `BOOL` the library function returns:

```llvm
  %call = call i8 @STRING_EQUAL(ptr %text, ptr @utf08_literal_1)
  store i8 %call, ptr %same, align 1
```

Everything else that works on strings, such as `LEN`, `CONCAT`, `LEFT`, or `FIND`, is a library function declared in Structured Text and reached through the [generic lowerer](../participants/08-generic.md); codegen has no string instructions beyond copy.


## Validation

The validator checks encodings and single characters, not lengths. Assigning a `STRING` to a `WSTRING` or the other way round is an invalid assignment (E037), as is a literal longer than one character assigned to a `CHAR` (E065 together with E037). Between two `STRING` types of different lengths there is no check at all, in either direction: the copy is cut at codegen, so `short := 'far too long'` compiles silently and stores `far t`. Where a method implements an interface method, the string parameters must match in both length and encoding (E118).


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
