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

This initialization, however, does not take place during compile time. Instead, each pointer initialized with an address will be zero-initialized to a null pointer by default. The compiler collects all pointer initializations during compilation and emits constructor functions that run before the program starts:

- **Type/POU constructors** (`<TypeName>__ctor`) set up initialized fields and `FB_INIT` calls
- **Global constructor** (`__unit_<name>__ctor`) initializes all globals and invokes the relevant constructors

These constructors are registered in the global constructor list and run automatically at load time. No manual calls are required.

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
    prog();
END_FUNCTION
