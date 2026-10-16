// RUN: %COMPILE %s && %RUN | %CHECK %s
// Scenario: `THIS^.foo()` stands for the actual instance, so it dispatches through the vtable exactly
// like the unqualified `foo()`. A child or grandchild override must run, also for property accessors,
// for calls with named arguments and outputs, and for the body of a base that a child runs via `SUPER^()`.

FUNCTION_BLOCK Dog
    VAR
        bodyResult : DINT;
        stored : DINT;
    END_VAR

    METHOD say : DINT
        say := 1;
    END_METHOD

    METHOD add : DINT
        VAR_INPUT
            a : DINT;
        END_VAR
        VAR_OUTPUT
            tag : DINT;
        END_VAR

        add := a + 10;
        tag := 1;
    END_METHOD

    PROPERTY_GET name : DINT
        name := 1;
    END_PROPERTY

    PROPERTY_SET level : DINT
        stored := level;
    END_PROPERTY

    METHOD plain : DINT
        plain := say();
    END_METHOD

    METHOD viaThis : DINT
        viaThis := THIS^.say();
    END_METHOD

    METHOD viaThisNamed : DINT
        VAR
            t : DINT;
        END_VAR

        viaThisNamed := THIS^.add(a := 5, tag => t) * 10 + t;
    END_METHOD

    METHOD viaThisProperty : DINT
        viaThisProperty := THIS^.name;
    END_METHOD

    METHOD setViaThis
        THIS^.level := 7;
    END_METHOD

    bodyResult := THIS^.say();
END_FUNCTION_BLOCK

FUNCTION_BLOCK Puppy EXTENDS Dog
    METHOD say : DINT
        say := 2;
    END_METHOD

    METHOD add : DINT
        VAR_INPUT
            a : DINT;
        END_VAR
        VAR_OUTPUT
            tag : DINT;
        END_VAR

        add := a + 20;
        tag := 2;
    END_METHOD

    PROPERTY_GET name : DINT
        name := 2;
    END_PROPERTY

    PROPERTY_SET level : DINT
        stored := level * 100;
    END_PROPERTY

    SUPER^();
END_FUNCTION_BLOCK

FUNCTION_BLOCK Pup EXTENDS Puppy
    METHOD say : DINT
        say := 3;
    END_METHOD

    SUPER^();
END_FUNCTION_BLOCK

FUNCTION main : DINT
    VAR
        d : Dog;
        p : Puppy;
        g : Pup;
        ref : POINTER TO Dog;
    END_VAR

    // CHECK: plain: 1 2 3
    printf('plain: %d %d %d$N', d.plain(), p.plain(), g.plain());

    // CHECK: viaThis: 1 2 3
    printf('viaThis: %d %d %d$N', d.viaThis(), p.viaThis(), g.viaThis());

    // CHECK: viaThisNamed: 151 252 252
    printf('viaThisNamed: %d %d %d$N', d.viaThisNamed(), p.viaThisNamed(), g.viaThisNamed());

    // CHECK: viaThisProperty: 1 2 2
    printf('viaThisProperty: %d %d %d$N', d.viaThisProperty(), p.viaThisProperty(), g.viaThisProperty());

    // CHECK: setViaThis: 7 700 700
    d.setViaThis();
    p.setViaThis();
    g.setViaThis();
    printf('setViaThis: %d %d %d$N', d.stored, p.stored, g.stored);

    // CHECK: viaPointer: 3
    ref := ADR(g);
    printf('viaPointer: %d$N', ref^.viaThis());

    // CHECK: body: 1 2 3
    d();
    p();
    g();
    printf('body: %d %d %d$N', d.bodyResult, p.bodyResult, g.bodyResult);
END_FUNCTION
