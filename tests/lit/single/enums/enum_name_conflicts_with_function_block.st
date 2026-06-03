// RUN: (%COMPILE %s && %RUN) | %CHECK %s

TYPE
    eW : (fooFunc, fooBlock, fooMethod, MyAlias);
END_TYPE

TYPE
    MyAlias : INT;
END_TYPE

FUNCTION fooFunc
    // Does nothing
END_FUNCTION

FUNCTION_BLOCK fooBlock
    METHOD PUBLIC fooMethod
        // Does nothing
    END_METHOD
END_FUNCTION_BLOCK

PROGRAM mainProg
    VAR
        e : eW;
    END_VAR

    // Should be able to assign the enum element to the enum without ambiguity
    e := fooFunc;

    // CHECK: 0
    printf('%d$N', e);

    // Should be able to assign the enum element to the enum without ambiguity
    e := fooBlock;

    // CHECK: 1
    printf('%d$N', e);

    // Should be able to assign the enum element to the enum without ambiguity
    e := fooMethod;

    // CHECK: 2
    printf('%d$N', e);

    // Should be able to assign the enum element to the enum without ambiguity
    e := MyAlias;

    // CHECK: 3
    printf('%d$N', e);

END_PROGRAM

FUNCTION main
    mainProg();
END_FUNCTION
