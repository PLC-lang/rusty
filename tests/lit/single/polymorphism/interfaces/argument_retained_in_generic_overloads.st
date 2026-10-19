// RUN: %COMPILE %s && %RUN | %CHECK %s
// Scenario: Two generic overloads share a name. Each binds a concrete instance to a function block's
// REFERENCE TO interface input, so each needs its own retained fat pointer global. The globals must
// not share a name, even though neither overload is called.

INTERFACE IA
    METHOD foo
    END_METHOD
END_INTERFACE

FUNCTION_BLOCK FbA IMPLEMENTS IA
    METHOD foo
        printf('FbA::foo$N');
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION_BLOCK Consumer
    VAR_INPUT
        in1 : REFERENCE TO IA;
    END_VAR

    in1.foo();
END_FUNCTION_BLOCK

VAR_GLOBAL
    consumer : Consumer;
    instance : FbA;
END_VAR

FUNCTION bind<T: ANY_INT> : T
    VAR_INPUT
        value : T;
    END_VAR

    consumer(in1 := instance);
    bind := value;
END_FUNCTION

FUNCTION bind<U: ANY_REAL> : U
    VAR_INPUT
        value : U;
    END_VAR

    consumer(in1 := instance);
    bind := value;
END_FUNCTION

FUNCTION main
    // CHECK: main: done
    printf('main: done$N');
END_FUNCTION
