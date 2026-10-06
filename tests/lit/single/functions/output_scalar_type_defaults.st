// RUN: %COMPILE %s && %RUN | %CHECK %s
// Outputs of an alias, subrange, enum, or struct type start every call at the default of their
// type, in a function and in a method alike. A scalar output needs no constructor call, so an
// alias of an elementary type must compile as an output.
TYPE MyInt : INT; END_TYPE
TYPE Preset : DINT := 20; END_TYPE
TYPE Pct : INT(0..100) := 50; END_TYPE
TYPE Color : (Red, Green := 5, Blue) := Green; END_TYPE
TYPE S : STRUCT
    a : INT := 7;
END_STRUCT END_TYPE

FUNCTION_BLOCK Fb
    METHOD m
        VAR_OUTPUT
            p : MyInt;
            q : Preset;
            r : Pct;
            c : Color;
            st : S;
        END_VAR
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION f
    VAR_OUTPUT
        p : MyInt;
        q : Preset;
        r : Pct;
        c : Color;
        st : S;
    END_VAR
END_FUNCTION

FUNCTION main
    VAR
        fb : Fb;
        a : MyInt;
        b : Preset;
        r : Pct;
        c : Color;
        st : S;
    END_VAR
    a := 1; b := 1; r := 1; c := Blue; st.a := 1;
    f(p => a, q => b, r => r, c => c, st => st);
    printf('function: a = %d, b = %d, r = %d, c = %d, st.a = %d$N', a, b, r, c, st.a);
    // CHECK: function: a = 0, b = 20, r = 50, c = 5, st.a = 7

    a := 1; b := 1; r := 1; c := Blue; st.a := 1;
    fb.m(p => a, q => b, r => r, c => c, st => st);
    printf('method: a = %d, b = %d, r = %d, c = %d, st.a = %d$N', a, b, r, c, st.a);
    // CHECK-NEXT: method: a = 0, b = 20, r = 50, c = 5, st.a = 7
END_FUNCTION
