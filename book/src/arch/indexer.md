# Indexer

The indexing step is responsible of building and maintaining the Symbol-Table (also called *Index*).
The *Index* contains all known referable objects such as *variables*, *data-types*, *POUs*, *Functions*, etc.
The Symbol-Table also maintains additional information about every referable object such as: the object's type, the objects' datatype, etc.

Indexing is performed by the *index* module.
It contains the index itself (a.k.a. Symbol Table), the *visitor* which collects all global names and their additional information as well as a data structure that handles compile time constant expressions (*constant_expressions*).

## The Index (Symbol Table)

The index stores information about all referable elements of the program.
Depending on the type of element, we store different meta-information alongside the name of the element.

| Index Field               |  Description                             |
|---------------------------|------------------------------------------|
| global_variables          |  All global variables accessible via their name.  |
| enum_global_variables     |  All enum elements accessible via their name (as if they were global variables, e.g. 'RED')  |
| member_variables          |  Member variables of structured types (Structs,Functionblocks, etc. This map allows to query all members of a container by name.) |
| implementations           |  All callable implementations (Programs, Functions, Actions, Functionblocks) accessible by their name.    |
| pous                      |  All pous (Programs, Functions, Functionblocks) with additional information.    |
| type_index                |  All data-types (intrinsic and complex) accessible via their name  |
| constant_expressions      |  The results of constant expressions that can be evaluated at compile time (e.g. the initializer of a constant: `VAR_GLOBAL CONST TAU := 3.1415 * 2; END_VAR`) |

There are 3 different type of entries in the index:

- **VariableIndexEntry**
The VariableIndexEntry holds information about every *Variable* in the source code and offers additional information relevant for linking, validation and code-generation.

```ignore
        ┌─────────────────────────────┐              ┌─────────────────┐
        │  VariableIndexEntry         │              │     <enum>      │
        │                             │              │   VariableType  │
        ├─────────────────────────────┤   var_type   ├─────────────────┤
        │                             │              │  - Local        │
        │  - name: String             ├─────────────►│  - Temp         │
        │  - qualified_name: String   │              │  - Input        │
        │  - is_constant: bool        │              │  - Output       │
        │  - location_in_parent: u32  │              │  - InOut        │
        │  - data_type_name: String   │              │  - Global       │
        │                             │              │  - Return       │
        └───────────┬─────────────────┘              └─────────────────┘
                    │
                    │initial_value
                    │
                    │
                    │            ┌──────────────────┐
                    │            │ ConstExpression  │
                    │       0..1 ├──────────────────┤
                    └───────────►│                  │
                                 │ ...              │
                                 │                  │
                                 └──────────────────┘
```

- **PouIndexEntry**
The PouIndexEntry offers information about all Program-Organization-Units.
The index entry offers information like the name of an instance-struct, the name of the registered implementation, etc.

```ignore
┌──────────────────────────┐
│       <abstract>         │
│       POUIndexEntry      │
├──────────────────────────┤
│                          │
└──────────────────────────┘
             ▲
             │
             │
             │     ┌──────────────────────────┐      ┌──────────────────────────┐
             │     │    ProgramIndexEntry     │      │    GenericParameter      │
             │     ├──────────────────────────┤      ├──────────────────────────┤
             │     │ - name: String           │      │ - name: String           │
             ├─────┤ - instanceStruct: String ├──┬──►│ - typeNature: TypeNature │
             │     │                          │  │   │                          │
             │     │                          │  │   │                          │
             │     └──────────────────────────┘  │   └──────────────────────────┘
             │                                   │
             │                                   │
             │                                   │
             │     ┌──────────────────────────┐  │
             │     │    FunctionIndexEntry    │  │ generics
             │     ├──────────────────────────┤  │
             │     │ - name: String           │  │
             ├─────┤                          ├──┤
             │     │                          │  │
             │     │                          │  │
             │     └──────────────────────────┘  │
             │                                   │
             │                                   │
             │                                   │
             │     ┌──────────────────────────┐  │
             │     │ FunctionBlockIndexEntry  │  │
             │     ├──────────────────────────┤  │
             │     │ - name: String           ├──┤
             ├─────┤ - instanceStruct: String │  │
             │     │                          │  │
             │     │                          │  │
             │     └──────────────────────────┘  │
             │                                   │
             │                                   │
             │                                   │
             │     ┌──────────────────────────┐  │
             │     │    ClassIndexEntry       │  │
             │     ├──────────────────────────┤  │
             │     │ - name: String           │  │
             └─────┤ - instanceStruct: String ├──┘
                   │                          │
                   │                          │
                   └──────────────────────────┘
```

- **ImplementationIndexEntry**
The ImplementationIndexEntry offers information about any callable implementation (Program, Functionblock, Function, etc.).
It also offers metadata about the implementation type, the name of the method to call and the name of the parameter-struct (this-struct) to pass to the function.

```ignore
                                                  ┌───────────────────────┐
        ┌──────────────────────────┐              │       <enum>          │
        │ ImplementationIndexEntry │              │   ImplementationType  │
        ├──────────────────────────┤     type     │                       │
        │                          ├─────────────►├───────────────────────┤
        │ - call_name: String      │              │   - Program           │
        │ - type_name: String      │              │   - Function          │
        │                          │              │   - FunctionBlock     │
        └──────────────────────────┘              │   - Action            │
                                                  │   - Class             │
                                                  │   - Method            │
                                                  │                       │
                                                  └───────────────────────┘
```

- **DataType**
The entry for a DataType offers information about any data-type supported by the program to be compiled (internal data types as well as user defined data types).
For each data-type we offer additional information such as it's initial value, its type-nature (in terms of generic functions - e.g: ANY_INT) and some additional information about the type's internal structure and size (e.g. is it a number/array/struct/etc).

```ignore
                      ┌─────────────┐                   ┌────────────────────┐
                      │  DataType   │                   │ ConstantExpression │
                      ├─────────────┤   initial_value   ├────────────────────┤
                      │             ├──────────────────►│                    │
                      │ - name      │                   │  ...               │
                      │             ├─────────┐         │                    │
                      └──────┬──────┘         │         └────────────────────┘
                             │                │
                             │                │         ┌────────────────────┐
                             │                │         │ TypeNature         │
                             │                │         ├────────────────────┤
                             │ information    │         │ - Any              │
                             │                └────────►│ - Derived          │
                             │                nature    │ - Elementary       │
                             │                          │ - Num              │
                             ▼                          │ - Int              │
                      ┌───────────────────────┐         │ - Signed           │
                      │    <abstract>         │         │ - ...              │
                      │  DataTypeInformation  │         └────────────────────┘
                      ├───────────────────────┤
                      │                       │
                      └───────────────────────┘
                                  ▲
                                  │
                                  │
                                  │
         ┌────────────────┬───────┴───────┬──────────────┬──────────────┐
         │                │               │              │              │
┌────────┴───────┐ ┌──────┴──────┐ ┌──────┴─────┐  ┌─────┴──────┐  ┌────┴─────┐
│ Struct         │ │  Array      │ │ Integer    │  │  String    │  │ ...      │
├────────────────┤ ├─────────────┤ ├────────────┤  ├────────────┤  ├──────────┤
│ - name         │ │- name       │ │ - name     │  │ - size     │  │ ...      │
│ - members      │ │- inner_type │ │ - signed   │  │ - encoding │  │          │
│                │ │- dimensions │ │ - size     │  │            │  │          │
└────────────────┘ └─────────────┘ └────────────┘  └────────────┘  └──────────┘
```
