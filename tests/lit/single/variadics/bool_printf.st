// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Verifies that BOOL values and boolean expressions are correctly promoted to
// 32-bit integers when passed as variadic arguments, matching C default argument
// promotion rules. Without this promotion the values can appear as 255/254 at
// -Onone because the raw i1/i8 is misread by printf.

PROGRAM mainProg
VAR
    vTrue  : BOOL := TRUE;
    vFalse : BOOL := FALSE;
END_VAR

    // Literals
    printf('FALSE: %u$N', FALSE);           // CHECK: FALSE: 0
    printf('TRUE: %u$N', TRUE);             // CHECK: TRUE: 1

    // NOT of literals
    printf('NOT FALSE: %u$N', NOT FALSE);   // CHECK: NOT FALSE: 1
    printf('NOT TRUE: %u$N', NOT TRUE);     // CHECK: NOT TRUE: 0

    // AND expressions
    printf('TRUE AND FALSE: %u$N', TRUE AND FALSE);   // CHECK: TRUE AND FALSE: 0
    printf('TRUE AND TRUE: %u$N', TRUE AND TRUE);     // CHECK: TRUE AND TRUE: 1

    // NOT of AND expressions
    printf('NOT (TRUE AND FALSE): %u$N', NOT (TRUE AND FALSE));  // CHECK: NOT (TRUE AND FALSE): 1
    printf('NOT (TRUE AND TRUE): %u$N', NOT (TRUE AND TRUE));    // CHECK: NOT (TRUE AND TRUE): 0

    // OR expressions
    printf('FALSE OR FALSE: %u$N', FALSE OR FALSE);  // CHECK: FALSE OR FALSE: 0
    printf('TRUE OR FALSE: %u$N', TRUE OR FALSE);    // CHECK: TRUE OR FALSE: 1

    // Variables
    printf('vFalse: %u$N', vFalse);        // CHECK: vFalse: 0
    printf('vTrue: %u$N', vTrue);          // CHECK: vTrue: 1

    // NOT of variables
    printf('NOT vFalse: %u$N', NOT vFalse); // CHECK: NOT vFalse: 1
    printf('NOT vTrue: %u$N', NOT vTrue);   // CHECK: NOT vTrue: 0

    // NOT of variable AND expressions
    printf('NOT (vTrue AND vFalse): %u$N', NOT (vTrue AND vFalse)); // CHECK: NOT (vTrue AND vFalse): 1
    printf('NOT (vTrue AND vTrue): %u$N', NOT (vTrue AND vTrue));   // CHECK: NOT (vTrue AND vTrue): 0

END_PROGRAM

FUNCTION main : DINT
    mainProg();
END_FUNCTION
