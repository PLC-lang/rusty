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
