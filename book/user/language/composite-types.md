# Arrays, Structs and Enumerations

A `TYPE` block declares a type of your own. Everything in this chapter lives in such a block, or directly in the declaration of a variable.

```iecst
TYPE Point:
    STRUCT
        x, y: DINT;
    END_STRUCT
END_TYPE
```


## Arrays

An array holds a fixed number of elements of one type. You write the range of the index, not the count:

```iecst
VAR
    samples: ARRAY[1..3] OF DINT := [10, 20, 30];
    grid: ARRAY[0..1, 0..2] OF DINT := [1, 2, 3, 4, 5, 6];
END_VAR

samples[2] := 25;
grid[1, 0] := 7;
```

The range can start anywhere, so `ARRAY[1..3]` and `ARRAY[0..2]` both hold three elements. A comma adds a dimension. The initial values of such an array stay in one flat list, in which the last index changes fastest.

The bounds must be known while the program is compiled, so they are literals or constants:

```iecst
VAR CONSTANT
    COUNT: INT := 16;
END_VAR

VAR
    buffer: ARRAY[0..COUNT - 1] OF BYTE;
END_VAR
```

An index that is a constant outside the range is rejected. An index that is computed while the program runs is not checked, by the compiler or at run time. The compiler emits the address calculation with no test, so a wrong index reads or writes memory outside the array. Check the index yourself where the value comes from outside.

A function can take an array of any size. That form, `ARRAY[*]`, is explained with the other [parameter rules](functions.md#arrays-of-any-size).


## Structs

An array holds many values of one type. A struct groups a few values of different types, and every member has a name. The members are read and written through a dot:

```iecst
TYPE Motor:
    STRUCT
        speed: INT;
        running: BOOL;
        name: STRING[20];
    END_STRUCT
END_TYPE

PROGRAM Plant
    VAR
        pump: Motor := (speed := 100, running := FALSE, name := 'pump');
    END_VAR

    pump.speed := 120;
END_PROGRAM
```

Members lie in memory in the order of their declaration. A struct can hold another struct, an array, or an instance of a function block, and the dot chains: `plant.pump.speed`.

Assigning one struct to another copies every member.


## Enumerations

An array and a struct collect values. An enumeration instead lists the values that one variable may take. Each name stands for a number, counting from zero, and a name can set its own value, after which counting continues from there:

```iecst
TYPE State: (Idle, Running := 5, Stopped);   (* 0, 5, 6 *)
END_TYPE

PROGRAM Machine
    VAR
        current: State := Idle;
    END_VAR

    IF current = Running THEN
        (* ... *)
    END_IF
END_PROGRAM
```

The names are visible without the type in front of them. A name that two enumerations declare resolves to the enumeration that was declared first, and the compiler does not warn you. Give every variant a name of its own, and write the type in front of a name where you want to be explicit:

```iecst
current := State#Stopped;
```

An enumeration is an integer underneath, so it fits everywhere an integer fits, and `CASE` works on it.


## Subranges

An enumeration limits a variable to a list of names. A subrange limits an integer to a range of numbers:

```iecst
TYPE Percent: INT (0..100);
END_TYPE
```

The compiler does not enforce the range by itself. It enforces it when the project provides a check function, and then every assignment to such a variable goes through that function, which decides what happens:

```iecst
FUNCTION CheckRangeSigned: DINT
    VAR_INPUT
        value: DINT;
        lower: DINT;
        upper: DINT;
    END_VAR

    IF value < lower THEN
        CheckRangeSigned := lower;
    ELSIF value > upper THEN
        CheckRangeSigned := upper;
    ELSE
        CheckRangeSigned := value;
    END_IF
END_FUNCTION
```

With that function in the project, an assignment of `200` to a `Percent` stores `100`. `CheckRangeUnsigned` does the same for the unsigned types. Without such a function, a subrange behaves like the type it is based on.


## Aliases

A subrange adds a range to an existing type. A type declaration that adds nothing makes an alias, and the alias can carry an initial value:

```iecst
TYPE Signal: INT := -1;
END_TYPE

PROGRAM Reader
    VAR
        reading: Signal;   (* an INT that starts at -1 *)
    END_VAR
END_PROGRAM
```

Use an alias to give a meaning to a plain type, for example `TYPE Celsius: INT; END_TYPE`.


## What's next

You now have the types. The next chapter combines their values into [expressions](expressions.md).
