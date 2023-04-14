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
 * LIMIT(Min, IN, Max)
 * 
 * This operator limits an input value to the range between Min and Max.
 * If IN lies between the range of Min and Max, IN is displayed as a result.
 * If IN > Max, Max is displayed as a result.
 * If IN < Min, Min is displayed as a result.
 * 
*************************)
{external}
FUNCTION LIMIT<T: ANY_ELEMENTARY> : T
VAR_INPUT
    MIN : T;
    IN : T;
    MAX : T;
END_VAR
END_FUNCTION