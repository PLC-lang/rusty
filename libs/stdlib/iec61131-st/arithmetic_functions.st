(* Definitions of arithmetic functions defined by the IEC61131-3 standard *)

{external}
VAR_GLOBAL
    PI_REAL : REAL;
    PI_LREAL : LREAL;
    FRAC_PI_2_REAL : REAL;
    FRAC_PI_2_LREAL : LREAL;
    FRAC_PI_4_REAL : REAL;
    FRAC_PI_4_LREAL : LREAL;
    E_REAL : REAL;
    E_LREAL : LREAL;
END_VAR

(********************
*
* This operator returns the value of adding up the operands.
* It overloads the variadic builtin implementation of ADD, which is implemented for ANY_NUM
*
*********************)
FUNCTION ADD < T1: ANY, T2: ANY >: T1
VAR_INPUT
    IN1: T1;
    IN2: T2...;
END_VAR
END_FUNCTION

(********************
*
* This operator produces the multiplication of the operands.
* It overloads the variadic builtin implementation of MUL, which is implemented for ANY_NUM
*
*********************)
FUNCTION MUL < T1: ANY, T2: ANY >: T1
VAR_INPUT
    IN1: T1;
    IN2: T2...;
END_VAR
END_FUNCTION

(* Calculates the square root of a given value *)
{external}
FUNCTION SQRT<T : ANY_REAL> : T
VAR_INPUT
    in : T;
END_VAR
END_FUNCTION

(** Logarithmic functions **)


(* Calculates the natural logarithm of the given value *)
{external}
FUNCTION LN<T : ANY_REAL> : T
VAR_INPUT
    in : T;
END_VAR
END_FUNCTION


(* Calculates the base 10 logarithm of the given value *)
{external}
FUNCTION LOG<T : ANY_REAL> : T
VAR_INPUT
    in : T;
END_VAR
END_FUNCTION

(* Calculates the natural exporenent of the given value *)
{external}
FUNCTION EXP<T : ANY_REAL> : T
VAR_INPUT
    in : T;
END_VAR
END_FUNCTION

(** Trigonometric functions **)


(* Calculates the sine of the given value in radiants *)
{external}
FUNCTION SIN<T : ANY_REAL> : T
VAR_INPUT
    rad : T;
END_VAR
END_FUNCTION

(* Calculates the cosine of the given value in radiants *)
{external}
FUNCTION COS<T : ANY_REAL> : T
VAR_INPUT
    rad : T;
END_VAR
END_FUNCTION

(* Calculates the tangent of the given value in radiants *)
{external}
FUNCTION TAN<T : ANY_REAL> : T
VAR_INPUT
    rad : T;
END_VAR
END_FUNCTION

(* Calculates the arc sine of the given value *)
{external}
FUNCTION ASIN<T : ANY_REAL> : T
VAR_INPUT
    in : T;
END_VAR
END_FUNCTION

(* Calculates the arc cosine of the given value *)
{external}
FUNCTION ACOS<T : ANY_REAL> : T
VAR_INPUT
    in : T;
END_VAR
END_FUNCTION

(* Calculates the arc tangent of the given value *)
{external}
FUNCTION ATAN<T : ANY_REAL> : T
VAR_INPUT
    in : T;
END_VAR
END_FUNCTION

(* Calculate the four quadrant arc tangent of the given values *)
{external}
FUNCTION ATAN2<T : ANY_REAL> : T
VAR_INPUT
    y : T;
    x : T;
END_VAR
END_FUNCTION

{external}
FUNCTION EXPT<T: ANY_REAL, U: ANY_NUM> : T
VAR_INPUT
    in1 : T;
    in2 : U;
END_VAR
END_FUNCTION

{external}
FUNCTION EXPT__REAL__DINT : REAL
VAR_INPUT
    in1 : REAL;
    in2 : DINT;
END_VAR
END_FUNCTION

{external}
FUNCTION EXPT__REAL__REAL : REAL
VAR_INPUT
    in1 : REAL;
    in2 : REAL;
END_VAR
END_FUNCTION

(* LREAL to REAL conversion will happen for the exponent
   This will produce the closest possible REAL value
   On overflow, infinity of the same sign as the input is produced *)
{external}
FUNCTION EXPT__REAL__LREAL : REAL
VAR_INPUT
    in1 : REAL;
    in2 : LREAL;
END_VAR
END_FUNCTION

{external}
FUNCTION EXPT__LREAL__DINT : LREAL
VAR_INPUT
    in1 : LREAL;
    in2 : DINT;
END_VAR
END_FUNCTION

{external}
FUNCTION EXPT__LREAL__REAL : LREAL
VAR_INPUT
    in1 : LREAL;
    in2 : REAL;
END_VAR
END_FUNCTION

{external}
FUNCTION EXPT__LREAL__LREAL : LREAL
VAR_INPUT
    in1 : LREAL;
    in2 : LREAL;
END_VAR
END_FUNCTION

FUNCTION EXPT__REAL__USINT : REAL
VAR_INPUT
    in1 : REAL;
    in2 : USINT;
END_VAR
    EXPT__REAL__USINT := EXPT__REAL__DINT(in1, in2);
END_FUNCTION

FUNCTION EXPT__REAL__UINT : REAL
VAR_INPUT
    in1 : REAL;
    in2 : UINT;
END_VAR
    EXPT__REAL__UINT := EXPT__REAL__DINT(in1, in2);
END_FUNCTION

FUNCTION EXPT__REAL__UDINT : REAL
VAR_INPUT
    in1 : REAL;
    in2 : UDINT;
END_VAR
    EXPT__REAL__UDINT := EXPT__REAL__DINT(in1, in2);
END_FUNCTION

FUNCTION EXPT__REAL__ULINT : REAL
VAR_INPUT
    in1 : REAL;
    in2 : ULINT;
END_VAR
    EXPT__REAL__ULINT := EXPT__REAL__DINT(in1, in2);
END_FUNCTION

FUNCTION EXPT__REAL__SINT : REAL
VAR_INPUT
    in1 : REAL;
    in2 : SINT;
END_VAR
    EXPT__REAL__SINT := EXPT__REAL__DINT(in1, in2);
END_FUNCTION

FUNCTION EXPT__REAL__INT : REAL
VAR_INPUT
    in1 : REAL;
    in2 : INT;
END_VAR
    EXPT__REAL__INT := EXPT__REAL__DINT(in1, in2);
END_FUNCTION

FUNCTION EXPT__REAL__LINT : REAL
VAR_INPUT
    in1 : REAL;
    in2 : LINT;
END_VAR
    EXPT__REAL__LINT := EXPT__REAL__DINT(in1, in2);
END_FUNCTION

FUNCTION EXPT__LREAL__USINT : LREAL
VAR_INPUT
    in1 : LREAL;
    in2 : USINT;
END_VAR
    EXPT__LREAL__USINT := EXPT__LREAL__DINT(in1, in2);
END_FUNCTION

FUNCTION EXPT__LREAL__UINT : LREAL
VAR_INPUT
    in1 : LREAL;
    in2 : UINT;
END_VAR
    EXPT__LREAL__UINT := EXPT__LREAL__DINT(in1, in2);
END_FUNCTION

FUNCTION EXPT__LREAL__UDINT : LREAL
VAR_INPUT
    in1 : LREAL;
    in2 : UDINT;
END_VAR
    EXPT__LREAL__UDINT := EXPT__LREAL__DINT(in1, in2);
END_FUNCTION

FUNCTION EXPT__LREAL__ULINT : LREAL
VAR_INPUT
    in1 : LREAL;
    in2 : ULINT;
END_VAR
    EXPT__LREAL__ULINT := EXPT__LREAL__DINT(in1, in2);
END_FUNCTION

FUNCTION EXPT__LREAL__SINT : LREAL
VAR_INPUT
    in1 : LREAL;
    in2 : SINT;
END_VAR
    EXPT__LREAL__SINT := EXPT__LREAL__DINT(in1, in2);
END_FUNCTION

FUNCTION EXPT__LREAL__INT : LREAL
VAR_INPUT
    in1 : LREAL;
    in2 : INT;
END_VAR
    EXPT__LREAL__INT := EXPT__LREAL__DINT(in1, in2);
END_FUNCTION

FUNCTION EXPT__LREAL__LINT : LREAL
VAR_INPUT
    in1 : LREAL;
    in2 : LINT;
END_VAR
    EXPT__LREAL__LINT := EXPT__LREAL__DINT(in1, in2);
END_FUNCTION
