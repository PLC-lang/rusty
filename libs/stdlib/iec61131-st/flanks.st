(******************************************************************************
Description: Rising Edge detection
Input:
  - CLK:  Signal to detect 
Output:
  - Q:  Edge detected 
Return: Output variable are used for return.
******************************************************************************)
{external} 
FUNCTION_BLOCK R_TRIG
VAR_INPUT
	CLK: BOOL;
END_VAR
VAR_OUTPUT
	Q: BOOL;
END_VAR
VAR
	M: BOOL;	
END_VAR
END_FUNCTION_BLOCK

(******************************************************************************
Description: Falling Edge detection
Input:
  - CLK:  Signal to detect 
Output:
  - Q:  Edge detected 
Return: Output variable is used for return.
******************************************************************************)
{external} 
FUNCTION_BLOCK F_TRIG
VAR_INPUT
	CLK: BOOL;
END_VAR
VAR_OUTPUT
	Q: BOOL;
END_VAR
VAR
	M: BOOL;	
END_VAR
END_FUNCTION_BLOCK