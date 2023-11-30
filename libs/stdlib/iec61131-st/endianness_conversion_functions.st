(******************************************************************************
Description: Converts data to big endian format from native endian format.
Input:
  - in: The input value in native endian format.
Return: The value converted to big endian format.
******************************************************************************)
{external}
FUNCTION TO_BIG_ENDIAN<T: ANY> : T
VAR_INPUT
    in : T;
END_VAR
END_FUNCTION

(******************************************************************************
Description: Converts data to little endian format from big endian format.
Input:
  - in: The input value in native endian format.
Return: The value converted to little endian format.
******************************************************************************)
{external}
FUNCTION TO_LITTLE_ENDIAN<T: ANY> : T
VAR_INPUT
    in : T;
END_VAR
END_FUNCTION

(******************************************************************************
Description: Converts data from big endian format to native endian format.
Input:
  - in: The input value in big endian format.
Return: The value converted to native endian format.
******************************************************************************)
{external}
FUNCTION FROM_BIG_ENDIAN<T: ANY> : T
VAR_INPUT
    in : T;
END_VAR
END_FUNCTION

(******************************************************************************
Description: Converts data from little endian format to native endian format.
Input:
  - in: The input value in little endian format.
Return: The value converted to native endian format.
******************************************************************************)
{external}
FUNCTION FROM_LITTLE_ENDIAN<T: ANY> : T
VAR_INPUT
    in : T;
END_VAR
END_FUNCTION
