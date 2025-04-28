# Program Organization Unit (POU)

## Definition

A POU is a executable unit available in an IEC61131-3 application.
It can be defined as either a Program, a Function, a Function Block, or an Action.

> Methods on classes are also considered POUs but are not covered by this document

A POU is defined as:

```iecst
<POU Type> name
(* parameters *)

(* code *)
END_<POU Type>
```

### Parameters

POUs can use input, output, or in/out parameters to pass data to the outside.
Such parameters are defined in a _variable block_  delimeted by `VAR_<TYPE>` and `END_VAR`
Supported parameter types are `VAR_INPUT`, `VAR_INPUT {ref}`, `VAR_OUTPUT` and `VAR_IN_OUT`

#### Input

Input parameters are typically copied into the target POU to be stored and read for later references.

A definition for input parameters is as follows:

```iecst
VAR_INPUT
    a : INT;
END_VAR
```

In some cases, especially when passing large strings or arrays, or when interacting with foreign code (see [External Functions](libraries/external_functions.md)) it is more efficient to avoid copying the variable values and just use a pointer to the required input.
This can be done either using the in/out variables or by specifying a special property `ref` on the input block.

Example:

```iecst
VAR_INPUT {ref}
    a : STRING;
END_VAR
```

Note that passing the ref property will convert all variables in that block to pointers, and should only be used in Functions.

#### In Out

In/Out parameters are required parameters that are always passed by reference.
They can be modified by the POU the call, and the changes are applied directly to the passed variable.
An In/Out parameter must always be passed in a POU call and cannot be stored.

#### Output

Output parameters are used to return the result(s) of the POU call.
They are passed by reference, but are optional.
If an output parameter is not passed in a call, its value is not persisted.

### Variables

In addition to parameters, a POU contains local variables, these can either be stored in the POU for later reference (`VAR`) or only created for a single call (`VAR_TEMP`)
In a function, all local variables are temporary.

## Specialization

In addition to the default behavior, each type of POU has some special cases.

### Function

Functions are _stateless_ sequences of callable code. They are not backed by any structs, and cannot hold any state accross multiple calls.
A function's input parameter can be passed by value, or by reference.

Functions also support a return type, the resulting definition is:

```iecst
    FUNCTION fnName : <TYPE>
    (* parameters *)
    VAR_INPUT (* by value *)
        x : INT;
    END_VAR
    VAR_INPUT {ref} (* by reference *)
        x : INT;
    END_VAR
    (* temporary variables *)
    VAR
        y : INT;
    END_VAR
    VAR_TEMP
        z : INT;
    END_VAR

    (* code *)
    END_FUNCTION
```

### Program

Programs are a static (i.e. `GLOBAL`) `STRUCT` that holds its state accross multiple calls.
A Program exists once, and only once in an application, and subsequent calls to a program will change and store the passed parameters as well as internal variables.
A program does not support passing input parameters by reference.

Example:

```iecst
PROGRAM prg
(* parameters *)
VAR_INPUT
    x : INT;
END_VAR
(* persisted variables *)
VAR
    y : INT;
END_VAR
(* temporary variables *)
VAR_TEMP
    z : INT;
END_VAR
(* code *)
END_PROGRAM
```

### Function Block

A function block is a `STRUCT` that can be initialized multiple times using different variables (i.e `instance`s).
A function block instance can hold its state (including input parameters) across multiple calls, but does not share any state with different instances.
A function block does not support passing input parameters by reference.

```iecst
FUNCTION_BLOCK fb
(* parameters *)
VAR_INPUT
    x : INT;
END_VAR
(* persisted variables *)
VAR
    y : INT;
END_VAR
(* temporary variables *)
VAR_TEMP
    z : INT;
END_VAR
(* code *)
END_FUNCTION_BLOCK
```

#### `FUNCTION_BLOCK` initialization
Function blocks can define a special method called `FB_INIT` that is automatically called when an instance is created. This is analogous to a constructor in object-oriented programming.

The `FB_INIT` method allows you to initialize the function block's variables to specific values. It is called during program initialization before any other code runs.

`FB_INIT` methods can neither have parameters nor a return type in their current implementation - violating this contract will lead to undefined behaviour at runtime.

```iecst
FUNCTION_BLOCK MyFB
VAR
    x : INT;
    y : INT;
END_VAR
    METHOD FB_INIT
        x := 1;
        y := 2;
    END_METHOD

    // Other methods and code...
END_FUNCTION_BLOCK
```

### Action

An action is represented by a parent struct, and does not define its own interface (VAR blocks).
An action can only be defined for Programs and Function Blocks.

An action is defined in 3 different ways, either in a container (`ACTIONS`) directly below the POU, in a named `ACTIONS` container, or using a qualified name on the action.

Example:

```iecst
FUNCTION_BLOCK fb
(* parameters *)
VAR_INPUT
    x : INT;
END_VAR
(* persisted variables *)
VAR
    y : INT;
END_VAR
(* temporary variables *)
VAR_TEMP
    z : INT;
END_VAR
(* code *)
END_FUNCTION_BLOCK

ACTIONS (* implicitly belongs to FB *)
    ACTION act
    (* code *)
    END_ACTION
END_ACTIONS

ACTIONS fb (* explicitly belongs to FB *)
    ACTION act2
    (* code *)
    END_ACTION
END_ACTIONS

ACTION fb.act3 (* linked to FB with name definition *)
(* code *)
END_ACTION
```
