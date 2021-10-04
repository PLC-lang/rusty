# Variables

# Constants
Variable declaration blocks can be delcared as CONSTANT. All variables 
of a constant declaration block become constants. Constant variables can not be changed and need to be initialized.

## Example 
```st
VAR_GLOBAL CONSTANT
    MAX_SIZE : INT := 99;
    MIN_LEN : INT := 1;
END_VAR

PROGRAM PLC_PRG
    VAR CONSTANT
        DEFAULT_INPUT : BOOL := FALSE;
    END_VAR
END_PROGRAM
```

## Variable Initialization
Initializers of variables are evaluated at compile time. Therefore 
they can only consist of literals, other constants or expressions
consisting of a combination of them. Note that initializers must not contain
recursive definitions.

## Example
```st
VAR_GLOBAL CONSTANT
    MIN_LEN : INT := 1;
    MAX_LEN : INT := 100;

    SIZE : INT := MAX_LEN - MIN_LEN;
END_VAR

PROGRAM PLC_PRG
    VAR_INPUT
        signals: ARRAY[0..SIZE] OF INT;
    END_VAR

    ...
END_PROGRAM
```
