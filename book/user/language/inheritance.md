# Inheritance and Interfaces

Two mechanisms let one piece of code work with many types: a function block can build on another one, and it can promise to provide a set of methods.


## Extending a function block

`EXTENDS` takes everything from the base block, its members and its methods, and adds to it:

```iecst
FUNCTION_BLOCK Sensor
    VAR
        raw : DINT;
    END_VAR

    METHOD Read : DINT
        Read := raw;
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION_BLOCK ScaledSensor EXTENDS Sensor
    METHOD OVERRIDE Read : DINT
        Read := SUPER^.Read() * 10;
    END_METHOD
END_FUNCTION_BLOCK
```

`OVERRIDE` replaces a method of the base. `SUPER^` is the same instance seen as the base type, so `SUPER^.Read()` runs the method that was replaced. Without `SUPER^`, a call to `Read()` inside `ScaledSensor` would call the new one again.

The body of the base runs when the derived block runs, and the members of the base are members of the derived block.


## Dispatch

A call through a variable of the base type runs the method of the actual instance, not the one of the declared type:

```iecst
VAR
    scaled : ScaledSensor;
    any : REF_TO Sensor;
END_VAR

any := REF(scaled);
value := any^.Read();   (* the method of ScaledSensor *)
```

This is what makes a list of different sensors possible: they all have `Read`, and each one brings its own.


## Interfaces

An interface is a set of method declarations without bodies. A function block that names it in `IMPLEMENTS` must provide those methods:

```iecst
INTERFACE ISensor
    METHOD Read : DINT
    END_METHOD
END_INTERFACE

FUNCTION_BLOCK Analog IMPLEMENTS ISensor
    VAR
        raw : DINT;
    END_VAR

    METHOD Read : DINT
        Read := raw;
    END_METHOD
END_FUNCTION_BLOCK
```

A variable of the interface type holds any instance that implements it, and a call through it reaches the instance:

```iecst
VAR
    analog : Analog;
    sensor : ISensor;
END_VAR

sensor := analog;
value := sensor.Read();
```

A parameter of an interface type is the usual way to write code that works with every implementation:

```iecst
FUNCTION Report : DINT
    VAR_INPUT
        device : ISensor;
    END_VAR

    Report := device.Read();
END_FUNCTION
```

Use an interface when the implementations have nothing in common but their operations, and `EXTENDS` when they share data or behavior.


## Classes

A `CLASS` is a function block without a body. It holds members and methods, and a call reaches it only through one of its methods:

```iecst
CLASS Formatter
    VAR
        width : DINT;
    END_VAR

    METHOD Pad : DINT
        Pad := width;
    END_METHOD
END_CLASS
```

An implementation in a class is an error (`E017`), and `THIS` is not available there (`E120`). Everything else, `EXTENDS`, `IMPLEMENTS`, methods, and properties, works as in a function block. Use a class for a type that has no cyclic behavior of its own.


## Access modifiers

A member or a method can carry `PUBLIC`, `PRIVATE`, `PROTECTED`, or `INTERNAL`, which state who the declaration is meant for: everybody, the declaring POU, the declaring POU and the POUs that extend it, or the project:

```iecst
FUNCTION_BLOCK Box
    VAR PROTECTED
        state : DINT;
    END_VAR

    METHOD PUBLIC Open : DINT
        Open := state;
    END_METHOD
END_FUNCTION_BLOCK
```

A POU and a method can also carry `ABSTRACT`, which says that a derived POU provides the body, or `FINAL`, which says that no derived POU replaces it.


## What's next

One more mechanism writes code that works for many types, and it does so without instances: [generic functions](generics.md).
