// RUN: %COMPILE %s 2> %t.err
// RUN: test ! -s %t.err
// RUN: %RUN

FUNCTION_BLOCK foo
    VAR
        value : __VOID;
    END_VAR
END_FUNCTION_BLOCK

FUNCTION main : DINT
    VAR
        instance : foo;
    END_VAR

    main := 0;
END_FUNCTION
