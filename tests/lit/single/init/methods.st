// RUN: (%COMPILE %s && %RUN) | %CHECK %s
VAR_GLOBAL CONSTANT
    g2 : STRING := 'Whats up';
    g1 : Transaction := (id := 5, amount := 10, message := g2);
END_VAR

VAR_GLOBAL 
    g0 : DINT := 5;
END_VAR

TYPE Transaction : STRUCT
    id      : DINT;
    amount  : DINT;
    message : STRING;
END_STRUCT END_TYPE

FUNCTION_BLOCK fb0
VAR
    s : STRING := 'hello from foo';
END_VAR
    METHOD fbMethod
        VAR
            local_s AT s: STRING;
        END_VAR
        VAR_TEMP
            local_s2 AT local_s: STRING;
            ptr : REF_TO STRING := REF(local_s2);
        END_VAR
            // CHECK: hello from foo
            printf('%s$N', ptr);
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION_BLOCK fb1
    VAR
        foo : DINT := 5;
    END_VAR

    METHOD fbMethod
        VAR
            bar : REFERENCE TO DINT := REF(foo);
        END_VAR

        printf('%d$N', bar); // CHECK: 5
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION_BLOCK fb2
    VAR
        foo AT g0 : DINT;
    END_VAR

    METHOD fbMethod
        VAR
            bar : REFERENCE TO DINT := REF(baz);
            baz : REFERENCE TO DINT := REF(foo);
        END_VAR

        printf('%d$N', baz); // CHECK: 5
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION_BLOCK fb3
    VAR
        foo : DINT := 5;
    END_VAR

    METHOD fbMethod
        VAR
            bar : REFERENCE TO DINT := REF(baz);
            baz : REFERENCE TO DINT := REF(foo);
        END_VAR

        printf('%d$N', baz); // CHECK: 5
    END_METHOD
END_FUNCTION_BLOCK


FUNCTION_BLOCK fb4
    VAR
        foo : Transaction := (id := 5, amount := 10, message := 'Whats up');
    END_VAR

    METHOD fbMethod
        VAR
            bar : REFERENCE TO Transaction := REF(baz);
            baz : REFERENCE TO Transaction := REF(foo);
        END_VAR

        printf('%d$N', foo.id);         // CHECK: 5
        printf('%d$N', foo.amount);     // CHECK: 10
        printf('%s$N', REF(foo.message));    // CHECK: Whats up
    END_METHOD
END_FUNCTION_BLOCK


FUNCTION_BLOCK fb5
    METHOD fbMethod
        VAR
            bar : REFERENCE TO DINT := REF(g0);
        END_VAR

        printf('%d$N', bar); // CHECK: 5
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION_BLOCK fb6
    METHOD fbMethod
        VAR
            bar : REFERENCE TO Transaction := REF(g1);
        END_VAR

        printf('%d$N', g1.id);         // CHECK: 5
        printf('%d$N', g1.amount);     // CHECK: 10
        printf('%s$N', REF(g1.message));    // CHECK: Whats up
    END_METHOD
END_FUNCTION_BLOCK

PROGRAM prog
    VAR 
        fb0: fb0;
        fb1: fb1;
        fb2: fb2;
        fb3: fb3;
    END_VAR
    METHOD progMethod
        VAR 
            fb4: fb4;
            fb5: fb5;
            fb6: fb6;
        END_VAR 
            fb0.fbMethod();
            fb1.fbMethod();
            fb2.fbMethod();
            fb3.fbMethod();
            fb4.fbMethod();
            fb5.fbMethod();
            fb6.fbMethod();
    END_METHOD
    progMethod();
END_PROGRAM

FUNCTION main : DINT 
VAR 
    fb0: fb0;
    fb1: fb1;
    fb2: fb2;
    fb3: fb3;
    fb4: fb4;
    fb5: fb5;
    fb6: fb6;
END_VAR 
    fb0.fbMethod();
    fb1.fbMethod();
    fb2.fbMethod();
    fb3.fbMethod();
    fb4.fbMethod();
    fb5.fbMethod();
    fb6.fbMethod();
    prog();
END_FUNCTION