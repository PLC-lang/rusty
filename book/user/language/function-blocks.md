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

    total := total + step;
END_FUNCTION_BLOCK
```


## Instances

An instance is declared like any other variable, and it is called by its own name:

```iecst
VAR
    fast: Counter;
    slow: Counter;
END_VAR

fast(step := 2);
fast(step := 2);
slow(step := 1);
```

`fast.total` is now `4` and `slow.total` is `1`. The two instances share the code and nothing else.

A call passes as many inputs as you want to change. A parameter that the call does not name keeps the value from the declaration, or the value that the last call left in the instance. This is the opposite of a [function](functions.md), where every call names every parameter.

Read a result after the call, or take it out during the call with `=>`:

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

The variable blocks decide what a caller can do with an instance:

| Block | From outside |
|---|---|
| `VAR_INPUT` | Read and write |
| `VAR_OUTPUT` | Read only. A write is an error (`E037`) |
| `VAR` | Neither. A read or a write is reported with `E049`, a warning |

```iecst
value := fast.total;   (* fine, an output *)
fast.total := 0;       (* error[E037]: VAR_OUTPUT variables cannot be assigned outside of their scope *)
fast.state := 0;       (* warning[E049]: Illegal access to private member Counter.state *)
```

So the inputs and the outputs are the interface of the block, and a plain `VAR` is its own business. When something else must reach such a value, give the block a [method or a property](methods-and-properties.md). Access modifiers are described in the [inheritance](inheritance.md#access-modifiers) chapter.


## Programs

A program is a function block with exactly one instance, and that instance is global:

```iecst
PROGRAM Plant
    VAR
        cycle: DINT;
        pumps: ARRAY[1..4] OF Counter;
    END_VAR

    cycle := cycle + 1;
END_PROGRAM
```

You call it by its own name, `Plant()`, because there is no other name to call. Use a program for the top level of an application, and a function block for everything that exists more than once.


## Actions

An action is a named piece of body that belongs to a POU and works on its data. It declares nothing of its own, which makes it a way to split a long body into parts that you can call separately:

```iecst
FUNCTION_BLOCK Valve
    VAR
        open: BOOL;
    END_VAR
END_FUNCTION_BLOCK

ACTION Valve.Open
    open := TRUE;
END_ACTION

ACTION Valve.Close
    open := FALSE;
END_ACTION
```

Call an action through the instance: `inlet.Open()`. Two other spellings exist, an `ACTIONS` container directly after the POU, and an `ACTIONS <name>` container anywhere in the project:

```iecst
ACTIONS Valve
    ACTION Toggle
        open := NOT open;
    END_ACTION
END_ACTIONS
```


## Initialization

The initial values in the declaration apply to every instance. When an instance needs more than that, declare the method `FB_INIT`. The compiler calls it once per instance, before the program starts:

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

`FB_INIT` takes no parameters and returns nothing.


## What's next

A function block can do more than run a body. The next chapter gives it [methods and properties](methods-and-properties.md).
