// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Positive coverage for the qualified leaf path: the addressed wrapper member itself is not a
// BOOL, but its `.out` leaf is. Toggling through `.out` must compile and run.

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
    mainTask_pi.bToggle1.out := FALSE;
    mainTask_pi.bToggle1.out := NOT mainTask_pi.bToggle1.out;
    printf('%u$N', mainTask_pi.bToggle1.out); // CHECK: 1

    mainTask_pi.bToggle1.out := NOT mainTask_pi.bToggle1.out;
    printf('%u$N', mainTask_pi.bToggle1.out); // CHECK: 0
END_PROGRAM

FUNCTION main : DINT
    mainProg();
END_FUNCTION
