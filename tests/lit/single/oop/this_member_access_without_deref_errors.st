// RUN: rm -f %T/%basename_t.out
// RUN: not %COMPILE --error-format clang %s 2>&1 | %CHECK %s
// RUN: test ! -e %T/%basename_t.out

// Regression test for #1728: `THIS.<member>` without an explicit `^` deref
// must be rejected by validation, not silently passed through to codegen
// (where it used to crash with "GEP pointee is not a struct").

// CHECK: {{.*}}this_member_access_without_deref_errors.st:{{[0-9]+}}:{{[0-9]+}}:{{.*}}error[E141]: Cannot access `a` on `POINTER TO fb`; dereference with `^` first
// CHECK: Compilation aborted due to critical errors

INTERFACE IAnimal
    METHOD legs : DINT
    END_METHOD
END_INTERFACE

FUNCTION_BLOCK fb
VAR
    a : IAnimal;
END_VAR
METHOD m : DINT
    THIS.a.legs();
END_METHOD
END_FUNCTION_BLOCK
