# Function Blocks and Programs

A function block is a type with memory. You declare it once, and every variable of that type is an instance with its own data, which survives between calls. That is what a controller needs: a filter keeps its last value, a counter keeps its count, a timer keeps its start.

```iecst
FUNCTION_BLOCK Counter
    VAR_INPUT
        step: DINT := 1;
    END_VAR
    VAR_OUTPUT
        total: DINT;
    END_VAR
    VAR
        calls: DINT;
    END_VAR

    calls := calls + 1;
    total := total + step;
END_FUNCTION_BLOCK
```


## Instances

An instance is declared like any other variable, and it is called by its own name:

```iecst
VAR
    fast: Counter;
    slow: Counter;
    value: DINT;
END_VAR

fast(step := 2);
fast(step := 2);
slow(step := 1);
```

`fast.total` is now `4` and `slow.total` is `1`. The two instances share the code and nothing else.

A call passes as many inputs as you want to change. A parameter that the call does not name keeps the value from the declaration, or the value that the last call left in the instance. This is the opposite of a [function](functions.md), where every call supplies every parameter.

Read a result after the call, or take it out as part of the call with `=>`:

```iecst
fast(step := 2);
value := fast.total;

fast(step := 2, total => value);
```

An instance can live anywhere a variable can: in a POU, in a struct, in an array, or in `VAR_GLOBAL`. A function block that holds another function block builds a larger unit out of smaller ones:

```iecst
FUNCTION_BLOCK Axis
    VAR
        position: Counter;
        speed: Counter;
    END_VAR

    position(step := 5);
    speed(step := 1);
END_FUNCTION_BLOCK
```


## What the outside may touch

The variable blocks decide what a caller can do with an instance. A caller reads and writes an input, reads an output, and leaves a plain `VAR` alone:

```iecst
fast.step := 3;        (* fine, an input *)
value := fast.total;   (* fine, an output *)
fast.total := 0;       (* error[E037]: VAR_OUTPUT variables cannot be assigned outside of their scope. *)
fast.calls := 0;       (* warning[E049]: Illegal access to private member Counter.calls *)
```

The write to an output stops the build; the access to a `VAR` is a warning only. The compiler reports that warning for every `VAR`, whatever [access modifier](inheritance.md#access-modifiers) the block gives it.

So the inputs and the outputs are the interface of the block, and a `VAR` is its own business. When something else must reach such a value, give the block a [method or a property](methods-and-properties.md).


## Programs

A program is a function block whose instance the compiler creates itself. There is one instance, it is global, and it keeps its data between calls like every other instance:

```iecst
PROGRAM Plant
    VAR
        cycle: DINT;
        pumps: ARRAY[1..4] OF Counter;
    END_VAR

    cycle := cycle + 1;
END_PROGRAM
```

You call it by its own name, `Plant()`, because the instance has no name of its own. Use a program for the top level of an application, and a function block for everything that exists more than once.


## Actions

An action is a named piece of body that belongs to a POU and works on its data. It declares nothing of its own, which makes it a way to split a long body into parts that you can call separately:

```iecst
FUNCTION_BLOCK Valve
    VAR
        state: BOOL;
    END_VAR
END_FUNCTION_BLOCK

ACTION Valve.Open
    state := TRUE;
END_ACTION

ACTION Valve.Close
    state := FALSE;
END_ACTION
```

Call an action through the instance: `inlet.Open()`. Two other spellings exist. An `ACTIONS` container directly after the POU takes the name of that POU, and an `ACTIONS <name>` container names its POU and stands anywhere in the project:

```iecst
ACTIONS Valve
    ACTION Toggle
        state := NOT state;
    END_ACTION
END_ACTIONS
```


## Initialization

The initial values in the declaration apply to every instance. When an instance needs more than that, declare the method `FB_INIT`:

```iecst
FUNCTION_BLOCK Buffer
    VAR
        size: DINT;
        ready: BOOL;
    END_VAR

    METHOD FB_INIT
        size := 128;
        ready := TRUE;
    END_METHOD
END_FUNCTION_BLOCK
```

The compiler calls `FB_INIT` once for each instance, when that instance is created. For a global instance and for a member of a program, that is before the program starts. Declare the method without parameters and without a return type, because the compiler calls it with no arguments.


## What's next

A function block can do more than run a body. The next chapter gives it [methods and properties](methods-and-properties.md).
