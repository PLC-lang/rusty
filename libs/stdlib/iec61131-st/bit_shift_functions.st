(**************************
 * 
 * SHL(IN, n)
 * 
 * This operator implements a bitwise shift of an operand to the left.
 * IN is shifted by n bit to the left and is filled from the right with zeros.
 * 
*************************)
{external}
FUNCTION SHL<T: ANY_BIT> : T
VAR_INPUT
	IN : T;
	n : UDINT;
END_VAR
END_FUNCTION

(**************************
 * 
 * SHR(IN, n)
 * 
 * This operator implements a bitwise shift of an operand to the right.
 * IN is shifted by n bit to the right. 
 * If an unsigned data type is used, filling from the left with zeros ensues. 
 * In the case of signed data types, an arithmetic shifting is implemented, 
 * i.e. it is filled with the value of the highest bit.
 * 
*************************)
{external}
FUNCTION SHR<T: ANY_BIT> : T
VAR_INPUT
	IN : T;
	n : UDINT;
END_VAR
END_FUNCTION

(**************************
 * 
 * ROL(IN, n)
 * 
 * This operator implements a bitwise rotation of an operand to the left.
 * IN is shifted n-times to the left, 
 * whereby the highest bit from the right is again inserted.
 * 
*************************)
{external}
FUNCTION ROL<T : ANY_BIT> : T 
VAR_INPUT
	IN : T;
	n : UDINT;
END_VAR
END_FUNCTION

(**************************
 * 
 * ROR(IN, n)
 * 
 * This operator implements a bitwise rotation of an operand to the right.
 * IN is shifted n-times to the right, 
 * whereby the lowest bit from the left is again inserted.
 * 
*************************)
{external}
FUNCTION ROR<T: ANY_BIT> : T 
VAR_INPUT
	IN : T; 
	n : UDINT;
END_VAR
END_FUNCTION

