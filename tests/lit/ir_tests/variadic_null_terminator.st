// RUN: %COMPILE_IR %s | %CHECK_IR %s
//
// Calls to untyped C-style variadics (`args : ...`) carry an implicit null pointer as their
// last argument. The callee has no way of its own to know how many arguments it received, so
// APIs that walk the list until a terminator read past the arguments without it. Format- and
// count-directed callees read only the arguments they expect and never see the terminator.
//
// Typed variadics keep their argument list unchanged: a `{sized} T...` passes (count, array)
// and a typed unsized `T...` has a concrete callee signature, so a terminator would corrupt
// the contract in both cases.

{external}
FUNCTION untyped : DINT
VAR_INPUT
    fmt : STRING;
    args : ...;
END_VAR
END_FUNCTION

{external}
FUNCTION typed : DINT
VAR_INPUT
    count : DINT;
    args : DINT...;
END_VAR
END_FUNCTION

{external}
FUNCTION sized : DINT
VAR_INPUT
    args : {sized} DINT...;
END_VAR
END_FUNCTION

FUNCTION main : DINT
VAR
    text : STRING := 'text';
END_VAR
    // A call without any variadic argument is still terminated, which is the case that makes a
    // terminator-based callee read past the argument list.
    untyped(text);
    // CHECK: call i32 (ptr, ...) @untyped(ptr {{.*}}, ptr null)

    untyped(text, DINT#1, DINT#2);
    // CHECK: call i32 (ptr, ...) @untyped(ptr {{.*}}, i32 1, i32 2, ptr null)

    // The `, !dbg` suffix pins the end of the argument list, so an added terminator fails here.
    typed(DINT#2, DINT#1, DINT#2);
    // CHECK: call i32 (i32, ...) @typed(i32 2, i32 1, i32 2), !dbg

    sized(DINT#1, DINT#2, DINT#3);
    // CHECK: call i32 @sized(i32 3, ptr {{.*}}), !dbg
END_FUNCTION
