# Codegen

Codegen is the fifth pipeline step. It turns the validated and fully lowered tree into LLVM IR, the input of the LLVM back end that produces machine code. In

```iecst
PROGRAM main
    VAR
        sintVar : SINT;
        dintVar : DINT;
    END_VAR

    sintVar := dintVar;
END_PROGRAM
```

the assignment is a node with two references, an annotation that says `dintVar` is a `DINT`, and a hint that says the value must become a `SINT`. A processor cannot run that. Codegen turns it into three instructions:

```llvm
%load_dintVar = load i32, ptr %dintVar        ; read dintVar
%1 = trunc i32 %load_dintVar to i8            ; cut it to 8 bits, the hint says SINT
store i8 %1, ptr %sintVar                     ; write sintVar
```

`dintVar` became an address inside the program's instance structure, its value was loaded, truncated to 8 bits because of the hint, and stored at the address of `sintVar`.

```mermaid
flowchart LR
    parse[Parse] --> index[Index] --> annotate[Annotate] --> validate[Validate] --> codegen[Codegen] --> link[Link]
    style codegen fill:#bfdbfe,stroke:#000,stroke-width:1px,stroke-dasharray:4 3,color:#0f172a
```

The stage receives the annotated project and produces one LLVM module per compilation unit, in parallel. Each module is built from the unit, the global index, the annotation map, and two things the resolver collected for this purpose: the set of names the unit depends on and the string literals it contains. The finished module is persisted as an object file, or as textual IR or bitcode with `--ir` or `--bc`, and handed to the [Linker](06-linker.md). With `--single-module`, and always with `-c`, all units are merged into one module first.

The chapter follows one project through the stage; each section takes the part that shows one mechanism:

```iecst
TYPE Speed : (Slow, Fast); END_TYPE

TYPE Point : STRUCT
    x : DINT;
    y : DINT;
END_STRUCT END_TYPE

VAR_GLOBAL
    counter : DINT := 7;
END_VAR

FUNCTION scale : DINT
    VAR_INPUT
        value : DINT;
        factor : INT;
    END_VAR
    VAR_IN_OUT
        total : DINT;
    END_VAR
    VAR_OUTPUT
        overflow : BOOL;
    END_VAR

    scale := value * factor;
    total := total + scale;
    overflow := scale > 100;
END_FUNCTION

FUNCTION_BLOCK Buffer
    VAR_INPUT
        limit : INT;
    END_VAR
    VAR
        count : DINT;
        values : ARRAY[0..3] OF DINT;
    END_VAR

    IF count < limit THEN
        values[count] := count;
        count := count + 1;
    END_IF
END_FUNCTION_BLOCK

PROGRAM main
    VAR
        bufferInstance : Buffer;
        i : DINT := 1;
        sintVar : SINT;
        text : STRING := 'hello';
        p : Point;
        speed : Speed;
        ptr : REF_TO DINT;
        flag : BOOL;
    END_VAR

    bufferInstance(limit := 5);
    i := scale(i, factor := 3, total := counter, overflow => flag);
    sintVar := i;
    text := 'world';
    p.x := i;
    ptr := REF(i);
    ptr^ := 2;
    CASE speed OF
        Slow: i := 0;
        Fast: i := 1;
    END_CASE
END_PROGRAM
```

`plc --ir` writes the complete module; the IR below is taken from it and trimmed.


## Building a module

A module is built in a fixed order: data types, global variables, a declaration for every function, the constants for member initializers and string literals, and last the function bodies. Each step can only use what the steps before it registered, because LLVM values refer to each other: a global needs its type, a signature needs its parameter types, a body needs the functions it calls.

The global index stores names, and LLVM works with values. The bridge is a second index that maps names to LLVM values; every step fills it and every later step reads it. Trimmed to its fields:

```rust
pub struct LlvmTypedIndex<'ink> {
    /// Lookups that fail here continue in the parent index
    parent_index: Option<&'ink LlvmTypedIndex<'ink>>,

    /// Type name to LLVM type, for data types and for POU instance structs
    type_associations: FxHashMap<String, AnyTypeEnum<'ink>>,
    pou_type_associations: FxHashMap<String, AnyTypeEnum<'ink>>,

    /// Variable name to global variable
    global_values: FxHashMap<String, GlobalValue<'ink>>,

    /// Type or variable name to its constant initial value
    initial_value_associations: FxHashMap<String, BasicValueEnum<'ink>>,

    /// Qualified variable name to the address it lives at inside the current function
    loaded_variable_associations: FxHashMap<String, PointerValue<'ink>>,

    /// POU name to LLVM function
    implementations: FxHashMap<String, FunctionValue<'ink>>,

    /// String literal to the global constant that holds it
    utf08_literals: FxHashMap<String, GlobalValue<'ink>>,
    utf16_literals: FxHashMap<String, GlobalValue<'ink>>,

    // ... other omitted fields
}
```

The parent link makes the index a stack of scopes: the module has one index with types, globals, and functions, and each function body gets a child index with the addresses of its own variables, so that `count` inside `Buffer` resolves to a member of the instance and not to anything global. Keys are lowercased, like in the global index.


## Data types

Every type the unit depends on becomes an LLVM type:

| Structured Text | LLVM |
|---|---|
| `BOOL`, `SINT`, `USINT`, `BYTE` | `i8` (comparisons produce `i1` and are widened) |
| `INT`, `UINT`, `WORD` | `i16` |
| `DINT`, `UDINT`, `DWORD`, `TIME`, `DATE` | `i32` |
| `LINT`, `ULINT`, `LWORD`, `LTIME`, `LDATE` | `i64` |
| `REAL`, `LREAL` | `float`, `double` |
| `STRING[n]`, `WSTRING[n]` | `[n+1 x i8]`, `[n+1 x i16]`, one slot for the terminator |
| `ARRAY[a..b, c..d] OF T` | `[len x T]`, all dimensions flattened into one length |
| `STRUCT`, `FUNCTION_BLOCK`, `PROGRAM`, `CLASS` | a named struct with one field per member, in declaration order |
| `POINTER TO`, `REF_TO`, `REFERENCE TO` | `ptr` |
| enum, subrange, alias | the LLVM type of the underlying type |

For the example, `Point`, `Buffer`, and `main` become named structs; `scale` is a function and gets none, its variables live on the stack; `Speed` is an `i32`:

```llvm
%Point  = type { i32, i32 }
%Buffer = type { ptr, i16, i32, [4 x i32] }
%main   = type { %Buffer, i32, i8, [81 x i8], %Point, i32, ptr, i8 }
```

The leading `ptr` of `Buffer` is the `__vtable` member that the polymorphism lowerer adds to every function block. `main` embeds its `Buffer` by value, and `text` is `[81 x i8]`: 80 characters plus the terminator.

Struct types are created in two passes, because a struct can contain a pointer to itself or to a struct declared later: the first pass registers an empty named struct for every struct type, the second fills in the members. Temporary variables and the return variable of a function are not members; they live on the stack.

Once the types exist, codegen computes a constant initial value for each: a struct from the evaluated initializers of its members, an array or string from its literal. A type's initial value can depend on another type's, so the computation runs as a queue that stops when a full pass makes no progress.


## Global variables

The dependency set decides which globals a module declares. Every global variable and every program instance the unit refers to becomes an LLVM global. A variable declared in this unit gets its initial value: the evaluated initializer, otherwise the initial value of its type, otherwise zero:

```llvm
@counter       = global i32 7
@main_instance = global %main { %Buffer zeroinitializer, i32 1, i8 0, [81 x i8] c"hello\00...", %Point zeroinitializer, i32 0, ptr null, i8 0 }
@Speed.Slow    = unnamed_addr constant i32 0
@Speed.Fast    = unnamed_addr constant i32 1
```

A program instance is named after the program with an `_instance` suffix and initialized with the constant of its type; `i := 1` and `text := 'hello'` are visible in it. Enum variants are constants named by their qualified name. A variable declared in another file appears as `@counter = external global i32`, without a value; the linker connects the two.


## Functions

Structured Text separates stateless from stateful POUs, and codegen makes the split visible in every signature:

- **Stateless:** a **function**. A call knows nothing of the call before it, so its variables live on the stack; the LLVM function takes the inputs as arguments and returns the result.
- **Stateful:** a **function block** or **program**. Inputs, outputs, and locals are members of its instance struct, which survives between calls; the LLVM function takes one argument, a pointer to that instance, and returns nothing. **Methods** and **actions** are ordinary LLVM functions that take the instance pointer first; a method adds its own parameters after it.

```llvm
define i32  @scale(i32 %0, i16 %1, ptr %2, ptr %3)   ; value, factor, total (in-out), overflow (output)
define void @Buffer(ptr %0)                           ; the instance
define void @main(ptr %0)                             ; the instance
```

In both kinds, `VAR_IN_OUT` and `VAR_OUTPUT` parameters are pointers, and aggregate inputs (structs, arrays, strings) are passed as pointers and copied into a local inside the callee.

Functions are created in two passes, like structs: first a declaration for every POU the unit depends on, including POUs from other units, then the bodies of the POUs declared in this unit. If `scale` were in a second file, the module of `main` would contain `declare i32 @scale(i32, i16, ptr, ptr)` with no body.

A body starts by making every variable addressable, so the statements can treat both kinds alike. A function copies each argument into a stack slot and starts its return variable at zero; a stateful POU computes one pointer per member into the instance:

```llvm
define i32 @scale(i32 %0, i16 %1, ptr %2, ptr %3) {
entry:
  %scale = alloca i32                 ; stack slot for the return variable
  %value = alloca i32                 ; stack slot for value
  store i32 %0, ptr %value            ; copy the argument into it
  %factor = alloca i16
  store i16 %1, ptr %factor
  %total = alloca ptr                 ; in-out: the slot holds the caller's address
  store ptr %2, ptr %total
  %overflow = alloca ptr              ; output: same
  store ptr %3, ptr %overflow
  store i32 0, ptr %scale             ; the return variable starts at zero
  ...
  %scale_ret = load i32, ptr %scale   ; read the return variable
  ret i32 %scale_ret                  ; and return it
}

define void @Buffer(ptr %0) {
entry:
  %__vtable = getelementptr inbounds nuw %Buffer, ptr %0, i32 0, i32 0   ; address of member 0 in the instance
  %limit    = getelementptr inbounds nuw %Buffer, ptr %0, i32 0, i32 1   ; member 1
  %count    = getelementptr inbounds nuw %Buffer, ptr %0, i32 0, i32 2   ; member 2
  %values   = getelementptr inbounds nuw %Buffer, ptr %0, i32 0, i32 3   ; member 3
  ...
  ret void                                                               ; nothing to return
}
```

Every address is registered in the function's child index under its qualified name (`Buffer.count`). A function ends by loading the variable that carries its own name and returning it.

## Expressions

Two rules drive almost everything the expression generator does.

**A reference is an address until a value is needed.** `p.x` is one `getelementptr` per member step, `values[count]` a pointer computation into the array. On the left of an assignment the address is the target of a store; as an operand, its value is loaded. For `p.x := i`:

```llvm
%x      = getelementptr inbounds nuw %Point, ptr %p, i32 0, i32 0   ; address of p.x: member 0 of p
%load_i = load i32, ptr %i                                          ; read i
store i32 %load_i, ptr %x                                           ; write it to p.x
```

An array index becomes an offset first: the index minus the lower bound, times the stride of the dimension. For `values[count] := count`, with lower bound `0` and one dimension, both corrections are trivial and still emitted:

```llvm
%tmpVar2 = mul i32 1, %load_count                                              ; count times the stride (1)
%tmpVar3 = add i32 %tmpVar2, 0                                                 ; minus the lower bound (0)
%tmpVar4 = getelementptr inbounds [4 x i32], ptr %values, i32 0, i32 %tmpVar3  ; address of values[offset]
store i32 %load_count5, ptr %tmpVar4                                           ; write count there
```

Pointer variables that the resolver marked as auto-dereferencing (`VAR_IN_OUT`, `REFERENCE TO`) get one extra load. Inside `scale`, reading `total` loads the address from the stack slot first, then the value behind it. An explicit pointer works the same way without the marker: `ptr := REF(i)` stores the address of `i`, and `ptr^ := 2` loads that address and stores through it:

```llvm
%deref1     = load ptr, ptr %total     ; the address the caller passed
%load_total = load i32, ptr %deref1    ; the value behind it

store ptr %i, ptr %ptr                 ; ptr := REF(i)
%deref = load ptr, ptr %ptr            ; ptr^ := 2
store i32 2, ptr %deref
```

**The type hint decides the conversion.** Whenever a value flows into a place of a different type, codegen compares the annotated type with the hint and emits the cast: extension and truncation between integers, conversion between integers and floats, widening or narrowing between floats. In `scale := value * factor`, `factor` is an `INT` hinted to `DINT`; `sintVar := i` is the introduction's truncation:

```llvm
%4      = sext i16 %load_factor to i32   ; widen factor from INT to DINT
%tmpVar = mul i32 %load_value, %4        ; value * factor

%2 = trunc i32 %load_i1 to i8            ; cut i from DINT to SINT
store i8 %2, ptr %sintVar                ; write sintVar
```

Literals are created directly in the hinted type, so `factor := 3` for an `INT` parameter produces `i16 3` and no cast.

Binary and unary expressions become the matching integer or float instruction. Comparisons produce an `i1`, which is widened to `i8` because that is the width of `BOOL`; where a branch needs a condition, the `i8` is compared against zero to get an `i1` back. For `overflow := scale > 100`:

```llvm
%tmpVar5 = icmp sgt i32 %load_scale4, 100   ; scale > 100, as i1
%5       = zext i1 %tmpVar5 to i8           ; widen to the width of BOOL
store i8 %5, ptr %deref3                    ; write overflow through the output pointer
```

An expression cannot fail here: every name was resolved and every type checked in the previous stages. A case codegen still cannot handle stops the run with an internal codegen error.


## Statements

**Assignments.** A single value (integer, real, pointer) is a store. An aggregate (struct, array, string) is a `memcpy` of the target's size; for strings the copy is capped at the target's length, so a longer value is cut, never overflowed. A string literal is a private global constant, created once per module. For `text := 'world'`:

```llvm
@utf08_literal_1 = private unnamed_addr constant [6 x i8] c"world\00"   ; the literal, once per module

call void @llvm.memcpy.p0.p0.i32(ptr align 1 %text, ptr align 1 @utf08_literal_1, i32 6, i1 false)   ; copy 6 bytes into text
```

**Calls to functions.** The arguments are evaluated into the declared parameter order, named and positional alike, unpassed parameters are filled with their default values, and a `call` is emitted. By-value inputs are passed as values, `VAR_IN_OUT` and `VAR_OUTPUT` parameters as addresses, so the callee writes them directly and no copy back is needed. For `i := scale(i, factor := 3, total := counter, overflow => flag)`:

```llvm
%call = call i32 @scale(i32 %load_i, i16 3, ptr @counter, ptr %flag)   ; value, factor, address of total, address of overflow
store i32 %call, ptr %i                                                ; write the result to i
```

**Calls to function blocks and programs.** Their inputs are members of the instance, so codegen stores every passed argument into the member, calls the POU with the instance pointer, and copies `=>` outputs back out afterwards. For `bufferInstance(limit := 5)`:

```llvm
%1 = getelementptr inbounds %Buffer, ptr %bufferInstance, i32 0, i32 1   ; address of bufferInstance.limit
store i16 5, ptr %1                                                      ; limit := 5
call void @Buffer(ptr %bufferInstance)                                   ; run the body on the instance
```

Built-in functions such as `ADR`, `REF`, `SIZEOF`, or `MUX` have their own code generators; `REF(i)` above produced no call, only the address of `i`.

**`IF`** becomes one block per branch and a `continue` block after them; the condition is narrowed to `i1` and branched on. For the `IF` in `Buffer`:

```llvm
  %3 = icmp ne i8 %2, 0                                 ; narrow the BOOL to i1
  br i1 %3, label %condition_body, label %continue      ; jump to the branch or past it

condition_body:
  ...
  br label %continue                                    ; join after the branch

continue:
  ret void
```

**`CASE`** becomes a `switch` for literal labels and a chain of comparisons for range labels. The enum variants are constants, so both labels end up in the switch:

```llvm
  switch i32 %load_speed, label %else [   ; no label matched: ELSE (empty here)
    i32 0, label %case                    ; Slow
    i32 1, label %case3                   ; Fast
  ]
```

**Loops.** `FOR`, `WHILE`, and `REPEAT` all arrive as `WHILE TRUE` loops, rewritten by the [loop desugaring participant](../participants/01-loop-desugar.md); the exit conditions are ordinary `IF ... EXIT` statements in the body. Codegen emits only the skeleton, a `while_body` block that branches back to itself and a `continue` block after it, and `EXIT` and `CONTINUE` become jumps to those two blocks.


## Initialization

Constant initial values go into the global instance directly, as shown under Global variables. Everything that cannot be a compile-time constant, such as a pointer to another variable or a nested function block, is handled by initializer functions. The [init participant](../participants/06-init.md) created them before codegen as ordinary POUs with a `__ctor` suffix, one per type and one per unit, and codegen treats them like any other function. For the example (the unit constructor's real name also carries a hash of the file path):

```llvm
@llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 65535, ptr @__unit_cg_st__ctor, ptr null }]   ; run before the program starts

define void @main__ctor(ptr %0) {
entry:
  ...
  call void @Buffer__ctor(ptr %bufferInstance)   ; construct the embedded function block
  store i32 1, ptr %i                            ; i := 1
  call void @llvm.memcpy.p0.p0.i32(ptr align 1 %text, ptr align 1 @utf08_literal_0, i32 6, i1 false)   ; text := 'hello'
  call void @Point__ctor(ptr %p)                 ; construct the struct member
  ...
  ret void
}

define void @__unit_cg_st__ctor() {
entry:
  store i32 7, ptr @counter                                          ; counter := 7
  call void @__vtable_Buffer__ctor(ptr @__vtable_Buffer_instance)   ; fill the method table
  call void @main__ctor(ptr @main_instance)                         ; construct the program instance
  ret void
}
```

`main__ctor` receives the instance, calls the constructor of the embedded `Buffer`, and stores the initial values of `i` and `text`. The unit constructor calls it for the global instance, and the `llvm.global_ctors` entry makes the unit constructor run before the program starts.

> **Note**
>
> Generated code is not written for readability. A member gets its pointer computed at the start of every body whether it is used or not, an aggregate is initialized twice (in the global's constant and by its constructor), the array offset above is multiplied by one and has zero added, and function blocks carry a `__vtable` member. LLVM's optimizer removes most of this at `-O1` and above.


## Output

For an object file, LLVM runs its optimization passes at the chosen `-O` level and emits machine code for the target triple; position-independent code is selected from the output format and the `--fpic` and `--fno-pic` flags. IR and bitcode are written as they are, without optimization. With `-g`, a debug builder runs alongside the generators and attaches DWARF debug information: one entry per POU, one per member and local variable, and a source location per statement.


## Where it lives

| What | Where |
|---|---|
| Codegen | `src/codegen.rs`, `src/codegen/` |


## What's next

Each unit is now an object file with machine code, but the objects refer to each other: `main` calls a `scale` it may not define, and every unit's constructor expects to be called before the program starts. Resolving these references, adding the standard library, and producing a program or a shared object is the job of the [Linker](06-linker.md).
