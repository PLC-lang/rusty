// RUN: (%COMPILE %s && %RUN) | %CHECK %s

FUNCTION_BLOCK F1
VAR
    a: INT;
    b: DINT;
END_VAR
END_FUNCTION_BLOCK

PROGRAM mainProg
VAR
    myInt: INT;
    myRefInt: REF_TO INT := REF(myInt);
    myF1: F1;
    myRefF1: REF_TO F1 := REF(myF1);
END_VAR
END_PROGRAM

FUNCTION main
VAR
    tmpInt: INT;
    tmpA: INT;
    tmpB: DINT;
END_VAR
    // Initialize through direct variables
    mainProg.myInt := 11;
    mainProg.myF1.a := 22;
    mainProg.myF1.b := 33;

    // Read through references
    tmpInt := mainProg.myRefInt^;
    tmpA := mainProg.myRefF1^.a;
    tmpB := mainProg.myRefF1^.b;

    printf('ref_read_int=%d$N', tmpInt); // CHECK: ref_read_int=11
    printf('ref_read_f1_a=%d$N', tmpA); // CHECK: ref_read_f1_a=22
    printf('ref_read_f1_b=%d$N', tmpB); // CHECK: ref_read_f1_b=33

    // Write through references
    mainProg.myRefInt^ := 44;
    mainProg.myRefF1^.a := 55;
    mainProg.myRefF1^.b := 66;

    printf('direct_read_int=%d$N', mainProg.myInt); // CHECK: direct_read_int=44
    printf('direct_read_f1_a=%d$N', mainProg.myF1.a); // CHECK: direct_read_f1_a=55
    printf('direct_read_f1_b=%d$N', mainProg.myF1.b); // CHECK: direct_read_f1_b=66
END_FUNCTION
