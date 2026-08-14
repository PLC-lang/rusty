// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Positive coverage for using the qualified leaf as an operand in a larger BOOL flow.

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
    toggled : BOOL;
END_VAR

    mainTask_pi.bToggle1.out := FALSE;
    toggled := NOT mainTask_pi.bToggle1.out;
    mainTask_pi.bToggle1.out := toggled;
    printf('%u$N', mainTask_pi.bToggle1.out); // CHECK: 1
END_PROGRAM

FUNCTION main : DINT
    mainProg();
END_FUNCTION
