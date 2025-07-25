// RUN: (%COMPILE %s && %RUN) | %CHECK %s

TYPE FnCollection:
    STRUCT
        fooPtr : REF_TO foo := REF(foo);
        barPtr : REF_TO bar := REF(bar);
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

    fnCollectionInstance.fooPtr^(); // CHECK: Hello from foo
    fnCollectionInstance.barPtr^(3.14); // CHECK: Hello from bar (in = 3.140000)
    fnCollectionInstance.bazPtr^(2.71, outVar); // CHECK: Hello from baz (in = 2.710000)

    printf('out from baz: %d$N', outVar); // CHECK: out from baz: 12345
END_FUNCTION