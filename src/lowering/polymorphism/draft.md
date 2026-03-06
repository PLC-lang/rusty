**TODO**: Replace ASCII diagrams with tldraw SVG images (color support, easier to visualize vtable/itable relations)

# Polymorphism in Structured Text

Polymorphism is supported as part of object-oriented programming in Structured Text (ST). There are two types of polymorphism:
1. Pointer variables to a class or function block, e.g. `refMyFb: POINTER TO MyFb;` or `refMyFb: REF_TO MyFb;`
2. Interface-typed variables, e.g. `refMyInterface: MyInterface;`

In case (1), any instance of a type derived from the base type can be assigned to the reference. In case (2), any instance of a class or function block that implements the interface can be assigned. In both cases, calling a method executes the implementation defined by the **actual (runtime) type** of the assigned instance, not the statically declared type of the variable. Consider
```iecst
VAR
    instanceA: FbA; // Has methods `foo`, `bar`
    instanceB: FbB; // Has methods `foo`, `bar` and `baz`; extends from `FbA`, inheriting `foo` but overriding `bar`
    instanceC: FbC; // Has methods `foo`

    refInstance: POINTER TO FbA;
    refInterface: InterfaceAC; // Defines method `foo`; both FbA and FbC implement InterfaceAC
END_VAR

// Base type
refInstance := ADR(instanceA);
refInstance^.foo(); // Calls FbA::foo
refInstance^.bar(); // Calls FbA::bar

// Derived type. Assignment works because FbB derives from FbA. Only methods defined in FbA are accessible though.
refInstance := ADR(instanceB);
refInstance^.foo(); // Calls FbA::foo (inherited)
refInstance^.bar(); // Calls FbB::bar (overridden)

// FbA implements InterfaceAC, so this assignment is valid
refInterface := instanceA;
refInterface.foo(); // Calls FbA::foo

// FbC also implements InterfaceAC, so this assignment is also valid
refInterface := instanceC;
refInterface.foo(); // Calls FbC::foo
```
To call the correct method at runtime (dynamic dispatch), the compiler must generate supporting data structures and lookup logic. Before explaining the actual implementation, we should first define the core data structure used in dynamic dispatch, namely virtual tables (from now on referred to as vtables).

VTables are structs of function pointers, where each field points to a method implementation. Each class or function block type has exactly one vtable, generated at compile time. Every instance embeds a pointer to its type's vtable (as a hidden first field), which is then used at runtime to resolve method calls. For example
```
┌─VTable FbA─┐     ┌────────────┐     ┌─VTable FbB─┐
├────────────┤  ┌─▶│  FbA::foo  │◀─┐  ├────────────┤
│    foo     │──┘  ├────────────┤  └──│    foo     │
├────────────┤  ┌─▶│  FbA::bar  │     ├────────────┤
│    bar     │──┘  ├────────────┤  ┌──│    bar     │
└────────────┘     │  FbB::bar  │◀─┘  ├────────────┤
                   ├────────────┤  ┌──│    baz     │
                   │  FbB::baz  │◀─┘  └────────────┘
                   ├────────────┤
                   │    ...     │
                   └────────────┘
```

Here `FbA`'s vtable fields point to `FbA::foo` and `FbA::bar` respectively. Similarly `FbB` points to its own implementations `FbB::bar` (overridden) and `FbB::baz` (unique to `FbB`), but because it inherits `foo`, that vtable field points to the parent's implementation, i.e. `FbA::foo`. At runtime, when calling `refInstance^.bar()`, a lookup is done via the instance's vtable pointer to fetch the function pointer for the correct method. The key takeaway from polymorphism is: **dynamic dispatch is just indirect function calls through function pointers**.

The next two sections describe the implementation for these two polymorphism types in more detail.

## 1. Class and Function Block Polymorphism
As mentioned in the introduction, any derived POU instance can be assigned to a reference of its base type. For example, assume we have a hierarchy where `FbA` is the parent of `FbB` which in turn is the parent of `FbC`. This would allow for
```iecst
VAR
    instanceA: FbA; // Has method foo
    instanceB: FbB; // Has method bar, baz
    instanceC: FbC; // Has method qux

    refInstanceA: POINTER TO FbA;
END_VAR

// All of these assignments are valid, because A, B and C all share a common interface due to inheritance.
// Put differently, instanceB and instanceC have, at minimum, the same methods as instanceA.
refInstanceA := ADR(instanceA);
refInstanceA^.foo(); // Calls FbA::foo
refInstanceA := ADR(instanceB);
refInstanceA^.foo(); // Calls FbB::foo (or FbA::foo if not overridden)
refInstanceA := ADR(instanceC);
refInstanceA^.foo(); // Calls FbC::foo (or the closest ancestor that overrides foo)
```
Now, in order to achieve dynamic dispatch, the compiler needs to perform a vtable lookup to execute the correct method. Rusty achieves this by patching any method call like so
```diff
-refInstanceA^.foo();
+__vtable_FbA#(refInstanceA^.__vtable^).foo^(FbA#(refInstanceA^), /* potentially other arguments */);
```
That in turn requires that classes and function blocks have a `__vtable` member field, which the compiler injects into the POU definition like so
```diff
FUNCTION_BLOCK FbA
    VAR
+       __vtable: POINTER TO __VOID;
        // ...potentially other member fields
    END_VAR
END_FUNCTION_BLOCK
```
The `__vtable` field is initialized at construction time by a separate initializer pass, which assigns it to `ADR(__vtable_FbA_instance)`. That in turn requires a `__vtable_FbA` struct definition whose members carry default initializers, each pointing to the corresponding method implementation. Function blocks also include a `__body` entry representing their callable body (classes do not, since they are not directly callable). For clarity, the ASCII diagrams in this document omit `__body` and show only named methods:
```diff
+TYPE __vtable_FbA:
+    STRUCT
+        __body: __FPOINTER FbA := ADR(FbA);
+        foo: __FPOINTER FbA.foo := ADR(FbA.foo);
+        bar: __FPOINTER FbA.bar := ADR(FbA.bar);
+    END_STRUCT
+END_TYPE
```
And finally a global instance of that struct, one per POU:
```diff
VAR_GLOBAL
+   __vtable_FbA_instance: __vtable_FbA;
END_VAR
```

For derived POUs the process is pretty much the same, except that they do not have their own `__vtable` member field but rather access the "root" parent's `__vtable` and override it. This is also why the vtable is a void pointer: different vtables (and therefore different types) may be assigned to the root's `__vtable` field. For example, inspecting the `A <- B <- C` POU inheritance chain we get
```diff
FUNCTION_BLOCK FbA
    VAR
+       __vtable: POINTER TO __VOID; // Initialized to ADR(__vtable_FbA_instance)
    END_VAR
END_FUNCTION_BLOCK

FUNCTION_BLOCK FbB
    VAR
+       __FbA: FbA; // Parent, with __vtable overridden to ADR(__vtable_FbB_instance)
    END_VAR
END_FUNCTION_BLOCK

FUNCTION_BLOCK FbC
    VAR
+       __FbB: FbB; // Parent, with __FbA.__vtable overridden to ADR(__vtable_FbC_instance)
    END_VAR
END_FUNCTION_BLOCK
```
The parent member fields (e.g. `__FbA`) are created by the inheritance lowering pass, and the vtable pointer assignments are handled by the initializer pass.

One more note: methods called within other methods also need to go through the vtable, because an inherited method may call an overridden method, e.g.
```diff
METHOD foo
    // This call must be evaluated at runtime, because a child POU might have overridden it
-   bar();
+   __vtable_FbA#(THIS^.__vtable^).bar^(FbA#(THIS^));
END_METHOD
```

Now that we know how vtables are stored and accessed, we should answer why `__vtable_FbA#(refInstanceA^.__vtable^).foo^(FbA#(refInstanceA^))` works in the first place. That is, why can we simply "cast" one vtable to another? Let's take a look at the vtable layouts:
```
┌─VTable FbA─┐   ┌─VTable FbB─┐   ┌─VTable FbC─┐
├────────────┤   ├────────────┤   ├────────────┤
│    foo     │   │    foo     │   │    foo     │
└────────────┘   ├────────────┤   ├────────────┤
                 │    bar     │   │    bar     │
                 ├────────────┤   ├────────────┤
                 │    baz     │   │    baz     │
                 └────────────┘   ├────────────┤
                                  │    qux     │
                                  └────────────┘
```

Notice how the order of function pointers is stable? At the top we have the function pointers of the parent class(es), followed by the function pointers of our own. This works because the generated vtable structs have a guaranteed sequential layout with no field reordering; each derived vtable is a strict prefix extension of its parent's. This is the reason why casting one vtable to another works in linear inheritance: we simply "reinterpret" the vtable as the parent type, cutting off trailing fields but keeping the content of the existing ones. In other words, upcasting from a derived class to a parent requires no runtime conversion. Note that this property only holds for single/linear inheritance chains; interfaces require a different dispatch mechanism (see Section 2).

**Putting it all together**: The compiler does the following to achieve dynamic dispatch for classes and function blocks:
1. Generate a vtable data structure for every class and function block, populated by function pointers for every method defined within these individual POUs
2. Generate a global variable instance for each generated vtable, initialized with the correct addresses for the function pointers
3. Generate and inject a `__vtable` member field for every non-extended class or function block of type `POINTER TO __VOID;`, initialized by the global variable instance
4. Transform **method** calls to make use of the lookup table, where
    1. Method is called from within another method
    2. Method is called by a variable of type `POINTER TO <CLASS|FUNCTION_BLOCK>`
    3. ...but leave `THIS` and `SUPER` calls untouched, since those are expected to be statically dispatched


## 2. Interface Polymorphism
Again, as mentioned in the introduction, interfaces can be used as a variable type to assign any concrete instance variable to them, assuming the POU implements the interface. For example
```iecst
VAR
    instanceFbA: FbA; // Implements interface IA (method foo)
    instanceFbB: FbB; // Implements interface IA (method foo) and IB (method bar)

    refInterface: IA;
END_VAR

refInterface := instanceFbA;
refInterface.foo(); // Calls FbA::foo

// Here we assign an instance of POU FbB to interface IA, which works because FbB implements IA
refInterface := instanceFbB;
refInterface.foo(); // Calls FbB::foo
```

### 2.1 The Problem: Why VTables Don't Work for Interfaces

With that in mind, let's try to apply our findings from Section 1 onto interfaces. Assume we have the following interface definitions
```
//   IA
//  /  \
// IB   IC
//  \  /
//   ID
//
// IA: foo
// IB EXTENDS IA: foo, bar
// IC EXTENDS IA: foo, baz
// ID EXTENDS IB, IC: foo, bar, baz, qux
```
and also some function blocks implementing them plus a function making use of polymorphism
```iecst
VAR
    instanceD: FbD; // Implements interface ID (foo, bar, baz, qux)

    refInterfaceB: IB;
    refInterfaceC: IC;
END_VAR

refInterfaceB := instanceD;
refInterfaceB.foo();
refInterfaceB.bar();

refInterfaceC := instanceD;
refInterfaceC.foo();
refInterfaceC.baz();
```

Two problems arise:
1. What types do `refInterfaceB` and `refInterfaceC` have?
2. How do we upcast `instanceD`'s vtable to interfaces IB or IC's vtable given their layouts are incompatible?

First, let's tackle the vtable issue. Assume for each interface declaration there is a corresponding function block that implements them. If we were to naively build vtables with each POU's methods in declaration order, we would get
```
┌─VTable FbA─┐   ┌─VTable FbB─┐   ┌─VTable FbC─┐   ┌─VTable FbD─┐
├────────────┤   ├────────────┤   ├────────────┤   ├────────────┤
│    foo     │   │    foo     │   │    foo     │   │    foo     │
└────────────┘   ├────────────┤   ├────────────┤   ├────────────┤
                 │    bar     │   │    baz     │   │    bar     │
                 └────────────┘   └────────────┘   ├────────────┤
                                                   │    baz     │
                                                   ├────────────┤
                                                   │    qux     │
                                                   └────────────┘
```
Clearly upcasting from vtable `FbD` to `FbB` works (both have `foo` at slot 0 and `bar` at slot 1), but `FbD` to `FbC` does not work because by doing so `bar` in `FbD` would be interpreted as `baz`. That is
```iecst
refInterfaceC := instanceD;
refInterfaceC.baz(); // This would call `FbD::bar` rather than `FbD::baz`!
```

If we were to swap `bar` and `baz`'s order in `FbD`, then upcasting from `FbD` to `FbC` would work, but from `FbD` to `FbB` would break. There is no single layout that satisfies both. We need a different approach.

### 2.2 Interface Tables (ITables)

The solution is to introduce a separate data structure: interface tables, or short itables. The idea is to have **one itable struct per interface** and **one global itable instance per (interface, POU) pair** where the POU implements the interface (directly or indirectly). Each itable struct contains function pointer fields matching the interface's method signatures, and each instance populates those pointers with the POU's concrete implementations.

For our diamond hierarchy example, the compiler generates the following itable struct definitions:
```diff
+TYPE __itable_IA:
+    STRUCT
+        foo: __FPOINTER TO IA.foo;
+    END_STRUCT
+END_TYPE
+
+TYPE __itable_IB:
+    STRUCT
+        foo: __FPOINTER TO IA.foo;
+        bar: __FPOINTER TO IB.bar;
+    END_STRUCT
+END_TYPE
+
+TYPE __itable_IC:
+    STRUCT
+        foo: __FPOINTER TO IA.foo;
+        baz: __FPOINTER TO IC.baz;
+    END_STRUCT
+END_TYPE
+
+TYPE __itable_ID:
+    STRUCT
+        foo: __FPOINTER TO IA.foo;
+        bar: __FPOINTER TO IB.bar;
+        baz: __FPOINTER TO IC.baz;
+        qux: __FPOINTER TO ID.qux;
+    END_STRUCT
+END_TYPE
```
Note how the function pointer types reference the original interface method POU (e.g. `IA.foo`), which already exists in the index as a registered implementation. This avoids the need for separate forward declaration stubs. Also note that inherited methods are included: `__itable_IB` contains both `foo` (from `IA`) and `bar` (from `IB`), with inherited methods appearing first in DFS order.

Then, the compiler generates global instances for every (interface, POU) combination:
```diff
+VAR_GLOBAL
+   // FbA implements IA directly
+   __itable_IA_FbA_instance: __itable_IA := (foo := ADR(FbA.foo));
+
+   // FbB implements IB which extends IA, so two instances are needed
+   __itable_IA_FbB_instance: __itable_IA := (foo := ADR(FbB.foo));
+   __itable_IB_FbB_instance: __itable_IB := (foo := ADR(FbB.foo), bar := ADR(FbB.bar));
+
+   // Similarly for FbC: implements IC which extends IA
+   __itable_IA_FbC_instance: __itable_IA := (foo := ADR(FbC.foo));
+   __itable_IC_FbC_instance: __itable_IC := (foo := ADR(FbC.foo), baz := ADR(FbC.baz));
+
+   // FbD implements ID which extends IB and IC, both of which extend IA.
+   // Four instances are needed (one per unique interface in the hierarchy).
+   __itable_IA_FbD_instance: __itable_IA := (foo := ADR(FbD.foo));
+   __itable_IB_FbD_instance: __itable_IB := (foo := ADR(FbD.foo), bar := ADR(FbD.bar));
+   __itable_IC_FbD_instance: __itable_IC := (foo := ADR(FbD.foo), baz := ADR(FbD.baz));
+   __itable_ID_FbD_instance: __itable_ID := (foo := ADR(FbD.foo), bar := ADR(FbD.bar), baz := ADR(FbD.baz), qux := ADR(FbD.qux));
+END_VAR
```

While verbose, this solves our layout incompatibility problem entirely. There is no need to upcast one itable to another. Instead we just swap the address of the itable pointer to the correct global variable instance. Each interface has its own consistent layout, and each POU gets its own instance with the correct function pointers.

Two additional cases are worth calling out:

**POU inheritance**: When a POU extends another POU that implements an interface, the child POU inherits the interface obligation. For example, if `FbB EXTENDS FbA` and `FbA IMPLEMENTS IA`, then `FbB` also needs an `__itable_IA_FbB_instance`. If `FbB` overrides a method, its itable instance points to the override; otherwise it points to the inherited implementation.

**Method resolution**: When populating an itable instance, the compiler walks the POU's inheritance chain to find the most derived implementation for each method. For example if `FbA` defines `foo`, `FbB EXTENDS FbA` overrides `foo`, and `FbC EXTENDS FbB` does not, then `FbC`'s itable will point `foo` to `FbB.foo`.

### 2.3 The Fat Pointer

With itables solving the function pointer lookup problem, we still need to answer: what type does an interface variable have? Interfaces are shallow constructs with no state. They serve purely as a contract that certain methods exist. However, for dispatch we need two things:
1. A way to find the correct itable (to call the right method)
2. A way to pass the concrete instance's data to that method (so it can access state)

This leads to the "fat pointer" struct:
```diff
+TYPE __FATPOINTER:
+   STRUCT
+       data:  POINTER TO __VOID;
+       table: POINTER TO __VOID;
+   END_STRUCT
+END_TYPE
```

The `data` field holds a pointer to the concrete POU instance, and the `table` field holds a pointer to the correct itable. Both are void pointers because different concrete types and different itable types may be assigned over the lifetime of the variable.

The compiler replaces every interface-typed variable declaration with `__FATPOINTER`. This happens uniformly across all variable kinds:
```diff
VAR
-   reference: IA;
+   reference: __FATPOINTER;
END_VAR

VAR_INPUT
-   param: IA;
+   param: __FATPOINTER;
END_VAR

// Also works for arrays
VAR
-   refs: ARRAY[1..3] OF IA;
+   refs: ARRAY[1..3] OF __FATPOINTER;
END_VAR

// And function return types
-FUNCTION producer : IA
+FUNCTION producer : __FATPOINTER
```

The `__FATPOINTER` struct is generated on demand, meaning it is only injected into the compilation units when at least one interface is used as a variable type. If no code uses interface variables, no fat pointer struct is emitted.

### 2.4 Dispatch Transformations

With itables and fat pointers in place, the compiler can now transform all interface-related operations. There are three categories of transformations.

#### 2.4.1 Assignments

When a concrete POU instance is assigned to an interface variable, the compiler expands the single assignment into two: one for the data pointer and one for the itable pointer.
```diff
-reference := instanceFbA;
+reference.data  := ADR(instanceFbA);
+reference.table := ADR(__itable_IA_FbA_instance);
```

This also works with array elements:
```diff
-refs[1] := instanceFbA;
+refs[1].data  := ADR(instanceFbA);
+refs[1].table := ADR(__itable_IA_FbA_instance);
```

The compiler determines the correct itable instance name from the type annotations: the type of the right hand side gives the POU name, and the type hint (expected type on the left) gives the interface name.

#### 2.4.2 Method Calls

When a method is called on an interface variable, the compiler transforms it into an indirect call through the itable. The transformation has four steps:

**Step 1**: Prepend the data pointer as the implicit first argument (this is the `self` parameter that the method expects):
```diff
-reference.foo(args);
+reference.foo(reference.data^, args);
```

**Step 2**: Replace the base of the operator with a dereferenced `.table` access:
```diff
-reference.foo(reference.data^, args);
+reference.table^.foo(reference.data^, args);
```

**Step 3**: Cast the itable access to the concrete itable type so the compiler knows the struct layout:
```diff
-reference.table^.foo(reference.data^, args);
+__itable_IA#(reference.table^).foo(reference.data^, args);
```

**Step 4**: Dereference the function pointer to perform the indirect call:
```diff
-__itable_IA#(reference.table^).foo(reference.data^, args);
+__itable_IA#(reference.table^).foo^(reference.data^, args);
```

Putting those steps together:
```diff
-reference.foo(1, 2);
+__itable_IA#(reference.table^).foo^(reference.data^, 1, 2);
```

This also works with named arguments:
```diff
-reference.foo(a := 10, b := 20);
+__itable_IA#(reference.table^).foo^(reference.data^, a := 10, b := 20);
```

And with nested interface calls (lowered bottom up):
```diff
-reference.baz(reference.foo(reference.bar()), 42);
+__itable_IA#(reference.table^).baz^(reference.data^, __itable_IA#(reference.table^).foo^(reference.data^, __itable_IA#(reference.table^).bar^(reference.data^)), 42);
```

#### 2.4.3 Call Arguments

When a concrete POU instance is passed as an argument to a function that expects an interface type, the compiler allocates a temporary fat pointer, populates it, and passes it in place of the original argument:
```diff
-consumer(instanceFbA);
+alloca __fatpointer_0: __FATPOINTER;
+__fatpointer_0.data  := ADR(instanceFbA);
+__fatpointer_0.table := ADR(__itable_IA_FbA_instance);
+consumer(__fatpointer_0);
```

This works with named arguments too:
```diff
-consumer(in := instanceFbA);
+alloca __fatpointer_0: __FATPOINTER;
+__fatpointer_0.data  := ADR(instanceFbA);
+__fatpointer_0.table := ADR(__itable_IA_FbA_instance);
+consumer(in := __fatpointer_0);
```

Multiple interface arguments in a single call each get their own temporary:
```diff
-consumer(instanceA, instanceB, instanceC);
+alloca __fatpointer_0: __FATPOINTER;
+__fatpointer_0.data  := ADR(instanceA);
+__fatpointer_0.table := ADR(__itable_IA_FbA_instance);
+alloca __fatpointer_1: __FATPOINTER;
+__fatpointer_1.data  := ADR(instanceB);
+__fatpointer_1.table := ADR(__itable_IA_FbB_instance);
+alloca __fatpointer_2: __FATPOINTER;
+__fatpointer_2.data  := ADR(instanceC);
+__fatpointer_2.table := ADR(__itable_IA_FbC_instance);
+consumer(__fatpointer_0, __fatpointer_1, __fatpointer_2);
```

The preamble (allocas and assignments) is hoisted before the call. When the call is nested inside an assignment (e.g. `result := consumer(instance)`), the preamble is hoisted above the entire assignment so that the fat pointer is fully constructed before the call executes.

### 2.5 Interaction with Aggregate Return Lowering

The interface dispatch lowering runs before the aggregate return lowering (`AggregateTypeLowerer`). This matters because functions returning aggregate types (like `STRING` or structs) undergo their own transformation where the return value is moved into a `VAR_IN_OUT` parameter and callers allocate a temporary to receive it.

When both transformations apply to the same call (e.g. an interface method returning `STRING` that also takes an interface argument), the interface dispatch pass produces an expression list with the fat pointer preamble followed by the call. The aggregate lowerer then processes each element of that list individually, ensuring the ordering is preserved. For example:
```diff
// User code:
-result := reference.foo(instance);

// After interface dispatch lowering:
+alloca __fatpointer_0: __FATPOINTER;
+__fatpointer_0.data  := ADR(instance);
+__fatpointer_0.table := ADR(__itable_IA_FbA_instance);
+result := __itable_IA#(reference.table^).foo^(reference.data^, __fatpointer_0);

// After aggregate return lowering:
+alloca __fatpointer_0: __FATPOINTER;
+__fatpointer_0.data  := ADR(instance);
+__fatpointer_0.table := ADR(__itable_IA_FbA_instance);
+alloca __0: STRING;
+__itable_IA#(reference.table^).foo^(reference.data^, __0, __fatpointer_0);
+result := __0;
```

## 3. Pipeline Integration

The polymorphism lowering is orchestrated by `PolymorphismLowerer` which plugs into the compiler pipeline as a `PipelineParticipantMut`. It runs at two points in the pipeline:

**`post_index`**: After the initial index is built from the parsed AST, the table generators run. This includes both the virtual table generator (for classes and function blocks) and the interface table generator. The table generator produces the itable struct definitions and global itable instances, appending them to the compilation units. Itable definitions are placed in the compilation unit where the interface is defined, and global instances are placed in the unit where the implementing POU is defined. This ensures multi-file compilation works correctly, with each artifact local to its respective unit. After table generation, the project is re-indexed so the new types and globals are visible to subsequent phases.

**`post_annotate`**: After type resolution and annotation, the dispatch lowerers run. The interface dispatch lowerer runs first (replacing interface type declarations with `__FATPOINTER`, lowering assignments, method calls, and call arguments), followed by the POU dispatch lowerer (vtable patching for class/function block polymorphism). After dispatch lowering, the project is re-indexed and re-annotated so that newly injected types like `__FATPOINTER` are properly resolved for code generation.

In summary, the pipeline looks like:
```
Parse → Index → [PolymorphismLowerer::post_index] → Re-index → Annotate → [PolymorphismLowerer::post_annotate] → Re-index → Re-annotate → Codegen
                 ├─ VTable generation                                       ├─ Interface dispatch lowering
                 └─ ITable generation                                       └─ POU dispatch lowering
```

### 3.1 Codegen Considerations

A few adjustments were needed in the code generation phase to support interface polymorphism:

**Interface method stubs**: The indexer registers an implementation entry for each interface method so that codegen can look up the function signature when generating indirect calls through itables. These methods have no body, but their LLVM function stubs provide the type templates that `build_indirect_call` needs.

**Skipping struct lookup for interfaces**: When generating a method, the POU generator normally looks up the associated class's struct type. For interface methods this lookup is skipped because interfaces don't have struct types. The "self" parameter is simply a void pointer passed through the fat pointer's `data` field.

**Debug info for synthesized types**: The `__FATPOINTER` struct is synthesized by the lowering pass and has an internal source location. The debug builder excludes all internal types from debug info. Functions whose parameters reference internal types (e.g. `__FATPOINTER`) gracefully omit those parameters from the DWARF subroutine type, and struct members whose types are internal are skipped in the debug metadata.

## 4. Complete Example

To tie everything together, let's trace through a complete example from user code to lowered form.

**User code** (across multiple files):
```iecst
// ia.st
INTERFACE IA
    METHOD describe : DINT
    END_METHOD
END_INTERFACE

// fb_a.st
FUNCTION_BLOCK FbA IMPLEMENTS IA
    METHOD describe : DINT
        printf('FbA$N');
        describe := 1;
    END_METHOD
END_FUNCTION_BLOCK

// fb_b.st
FUNCTION_BLOCK FbB IMPLEMENTS IA
    METHOD describe : DINT
        printf('FbB$N');
        describe := 2;
    END_METHOD
END_FUNCTION_BLOCK

// main.st
FUNCTION main
    VAR
        instA: FbA;
        instB: FbB;
        refs:  ARRAY[1..2] OF IA;
        i:     DINT;
    END_VAR

    refs[1] := instA;
    refs[2] := instB;

    FOR i := 1 TO 2 DO
        printf('id=%d$N', refs[i].describe());
    END_FOR;
END_FUNCTION
```

**After table generation** (post_index), the following artifacts are added:

In `ia.st`'s compilation unit:
```iecst
TYPE __itable_IA:
    STRUCT
        describe: __FPOINTER IA.describe;
    END_STRUCT
END_TYPE
```

In `fb_a.st`'s compilation unit:
```iecst
VAR_GLOBAL
    __itable_IA_FbA_instance: __itable_IA := (describe := ADR(FbA.describe));
END_VAR
```

In `fb_b.st`'s compilation unit:
```iecst
VAR_GLOBAL
    __itable_IA_FbB_instance: __itable_IA := (describe := ADR(FbB.describe));
END_VAR
```

**After dispatch lowering** (post_annotate), the main function becomes:
```iecst
FUNCTION main
    VAR
        instA: FbA;
        instB: FbB;
        refs:  ARRAY[1..2] OF __FATPOINTER;
        i:     DINT;
    END_VAR

    // refs[1] := instA  →  two field assignments
    refs[1].data  := ADR(instA);
    refs[1].table := ADR(__itable_IA_FbA_instance);

    // refs[2] := instB  →  two field assignments
    refs[2].data  := ADR(instB);
    refs[2].table := ADR(__itable_IA_FbB_instance);

    FOR i := 1 TO 2 DO
        // refs[i].describe()  →  indirect call through itable
        printf('id=%d$N', __itable_IA#(refs[i].table^).describe^(refs[i].data^));
    END_FOR;
END_FUNCTION
```

At runtime, when `i = 1`, `refs[1].table` points to `__itable_IA_FbA_instance`, so `describe` resolves to `FbA.describe`. When `i = 2`, `refs[2].table` points to `__itable_IA_FbB_instance`, so `describe` resolves to `FbB.describe`. The output would be:
```
FbA
id=1
FbB
id=2
```
