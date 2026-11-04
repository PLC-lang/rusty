// RUN: %COMPILE %s && %RUN | %CHECK %s
// Scenario: interface variables passed to by-reference parameters (VAR_IN_OUT, VAR_OUTPUT,
// REFERENCE TO) bind the caller's fat pointer: the callee reads through it and its writes reach
// the caller, for plain variables, array elements, struct members, forwarded parameters, function
// block parameters and methods called through an interface.

INTERFACE IA
    METHOD foo : DINT
    END_METHOD
END_INTERFACE

FUNCTION_BLOCK FbA IMPLEMENTS IA
    VAR_INPUT
        v : DINT;
    END_VAR
    METHOD foo : DINT
        foo := v;
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION_BLOCK FbA2 EXTENDS FbA
    METHOD foo : DINT
        foo := v * 10;
    END_METHOD
END_FUNCTION_BLOCK

TYPE Holder : STRUCT
    r : IA;
END_STRUCT END_TYPE

VAR_GLOBAL
    gA : FbA := (v := 7);
    gA2 : FbA2;
END_VAR

FUNCTION f_inout : DINT
    VAR_IN_OUT
        x : IA;
    END_VAR
    f_inout := x.foo();
    x := gA2;
END_FUNCTION

FUNCTION f_out : DINT
    VAR_OUTPUT
        x : IA;
    END_VAR
    x := gA;
END_FUNCTION

FUNCTION f_ref : DINT
    VAR_INPUT
        x : REFERENCE TO IA;
    END_VAR
    f_ref := x.foo();
END_FUNCTION

FUNCTION f_forward : DINT
    VAR_IN_OUT
        x : IA;
    END_VAR
    f_forward := f_inout(x);
END_FUNCTION

FUNCTION_BLOCK FbParams
    VAR_IN_OUT
        x : IA;
    END_VAR
    VAR_OUTPUT
        o : IA;
        r : DINT;
    END_VAR
    r := x.foo();
    x := gA2;
    o := gA;
END_FUNCTION_BLOCK

INTERFACE IM
    METHOD m : DINT
        VAR_IN_OUT
            x : IA;
        END_VAR
    END_METHOD
END_INTERFACE

FUNCTION_BLOCK FbM IMPLEMENTS IM
    METHOD m : DINT
        VAR_IN_OUT
            x : IA;
        END_VAR
        m := x.foo();
        x := gA;
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION main : DINT
    VAR
        a : FbA := (v := 1);
        ia : IA;
        arr : ARRAY[1..2] OF IA;
        h : Holder;
        io : IA;
        fb : FbParams;
        fbM : FbM;
        im : IM;
    END_VAR
    gA2.v := 8;

    ia := a;
    // CHECK: in-out read 1
    printf('in-out read %d$N', f_inout(ia));
    // CHECK-NEXT: in-out write 80
    printf('in-out write %d$N', ia.foo());

    ia := a;
    f_out(ia);
    // CHECK-NEXT: output positional 7
    printf('output positional %d$N', ia.foo());

    ia := a;
    f_out(x => ia);
    // CHECK-NEXT: output named 7
    printf('output named %d$N', ia.foo());

    // CHECK-NEXT: reference 7
    printf('reference %d$N', f_ref(ia));

    arr[2] := a;
    f_inout(arr[2]);
    // CHECK-NEXT: array element 80
    printf('array element %d$N', arr[2].foo());

    h.r := a;
    f_out(h.r);
    // CHECK-NEXT: struct member 7
    printf('struct member %d$N', h.r.foo());

    ia := a;
    // CHECK-NEXT: forwarded read 1
    printf('forwarded read %d$N', f_forward(ia));
    // CHECK-NEXT: forwarded write 80
    printf('forwarded write %d$N', ia.foo());

    ia := a;
    fb(x := ia, o => io);
    // CHECK-NEXT: function block in-out read 1, write 80, output 7
    printf('function block in-out read %d, write %d, output %d$N', fb.r, ia.foo(), io.foo());

    ia := a;
    im := fbM;
    // CHECK-NEXT: dispatched read 1
    printf('dispatched read %d$N', im.m(ia));
    // CHECK-NEXT: dispatched write 7
    printf('dispatched write %d$N', ia.foo());
END_FUNCTION
