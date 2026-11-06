(**************************
 *
 * MAX(IN0, IN1, ...)
 *
 * This operator produces the larger value of the given operands.
 *
*************************)
{external}
FUNCTION MAX<T: ANY_ELEMENTARY> : T
VAR_INPUT
    IN : {sized} T...;
END_VAR
END_FUNCTION

(**************************
 *
 * MIN(IN0, IN1, ...)
 *
 * This operator produces the smaller value of the given operands.
 *
*************************)
{external}
FUNCTION MIN<T: ANY_ELEMENTARY> : T
VAR_INPUT
    IN : {sized} T...;
END_VAR
END_FUNCTION

(**************************
 *
 * LIMIT(MN, IN, MX)
 *
 * This operator limits an input value to the range between MN and MX.
 * If IN lies between the range of MN and MX, IN is displayed as a result.
 * If IN > MX, MX is displayed as a result.
 * If IN < MN, MN is displayed as a result.
 * The input names MN, IN and MX are those of IEC 61131-3 (table 31), so
 * that a formal call LIMIT(MN := a, IN := b, MX := c) resolves.
 *
*************************)
{external}
FUNCTION LIMIT<T: ANY_ELEMENTARY> : T
VAR_INPUT
    MN : T;
    IN : T;
    MX : T;
END_VAR
END_FUNCTION
