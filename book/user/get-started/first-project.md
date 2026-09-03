# A First Project

You build something small but real: a plant with two tanks, in two files, driven by a project file.


## A function block

A function block is a piece of code with memory. You declare it once and use it as often as you want, and every use has its own data.

Write `src/tank.st`:

```iecst
FUNCTION_BLOCK Tank
    VAR_INPUT
        inflow : DINT;
    END_VAR
    VAR_OUTPUT
        level : DINT;
        full : BOOL;
    END_VAR
    VAR CONSTANT
        CAPACITY : DINT := 10;
    END_VAR

    level := level + inflow;
    IF level >= CAPACITY THEN
        level := CAPACITY;
        full := TRUE;
    END_IF
END_FUNCTION_BLOCK
```

`VAR_INPUT` is what the caller gives, `VAR_OUTPUT` is what the caller reads back, and `VAR CONSTANT` is a value that nobody can write.


## A program that uses it

A program is a function block with exactly one instance, which makes it the natural place for the state of the plant.

Write `src/main.st`:

```iecst
{external}
FUNCTION printf : DINT
    VAR_INPUT {ref}
        format : STRING;
    END_VAR
    VAR_INPUT
        args : ...;
    END_VAR
END_FUNCTION

PROGRAM Plant
    VAR
        left  : Tank;
        right : Tank;
        cycle : DINT;
    END_VAR

    cycle := cycle + 1;
    left(inflow := 4);
    right(inflow := 7);
    printf('cycle %d: left=%d right=%d full=%d$N', cycle, left.level, right.level, right.full);
END_PROGRAM

FUNCTION main : DINT
    Plant();
    Plant();
END_FUNCTION
```

`left` and `right` are two instances of the same function block. `left(inflow := 4)` calls the instance and gives its input a value. After the call, `left.level` reads its output.

The two files know each other without an import. Everything that a project declares is visible in the whole project.


## The project file

Put the inputs and the kind of artifact into `plc.json`, next to the `src` directory:

```json
{
    "name": "tank",
    "files": [ "src/*.st" ],
    "compile_type": "Static"
}
```

`plc build` reads that file:

```bash
plc build --linker=cc
./build/tank.out
```

```
cycle 1: left=4 right=7 full=0
cycle 2: left=8 right=10 full=1
```

The output shows what a function block is for. Each instance kept its own level between the two cycles, and the right tank reached its capacity in the second one.

The build wrote everything into `build/`: one object file per source file, and the artifact `tank.out`, which is named after the project because the file sets no `output`.


## What's next

You can write, build, and run a project. The [Language Guide](../language/README.md) starts at the beginning and explains the language itself, from the shape of a source file to interfaces and generics.
