(******************************************************************************
Description: Bistable function, set dominant
Input:
  - SET1: Dominant input of bistable function
  - RESET: Input of bistable function
Output:
  - Q1: Result of bistable function
Return: Output is used as return
******************************************************************************)
{external}
FUNCTION_BLOCK SR
VAR_INPUT
    SET1: BOOL;
    RESET: BOOL;
END_VAR
VAR_OUTPUT
    Q1: BOOL;
END_VAR
END_FUNCTION_BLOCK

(******************************************************************************
Description: Bistable function, reset dominant
Input:
  - SET: Input of bistable function
  - RESET1: Dominant input of bistable function
Output:
  - Q1: Result of bistable function
Return: Output is used as return
******************************************************************************)
{external}
FUNCTION_BLOCK RS
VAR_INPUT
    SET0:    BOOL;
    RESET1: BOOL;
END_VAR
VAR_OUTPUT
    Q1:     BOOL;
END_VAR
END_FUNCTION_BLOCK
