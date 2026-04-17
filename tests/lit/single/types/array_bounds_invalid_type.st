// RUN: not %COMPILE %s
// Non-integer range bounds must be rejected during validation with a proper
// E008 diagnostic, not crash codegen with an internal "Cannot extract int
// constant" error or a panic.

PROGRAM mainProg
VAR
    bo  : BOOL := FALSE;
    arr : ARRAY[FALSE .. TRUE] OF BOOL;
END_VAR
    arr[false] := bo;
END_PROGRAM
