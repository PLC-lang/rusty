// RUN: (%COMPILE %s && %RUN) | %CHECK %s

VAR_GLOBAL
    ps: INT := 42;
END_VAR

PROGRAM mainProg
VAR
    fb: foo := (s1 := REF(ps));
END_VAR
END_PROGRAM

FUNCTION_BLOCK foo
VAR
    default_target: INT := 11;
END_VAR
VAR_INPUT
    s1: REF_TO INT := REF(default_target);
END_VAR

METHOD get_value : INT
    get_value := s1^;
END_METHOD

METHOD set_value
VAR_INPUT
    v: INT;
END_VAR
    s1^ := v;
END_METHOD
END_FUNCTION_BLOCK

FUNCTION main
VAR
    out_val: INT;
END_VAR
    out_val := mainProg.fb.get_value();
    printf('read_from_ref=%d$N', out_val); // CHECK: read_from_ref=42

    mainProg.fb.set_value(v := 77);
    printf('write_via_ref=%d$N', ps); // CHECK: write_via_ref=77
END_FUNCTION
