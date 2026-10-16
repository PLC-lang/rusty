// RUN: %COMPILE %s && %RUN | %CHECK %s
// Scenario: A method or body of one POU calls methods and properties of another function block through a
// `REFERENCE TO` or a `POINTER TO`. The call must dispatch through the virtual table of that instance, not
// of the enclosing POU, also when the enclosing POU has a method with the same name. This holds in function
// blocks, classes, and programs, and a derived instance behind the pointer runs its override.

FUNCTION_BLOCK fb
    VAR
        stored : INT;
    END_VAR

    PROPERTY_GET prop : INT
        prop := stored;
    END_PROPERTY

    PROPERTY_SET prop : INT
        stored := prop;
    END_PROPERTY

    METHOD foo : DINT
        foo := 42;
    END_METHOD

    METHOD add : DINT
        VAR_INPUT
            a : DINT;
        END_VAR

        add := a + 100;
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION_BLOCK fbDerived EXTENDS fb
    METHOD foo : DINT
        foo := 43;
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION_BLOCK holder
    VAR
        rfb : REFERENCE TO fb;
        pfb : POINTER TO fb;
        bodyResult : DINT;
    END_VAR

    METHOD a : DINT
        a := 1;
    END_METHOD

    METHOD foo : DINT
        foo := 3;
    END_METHOD

    METHOD run : DINT
        rfb.prop := rfb.prop + 1;
        pfb^.prop := pfb^.prop + 1;
        run := rfb.foo() * 1000 + pfb^.add(5) + foo();
    END_METHOD

    bodyResult := pfb^.foo();
END_FUNCTION_BLOCK

CLASS user
    VAR
        pfb : POINTER TO fb;
    END_VAR

    METHOD foo : DINT
        foo := 4;
    END_METHOD

    METHOD run : DINT
        run := pfb^.foo();
    END_METHOD
END_CLASS

PROGRAM mainProg
    VAR
        myfb : fb;
        rfb : REFERENCE TO fb;
        pfb : POINTER TO fb;
    END_VAR

    METHOD baz : INT
        myfb.prop := myfb.prop + 1;
        myfb.foo();

        rfb.prop := rfb.prop + 1;
        rfb.foo();

        pfb^.prop := pfb^.prop + 1;
        pfb^.foo();

        baz := myfb.prop;
    END_METHOD
END_PROGRAM

FUNCTION main : DINT
    VAR
        f : fb;
        d : fbDerived;
        h : holder;
        u : user;
    END_VAR

    // CHECK: holder: 42108 2
    h.rfb REF= f;
    h.pfb := ADR(f);
    printf('holder: %d %d$N', h.run(), f.prop);

    // CHECK: holder body: 43
    h.pfb := ADR(d);
    h();
    printf('holder body: %d$N', h.bodyResult);

    // CHECK: class: 43
    u.pfb := ADR(d);
    printf('class: %d$N', u.run());

    // CHECK: program: 3
    mainProg.rfb REF= mainProg.myfb;
    mainProg.pfb := ADR(mainProg.myfb);
    printf('program: %d$N', mainProg.baz());
END_FUNCTION
