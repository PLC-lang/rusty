// RUN: not %COMPILE %s --error-format clang 2>&1 | %CHECK %s
// A type with no TO_STRING monomorphization must produce a clean compile error, not a silent
// mis-dispatch to TO_STRING__STRING that reinterprets the argument's bytes and crashes at runtime.
// A user STRUCT never gets a monomorph, so this stays valid as follow-up PRs add the remaining
// scalar conversions (BOOL/SINT/INT/WORD).
TYPE MyStruct : STRUCT x : DINT; END_STRUCT END_TYPE

FUNCTION main : DINT
VAR
    st : MyStruct;
    s : STRING;
END_VAR
    // CHECK: {{.*}}error[E048]: Could not resolve reference to TO_STRING__MyStruct
    s := TO_STRING(st);
END_FUNCTION
