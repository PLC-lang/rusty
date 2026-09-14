# POUs

A program organization unit (POU) contains code and declarations. Programs, function blocks, and classes keep state in an instance. Functions keep their local data on the stack. Methods and actions use their owner's instance. These storage rules determine the generated layouts and call signatures.

The example uses a function, a function block, a class, a method, and an action. Follow how their variables become instance fields or stack slots:

```iecst
FUNCTION scale: DINT
    VAR_INPUT
        value: DINT;
        factor: INT := 2;
    END_VAR
    VAR_IN_OUT
        count: DINT;
    END_VAR
    VAR_OUTPUT
        overflow: BOOL;
    END_VAR
    VAR
        tmp: DINT;
    END_VAR

    count := count + 1;
    tmp := value * factor;
    overflow := tmp < value;
    scale := tmp;
END_FUNCTION

FUNCTION_BLOCK Counter
    VAR_INPUT
        step: DINT := 1;
    END_VAR
    VAR_OUTPUT
        total: DINT;
    END_VAR
    VAR
        calls: DINT;
    END_VAR
    VAR_TEMP
        scratch: DINT;
    END_VAR

    METHOD reset
        total := 0;
        calls := 0;
    END_METHOD

    scratch := step;
    total := total + scratch;
    calls := calls + 1;
END_FUNCTION_BLOCK

ACTIONS Counter
    ACTION double
        step := step * 2;
    END_ACTION
END_ACTIONS

CLASS Limits
    VAR
        max: DINT := 100;
    END_VAR

    METHOD exceeded: BOOL
        VAR_INPUT
            candidate: DINT;
        END_VAR

        exceeded := candidate > max;
    END_METHOD
END_CLASS

PROGRAM main
    VAR
        counter: Counter;
        limits: Limits;
        hits: DINT;
        flag: BOOL;
    END_VAR

    counter(step := 5, total => hits);
    counter.double();
    hits := scale(counter.total, count := hits, overflow => flag);
    IF limits.exceeded(hits) THEN
        counter.reset();
    END_IF
END_PROGRAM
```


## Declaration

The parser splits every POU into two nodes: a declaration with the name, the kind, the return type, and the variable blocks, and an implementation with the statements (see [Lexer and Parser](../pipeline/01-lexer-parser.md)).

A method has its own POU declaration, such as `Counter.reset`, with a reference to its parent. The parser stores parent and method side by side. An action has only an implementation, such as `Counter.double`, which uses `Counter` as its type. A class also has an implementation; its body is empty in a valid program.


## Index

The index records POU declarations, implementations, and variable layouts. Actions reuse the parent's layout. The POU entry identifies the kind:

```rust,noplayground
pub enum PouIndexEntry {
    /// One static instance, held in instance_variable, of the struct instance_struct_name
    Program { name, instance_struct_name, instance_variable, .. },

    /// Many instances; the struct is instance_struct_name; super_class and interfaces for polymorphism
    FunctionBlock { name, instance_struct_name, super_class, interfaces, .. },

    /// Like a function block, but not callable as a whole
    Class { name, instance_struct_name, super_class, interfaces, .. },

    /// No instance; the return type and whether the parameter list is variadic or generic
    Function { name, return_type, generics, is_variadic, .. },

    /// Belongs to parent_name; its own parameters live in the struct instance_struct_name
    Method { name, parent_name, return_type, instance_struct_name, .. },

    /// Belongs to parent_name and shares its instance struct
    Action { name, parent_name, instance_struct_name, .. },
}
```

The implementation entry connects a callable name with the body and its kind:

```rust,noplayground
pub struct ImplementationIndexEntry {
    /// The name a call uses, "Counter.reset" for a method
    call_name: String,

    /// The struct type the body runs on; the parent for an action, the method's own struct for a method
    type_name: String,

    /// The class or function block a method belongs to
    associated_class: Option<String>,

    /// Program, Function, FunctionBlock, Class, Method, Action, or a generated constructor
    implementation_type: ImplementationType,

    // ... other omitted fields
}
```

The type is the instance struct, registered under the name of the POU in the type index. It has one member per declared variable, in declaration order, whatever block the variable is in, and each member records its block as an argument type.

Two rules change the type a member stores. A `VAR_IN_OUT`, and a `VAR_OUTPUT` of a function or method, is passed by reference, so the member gets an auto-dereferencing pointer type, `__auto_pointer_to_DINT`, that the indexer registers on demand. And a function with a return type gets one extra member named like the function, so `scale.scale` is a `DINT` member with argument type `Return`. For the example:

```
scale            Function, return_type "DINT"
    scale.value       DINT                     ByVal(Input)     0
    scale.factor      INT                      ByVal(Input)     1   initial value: 2
    scale.count       __auto_pointer_to_DINT   ByRef(InOut)     2
    scale.overflow    __auto_pointer_to_BOOL   ByRef(Output)    3
    scale.tmp         DINT                     ByVal(Local)     4
    scale.scale       DINT                     ByVal(Return)    5

Counter          FunctionBlock, instance struct "Counter"
    Counter.step      DINT                     ByVal(Input)     0   initial value: 1
    Counter.total     DINT                     ByVal(Output)    1
    Counter.calls     DINT                     ByVal(Local)     2
    Counter.scratch   DINT                     ByVal(Temp)      3

Counter.reset    Method, parent "Counter", instance struct "Counter.reset" (no members)
Counter.double   Action, parent "Counter", instance struct "Counter"

Limits           Class, instance struct "Limits"
    Limits.max        DINT                     ByVal(Local)     0   initial value: 100

Limits.exceeded  Method, parent "Limits", return_type "BOOL"
    Limits.exceeded.candidate   DINT           ByVal(Input)     0
    Limits.exceeded.exceeded    BOOL           ByVal(Return)    1

main             Program, instance variable "main_instance" of type "main"
    main.counter      Counter                  ByVal(Local)     0
    main.limits       Limits                   ByVal(Local)     1
    main.hits         DINT                     ByVal(Local)     2
    main.flag         BOOL                     ByVal(Local)     3
```

The struct of a function exists only so that `scale.tmp` can be looked up like any other member; no instance is ever created. A `VAR_OUTPUT` of a function block is a by-value member, because the instance keeps it; in a function it is a by-reference parameter, because there is no instance to keep it in.

A program also records its single instance as a global variable entry named `main_instance`. A function block or class registers a default-instance entry `__Counter__init` as well, which no later stage reads (see the [Initializers](07-initializers.md) chapter).


## Annotations

Function and method names receive `Function` annotations with return types. Program and action names receive `Program` annotations. Function block and class instances are variables of the corresponding POU type. Member references use the declaring POU in their qualified name, including parameters and return variables:

```
    counter(step := 5, total => hits);
    ^^^^^^^                        { kind: Variable, qualified_name: "main.counter", resulting_type: "Counter" }
            ^^^^^^^^^              { kind: None,                                     hint: Argument { resulting_type: "DINT", position: 0, pou: "Counter" } }
            ^^^^                   { kind: Variable, qualified_name: "Counter.step",  resulting_type: "DINT" }
                       ^^^^^^^^^^^^^  { kind: None,                                  hint: Argument { resulting_type: "DINT", position: 1, pou: "Counter" } }

    counter.double();
            ^^^^^^                 { kind: Program,  qualified_name: "Counter.double" }

    hits := scale(counter.total, count := hits, overflow => flag);
            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^  { kind: Value, resulting_type: "DINT", hint: "DINT" }
            ^^^^^                  { kind: Function, qualified_name: "scale", return_type: "DINT" }
                  ^^^^^^^^^^^^^    { kind: Variable, qualified_name: "Counter.total", resulting_type: "DINT", hint: Argument { resulting_type: "DINT", position: 0, pou: "scale" } }
                                 ^^^^^^^^^^^^^  { kind: None, hint: Argument { resulting_type: "__auto_pointer_to_DINT", position: 2, pou: "scale" } }
                                 ^^^^^          { kind: Variable, qualified_name: "scale.count", argument_type: ByRef(InOut), auto_deref: Default }

    IF limits.exceeded(hits) THEN
       ^^^^^^^^^^^^^^^^^^^^^       { kind: Value,    resulting_type: "BOOL" }
              ^^^^^^^^             { kind: Function, qualified_name: "Limits.exceeded", return_type: "BOOL" }
```

An argument hint carries the position of the parameter in the declaration of the callee and the POU that declares it. Codegen uses the position to find the struct member or the argument slot, and the POU to walk an `EXTENDS` chain when the parameter belongs to a base.

Inside the callee the return member is an ordinary variable: `scale := tmp` in `scale` annotates the left side as `scale.scale` with argument type `Return`, and `exceeded := candidate > max` as `Limits.exceeded.exceeded`. The `VAR_IN_OUT` member `count` carries the auto-dereference marker, which is what makes `count := count + 1` in the body read and write through the pointer without a `^`.


## Lowering

Several participants change POU declarations. The [polymorphism lowerer](../participants/03-polymorphism.md) adds method tables and a `__vtable` member to root classes and function blocks. Derived types access that member through the base inserted by the [inheritance lowerer](../participants/10-inheritance.md). The [init participant](../participants/06-init.md) creates constructors. The [aggregate-return lowerer](../participants/09-aggregate-return.md) adds result parameters for strings, arrays, and structs.

After lowering, `Counter` has four instance fields instead of three. `scratch` remains a stack variable. The unit also contains ten type and POU constructors of kind `Init`, plus one unit constructor.


## Codegen

### Layout

Every stateful POU becomes a named struct with one field per member that is not a `VAR_TEMP`, in member order; a function has no struct. A program instance is a global initialized with the constant initial values of its members, nested instances included:

```llvm
%main = type { %Counter, %Limits, i32, i8 }
%Counter = type { ptr, i32, i32, i32 }
%Limits = type { ptr, i32 }

@main_instance = global %main { %Counter { ptr null, i32 1, i32 0, i32 0 }, %Limits { ptr null, i32 100 }, i32 0, i8 0 }
```

The first `ptr` of `Counter` and `Limits` is the `__vtable` member; the constructors store the table addresses at start-up (see [Init](../participants/06-init.md), and the Initialization section of [Codegen](../pipeline/05-codegen.md)).

### Signatures

Every POU becomes exactly one LLVM function, named after its call name with `.` replaced by `__`. The kind decides the arguments:

| POU | Signature | Arguments |
|---|---|---|
| Program, function block, action | `void @Counter(ptr)` | the instance |
| Class | `void @Limits(ptr)` | the instance; the body is empty |
| Method | `i8 @Limits__exceeded(ptr, i32)` | the instance, then the method's own parameters like a function |
| Function | `i32 @scale(i32, i16, ptr, ptr)` | one argument per `VAR_INPUT` by value, one pointer per `VAR_IN_OUT` and `VAR_OUTPUT`; the return type is the result |

A method receives its own parameters like a function and accesses parent members through the first argument, the instance pointer.

### Bodies

A body starts by making every member addressable, and the two kinds differ here. A stateful body computes one pointer per struct member into the instance and allocates a stack slot only for `VAR_TEMP`:

```llvm
define void @Counter(ptr %0) {
entry:
  %this = alloca ptr, align 8
  store ptr %0, ptr %this, align 8
  %__vtable = getelementptr inbounds nuw %Counter, ptr %0, i32 0, i32 0
  %step = getelementptr inbounds nuw %Counter, ptr %0, i32 0, i32 1
  %total = getelementptr inbounds nuw %Counter, ptr %0, i32 0, i32 2
  %calls = getelementptr inbounds nuw %Counter, ptr %0, i32 0, i32 3
  %scratch = alloca i32, align 4
  store i32 0, ptr %scratch, align 4
  ...
  ret void
}
```

The `%this` slot provides the instance pointer for `THIS^` in function blocks and their methods and actions. Classes do not get this slot. A method or action of `Counter` computes the same member pointers, so `reset` writes directly to the instance. Parent `VAR_TEMP` variables get fresh stack slots in each method.

A function allocates stack slots for its parameters, locals, and return variable. It stores incoming arguments in those slots and zeroes the return variable:

```llvm
define i32 @scale(i32 %0, i16 %1, ptr %2, ptr %3) {
entry:
  %scale = alloca i32, align 4
  %value = alloca i32, align 4
  store i32 %0, ptr %value, align 4
  %factor = alloca i16, align 4
  store i16 %1, ptr %factor, align 2
  %count = alloca ptr, align 8
  store ptr %2, ptr %count, align 8
  %overflow = alloca ptr, align 8
  store ptr %3, ptr %overflow, align 8
  %tmp = alloca i32, align 4
  store i32 0, ptr %tmp, align 4
  store i32 0, ptr %scale, align 4
  ...
  %scale_ret = load i32, ptr %scale, align 4
  ret i32 %scale_ret
}
```

`count` and `overflow` hold pointers, and every access loads the pointer first and then the value behind it; that is the auto-dereference the resolver marked. Writing `overflow` writes the caller's `flag` directly.

### Calls

The kind of the callee decides how a call is built. A call to a stateful POU stores each passed argument into the member of the instance, calls the function with the instance pointer, and, after the call, copies every `=>` output out of the instance into its target. Unpassed inputs keep their last value:

```llvm
  %1 = getelementptr inbounds %Counter, ptr %counter, i32 0, i32 1
  store i32 5, ptr %1, align 4
  call void @Counter(ptr %counter)
  %2 = getelementptr inbounds %Counter, ptr %counter, i32 0, i32 2
  %3 = load i32, ptr %2, align 4
  store i32 %3, ptr %hits, align 4
  call void @Counter__double(ptr %counter)
```

A program call passes its global instance; an action call passes its owner's instance. A function call places arguments in parameter order, uses defaults for omitted inputs, and passes addresses for by-reference parameters. The callee writes through those addresses:

```llvm
  %load_total = load i32, ptr %total, align 4
  %call = call i32 @scale(i32 %load_total, i16 2, ptr %hits, ptr %flag)
  store i32 %call, ptr %hits, align 4
```

`factor` was not passed, so its default `2` appears as the literal `i16 2`. A method call is a function call with the instance in front: `call i8 @Limits__exceeded(ptr %limits, i32 %load_hits)`. A call of an unqualified method or action inside a body takes the instance from the first argument of the running function.

Aggregate `VAR_INPUT` parameters (strings, arrays, structs) are passed as pointers even though they are by-value, and the callee copies the value into a local of its own; the [Strings](03-strings.md) chapter shows the copy. `VAR_INPUT {ref}` skips the copy and makes the parameter a pointer that the body reads through.


## Validation

The rules that concern the POU as a whole live in the POU validator. A program, function block, or class must not declare a return type (E026). A class must not declare `VAR_INPUT`, `VAR_OUTPUT`, or `VAR_IN_OUT` (E019) and must not have a body (E017). An `ACTIONS` block must name its container (E022). `EXTENDS` and `IMPLEMENTS` are allowed on classes and function blocks only (E110), the base and the interfaces must exist (E048), and an implementing method must match the declared signature (E112, E118).

Calls are checked in the statement validator: every `VAR_IN_OUT` of the callee must receive an argument (E030), and a reference to an action without the call parentheses is reported (E095). A bare reference to a program is accepted. Duplicate POU names are found by the global validation of the index (E004).


## At a glance

| Structured Text | Index | Annotation of a reference | LLVM |
|---|---|---|---|
| `PROGRAM main` | `Program` entry, struct `main`, global `main_instance` | `Program` | `%main`, `@main_instance`, `void @main(ptr)` |
| `FUNCTION_BLOCK Counter` | `FunctionBlock` entry, struct `Counter` | `Variable` of type `Counter` | `%Counter`, `void @Counter(ptr)` |
| `CLASS Limits` | `Class` entry, struct `Limits` | `Variable` of type `Limits` | `%Limits`, empty `void @Limits(ptr)` |
| `FUNCTION scale: DINT` | `Function` entry, struct `scale` with return member `scale.scale` | `Function`, return type `DINT` | `i32 @scale(...)`, no struct |
| `METHOD reset` | `Method` entry, struct `Counter.reset` for its parameters | `Function` | `void @Counter__reset(ptr, ...)` |
| `ACTION double` | `Action` entry, parent's struct | `Program` | `void @Counter__double(ptr)` |
| `VAR_INPUT x` | `ByVal(Input)` member | | struct field, or a by-value argument in a function |
| `VAR_OUTPUT x` | `ByVal(Output)` member, `ByRef(Output)` in a function | | struct field copied out after the call, or a pointer argument |
| `VAR_IN_OUT x` | `ByRef(InOut)` member of type `__auto_pointer_to_T` | `auto_deref: Default` | `ptr` field or argument, loaded before every access |
| `VAR_TEMP x` | `ByVal(Temp)` member | | stack slot in every body of the POU, not a struct field |
