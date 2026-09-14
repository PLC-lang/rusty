# Methods and Properties

A function block can do more than run one body. A method adds an operation, a property adds a value that looks like a variable but runs code.


## Methods

A method is declared inside the function block, it has its own parameters and its own result, and it works on the data of the instance:

```iecst
FUNCTION_BLOCK Tank
    VAR
        level: DINT;
    END_VAR

    METHOD Fill: DINT
        VAR_INPUT
            amount: DINT;
        END_VAR

        level := level + amount;
        Fill := level;
    END_METHOD
END_FUNCTION_BLOCK
```

Call it through an instance:

```iecst
VAR
    inlet: Tank;
    value: DINT;
END_VAR

value := inlet.Fill(amount := 3);
```

A method follows the call rules of a [function](functions.md#calling): every parameter is supplied, by position or by name. The body of the block and its methods share the data of the instance.

Inside a method, `THIS^` names the instance itself. You need it when a parameter and a member have the same name, and it makes the intention clear in the other cases:

```iecst
METHOD SetLevel
    VAR_INPUT
        level: DINT;
    END_VAR

    THIS^.level := level;
END_METHOD
```

`THIS` works in a function block, in its methods, and in its actions. A `CLASS` does not have it, and the compiler reports `E120` there.


## Properties

A property is a value with code behind it. It has a getter, a setter, or both. Inside an accessor, the name of the property is the value, exactly as the name of a function is its result:

```iecst
FUNCTION_BLOCK Tank
    VAR
        level: DINT;
    END_VAR

    PROPERTY_GET Percent: DINT
        Percent := level * 10;
    END_PROPERTY

    PROPERTY_SET Percent: DINT
        level := Percent / 10;
    END_PROPERTY
END_FUNCTION_BLOCK
```

From outside, the property is used like a member, and the accessor runs:

```iecst
inlet.Percent := 50;      (* runs the setter, level becomes 5 *)
reading := inlet.Percent; (* runs the getter, 50 *)
```

A property with a getter only is read-only. A write to it is reported:

```
error[E048]: PROPERTY_SET for property `Percent` is not defined
```


## Method or property

Use a property when the caller thinks of the thing as a value: a level, a limit, a state. Use a method when the caller thinks of it as an action, when it needs arguments, or when it is expensive, because a property that hides work surprises the reader.


## What's next

The next chapter lets one function block build on another, with [inheritance and interfaces](inheritance.md).
