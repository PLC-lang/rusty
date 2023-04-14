(******************************************************************************
Description: Validation function for ANY_REAL
Input:
  - IN: ANY_REAL
Return:
  - FALSE: If input is Not-a-Number(NaN) or infinite(+Inf, -Inf)
  - TRUE: Otherwise
******************************************************************************)
{external}
FUNCTION IS_VALID <T: ANY_REAL> : BOOL
VAR_INPUT
	IN : T;
END_VAR
END_FUNCTION

(******************************************************************************
Description: Validation function for ANY_BIT
Validate if the binary representation of the input
would result in a valid BCD representation
Input:
  - IN: ANY_BIT
Return:
  - TRUE: The value of each 4bits represented digit must be between 0-9
  - FALSE: If the value of any 4bits represented digit is higher than 9
******************************************************************************)
{external}
FUNCTION IS_VALID_BCD <T: ANY_BIT> : BOOL
VAR_INPUT
	IN : T;
END_VAR
END_FUNCTION