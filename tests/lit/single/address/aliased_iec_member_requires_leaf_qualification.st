// RUN: not %COMPILE %s 2>&1 | %CHECK %s
// Regression coverage for addressed aggregate members: `bToggle1` is the addressed wrapper,
// so using it as a BOOL without `.out` should fail validation.

// CHECK: {{.*Unary `NOT` requires a bit or integer operand, got `PI_T_mainTask_bToggle1`.*}}
// CHECK: {{.*Compilation aborted due to critical errors.}}

TYPE PI_T_mainTask_bToggle1 : STRUCT
    out : BOOL;
END_STRUCT
END_TYPE

TYPE PI_T_mainTask : STRUCT
    bToggle1 AT %QX1.2.3.4 : PI_T_mainTask_bToggle1;
END_STRUCT
END_TYPE

VAR_GLOBAL
    mainTask_pi : PI_T_mainTask;
END_VAR

PROGRAM mainProg
VAR
    cycleCnt : UDINT := 1;
END_VAR

cycleCnt := cycleCnt + 1;
mainTask_pi.bToggle1 := NOT mainTask_pi.bToggle1;

END_PROGRAM

FUNCTION main : DINT
    mainProg();
END_FUNCTION
