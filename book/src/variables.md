# Variables

# Constants

Variable declaration blocks can be delcared as CONSTANT.
All variables of a constant declaration block become constants.
Constant variables can not be changed and need to be initialized.

## Example

```iecst
TYPE OneInt : INT := 1; END_TYPE

VAR_GLOBAL CONSTANT
    MAX_SIZE : INT := 99;
    MIN_LEN : INT := 1;
    counter : OneInt;  (* 1 *)
END_VAR

PROGRAM PLC_PRG
    VAR CONSTANT
        DEFAULT_INPUT : BOOL := FALSE;
    END_VAR
END_PROGRAM
```

## Variable Initialization

Initializers of variables are evaluated at compile time.
Therefore they can only consist of literals, other constants or expressions consisting of a combination of them.
Note that initializers must not contain recursive definitions.

If a variable has no initializer, the variable may be initialized with it's datatype's default value or else with `0`.

### Array Initialization

Arrays can be initialized using array literals.
If the array-initial value does not contain all required elements, the array's inner type's default value will be used to fill the missing values.

## Example

```iecst
TYPE SignalValue : INT := -1; END_TYPE

VAR_GLOBAL CONSTANT
    MIN_LEN : INT := 1;
    MAX_LEN : INT := 100;

    SIZE : INT := MAX_LEN - MIN_LEN;
END_VAR

PROGRAM PLC_PRG
    VAR_INPUT
        signals: ARRAY[0..SIZE] OF SignalValue := [99, 99]; (* rest is -1 *)
    END_VAR

    ...
END_PROGRAM
```

### Pointer Initialization

A pointer variable can be initialized with the address of a global reference or an IEC-address using the `AT` or `REFERENCE TO` syntax. `REF_TO` pointers can be initialized using the built-in `REF` function in its initializer.

This initialization, however, does not take place during compile time. Instead, each pointer initialized with an address will be zero-initialized to a null pointer by default. The compiler collects all pointer initializations during compilation and creates internal initializer functions for each POU. These functions are then called in a single overarching project-initialization function, which can be called either manually in your main function or by the runtime. Additionally, global variables — whether they are initialized pointers or POU instances containing pointer initializers — are also assigned within this overarching function.

This function follows a naming scheme (`__init___<project name>`) that varies slightly depending on whether a build config (`plc.json`) was used.

- **When using a build config (`plc.json`)**, the project name is used:

    _Build config snippet:_
    ```json
    {
        "name": "myProject",
        "files": []
    }
    ```
    _Resulting symbol:_
    ```iecst
        __init___myProject()
    ```

- **When compiling without a build config**, the name of the first file passed via CLI is used as the base for the name.

    _CLI command:_
    ```bash
        # build command
        plc myFile1.st myFile2.st
    ```
    _Resulting symbol:_
    ```iecst
        __init___myFile1_st()
    ```

It is important to note that if there are pointer initializations present in your project, failing to call the initialization function in your runtime or in `main` will result in **null pointer dereferences** at runtime.

### Example
_myProject.st_:
```iecst
VAR_GLOBAL
    myGlobal : STRING;
END_VAR

PROGRAM prog
VAR
    myString : REF_TO STRING := REF(myGlobal);
    myOtherString : REFERENCE TO STRING REF= myGlobal;
    myAlias AT myGlobal: STRING;
    myAnalogSignal AT %IX1.0 : REAL;
END_VAR
    // ...
END_PROGRAM

FUNCTION main: DINT
    __init___myProject_st();
    prog();
END_FUNCTION
