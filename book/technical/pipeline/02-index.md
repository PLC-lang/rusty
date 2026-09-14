# Index

The index is the project's symbol table. It records declared names, their kinds, and their types. The indexer reads declaration sections and records implementations, but does not resolve expressions in their bodies. For

```iecst
FUNCTION_BLOCK Buffer
    // Declarations: indexed, they introduce Buffer.limit and Buffer.values
    VAR_INPUT
        limit: INT;
    END_VAR
    VAR
        values: ARRAY[1..5] OF DINT;
    END_VAR
END_FUNCTION_BLOCK

PROGRAM main
    // Declarations: indexed, they introduce main.bufferInstance and main.i
    VAR
        bufferInstance: Buffer;
        i: DINT;
    END_VAR

    // Body: skipped, it only references names defined in declarations elsewhere
    bufferInstance.values[i] := bufferInstance.limit;
END_PROGRAM
```

the index records:

- `Buffer` is a function block.
- `Buffer.limit` is an input variable of type `INT`.
- `Buffer.values` is a local variable of an array type.
- `main` is a program.
- `main.bufferInstance` is a local variable of type `Buffer`.
- `main.i` is a local variable of type `DINT`.

The declarations can be in different files. The index combines them so that later stages can look up names across the project.

```mermaid
flowchart LR
    parse[Parse] --> index[Index] --> annotate[Annotate] --> validate[Validate] --> codegen[Codegen] --> link[Link]
    style index fill:#bfdbfe,stroke:#000,stroke-width:1px,stroke-dasharray:4 3,color:#0f172a
```

The stage receives the parsed compilation units and returns them with one global index. Each unit is pre-processed and indexed into a table of its own, the tables are merged with the built-in declarations, and the constant expressions collected on the way are evaluated.


## Pre-processing

Index entries refer to types by name. For `limit: INT`, the variable entry stores `"INT"`; finding that type requires another lookup. An inline type such as `ARRAY[1..5] OF DINT` has no name yet.

Pre-processing fixes this before indexing starts. It walks each unit, turns every anonymous type into a named type declaration, and replaces the inline definition with a reference to the new name. The name is built from the container and the member; for a return type it is the function name and `return`:

```diff
+TYPE
+    __describe_return: STRING[80];
+    __describe_values: ARRAY[1..5] OF DINT;
+END_TYPE
+
-FUNCTION describe: STRING[80]
+FUNCTION describe: __describe_return
     VAR_INPUT
-        values: ARRAY[1..5] OF DINT;
+        values: __describe_values;
     END_VAR
 END_FUNCTION
```

The same happens to pointers (`REF_TO INT`) and to the type parameters of generic functions (`__ADD__T` for parameter `T` of `ADD`).

Pre-processing also gives enum variants explicit values. `TYPE Speed: (Slow, Normal, Fast := 10); END_TYPE` becomes `Slow := 0, Normal := Speed#Slow + 1, Fast := 10`.

For a hardware binding such as `sensor AT %IX0.0: BOOL`, it creates the global `__PI_0_0`. The variable `sensor` becomes an alias pointer to that global, with the generated type `__global_sensor`. The [Hardware Map](../outputs/01-hardware-map.md) chapter follows this connection.

Generated helper names often use a double underscore prefix. This is a naming convention, not an enforced restriction. The `pre_index` participants run before pre-processing, so some generated declarations already exist at this point.


## The index

This is what the index holds, trimmed to its fields:

```rust,noplayground
pub struct Index {
    /// Variables declared in VAR_GLOBAL blocks
    global_variables: SymbolMap<String, VariableIndexEntry>,

    /// Generated globals holding the default value of a struct, array, string, or POU instance
    global_initializers: SymbolMap<String, VariableIndexEntry>,

    /// Enum variants, keyed by the variant name alone
    enum_global_variables: SymbolMap<String, VariableIndexEntry>,

    /// Programs, functions, function blocks, classes, methods, and actions
    pous: SymbolMap<String, PouIndexEntry>,

    /// Interfaces
    interfaces: SymbolMap<String, InterfaceIndexEntry>,

    /// Properties, keyed by the POU that declares them
    properties: SymbolMap<String, Identifier>,

    /// Bodies, keyed by call name
    implementations: FxIndexMap<String, ImplementationIndexEntry>,

    /// Types: built-in, user-declared, created by pre-processing, and the instance struct of every POU
    type_index: TypeIndex,

    /// Initializers, array bounds, and string sizes, as expressions until evaluated
    constant_expressions: ConstExpressions,

    /// Size and alignment of the primitive types on the target
    data_layout: DataLayout,

    /// Jump labels, keyed by POU
    labels: FxIndexMap<String, SymbolMap<String, Label>>,

    /// VAR_CONFIG declarations
    config_variables: Vec<ConfigVariable>,
}
```

Structured Text is case-insensitive, so every key is lowercased on insert and on lookup: `Buffer`, `buffer`, and `BUFFER` find the same entry. Above the raw maps sit lookup helpers for the common questions of the later stages: the effective type behind an alias chain, a member of a container including its super classes, the parameters of a POU in call order.


## Indexing a unit

The indexer walks the declarations of a unit and puts an entry into the maps above for each one. For

```iecst
FUNCTION_BLOCK Counter
    VAR_INPUT
        step: DINT := 1;
    END_VAR
    VAR_OUTPUT
        count: DINT;
    END_VAR

    count := count + step;
END_FUNCTION_BLOCK
```

the indexer creates two member entries in declaration order: `Counter.step` and `Counter.count`. Both have type `DINT`; `step` is input 0 and `count` is output 1. The indexer stores the initializer `1` in the constant store and keeps its ID on `step`. It does not evaluate the expression yet.

With the declaration part done, the indexer knows the full shape of a `Counter` and registers it in three maps. The type index gets the instance struct, a type named `Counter` with the two entries as members in order; this struct is the memory layout of every instance. The POU map gets the entry that says `Counter` is a function block with that struct.

The global initializers get a variable `__Counter__init` of type `Counter`, a default instance to copy from (see the [Initializers](../internals/07-initializers.md) chapter).

The implementation entry records that `Counter` has a body. The resolver later connects `count := count + step` to the member entries.

Visualized, trimmed to the maps that received entries:

```
Index {
    pous: {
        "counter": FunctionBlock { name: "Counter", instance_struct_name: "Counter" },
    },
    type_index: {
        pou_types: {
            "counter": Struct {
                name: "Counter",
                members: [
                    { name: "step",  qualified_name: "Counter.step",  data_type_name: "DINT", argument_type: Input,  location_in_parent: 0, initial_value: ConstId(0) },
                    { name: "count", qualified_name: "Counter.count", data_type_name: "DINT", argument_type: Output, location_in_parent: 1, initial_value: None },
                ],
            },
        },
    },
    global_initializers: {
        "__counter__init": { name: "__Counter__init", data_type_name: "Counter" },
    },
    implementations: {
        "counter": { call_name: "Counter", type_name: "Counter", kind: FunctionBlock },
    },
    constant_expressions: [
        ConstId(0): { expression: 1, target_type: "DINT", state: Unresolved },
    ],
}
```

The keys are the lowercased names. The member entries live inside the struct type; the index has no map of members of its own, and `find_member("Counter", "step")` looks up the type and searches its members.

The other declaration kinds follow the same walk with fewer stops. Global variables become entries in `global_variables`. A struct becomes a type in `type_index` with its members as variable entries, plus a `global_initializers` entry. An enum becomes a type and, in addition, one constant global per variant in `enum_global_variables`, so that `Slow` resolves without the qualifier `Speed.Slow`. Array bounds and string sizes go to `constant_expressions` like initializers.

The [Internals](../internals/README.md) chapters show how each language construct uses these records.


## Merging

Units are indexed concurrently, one table each, and the tables are merged in unit order into one global index. Merging appends: a name declared in two files ends with two entries under one key, which validation later reports as a duplicate. Constant expressions are copied into the global constant store, receive new IDs, and the entries that hold an ID are updated.

Two more tables are merged in after the user's units. The built-in types, `BOOL`, `INT`, `DINT`, `REAL`, `STRING`, `TIME`, and the rest, are constructed directly. The built-in functions, `ADR`, `SIZEOF`, `MUX`, `SEL`, the generic arithmetic and comparison functions, and the array bound functions, are Structured Text declarations embedded in the compiler. They are parsed, pre-processed, and indexed like any other unit, and merged last. A user function named `add` therefore shares a key with the built-in `ADD` and is a duplicate symbol.

Visualized for two files, showing only the POU map:

```
a.st      pous: { scale }
b.st      pous: { main, buffer }
built-in  pous: { adr, sizeof, mux, sel, ... }

merged    pous: { scale, main, buffer, adr, sizeof, mux, sel, ... }
```


## Constant evaluation

Later stages need constant values for array bounds, string sizes, initializers, and enum variants. Evaluation runs after merging, when all declarations are available. In

```iecst
VAR_GLOBAL CONSTANT
    SCALE_FACTOR: DINT := MAX_ITEMS + 1;
    MAX_ITEMS: DINT := 3;
    TOO_BIG: SINT := 300;
END_VAR

VAR_GLOBAL
    sensor AT %IX0.0: BOOL;
    NOT_CONST: DINT := 5;
END_VAR

VAR_GLOBAL CONSTANT
    C: DINT := NOT_CONST + 1;
END_VAR
```

the constant store holds one expression per initializer, plus the address `__PI_0_0` that pre-processing gave `sensor`. The evaluator works through them as a queue and tries to fold each into a literal. An expression that depends on a name that is not resolved yet goes to the back of the queue; every other expression is marked resolved, or unresolvable with a reason. The first pass over the example:

1. `MAX_ITEMS + 1`: `MAX_ITEMS` is not resolved yet, back of the queue.
2. `3`: already a literal, resolved.
3. `300`: a literal, but it initializes a `SINT` and does not fit; unresolvable, "This will overflow for type SINT".
4. `__PI_0_0`: an address, and addresses exist only after codegen has allocated the globals; unresolvable, "Try to re-resolve during codegen", which codegen understands as an instruction.
5. `5`: resolved.
6. `NOT_CONST + 1`: references a variable that is not declared constant; unresolvable, "NOT_CONST is no const reference".

The queue now holds only `MAX_ITEMS + 1`. On the second pass `MAX_ITEMS` is known to be `3`, and the expression resolves to `4`. The loop stops when a full pass makes no progress.

Visualized:

```
VAR_GLOBAL CONSTANT
    SCALE_FACTOR: DINT := MAX_ITEMS + 1;
                          ^^^^^^^^^^^^^   { target_type: "DINT", Resolved(4) }
    MAX_ITEMS: DINT := 3;
                       ^                  { target_type: "DINT", Resolved(3) }
    TOO_BIG: SINT := 300;
                     ^^^                  { target_type: "SINT", Unresolvable("This will overflow for type SINT") }
END_VAR

VAR_GLOBAL
    sensor AT %IX0.0: BOOL;
              ^^^^^^                       { target_type: "__global_sensor", Unresolvable("Try to re-resolve during codegen") }
    NOT_CONST: DINT := 5;
                       ^                  { target_type: "DINT", Resolved(5) }
END_VAR

VAR_GLOBAL CONSTANT
    C: DINT := NOT_CONST + 1;
               ^^^^^^^^^^^^^              { target_type: "DINT", Unresolvable("NOT_CONST is no const reference") }
END_VAR
```

Validation turns the stored reasons into diagnostics: `TOO_BIG` becomes a warning, and `C` an error that aborts the compilation, because codegen has no value to write for it. As a last step, every enum without an explicit default gets one: the variant that evaluates to zero, or the first variant if none does.


## Where it lives

| What | Where |
|---|---|
| Pre-processing | `compiler/plc_ast` |
| Index | `src/index.rs`, `src/index/` |
| Constant evaluation | `src/resolver/` |
| Types | `src/typesystem.rs` |
| Built-ins | `src/builtins.rs` |


## What's next

The index now holds the declarations, but body references still need meaning. The [Resolver](03-resolver.md) connects `bufferInstance.values[i]` and `bufferInstance.limit` to these entries and determines their types.
