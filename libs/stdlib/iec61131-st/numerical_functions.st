(**************************
 * 
 *This operator returns the absolute value of a number.
 * 
*************************)
FUNCTION ABS<T : ANY_NUM> : T 
VAR_INPUT
	IN : T;
END_VAR

END_FUNCTION

(* Specialized implementation of ABS for DINT *)
/* FUNCTION ABS__DINT : DINT */
FUNCTION ABS__DINT : DINT
VAR_INPUT
	IN : DINT;
END_VAR
	ABS__DINT := IN;
	IF ABS__DINT < 0 THEN
		ABS__DINT := -1 * ABS__DINT;
	END_IF
END_FUNCTION

(* Specialized implementation of ABS for SINT *)
FUNCTION ABS__SINT : SINT
VAR_INPUT
	IN : SINT;
END_VAR
	ABS__SINT := ABS__DINT(IN);
END_FUNCTION

(* Specialized implementation of ABS for INT *)
FUNCTION ABS__INT : INT
VAR_INPUT
	IN : INT;
END_VAR
	ABS__INT := ABS__DINT(IN);
END_FUNCTION

(* Specialized implementation of ABS for LINT *)
FUNCTION ABS__LINT : LINT
VAR_INPUT
	IN : LINT;
END_VAR
	ABS__LINT := IN;
	IF ABS__LINT < 0 THEN
		ABS__LINT := -1 * ABS__LINT;
	END_IF
END_FUNCTION

(* Specialized implementation of ABS for REAL *)
FUNCTION ABS__REAL : REAL
VAR_INPUT
	IN : REAL;
END_VAR
	ABS__REAL := ABS__LREAL(IN);
END_FUNCTION

(* Specialized implementation of ABS for LREAL *)
FUNCTION ABS__LREAL : LREAL
VAR_INPUT
	IN : LREAL;
END_VAR
	ABS__LREAL := IN;
	IF ABS__LREAL < 0 THEN
		ABS__LREAL := -1.0 * ABS__LREAL;
	END_IF
END_FUNCTION

(**************************
 * 
 * Rounds a given ANY_REAL to the nearest integer
 * Rounds half values away from zero
 * 
*************************)
{external}
FUNCTION ROUND<T : ANY_REAL> : T 
VAR_INPUT
	in : T;
END_VAR
END_FUNCTION
