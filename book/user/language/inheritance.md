# Inheritance and Interfaces

Two mechanisms let one piece of code work with many types: a function block can build on another one, and it can promise to provide a set of methods.


## Extending a function block

`EXTENDS` takes everything from the base block, its members and its methods, and adds to it:

```iecst
FUNCTION_BLOCK Sensor
    VAR
        raw: DINT;
    END_VAR

    METHOD Read: DINT
        Read := raw;
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION_BLOCK ScaledSensor EXTENDS Sensor
    METHOD OVERRIDE Read: DINT
        Read := SUPER^.Read() * 10;
    END_METHOD
END_FUNCTION_BLOCK
```

`OVERRIDE` replaces a method of the base. `SUPER^` is the same instance seen as the base type, so `SUPER^.Read()` runs the method that was replaced. Without `SUPER^`, a call to `Read()` inside `ScaledSensor` reaches the new one.

The members of the base are members of the derived block, but the body is not. A call to the derived block runs its own body only. Write `SUPER^();` in that body to run the body of the base as well.


## Dispatch

A call through a variable of the base type runs the method of the actual instance, not the one of the declared type:

```iecst
VAR
    scaled: ScaledSensor;
    any: REF_TO Sensor;
    value: DINT;
END_VAR

any := ADR(scaled);
value := any^.Read();   (* the method of ScaledSensor *)
```

Take the address with `ADR`. `REF` gives a pointer to the exact type of its argument, so `any := REF(scaled)` warns that `REF_TO Sensor` and `ScaledSensor` are different types, although the call works.

This is what makes a list of different sensors possible: they all have `Read`, and each one brings its own.


## Interfaces

An interface is a set of method declarations without bodies. A function block that names it in `IMPLEMENTS` must provide those methods, and a missing one is rejected:

```iecst
INTERFACE ISensor
    METHOD Read: DINT
    END_METHOD
END_INTERFACE

FUNCTION_BLOCK Analog IMPLEMENTS ISensor
    VAR
        raw: DINT;
    END_VAR

    METHOD Read: DINT
        Read := raw;
    END_METHOD
END_FUNCTION_BLOCK
```

A variable of the interface type holds any instance that implements it, and a call through it reaches the instance:

```iecst
VAR
    analog: Analog;
    sensor: ISensor;
    value: DINT;
END_VAR

sensor := analog;
value := sensor.Read();
```

A parameter of an interface type is the usual way to write code that works with every implementation:

```iecst
FUNCTION Report: DINT
    VAR_INPUT
        device: ISensor;
    END_VAR

    Report := device.Read();
END_FUNCTION
```

Use an interface when the implementations have nothing in common but their operations, and `EXTENDS` when they share data or behavior.


## Classes

Neither mechanism is limited to the function block. A `CLASS` is a function block without a body. It holds members and methods, and you reach it through its methods:

```iecst
CLASS Formatter
    VAR
        width: DINT;
    END_VAR

    METHOD Pad: DINT
        Pad := width;
    END_METHOD
END_CLASS
```

A statement outside a method is rejected, because a class cannot have an implementation, and `THIS` is not available in a class either. Everything else, `EXTENDS`, `IMPLEMENTS`, methods, and properties, works as in a function block. Use a class for a type that has no cyclic behavior of its own.


## Access modifiers

A block that others extend often wants to say which of its declarations they may use. A member or a method can carry `PUBLIC`, `PRIVATE`, `PROTECTED`, or `INTERNAL` for that:

```iecst
FUNCTION_BLOCK Box
    VAR PROTECTED
        state: DINT;
    END_VAR

    METHOD PUBLIC Open: DINT
        Open := state;
    END_METHOD
END_FUNCTION_BLOCK
```

> [!WARNING]
> The compiler parses the four modifiers and then ignores them. They do not change what a caller may touch.

Every `VAR` member counts as private to the block that declares it, whatever the modifier says. A read of `state` from outside, and also from a block that extends `Box`, gets the warning `Illegal access to private member Box.state`. A modifier on a method changes nothing at all, so a `PRIVATE` method can be called from anywhere. The [variable block](function-blocks.md#what-the-outside-may-touch) decides what the outside may touch, not the modifier.

A POU and a method can also carry `ABSTRACT` or `FINAL`, and these two are parsed and ignored in the same way. A block declared `FINAL` can still be extended, a method declared `FINAL` can still be replaced, and a variable can take the type of a block declared `ABSTRACT`. A method declared `ABSTRACT` has no body, and a call to it returns the default value of its result type.


## What's next

One more mechanism writes code that works for many types, and it does so without instances: [generic functions](generics.md).
