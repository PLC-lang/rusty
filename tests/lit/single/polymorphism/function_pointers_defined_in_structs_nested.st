// RUN: (%COMPILE %s && %RUN) | %CHECK %s

TYPE FnCollection:
    STRUCT
        levelOne : LevelOne;
    END_STRUCT
END_TYPE

TYPE LevelOne:
    STRUCT
        fooPtr : REF_TO foo := REF(foo);
        levelTwo : LevelTwo;
    END_STRUCT
END_TYPE

TYPE LevelTwo:
    STRUCT
        barPtr : REF_TO bar := REF(bar);
        levelThree : LevelThree;
    END_STRUCT
END_TYPE

TYPE LevelThree:
    STRUCT
        bazPtr : REF_TO baz := REF(baz);
    END_STRUCT
END_TYPE

FUNCTION foo
    printf('Hello from foo$N');
END_FUNCTION

FUNCTION bar : DINT
    VAR_INPUT
        in : REAL;
    END_VAR

    printf('Hello from bar (in = %f)$N', in);
    bar := 12345;
END_FUNCTION

FUNCTION baz : DINT
    VAR_INPUT
        in : REAL;
    END_VAR

    VAR_OUTPUT
        out : DINT;
    END_VAR

    out := 12345;
    printf('Hello from baz (in = %f)$N', in);
END_FUNCTION

FUNCTION main
    VAR
        fnCollectionInstance : FnCollection;
        outVar : DINT;
    END_VAR

    fnCollectionInstance.levelOne.fooPtr^(); // CHECK: Hello from foo
    fnCollectionInstance.levelOne.levelTwo.barPtr^(3.14); // CHECK: Hello from bar (in = 3.140000)
    fnCollectionInstance.levelOne.levelTwo.levelThree.bazPtr^(2.71, outVar); // CHECK: Hello from baz (in = 2.710000)

    printf('out from baz: %d$N', outVar); // CHECK: out from baz: 12345
END_FUNCTION