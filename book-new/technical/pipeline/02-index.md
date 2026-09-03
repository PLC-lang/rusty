# Index

The index is the symbol table of the project. It records for every declared name what it is and what type it has. Only declarations matter for that: a statement body can use names, but it cannot introduce any, because every variable, type, and POU in Structured Text is declared in a declaration section. The index therefore reads the declarations and skips the bodies. For

```iecst
FUNCTION_BLOCK Buffer
    // Declarations: indexed, they introduce Buffer.limit and Buffer.values
    VAR_INPUT
        limit : INT;
    END_VAR
    VAR
        values : ARRAY[1..5] OF DINT;
    END_VAR
END_FUNCTION_BLOCK

PROGRAM main
    // Declarations: indexed, they introduce main.bufferInstance and main.i
    VAR
        bufferInstance : Buffer;
        i : DINT;
    END_VAR

    // Body: skipped, it only references names that are declared elsewhere
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

Those declarations may sit in another file; the index collects the declarations of all files into one table, so that every later stage can look a name up.

```mermaid
flowchart LR
    parse[Parse] --> index[Index] --> annotate[Annotate] --> validate[Validate] --> codegen[Codegen] --> link[Link]
    style index fill:#bfdbfe,stroke:#000,stroke-width:1px,stroke-dasharray:4 3,color:#0f172a
```

The stage receives the parsed compilation units and returns them together with one global index. Each unit is pre-processed and indexed into a table of its own, the tables are merged with the built-in declarations, and the constant expressions collected on the way are evaluated. The index is never updated in place: whenever a participant rewrites the tree, the stage runs again and replaces it.


## Pre-processing

The index is name based. An entry never points at another entry; it stores the name of its type as a string, and following it is another lookup. For `limit : INT` the entry stores `"INT"`. An anonymous type such as `ARRAY[1..5] OF DINT` or `STRING[80]` has no name to store.

Pre-processing fixes this before indexing starts. It walks each unit, turns every anonymous type into a named type declaration, and replaces the inline definition with a reference to the new name. The name is built from the container and the member; for a return type it is the function name and `return`:

```diff
+TYPE
+    __describe_return : STRING[80];
+    __describe_values : ARRAY[1..5] OF DINT;
+END_TYPE
+
-FUNCTION describe : STRING[80]
+FUNCTION describe : __describe_return
     VAR_INPUT
-        values : ARRAY[1..5] OF DINT;
+        values : __describe_values;
     END_VAR
 END_FUNCTION
```

The same happens to pointers (`REF_TO INT`) and to the type parameters of generic functions (`__ADD__T` for parameter `T` of `ADD`).

Pre-processing also normalizes two constructs, so that later stages see one shape instead of several. Enum variants all get an explicit value: `TYPE Speed : (Slow, Normal, Fast := 10); END_TYPE` becomes `Slow := 0, Normal := Speed#Slow + 1, Fast := 10`. A hardware address such as `sensor AT %IX0.0 : BOOL` gets a generated global named `__PI_0_0` that backs the address, and `sensor` becomes an alias pointer to it, of a generated type `__global_sensor`.

All names the compiler generates, here and in the lowering participants later, start with a double underscore. The prefix marks them as reserved and keeps them apart from user names. This is the first point in the pipeline where the AST is modified.

> **Developer Note**
>
> The prefix is a convention the compiler does not enforce: `__foo` compiles without a diagnostic, and a user name that matches a generated one is only caught indirectly, by the duplicate check or by a misleading type mismatch. See `bugs.md`.


## The index

This is what the index holds, trimmed to its fields:

```rust
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

Structured Text is case-insensitive, so every key is lowercased on insert and on lookup; `Buffer`, `buffer`, and `BUFFER` find the same entry. On top of the raw maps, the index offers lookup helpers for the common questions of later stages: the effective type behind an alias chain, a member of a container including its super classes, the parameters of a POU in call order.


## Indexing a unit

The indexer walks the declarations of a unit and puts an entry into the maps above for each one. For

```iecst
FUNCTION_BLOCK Counter
    VAR_INPUT
        step : DINT := 1;
    END_VAR
    VAR_OUTPUT
        count : DINT;
    END_VAR

    count := count + step;
END_FUNCTION_BLOCK
```

the walk reaches `FUNCTION_BLOCK Counter` and knows that everything up to `END_FUNCTION_BLOCK` belongs to a function block named `Counter`. In `VAR_INPUT` it finds `step` and creates a member entry: the variable `Counter.step`, an input of type `DINT`, at position 0. The initializer `1` is an expression, and the indexer does not evaluate expressions. It puts the expression into the constant store, a list of all constant expressions of the project that hands out an id for each, and stores that id in the entry. In `VAR_OUTPUT` it finds `count` and creates the second member entry, the output `Counter.count` of type `DINT` at position 1, without initializer.

With the declaration part done, the indexer knows the full shape of a `Counter` and registers it in three maps. The type index gets the instance struct, a type named `Counter` with the two entries as members in order; this struct is the memory layout of every instance. The POU map gets the entry that says `Counter` is a function block with that struct. The global initializers get a variable `__Counter__init` of type `Counter`, a default instance meant to be copied from; no later stage reads this map today, codegen derives default values from the type index instead (see the [Initializers](../internals/07-initializers.md) chapter).

Then the body. The indexer only records that an implementation exists for `Counter` and belongs to the type `Counter`. The statement `count := count + step` is not looked at: a body can only use names, never declare them. Connecting it to the entries is the job of the resolver.

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

The other declaration kinds follow the same walk with fewer stops. Global variables become entries in `global_variables`. A struct becomes a type in `type_index` with its members as variable entries, plus a `global_initializers` entry. An enum becomes a type and, in addition, one constant global per variant in `enum_global_variables`, so that `Slow` resolves without the qualifier `Speed.Slow`. Array bounds and string sizes go to `constant_expressions` like initializers. The values in these maps are four kinds of records: variable entries, POU entries, implementation entries (`src/index.rs`), and types (`src/typesystem.rs`). What they hold for each construct, including functions, by-reference parameters, and actions, is described in the [Internals](../internals/README.md) chapters.


## Merging

Units are indexed concurrently, one table each, and the tables are merged in unit order into one global index. Merging appends: a name declared in two files ends up with two entries under one key, which validation later reports as a duplicate. Constant expressions are copied into the global constant store and receive new ids, and the entries that hold an id are updated.

Two more tables are merged in after the user's units. The built-in types, `BOOL`, `INT`, `DINT`, `REAL`, `STRING`, `TIME`, and the rest, are constructed directly. The built-in functions, `ADR`, `SIZEOF`, `MUX`, `SEL`, the generic arithmetic and comparison functions, and the array bound functions, are Structured Text declarations embedded in the compiler; they are parsed, pre-processed, and indexed like any other unit and merged last. A user function named `add` therefore shares a key with the built-in `ADD` and is a duplicate symbol.

Visualized for two files, showing only the POU map:

```
a.st      pous: { scale }
b.st      pous: { main, buffer }
built-in  pous: { adr, sizeof, mux, sel, ... }

merged    pous: { scale, main, buffer, adr, sizeof, mux, sel, ... }
```


## Constant evaluation

Array bounds, string sizes, initial values, and enum variant values are expressions, and later stages need them as values: codegen has to know how many elements an array has. Only now, with all declarations merged, is every name such an expression can use known, so the constant store is evaluated as the last step. In

```iecst
VAR_GLOBAL CONSTANT
    SCALE_FACTOR : DINT := MAX_ITEMS + 1;
    MAX_ITEMS : DINT := 3;
    TOO_BIG : SINT := 300;
END_VAR

VAR_GLOBAL
    sensor AT %IX0.0 : BOOL;
    NOT_CONST : DINT := 5;
END_VAR

VAR_GLOBAL CONSTANT
    C : DINT := NOT_CONST + 1;
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
    SCALE_FACTOR : DINT := MAX_ITEMS + 1;
                           ^^^^^^^^^^^^^   { target_type: "DINT", Resolved(4) }
    MAX_ITEMS : DINT := 3;
                        ^                  { target_type: "DINT", Resolved(3) }
    TOO_BIG : SINT := 300;
                      ^^^                  { target_type: "SINT", Unresolvable("This will overflow for type SINT") }
END_VAR

VAR_GLOBAL
    sensor AT %IX0.0 : BOOL;
              ^^^^^^                       { target_type: "__global_sensor", Unresolvable("Try to re-resolve during codegen") }
    NOT_CONST : DINT := 5;
                        ^                  { target_type: "DINT", Resolved(5) }
END_VAR

VAR_GLOBAL CONSTANT
    C : DINT := NOT_CONST + 1;
                ^^^^^^^^^^^^^              { target_type: "DINT", Unresolvable("NOT_CONST is no const reference") }
END_VAR
```

Validation turns the stored reasons into diagnostics: `TOO_BIG` becomes a warning, and `C` an error that aborts the compilation, because codegen could not emit a value for it. As a last step, enums without an explicit default get one: the variant that evaluates to zero, or the first variant if none does.


## Where it lives

| What | Where |
|---|---|
| Pre-processing | `compiler/plc_ast` |
| Index | `src/index.rs`, `src/index/` |
| Constant evaluation | `src/resolver/` |
| Types | `src/typesystem.rs` |
| Built-ins | `src/builtins.rs` |


## What's next

The index knows every declaration, but the statement bodies are still untouched trees of names. `bufferInstance.values[i] := bufferInstance.limit` is not yet connected to the entries for `main.bufferInstance`, `Buffer.values`, or `Buffer.limit`, and no expression has a type. Making those connections, one expression at a time, is the job of the next stage, the [Resolver](03-resolver.md).
