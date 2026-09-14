# Direct and Hardware Access

Two things use the `%` sign: reading a part of a value, and binding a variable to a hardware address.


## Direct access on a value

`%<size><position>` after a value reads a part of it. The standard allows this for bit string types, and this compiler allows it for every integer type as well.

| Letter | Reads | Example |
|---|---|---|
| `X` | 1 bit | `%X1` |
| `B` | 8 bit | `%B1` |
| `W` | 16 bit | `%W1` |
| `D` | 32 bit | `%D1` |

The position counts parts of that size, from `0`. For a bit, the `%X` can be left out.

```iecst
FUNCTION main: DINT
    VAR
        variable: LWORD;
        bitTarget: BOOL;
        byteTarget: BYTE;
        wordTarget: WORD;
        dwordTarget: DWORD;
    END_VAR

    variable    := 16#AB_CD_EF_12_34_56_78_90;
    bitTarget   := variable.%X63;   (* the last bit *)
    byteTarget  := variable.%B7;    (* the last byte *)
    wordTarget  := variable.%W3;    (* the last word *)
    dwordTarget := variable.%D1;    (* the last double word *)

    bitTarget   := variable.%D1.%W1.%B1.%X1;   (* accesses can be chained *)
END_FUNCTION
```

The position can also be a variable. The standard allows a literal only; this is an extension.

```iecst
access_var := 63;
bitTarget  := variable.%Xaccess_var;
```

A variable position needs the `%X`, because the short form without it is not allowed there. The variable must be a plain name, not a qualified one.


## Hardware addresses

`AT %<area><size><position>` binds a variable to an address of the process image.

| Area | Meaning |
|---|---|
| `I` | Input |
| `Q` | Output |
| `M` | Memory |

The size letter is the same as above, and the position has one or more parts, separated by `.`:

```iecst
VAR_GLOBAL
    inBit   AT %IX1.0: BOOL;
    outWord AT %QW2.5: WORD;
    memory  AT %MD3: DWORD;
END_VAR
```

`AT` also binds a variable to another variable, which makes it an alias:

```iecst
VAR
    alias AT shared: STRING;
END_VAR
```


## Addresses for instances

A function block that is instantiated more than once cannot name a fixed address in its declaration, because every instance needs its own. The declaration writes a template with `*` instead, and a `VAR_CONFIG` block gives the address of each instance:

```iecst
FUNCTION_BLOCK Sensor
    VAR
        raw AT %I*: INT;
    END_VAR
END_FUNCTION_BLOCK

PROGRAM Cycle
    VAR
        s1: Sensor;
        s2: Sensor;
    END_VAR
END_PROGRAM

VAR_CONFIG
    Cycle.s1.raw AT %IW1.2: INT;
    Cycle.s2.raw AT %IW1.3: INT;
END_VAR
```

The path in `VAR_CONFIG` names the program, the instance, and the member. The type must be the type of the member.

A build can write the list of all bound variables with their addresses as a file, with `--hwmap-file`. A tool that reads the symbols of the binary, for example a live monitor in an IDE, needs that list. The [command line reference](../reference/command-line.md) describes the option.


## What's next

The last chapter here is about code that is not text at all: [graphical programs](cfc.md).
