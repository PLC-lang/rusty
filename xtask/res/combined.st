(********************
*
* Concatenates DATE and TOD to DT
*
******************** *)
{external}
FUNCTION CONCAT_DATE_TOD : DT
VAR_INPUT
    date_input : DATE;
    tod_input : TOD;
END_VAR
END_FUNCTION

(********************
*
* Concatenates DATE and LTOD to DT
*
******************** *)
FUNCTION CONCAT_DATE_LTOD : DT
VAR_INPUT
    date_input : DATE;
    tod_input : LTOD;
END_VAR
    CONCAT_DATE_LTOD := CONCAT_DATE_TOD(date_input, tod_input);
END_FUNCTION

(********************
*
* Concatenates year, month and day of type ANY_INT to DATE
*
******************** *)
{external}
FUNCTION CONCAT_DATE < T : ANY_INT > : DATE
VAR_INPUT
    year : T;
    month : T;
    day : T;
END_VAR
END_FUNCTION

(********************
*
* Concatenates hour, minute, second, millisecond of type ANY_INT to TOD
*
******************** *)
{external}
FUNCTION CONCAT_TOD < T : ANY_INT > : TOD
VAR_INPUT
    hour : T;
    minute : T;
    second : T;
    millisecond : T;
END_VAR
END_FUNCTION

(********************
*
* Concatenates hour, minute, second, millisecond of type ANY_INT to LTOD
*
******************** *)
FUNCTION CONCAT_LTOD < T : ANY_INT > : LTOD
VAR_INPUT
    hour : T;
    minute : T;
    second : T;
    millisecond : T;
END_VAR
END_FUNCTION

(* Specialized implementation of CONCAT_LTOD for SINT *)
FUNCTION CONCAT_LTOD__SINT : LTOD
VAR_INPUT
    hour : SINT;
    minute : SINT;
    second : SINT;
    millisecond : SINT;
END_VAR
    CONCAT_LTOD__SINT := CONCAT_TOD(hour, minute, second, millisecond);
END_FUNCTION

(* Specialized implementation of CONCAT_LTOD for USINT *)
FUNCTION CONCAT_LTOD__USINT : LTOD
VAR_INPUT
    hour : USINT;
    minute : USINT;
    second : USINT;
    millisecond : USINT;
END_VAR
    CONCAT_LTOD__USINT := CONCAT_TOD(hour, minute, second, millisecond);
END_FUNCTION

(* Specialized implementation of CONCAT_LTOD for INT *)
FUNCTION CONCAT_LTOD__INT : LTOD
VAR_INPUT
    hour : INT;
    minute : INT;
    second : INT;
    millisecond : INT;
END_VAR
    CONCAT_LTOD__INT := CONCAT_TOD(hour, minute, second, millisecond);
END_FUNCTION

(* Specialized implementation of CONCAT_LTOD for UINT *)
FUNCTION CONCAT_LTOD__UINT : LTOD
VAR_INPUT
    hour : UINT;
    minute : UINT;
    second : UINT;
    millisecond : UINT;
END_VAR
    CONCAT_LTOD__UINT := CONCAT_TOD(hour, minute, second, millisecond);
END_FUNCTION

(* Specialized implementation of CONCAT_LTOD for DINT *)
FUNCTION CONCAT_LTOD__DINT : LTOD
VAR_INPUT
    hour : DINT;
    minute : DINT;
    second : DINT;
    millisecond : DINT;
END_VAR
    CONCAT_LTOD__DINT := CONCAT_TOD(hour, minute, second, millisecond);
END_FUNCTION

(* Specialized implementation of CONCAT_LTOD for UDINT *)
FUNCTION CONCAT_LTOD__UDINT : LTOD
VAR_INPUT
    hour : UDINT;
    minute : UDINT;
    second : UDINT;
    millisecond : UDINT;
END_VAR
    CONCAT_LTOD__UDINT := CONCAT_TOD(hour, minute, second, millisecond);
END_FUNCTION

(* Specialized implementation of CONCAT_LTOD for LINT *)
FUNCTION CONCAT_LTOD__LINT : LTOD
VAR_INPUT
    hour : LINT;
    minute : LINT;
    second : LINT;
    millisecond : LINT;
END_VAR
    CONCAT_LTOD__LINT := CONCAT_TOD(hour, minute, second, millisecond);
END_FUNCTION

(* Specialized implementation of CONCAT_LTOD for ULINT *)
FUNCTION CONCAT_LTOD__ULINT : LTOD
VAR_INPUT
    hour : ULINT;
    minute : ULINT;
    second : ULINT;
    millisecond : ULINT;
END_VAR
    CONCAT_LTOD__ULINT := CONCAT_TOD(hour, minute, second, millisecond);
END_FUNCTION

(********************
*
* Concatenates year, month, day, hour, minute, second, millisecond of type ANY_INT to DT
*
******************** *)
FUNCTION CONCAT_DT < T : ANY_INT > : DT
VAR_INPUT
    year : T;
    month : T;
    day : T;
    hour : T;
    minute : T;
    second : T;
    millisecond : T;
END_VAR
END_FUNCTION

(* Specialized implementation of CONCAT_DT for INT *)
FUNCTION CONCAT_DT__INT : DT
VAR_INPUT
    year : INT;
    month : INT;
    day : INT;
    hour : INT;
    minute : INT;
    second : INT;
    millisecond : INT;
END_VAR
    CONCAT_DT__INT := CONCAT_DATE_TOD(CONCAT_DATE(year, month, day), CONCAT_TOD(hour, minute, second, millisecond));
END_FUNCTION

(* Specialized implementation of CONCAT_DT for UINT *)
FUNCTION CONCAT_DT__UINT : DT
VAR_INPUT
    year : UINT;
    month : UINT;
    day : UINT;
    hour : UINT;
    minute : UINT;
    second : UINT;
    millisecond : UINT;
END_VAR
    CONCAT_DT__UINT := CONCAT_DATE_TOD(CONCAT_DATE(year, month, day), CONCAT_TOD(hour, minute, second, millisecond));
END_FUNCTION

(* Specialized implementation of CONCAT_DT for DINT *)
FUNCTION CONCAT_DT__DINT : DT
VAR_INPUT
    year : DINT;
    month : DINT;
    day : DINT;
    hour : DINT;
    minute : DINT;
    second : DINT;
    millisecond : DINT;
END_VAR
    CONCAT_DT__DINT := CONCAT_DATE_TOD(CONCAT_DATE(year, month, day), CONCAT_TOD(hour, minute, second, millisecond));
END_FUNCTION

(* Specialized implementation of CONCAT_DT for UDINT *)
FUNCTION CONCAT_DT__UDINT : DT
VAR_INPUT
    year : UDINT;
    month : UDINT;
    day : UDINT;
    hour : UDINT;
    minute : UDINT;
    second : UDINT;
    millisecond : UDINT;
END_VAR
    CONCAT_DT__UDINT := CONCAT_DATE_TOD(CONCAT_DATE(year, month, day), CONCAT_TOD(hour, minute, second, millisecond));
END_FUNCTION

(* Specialized implementation of CONCAT_DT for LINT *)
FUNCTION CONCAT_DT__LINT : DT
VAR_INPUT
    year : LINT;
    month : LINT;
    day : LINT;
    hour : LINT;
    minute : LINT;
    second : LINT;
    millisecond : LINT;
END_VAR
    CONCAT_DT__LINT := CONCAT_DATE_TOD(CONCAT_DATE(year, month, day), CONCAT_TOD(hour, minute, second, millisecond));
END_FUNCTION

(* Specialized implementation of CONCAT_DT for ULINT *)
FUNCTION CONCAT_DT__ULINT : DT
VAR_INPUT
    year : ULINT;
    month : ULINT;
    day : ULINT;
    hour : ULINT;
    minute : ULINT;
    second : ULINT;
    millisecond : ULINT;
END_VAR
    CONCAT_DT__ULINT := CONCAT_DATE_TOD(CONCAT_DATE(year, month, day), CONCAT_TOD(hour, minute, second, millisecond));
END_FUNCTION

(********************
*
* Concatenates year, month, day, hour, minute, second, millisecond of type ANY_INT to LDT
*
******************** *)
FUNCTION CONCAT_LDT < T : ANY_INT > : LDT
VAR_INPUT
    year : T;
    month : T;
    day : T;
    hour : T;
    minute : T;
    second : T;
    millisecond : T;
END_VAR
END_FUNCTION

(* Specialized implementation of CONCAT_LDT for INT *)
FUNCTION CONCAT_LDT__INT : LDT
VAR_INPUT
    year : INT;
    month : INT;
    day : INT;
    hour : INT;
    minute : INT;
    second : INT;
    millisecond : INT;
END_VAR
    CONCAT_LDT__INT := CONCAT_DATE_LTOD(CONCAT_DATE(year, month, day), CONCAT_LTOD(hour, minute, second, millisecond));
END_FUNCTION

(* Specialized implementation of CONCAT_LDT for UINT *)
FUNCTION CONCAT_LDT__UINT : LDT
VAR_INPUT
    year : UINT;
    month : UINT;
    day : UINT;
    hour : UINT;
    minute : UINT;
    second : UINT;
    millisecond : UINT;
END_VAR
    CONCAT_LDT__UINT := CONCAT_DATE_LTOD(CONCAT_DATE(year, month, day), CONCAT_LTOD(hour, minute, second, millisecond));
END_FUNCTION

(* Specialized implementation of CONCAT_LDT for DINT *)
FUNCTION CONCAT_LDT__DINT : LDT
VAR_INPUT
    year : DINT;
    month : DINT;
    day : DINT;
    hour : DINT;
    minute : DINT;
    second : DINT;
    millisecond : DINT;
END_VAR
    CONCAT_LDT__DINT := CONCAT_DATE_LTOD(CONCAT_DATE(year, month, day), CONCAT_LTOD(hour, minute, second, millisecond));
END_FUNCTION

(* Specialized implementation of CONCAT_LDT for UDINT *)
FUNCTION CONCAT_LDT__UDINT : LDT
VAR_INPUT
    year : UDINT;
    month : UDINT;
    day : UDINT;
    hour : UDINT;
    minute : UDINT;
    second : UDINT;
    millisecond : UDINT;
END_VAR
    CONCAT_LDT__UDINT := CONCAT_DATE_LTOD(CONCAT_DATE(year, month, day), CONCAT_LTOD(hour, minute, second, millisecond));
END_FUNCTION

(* Specialized implementation of CONCAT_LDT for LINT *)
FUNCTION CONCAT_LDT__LINT : LDT
VAR_INPUT
    year : LINT;
    month : LINT;
    day : LINT;
    hour : LINT;
    minute : LINT;
    second : LINT;
    millisecond : LINT;
END_VAR
    CONCAT_LDT__LINT := CONCAT_DATE_LTOD(CONCAT_DATE(year, month, day), CONCAT_LTOD(hour, minute, second, millisecond));
END_FUNCTION

(* Specialized implementation of CONCAT_LDT for ULINT *)
FUNCTION CONCAT_LDT__ULINT : LDT
VAR_INPUT
    year : ULINT;
    month : ULINT;
    day : ULINT;
    hour : ULINT;
    minute : ULINT;
    second : ULINT;
    millisecond : ULINT;
END_VAR
    CONCAT_LDT__ULINT := CONCAT_DATE_LTOD(CONCAT_DATE(year, month, day), CONCAT_LTOD(hour, minute, second, millisecond));
END_FUNCTION

(********************
*
* Splits DATE into year, month, day of type ANY_INT
*
******************** *)
FUNCTION SPLIT_DATE < T : ANY_INT > : INT
VAR_INPUT
    in : DATE;
END_VAR
VAR_OUTPUT
    year : T;
    month : T;
    day : T;
END_VAR
END_FUNCTION

(* Specialized implementation of SPLIT_DATE for INT *)
{external}
FUNCTION SPLIT_DATE__INT : INT
VAR_INPUT
    in : DATE;
END_VAR
VAR_OUTPUT
    year : INT;
    month : INT;
    day : INT;
END_VAR
END_FUNCTION

(* Specialized implementation of SPLIT_DATE for UINT *)
{external}
FUNCTION SPLIT_DATE__UINT : INT
VAR_INPUT
    in : DATE;
END_VAR
VAR_OUTPUT
    year : UINT;
    month : UINT;
    day : UINT;
END_VAR
END_FUNCTION

(* Specialized implementation of SPLIT_DATE for DINT *)
{external}
FUNCTION SPLIT_DATE__DINT : INT
VAR_INPUT
    in : DATE;
END_VAR
VAR_OUTPUT
    year : DINT;
    month : DINT;
    day : DINT;
END_VAR
END_FUNCTION

(* Specialized implementation of SPLIT_DATE for UDINT *)
{external}
FUNCTION SPLIT_DATE__UDINT : INT
VAR_INPUT
    in : DATE;
END_VAR
VAR_OUTPUT
    year : UDINT;
    month : UDINT;
    day : UDINT;
END_VAR
END_FUNCTION

(* Specialized implementation of SPLIT_DATE for LINT *)
{external}
FUNCTION SPLIT_DATE__LINT : INT
VAR_INPUT
    in : DATE;
END_VAR
VAR_OUTPUT
    year : LINT;
    month : LINT;
    day : LINT;
END_VAR
END_FUNCTION

(* Specialized implementation of SPLIT_DATE for ULINT *)
{external}
FUNCTION SPLIT_DATE__ULINT : INT
VAR_INPUT
    in : DATE;
END_VAR
VAR_OUTPUT
    year : ULINT;
    month : ULINT;
    day : ULINT;
END_VAR
END_FUNCTION

(********************
*
* Splits TOD into hour, minute, second, millisecond of type ANY_INT
*
******************** *)
FUNCTION SPLIT_TOD < T : ANY_INT > : INT
VAR_INPUT
    in : TOD;
END_VAR
VAR_OUTPUT
    hour : T;
    minute : T;
    second : T;
    millisecond : T;
END_VAR
END_FUNCTION

(* Specialized implementation of SPLIT_TOD for INT *)
{external}
FUNCTION SPLIT_TOD__INT : INT
VAR_INPUT
    in : TOD;
END_VAR
VAR_OUTPUT
    hour : INT;
    minute : INT;
    second : INT;
    millisecond : INT;
END_VAR
END_FUNCTION

(* Specialized implementation of SPLIT_TOD for UINT *)
{external}
FUNCTION SPLIT_TOD__UINT : INT
VAR_INPUT
    in : TOD;
END_VAR
VAR_OUTPUT
    hour : UINT;
    minute : UINT;
    second : UINT;
    millisecond : UINT;
END_VAR
END_FUNCTION

(* Specialized implementation of SPLIT_TOD for DINT *)
{external}
FUNCTION SPLIT_TOD__DINT : INT
VAR_INPUT
    in : TOD;
END_VAR
VAR_OUTPUT
    hour : DINT;
    minute : DINT;
    second : DINT;
    millisecond : DINT;
END_VAR
END_FUNCTION

(* Specialized implementation of SPLIT_TOD for UDINT *)
{external}
FUNCTION SPLIT_TOD__UDINT : INT
VAR_INPUT
    in : TOD;
END_VAR
VAR_OUTPUT
    hour : UDINT;
    minute : UDINT;
    second : UDINT;
    millisecond : UDINT;
END_VAR
END_FUNCTION

(* Specialized implementation of SPLIT_TOD for LINT *)
{external}
FUNCTION SPLIT_TOD__LINT : INT
VAR_INPUT
    in : TOD;
END_VAR
VAR_OUTPUT
    hour : LINT;
    minute : LINT;
    second : LINT;
    millisecond : LINT;
END_VAR
END_FUNCTION

(* Specialized implementation of SPLIT_TOD for ULINT *)
{external}
FUNCTION SPLIT_TOD__ULINT : INT
VAR_INPUT
    in : TOD;
END_VAR
VAR_OUTPUT
    hour : ULINT;
    minute : ULINT;
    second : ULINT;
    millisecond : ULINT;
END_VAR
END_FUNCTION

(********************
*
* Splits LTOD into hour, minute, second, millisecond of type ANY_INT
*
******************** *)
FUNCTION SPLIT_LTOD < T : ANY_INT > : INT
VAR_INPUT
    in : LTOD;
END_VAR
VAR_OUTPUT
    hour : T;
    minute : T;
    second : T;
    millisecond : T;
END_VAR
END_FUNCTION

(* Specialized implementation of SPLIT_LTOD for INT *)
FUNCTION SPLIT_LTOD__INT : INT
VAR_INPUT
    in : LTOD;
END_VAR
VAR_OUTPUT
    hour : INT;
    minute : INT;
    second : INT;
    millisecond : INT;
END_VAR
    SPLIT_LTOD__INT := SPLIT_TOD__INT(in, hour, minute, second, millisecond);
END_FUNCTION

(* Specialized implementation of SPLIT_LTOD for UINT *)
FUNCTION SPLIT_LTOD__UINT : INT
VAR_INPUT
    in : LTOD;
END_VAR
VAR_OUTPUT
    hour : UINT;
    minute : UINT;
    second : UINT;
    millisecond : UINT;
END_VAR
    SPLIT_LTOD__UINT := SPLIT_TOD__UINT(in, hour, minute, second, millisecond);
END_FUNCTION

(* Specialized implementation of SPLIT_LTOD for DINT *)
FUNCTION SPLIT_LTOD__DINT : INT
VAR_INPUT
    in : LTOD;
END_VAR
VAR_OUTPUT
    hour : DINT;
    minute : DINT;
    second : DINT;
    millisecond : DINT;
END_VAR
    SPLIT_LTOD__DINT := SPLIT_TOD__DINT(in, hour, minute, second, millisecond);
END_FUNCTION

(* Specialized implementation of SPLIT_LTOD for UDINT *)
FUNCTION SPLIT_LTOD__UDINT : INT
VAR_INPUT
    in : LTOD;
END_VAR
VAR_OUTPUT
    hour : UDINT;
    minute : UDINT;
    second : UDINT;
    millisecond : UDINT;
END_VAR
    SPLIT_LTOD__UDINT := SPLIT_TOD__UDINT(in, hour, minute, second, millisecond);
END_FUNCTION

(* Specialized implementation of SPLIT_LTOD for LINT *)
FUNCTION SPLIT_LTOD__LINT : INT
VAR_INPUT
    in : LTOD;
END_VAR
VAR_OUTPUT
    hour : LINT;
    minute : LINT;
    second : LINT;
    millisecond : LINT;
END_VAR
    SPLIT_LTOD__LINT := SPLIT_TOD__LINT(in, hour, minute, second, millisecond);
END_FUNCTION

(* Specialized implementation of SPLIT_LTOD for ULINT *)
FUNCTION SPLIT_LTOD__ULINT : INT
VAR_INPUT
    in : LTOD;
END_VAR
VAR_OUTPUT
    hour : ULINT;
    minute : ULINT;
    second : ULINT;
    millisecond : ULINT;
END_VAR
    SPLIT_LTOD__ULINT := SPLIT_TOD__ULINT(in, hour, minute, second, millisecond);
END_FUNCTION

(********************
*
* Splits DT into year, month, day, hour, minute, second, millisecond of type ANY_INT
*
******************** *)
FUNCTION SPLIT_DT < T : ANY_INT > : INT
VAR_INPUT
    in : DT;
END_VAR
VAR_OUTPUT
    year : T;
    month : T;
    day : T;
    hour : T;
    minute : T;
    second : T;
    millisecond : T;
END_VAR
END_FUNCTION

(* Specialized implementation of SPLIT_DT for INT *)
{external}
FUNCTION SPLIT_DT__INT : INT
VAR_INPUT
    in : DT;
END_VAR
VAR_OUTPUT
    year : INT;
    month : INT;
    day : INT;
    hour : INT;
    minute : INT;
    second : INT;
    millisecond : INT;
END_VAR
END_FUNCTION

(* Specialized implementation of SPLIT_DT for UINT *)
{external}
FUNCTION SPLIT_DT__UINT : INT
VAR_INPUT
    in : DT;
END_VAR
VAR_OUTPUT
    year : UINT;
    month : UINT;
    day : UINT;
    hour : UINT;
    minute : UINT;
    second : UINT;
    millisecond : UINT;
END_VAR
END_FUNCTION

(* Specialized implementation of SPLIT_DT for DINT *)
{external}
FUNCTION SPLIT_DT__DINT : INT
VAR_INPUT
    in : DT;
END_VAR
VAR_OUTPUT
    year : DINT;
    month : DINT;
    day : DINT;
    hour : DINT;
    minute : DINT;
    second : DINT;
    millisecond : DINT;
END_VAR
END_FUNCTION

(* Specialized implementation of SPLIT_DT for UDINT *)
{external}
FUNCTION SPLIT_DT__UDINT : INT
VAR_INPUT
    in : DT;
END_VAR
VAR_OUTPUT
    year : UDINT;
    month : UDINT;
    day : UDINT;
    hour : UDINT;
    minute : UDINT;
    second : UDINT;
    millisecond : UDINT;
END_VAR
END_FUNCTION

(* Specialized implementation of SPLIT_DT for LINT *)
{external}
FUNCTION SPLIT_DT__LINT : INT
VAR_INPUT
    in : DT;
END_VAR
VAR_OUTPUT
    year : LINT;
    month : LINT;
    day : LINT;
    hour : LINT;
    minute : LINT;
    second : LINT;
    millisecond : LINT;
END_VAR
END_FUNCTION

(* Specialized implementation of SPLIT_DT for ULINT *)
{external}
FUNCTION SPLIT_DT__ULINT : INT
VAR_INPUT
    in : DT;
END_VAR
VAR_OUTPUT
    year : ULINT;
    month : ULINT;
    day : ULINT;
    hour : ULINT;
    minute : ULINT;
    second : ULINT;
    millisecond : ULINT;
END_VAR
END_FUNCTION

(********************
*
* Splits LDT into year, month, day, hour, minute, second, millisecond of type ANY_INT
*
******************** *)
FUNCTION SPLIT_LDT < T : ANY_INT > : INT
VAR_INPUT
    in : LDT;
END_VAR
VAR_OUTPUT
    year : T;
    month : T;
    day : T;
    hour : T;
    minute : T;
    second : T;
    millisecond : T;
END_VAR
END_FUNCTION

(* Specialized implementation of SPLIT_LDT for INT *)
{external}
FUNCTION SPLIT_LDT__INT : INT
VAR_INPUT
    in : LDT;
END_VAR
VAR_OUTPUT
    year : INT;
    month : INT;
    day : INT;
    hour : INT;
    minute : INT;
    second : INT;
    millisecond : INT;
END_VAR
    SPLIT_LDT__INT := SPLIT_DT__INT(in, year, month, day, hour, minute, second, millisecond);
END_FUNCTION

(* Specialized implementation of SPLIT_LDT for UINT *)
{external}
FUNCTION SPLIT_LDT__UINT : INT
VAR_INPUT
    in : LDT;
END_VAR
VAR_OUTPUT
    year : UINT;
    month : UINT;
    day : UINT;
    hour : UINT;
    minute : UINT;
    second : UINT;
    millisecond : UINT;
END_VAR
    SPLIT_LDT__UINT := SPLIT_DT__UINT(in, year, month, day, hour, minute, second, millisecond);
END_FUNCTION

(* Specialized implementation of SPLIT_LDT for DINT *)
{external}
FUNCTION SPLIT_LDT__DINT : INT
VAR_INPUT
    in : LDT;
END_VAR
VAR_OUTPUT
    year : DINT;
    month : DINT;
    day : DINT;
    hour : DINT;
    minute : DINT;
    second : DINT;
    millisecond : DINT;
END_VAR
    SPLIT_LDT__DINT := SPLIT_DT__DINT(in, year, month, day, hour, minute, second, millisecond);
END_FUNCTION

(* Specialized implementation of SPLIT_LDT for UDINT *)
{external}
FUNCTION SPLIT_LDT__UDINT : INT
VAR_INPUT
    in : LDT;
END_VAR
VAR_OUTPUT
    year : UDINT;
    month : UDINT;
    day : UDINT;
    hour : UDINT;
    minute : UDINT;
    second : UDINT;
    millisecond : UDINT;
END_VAR
    SPLIT_LDT__UDINT := SPLIT_DT__UDINT(in, year, month, day, hour, minute, second, millisecond);
END_FUNCTION

(* Specialized implementation of SPLIT_LDT for LINT *)
{external}
FUNCTION SPLIT_LDT__LINT : INT
VAR_INPUT
    in : LDT;
END_VAR
VAR_OUTPUT
    year : LINT;
    month : LINT;
    day : LINT;
    hour : LINT;
    minute : LINT;
    second : LINT;
    millisecond : LINT;
END_VAR
    SPLIT_LDT__LINT := SPLIT_DT__LINT(in, year, month, day, hour, minute, second, millisecond);
END_FUNCTION

(* Specialized implementation of SPLIT_LDT for ULINT *)
{external}
FUNCTION SPLIT_LDT__ULINT : INT
VAR_INPUT
    in : LDT;
END_VAR
VAR_OUTPUT
    year : ULINT;
    month : ULINT;
    day : ULINT;
    hour : ULINT;
    minute : ULINT;
    second : ULINT;
    millisecond : ULINT;
END_VAR
    SPLIT_LDT__ULINT := SPLIT_DT__ULINT(in, year, month, day, hour, minute, second, millisecond);
END_FUNCTION

(********************
*
* Returns day of week form given DATE in SINT
* 0 -> sunday, 1 -> monday, ..., 6 -> saturday
*
******************** *)
{external}
FUNCTION DAY_OF_WEEK : SINT
VAR_INPUT
    in : DATE;
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
************************ *)
{external}
FUNCTION ROL < T : ANY_BIT > : T
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
************************ *)
{external}
FUNCTION ROR < T : ANY_BIT > : T
VAR_INPUT
    IN : T;
    n : UDINT;
END_VAR
END_FUNCTION

(********************
*
* Converts LWORD to DWORD
*
******************** *)
FUNCTION LWORD_TO_DWORD : DWORD
VAR_INPUT
    in : LWORD;
END_VAR
    LWORD_TO_DWORD := in;
END_FUNCTION

(********************
*
* Converts LWORD to WORD
*
******************** *)
FUNCTION LWORD_TO_WORD : WORD
VAR_INPUT
    in : LWORD;
END_VAR
    LWORD_TO_WORD := in;
END_FUNCTION

(********************
*
* Converts LWORD to BYTE
*
******************** *)
FUNCTION LWORD_TO_BYTE : BYTE
VAR_INPUT
    in : LWORD;
END_VAR
    LWORD_TO_BYTE := in;
END_FUNCTION

(********************
*
* Converts LWORD to BOOL
*
******************** *)
FUNCTION LWORD_TO_BOOL : BOOL
VAR_INPUT
    in : LWORD;
END_VAR
    LWORD_TO_BOOL := in.0;
END_FUNCTION

(********************
*
* Converts DWORD to LWORD
*
******************** *)
FUNCTION DWORD_TO_LWORD : LWORD
VAR_INPUT
    in : DWORD;
END_VAR
    DWORD_TO_LWORD := in;
END_FUNCTION

(********************
*
* Converts DWORD to WORD
*
******************** *)
FUNCTION DWORD_TO_WORD : WORD
VAR_INPUT
    in : DWORD;
END_VAR
    DWORD_TO_WORD := in;
END_FUNCTION

(********************
*
* Converts DWORD to BYTE
*
******************** *)
FUNCTION DWORD_TO_BYTE : BYTE
VAR_INPUT
    in : DWORD;
END_VAR
    DWORD_TO_BYTE := in;
END_FUNCTION

(********************
*
* Converts DWORD to BOOL
*
******************** *)
FUNCTION DWORD_TO_BOOL : BOOL
VAR_INPUT
    in : DWORD;
END_VAR
    DWORD_TO_BOOL := in.0;
END_FUNCTION

(********************
*
* Converts WORD to LWORD
*
******************** *)
FUNCTION WORD_TO_LWORD : LWORD
VAR_INPUT
    in : WORD;
END_VAR
    WORD_TO_LWORD := in;
END_FUNCTION

(********************
*
* Converts WORD to DWORD
*
******************** *)
FUNCTION WORD_TO_DWORD : DWORD
VAR_INPUT
    in : WORD;
END_VAR
    WORD_TO_DWORD := in;
END_FUNCTION

(********************
*
* Converts WORD to BYTE
*
******************** *)
FUNCTION WORD_TO_BYTE : BYTE
VAR_INPUT
    in : WORD;
END_VAR
    WORD_TO_BYTE := in;
END_FUNCTION

(********************
*
* Converts WORD to BOOL
*
******************** *)
FUNCTION WORD_TO_BOOL : BOOL
VAR_INPUT
    in : WORD;
END_VAR
    WORD_TO_BOOL := in.0;
END_FUNCTION

(********************
*
* Converts BYTE to LWORD
*
******************** *)
FUNCTION BYTE_TO_LWORD : LWORD
VAR_INPUT
    in : BYTE;
END_VAR
    BYTE_TO_LWORD := in;
END_FUNCTION

(********************
*
* Converts BYTE to DWORD
*
******************** *)
FUNCTION BYTE_TO_DWORD : DWORD
VAR_INPUT
    in : BYTE;
END_VAR
    BYTE_TO_DWORD := in;
END_FUNCTION

(********************
*
* Converts BYTE to WORD
*
******************** *)
FUNCTION BYTE_TO_WORD : WORD
VAR_INPUT
    in : BYTE;
END_VAR
    BYTE_TO_WORD := in;
END_FUNCTION

(********************
*
* Converts BYTE to BOOL
*
******************** *)
FUNCTION BYTE_TO_BOOL : BOOL
VAR_INPUT
    in : BYTE;
END_VAR
    BYTE_TO_BOOL := in.0;
END_FUNCTION

(********************
*
* Converts BYTE to CHAR
*
******************** *)
FUNCTION BYTE_TO_CHAR : CHAR
VAR_INPUT
    in : BYTE;
END_VAR
VAR
    ptr : REF_TO CHAR;
END_VAR
    ptr := &in;
    BYTE_TO_CHAR := ptr^;
END_FUNCTION

(********************
*
* Converts BOOL to LWORD
*
******************** *)
FUNCTION BOOL_TO_LWORD : LWORD
VAR_INPUT
    in : BOOL;
END_VAR
    IF in <> 0 THEN
        BOOL_TO_LWORD := 1;
    ELSE
        BOOL_TO_LWORD := 0;
    END_IF;
END_FUNCTION

(********************
*
* Converts BOOL to DWORD
*
******************** *)
FUNCTION BOOL_TO_DWORD : DWORD
VAR_INPUT
    in : BOOL;
END_VAR
    IF in <> 0 THEN
        BOOL_TO_DWORD := 1;
    ELSE
        BOOL_TO_DWORD := 0;
    END_IF;
END_FUNCTION

(********************
*
* Converts BOOL to WORD
*
******************** *)
FUNCTION BOOL_TO_WORD : WORD
VAR_INPUT
    in : BOOL;
END_VAR
    IF in <> 0 THEN
        BOOL_TO_WORD := 1;
    ELSE
        BOOL_TO_WORD := 0;
    END_IF;
END_FUNCTION

(********************
*
* Converts BOOL to BYTE
*
******************** *)
FUNCTION BOOL_TO_BYTE : BYTE
VAR_INPUT
    in : BOOL;
END_VAR
    IF in <> 0 THEN
        BOOL_TO_BYTE := 1;
    ELSE
        BOOL_TO_BYTE := 0;
    END_IF;
END_FUNCTION

(********************
*
* Converts CHAR to BYTE
*
******************** *)
FUNCTION CHAR_TO_BYTE : BYTE
VAR_INPUT
    in : CHAR;
END_VAR
VAR
    ptr : REF_TO BYTE;
END_VAR
    ptr := &in;
    CHAR_TO_BYTE := ptr^;
END_FUNCTION

(********************
*
* Converts CHAR to WORD
*
******************** *)
FUNCTION CHAR_TO_WORD : WORD
VAR_INPUT
    in : CHAR;
END_VAR
    CHAR_TO_WORD := CHAR_TO_BYTE(in);
END_FUNCTION

(********************
*
* Converts CHAR to DWORD
*
******************** *)
FUNCTION CHAR_TO_DWORD : DWORD
VAR_INPUT
    in : CHAR;
END_VAR
    CHAR_TO_DWORD := CHAR_TO_BYTE(in);
END_FUNCTION

(********************
*
* Converts CHAR to LWORD
*
******************** *)
FUNCTION CHAR_TO_LWORD : LWORD
VAR_INPUT
    in : CHAR;
END_VAR
    CHAR_TO_LWORD := CHAR_TO_BYTE(in);
END_FUNCTION

(********************
*
* Converts WCHAR to WORD
*
******************** *)
FUNCTION WCHAR_TO_WORD : WORD
VAR_INPUT
    in : WCHAR;
END_VAR
VAR
    ptr : REF_TO WORD;
END_VAR
    ptr := &in;
    WCHAR_TO_WORD := ptr^;
END_FUNCTION

(********************
*
* Converts WCHAR to DWORD
*
******************** *)
FUNCTION WCHAR_TO_DWORD : DWORD
VAR_INPUT
    in : WCHAR;
END_VAR
    WCHAR_TO_DWORD := WCHAR_TO_WORD(in);
END_FUNCTION

(********************
*
* Converts WCHAR to LWORD
*
******************** *)
FUNCTION WCHAR_TO_LWORD : LWORD
VAR_INPUT
    in : WCHAR;
END_VAR
    WCHAR_TO_LWORD := WCHAR_TO_WORD(in);
END_FUNCTION (* *****************************************************************************
Description: Validation function for ANY_REAL
Input:
  - IN: ANY_REAL
Return:
  - FALSE: If input is Not-a-Number(NaN) or infinite(+Inf, -Inf)
  - TRUE: Otherwise
***************************************************************************** *)
{external}
FUNCTION IS_VALID < T : ANY_REAL > : BOOL
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
***************************************************************************** *)
{external}
FUNCTION IS_VALID_BCD < T : ANY_BIT > : BOOL
VAR_INPUT
    IN : T;
END_VAR
END_FUNCTION (* *****************************************************************************
Description: Bistable function, set dominant
Input:
  - SET1: Dominant input of bistable function
  - RESET: Input of bistable function
Output:
  - Q1: Result of bistable function
Return: Output is used as return
***************************************************************************** *)
{external}
FUNCTION_BLOCK SR
VAR_INPUT
    SET1 : BOOL;
    RESET : BOOL;
END_VAR
VAR_OUTPUT
    Q1 : BOOL;
END_VAR
END_FUNCTION_BLOCK

(******************************************************************************
Description: Bistable function, reset dominant
Input:
  - SET0: Input of bistable function
  - RESET1: Dominant input of bistable function
Output:
  - Q1: Result of bistable function
Return: Output is used as return
***************************************************************************** *)
{external}
FUNCTION_BLOCK RS
VAR_INPUT
    SET0 :   BOOL;
    RESET1 : BOOL;
END_VAR
VAR_OUTPUT
    Q1 :        BOOL;
END_VAR
END_FUNCTION_BLOCK (* *******************
*
* Converts WSTRING to STRING
*
******************** *)
FUNCTION WSTRING_TO_STRING : STRING[__STRING_LENGTH]
VAR_INPUT {ref}
    in : WSTRING[__STRING_LENGTH];
END_VAR
    WSTRING_TO_STRING_EXT(in, WSTRING_TO_STRING);
END_FUNCTION

{external}
FUNCTION WSTRING_TO_STRING_EXT : DINT
VAR_INPUT {ref}
    in : WSTRING[__STRING_LENGTH];
END_VAR
VAR_IN_OUT
    out : STRING[__STRING_LENGTH];
END_VAR
END_FUNCTION

(********************
*
* Converts WSTRING to WCHAR
*
******************** *)
FUNCTION WSTRING_TO_WCHAR : WCHAR
VAR_INPUT {ref}
    in : WSTRING;
END_VAR
VAR
    ptr : REF_TO WCHAR;
END_VAR
    ptr := &in;
    WSTRING_TO_WCHAR := ptr^;
END_FUNCTION

(********************
*
* Converts STRING to WSTRING
* Will return a WSTRING with default length of 81
*
******************** *)
FUNCTION STRING_TO_WSTRING : WSTRING[__STRING_LENGTH]
VAR_INPUT {ref}
    in : STRING[__STRING_LENGTH];
END_VAR
    STRING_TO_WSTRING_EXT(in, STRING_TO_WSTRING);
END_FUNCTION

{external}
FUNCTION STRING_TO_WSTRING_EXT : DINT
VAR_INPUT {ref}
    in : STRING[__STRING_LENGTH];
END_VAR
VAR_IN_OUT
    out : WSTRING[__STRING_LENGTH];
END_VAR
END_FUNCTION

(********************
*
* Converts STRING to CHAR
*
******************** *)
FUNCTION STRING_TO_CHAR : CHAR
VAR_INPUT {ref}
    in : STRING;
END_VAR
VAR
    ptr : REF_TO CHAR;
END_VAR
    ptr := &in;
    STRING_TO_CHAR := ptr^;
END_FUNCTION

(********************
*
* Converts WCHAR to WSTRING
*
******************** *)
FUNCTION WCHAR_TO_WSTRING : WSTRING
VAR_INPUT
    in : WCHAR;
END_VAR
VAR
    ptr : REF_TO WSTRING;
END_VAR
    ptr := &in;
    WCHAR_TO_WSTRING := ptr^;
END_FUNCTION

(********************
*
* Converts WCHAR to CHAR
*
******************** *)
{external}
FUNCTION WCHAR_TO_CHAR : CHAR
VAR_INPUT
    in : WCHAR;
END_VAR
END_FUNCTION

(********************
*
* Converts CHAR to STRING
*
******************** *)
FUNCTION CHAR_TO_STRING : STRING
VAR_INPUT
    in : CHAR;
END_VAR
VAR
    ptr : REF_TO STRING;
END_VAR
    ptr := &in;
    CHAR_TO_STRING := ptr^;
END_FUNCTION

(********************
*
* Converts CHAR to WCHAR
*
******************** *)
{external}
FUNCTION CHAR_TO_WCHAR : WCHAR
VAR_INPUT
    in : CHAR;
END_VAR
END_FUNCTION (* *************************
 *
 * MAX(IN0, IN1, ...)
 *
 * This operator produces the larger value of the given operands.
 *
************************ *)
{external}
FUNCTION MAX < T : ANY_ELEMENTARY > : T
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
************************ *)
{external}
FUNCTION MIN < T : ANY_ELEMENTARY > : T
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
************************ *)
{external}
FUNCTION LIMIT < T : ANY_ELEMENTARY > : T
VAR_INPUT
    MIN : T;
    IN : T;
    MAX : T;
END_VAR
END_FUNCTION{external}
(******************************************************************************
Description: Timer Pulse
Input:
  - IN: Trigger for Start of the Signal
  - PT: The length of the High-Signal
Output:
  - Q: The pulse
  - ET: The current phase of the High-Signal
Return: Output variables are used for return.
***************************************************************************** *)
FUNCTION_BLOCK TP
    VAR_INPUT
        IN : BOOL;
        PT : TIME;
    END_VAR
    VAR_OUTPUT
        Q  : BOOL;
        ET : TIME;
    END_VAR
    VAR
        __signal__ : BOOL; (* Value representing the internal signal *)
        __BUFFER__ : ARRAY[1..24] OF BYTE; (* Buffer used for internal implementation *)
    END_VAR
END_FUNCTION_BLOCK

{external}
(******************************************************************************
Description: Timer Pulse
Input:
  - IN: Trigger for Start of the Signal
  - PT: The length of the High-Signal
Output:
  - Q: The pulse
  - ET: The current phase of the High-Signal
Return: Output variables are used for return.
***************************************************************************** *)
FUNCTION_BLOCK TP_TIME
    VAR_INPUT
        IN : BOOL;
        PT : TIME;
    END_VAR
    VAR_OUTPUT
        Q  : BOOL;
        ET : TIME;
    END_VAR
    VAR
      __signal__ : BOOL; (* Value representing the internal signal *)
      __BUFFER__ : ARRAY[1..24] OF BYTE; (* Buffer used for internal implementation *)
    END_VAR
END_FUNCTION_BLOCK

{external}
(******************************************************************************
Description: Timer Pulse
Input:
  - IN: Trigger for Start of the Signal
  - PT: The length of the High-Signal
Output:
  - Q: The pulse
  - ET: The current phase of the High-Signal
Return: Output variables are used for return.
***************************************************************************** *)
FUNCTION_BLOCK TP_LTIME
    VAR_INPUT
        IN : BOOL;
        PT : TIME;
    END_VAR
    VAR_OUTPUT
        Q  : BOOL;
        ET : LTIME;
    END_VAR
    VAR
        __signal__ : BOOL; (* Value representing the internal signal *)
        __BUFFER__ : ARRAY[1..24] OF BYTE; (* Buffer used for internal implementation *)
    END_VAR
END_FUNCTION_BLOCK

{external}
(******************************************************************************
Description: Timer on delay.
Input:
  - IN: Starts timer with rising edge, resets timer with falling edge
  - PT: Time to pass, before Q is set
Output:
  - Q: Is TRUE, PT seconds after IN had a rising edge
  - ET: Elapsed time
Return: Output variables are used for return.
***************************************************************************** *)
FUNCTION_BLOCK TON
VAR_INPUT
    IN : BOOL;
    PT : TIME;
END_VAR
VAR_OUTPUT
    Q : BOOL;
    ET : TIME;
END_VAR
VAR
  __signal__ : BOOL; (* Value representing the internal signal *)
  __BUFFER__ : ARRAY[1..24] OF BYTE; (* Buffer used for internal implementation *)
END_VAR
END_FUNCTION_BLOCK

{external}
(******************************************************************************
Description: Timer on delay.
Input:
  - IN: Starts timer with rising edge, resets timer with falling edge
  - PT: Time to pass, before Q is set
Output:
  - Q: Is TRUE, PT seconds after IN had a rising edge
  - ET: Elapsed time
Return: Output variables are used for return.
***************************************************************************** *)
FUNCTION_BLOCK TON_TIME
VAR_INPUT
    IN : BOOL;
    PT : TIME;
END_VAR
VAR_OUTPUT
    Q : BOOL;
    ET : TIME;
END_VAR
VAR
  __signal__ : BOOL; (* Value representing the internal signal *)
  __BUFFER__ : ARRAY[1..24] OF BYTE; (* Buffer used for internal implementation *)
END_VAR
END_FUNCTION_BLOCK

{external}
(******************************************************************************
Description: Timer on delay.
Input:
  - IN: Starts timer with rising edge, resets timer with falling edge
  - PT: Time to pass, before Q is set
Output:
  - Q: Is TRUE, PT seconds after IN had a rising edge
  - ET: Elapsed time
Return: Output variables are used for return.
***************************************************************************** *)
FUNCTION_BLOCK TON_LTIME
VAR_INPUT
    IN : BOOL;
    PT : LTIME;
END_VAR
VAR_OUTPUT
    Q : BOOL;
    ET : TIME;
END_VAR
VAR
  __signal__ : BOOL; (* Value representing the internal signal *)
  __BUFFER__ : ARRAY[1..24] OF BYTE; (* Buffer used for internal implementation *)
END_VAR
END_FUNCTION_BLOCK

{external}
(******************************************************************************
Description: Timer of delay
Input:
  - IN: Starts timer with falling edge, resets timer with rising edge
  - PT: Time to pass, before Q is set
Output:
  - Q: is FALSE, PT seconds after IN had a falling edge
  - ET: Elapsed time
Return: Output variables are used for return.
***************************************************************************** *)
FUNCTION_BLOCK TOF
VAR_INPUT
    IN : BOOL;
    PT : TIME;
END_VAR
VAR_OUTPUT
    Q : BOOL;
    ET : TIME;
END_VAR
VAR
  __signal__ : BOOL; (* Value representing the internal signal *)
  __BUFFER__ : ARRAY[1..24] OF BYTE; (* Buffer used for internal implementation *)
END_VAR
END_FUNCTION_BLOCK

{external}
(******************************************************************************
Description: Timer of delay
Input:
  - IN: Starts timer with falling edge, resets timer with rising edge
  - PT: Time to pass, before Q is set
Output:
  - Q: is FALSE, PT seconds after IN had a falling edge
  - ET: Elapsed time
Return: Output variables are used for return.
***************************************************************************** *)
FUNCTION_BLOCK TOF_TIME
VAR_INPUT
    IN : BOOL;
    PT : TIME;
END_VAR
VAR_OUTPUT
    Q : BOOL;
    ET : TIME;
END_VAR
VAR
  __signal__ : BOOL; (* Value representing the internal signal *)
  __BUFFER__ : ARRAY[1..24] OF BYTE; (* Buffer used for internal implementation *)
END_VAR
END_FUNCTION_BLOCK

{external}
(******************************************************************************
Description: Timer of delay
Input:
  - IN: Starts timer with falling edge, resets timer with rising edge
  - PT: Time to pass, before Q is set
Output:
  - Q: is FALSE, PT seconds after IN had a falling edge
  - ET: Elapsed time
Return: Output variables are used for return.
***************************************************************************** *)
FUNCTION_BLOCK TOF_LTIME
VAR_INPUT
    IN : BOOL;
    PT : LTIME;
END_VAR
VAR_OUTPUT
    Q : BOOL;
    ET : LTIME;
END_VAR
VAR
  __signal__ : BOOL; (* Value representing the internal signal *)
  __BUFFER__ : ARRAY[1..24] OF BYTE; (* Buffer used for internal implementation *)
END_VAR
END_FUNCTION_BLOCK (* *******************
*
* Converts DT/LDT to DATE
*
******************** *)
{external}
FUNCTION DATE_AND_TIME_TO_DATE : DATE
VAR_INPUT
    in : DT;
END_VAR
END_FUNCTION

(********************
*
* Converts DT/LDT to TOD/LTOD
*
******************** *)
{external}
FUNCTION DATE_AND_TIME_TO_TIME_OF_DAY : TOD
VAR_INPUT
    in : DT;
END_VAR
END_FUNCTION

(********************
*
* Converts LTIME to TIME
*
******************** *)
FUNCTION LTIME_TO_TIME : TIME
VAR_INPUT
    in : LTIME;
END_VAR
    LTIME_TO_TIME := in;
END_FUNCTION

(********************
*
* Converts TIME to LTIME
*
******************** *)
FUNCTION TIME_TO_LTIME : LTIME
VAR_INPUT
    in : TIME;
END_VAR
    TIME_TO_LTIME := in;
END_FUNCTION

(********************
*
* Converts LDT to DT
*
******************** *)
FUNCTION LDT_TO_DT : DT
VAR_INPUT
    in : LDT;
END_VAR
    LDT_TO_DT := in;
END_FUNCTION

(********************
*
* Converts LDT to DATE
*
******************** *)
FUNCTION LDT_TO_DATE : DATE
VAR_INPUT
    in : LDT;
END_VAR
    LDT_TO_DATE := DATE_AND_TIME_TO_DATE(in);
END_FUNCTION

(********************
*
* Converts LDT to LTOD
*
******************** *)
FUNCTION LDT_TO_LTOD : LTOD
VAR_INPUT
    in : LDT;
END_VAR
    LDT_TO_LTOD := DATE_AND_TIME_TO_TIME_OF_DAY(in);
END_FUNCTION

(********************
*
* Converts LDT to TOD
*
******************** *)
FUNCTION LDT_TO_TOD : TOD
VAR_INPUT
    in : LDT;
END_VAR
    LDT_TO_TOD := DATE_AND_TIME_TO_TIME_OF_DAY(in);
END_FUNCTION

(********************
*
* Converts DT to LDT
*
******************** *)
FUNCTION DT_TO_LDT : LDT
VAR_INPUT
    in : DT;
END_VAR
    DT_TO_LDT := in;
END_FUNCTION

(********************
*
* Converts DT to DATE
*
******************** *)
FUNCTION DT_TO_DATE : DATE
VAR_INPUT
    in : DT;
END_VAR
    DT_TO_DATE := DATE_AND_TIME_TO_DATE(in);
END_FUNCTION

(********************
*
* Converts DT to LTOD
*
******************** *)
FUNCTION DT_TO_LTOD : LTOD
VAR_INPUT
    in : DT;
END_VAR
    DT_TO_LTOD := DATE_AND_TIME_TO_TIME_OF_DAY(in);
END_FUNCTION

(********************
*
* Converts DT to TOD
*
******************** *)
FUNCTION DT_TO_TOD : TOD
VAR_INPUT
    in : DT;
END_VAR
    DT_TO_TOD := DATE_AND_TIME_TO_TIME_OF_DAY(in);
END_FUNCTION

(********************
*
* Converts LTOD to TOD
*
******************** *)
FUNCTION LTOD_TO_TOD : TOD
VAR_INPUT
    in : LTOD;
END_VAR
    LTOD_TO_TOD := in;
END_FUNCTION

(********************
*
* Converts TOD to LTOD
*
******************** *)
FUNCTION TOD_TO_LTOD : LTOD
VAR_INPUT
    in : TOD;
END_VAR
    TOD_TO_LTOD := in;
END_FUNCTION (* *******************
*
* This operator returns the value of adding up the operands.
*
******************** *)
FUNCTION ADD < T1 : ANY, T2 : ANY > : T1
VAR_INPUT
    IN1 : T1;
    IN2 : T2;
END_VAR
END_FUNCTION

(* Specialized implementation of ADD for TIME *)
FUNCTION ADD__TIME__TIME : TIME
VAR_INPUT
    IN1 : TIME;
    IN2 : TIME;
END_VAR
    ADD__TIME__TIME := ADD_TIME(IN1, IN2);
END_FUNCTION

(********************
*
* This operator returns the value of adding up two TIME operands.
*
******************** *)
{external}
FUNCTION ADD_TIME : TIME
VAR_INPUT
    IN1 : TIME;
    IN2 : TIME;
END_VAR
END_FUNCTION

(********************
*
* This operator returns the value of adding up two LTIME operands.
*
******************** *)
FUNCTION ADD_LTIME : LTIME
VAR_INPUT
    IN1 : LTIME;
    IN2 : LTIME;
END_VAR
    ADD_LTIME := ADD_TIME(IN1, IN2);
END_FUNCTION

(* Specialized implementation of ADD for TOD *)
FUNCTION ADD__TIME_OF_DAY__TIME : TOD
VAR_INPUT
    IN1 : TOD;
    IN2 : TIME;
END_VAR
    ADD__TIME_OF_DAY__TIME := ADD_TOD_TIME(IN1, IN2);
END_FUNCTION

(********************
*
* This operator returns the value of adding up TOD and TIME.
* Panic on overflow
*
******************** *)
{external}
FUNCTION ADD_TOD_TIME : TOD
VAR_INPUT
    IN1 : TOD;
    IN2 : TIME;
END_VAR
END_FUNCTION

(********************
*
* This operator returns the value of adding up LTOD and LTIME.
* Panic on overflow
*
******************** *)
FUNCTION ADD_LTOD_LTIME : LTOD
VAR_INPUT
    IN1 : LTOD;
    IN2 : LTIME;
END_VAR
    ADD_LTOD_LTIME := ADD_TOD_TIME(IN1, IN2);
END_FUNCTION

(* Specialized implementation of ADD for DT *)
FUNCTION ADD__DATE_AND_TIME__TIME : DT
VAR_INPUT
    IN1 : DT;
    IN2 : TIME;
END_VAR
    ADD__DATE_AND_TIME__TIME := ADD_DT_TIME(IN1, IN2);
END_FUNCTION

(********************
*
* This operator returns the value of adding up DT and TIME.
* Panic on overflow
*
******************** *)
{external}
FUNCTION ADD_DT_TIME : DT
VAR_INPUT
    IN1 : DT;
    IN2 : TIME;
END_VAR
END_FUNCTION

(********************
*
* This operator returns the value of adding up LDT and LTIME.
* Panic on overflow
*
******************** *)
FUNCTION ADD_LDT_LTIME : LDT
VAR_INPUT
    IN1 : LDT;
    IN2 : LTIME;
END_VAR
    ADD_LDT_LTIME := ADD_DT_TIME(IN1, IN2);
END_FUNCTION

(********************
*
* This operator produces the subtraction of the operands.
*
******************** *)
FUNCTION SUB < T1 : ANY, T2 : ANY > : T1
VAR_INPUT
    IN1 : T1;
    IN2 : T2;
END_VAR
END_FUNCTION

(* Specialized implementation of SUB for TIME *)
FUNCTION SUB__TIME__TIME : TIME
VAR_INPUT
    IN1 : TIME;
    IN2 : TIME;
END_VAR
    SUB__TIME__TIME := SUB_TIME(IN1, IN2);
END_FUNCTION

(********************
*
* This operator produces the subtraction of two TIME operands
*
******************** *)
{external}
FUNCTION SUB_TIME : TIME
VAR_INPUT
    IN1 : TIME;
    IN2 : TIME;
END_VAR
END_FUNCTION

(********************
*
* This operator produces the subtraction of two LTIME operands
*
******************** *)
FUNCTION SUB_LTIME : LTIME
VAR_INPUT
    IN1 : LTIME;
    IN2 : LTIME;
END_VAR
    SUB_LTIME := SUB_TIME(IN1, IN2);
END_FUNCTION

(* Specialized implementation of SUB for DATE *)
FUNCTION SUB__DATE__DATE : TIME
VAR_INPUT
    IN1 : DATE;
    IN2 : DATE;
END_VAR
    SUB__DATE__DATE := SUB_DATE_DATE(IN1, IN2);
END_FUNCTION

(********************
*
* This operator produces the subtraction of two DATE operands returning TIME
* Panic on overflow
*
******************** *)
{external}
FUNCTION SUB_DATE_DATE : TIME
VAR_INPUT
    IN1 : DATE;
    IN2 : DATE;
END_VAR
END_FUNCTION

(********************
*
* This operator produces the subtraction of two LDATE operands returning LTIME
* Panic on overflow
*
******************** *)
FUNCTION SUB_LDATE_LDATE : LTIME
VAR_INPUT
    IN1 : LDATE;
    IN2 : LDATE;
END_VAR
    SUB_LDATE_LDATE := SUB_DATE_DATE(IN1, IN2);
END_FUNCTION

(* Specialized implementation of SUB for TOD and TIME *)
FUNCTION SUB__TIME_OF_DAY__TIME : TOD
VAR_INPUT
    IN1 : TOD;
    IN2 : TIME;
END_VAR
    SUB__TIME_OF_DAY__TIME := SUB_TOD_TIME(IN1, IN2);
END_FUNCTION

(********************
*
* This operator produces the subtraction of TOD and TIME returning TOD
* Panic on overflow
*
******************** *)
{external}
FUNCTION SUB_TOD_TIME : TOD
VAR_INPUT
    IN1 : TOD;
    IN2 : TIME;
END_VAR
END_FUNCTION

(********************
*
* This operator produces the subtraction of LTOD and LTIME returning LTOD
* Panic on overflow
*
******************** *)
FUNCTION SUB_LTOD_LTIME : LTOD
VAR_INPUT
    IN1 : LTOD;
    IN2 : LTIME;
END_VAR
    SUB_LTOD_LTIME := SUB_TOD_TIME(IN1, IN2);
END_FUNCTION

(* Specialized implementation of SUB for TOD *)
FUNCTION SUB__TIME_OF_DAY__TIME_OF_DAY : TIME
VAR_INPUT
    IN1 : TOD;
    IN2 : TOD;
END_VAR
    SUB__TIME_OF_DAY__TIME_OF_DAY := SUB_TOD_TOD(IN1, IN2);
END_FUNCTION

(********************
*
* This operator produces the subtraction of two TOD operands returning TIME
* Panic on overflow
*
******************** *)
{external}
FUNCTION SUB_TOD_TOD : TIME
VAR_INPUT
    IN1 : TOD;
    IN2 : TOD;
END_VAR
END_FUNCTION

(********************
*
* This operator produces the subtraction of two LTOD operands returning LTIME
* Panic on overflow
*
******************** *)
FUNCTION SUB_LTOD_LTOD : LTIME
VAR_INPUT
    IN1 : LTOD;
    IN2 : LTOD;
END_VAR
    SUB_LTOD_LTOD := SUB_TOD_TOD(IN1, IN2);
END_FUNCTION

(* Specialized implementation of SUB for DT and TIME *)
FUNCTION SUB__DATE_AND_TIME__TIME : DT
VAR_INPUT
    IN1 : DT;
    IN2 : TIME;
END_VAR
    SUB__DATE_AND_TIME__TIME := SUB_DT_TIME(IN1, IN2);
END_FUNCTION

(********************
*
* This operator produces the subtraction of DT and TIME returning DT
* Panic on overflow
*
******************** *)
{external}
FUNCTION SUB_DT_TIME : DT
VAR_INPUT
    IN1 : DT;
    IN2 : TIME;
END_VAR
END_FUNCTION

(********************
*
* This operator produces the subtraction of LDT and LTIME returning LDT
* Panic on overflow
*
******************** *)
FUNCTION SUB_LDT_LTIME : LDT
VAR_INPUT
    IN1 : LDT;
    IN2 : LTIME;
END_VAR
    SUB_LDT_LTIME := SUB_DT_TIME(IN1, IN2);
END_FUNCTION

(* Specialized implementation of SUB for DT *)
FUNCTION SUB__DATE_AND_TIME__DATE_AND_TIME : TIME
VAR_INPUT
    IN1 : DT;
    IN2 : DT;
END_VAR
    SUB__DATE_AND_TIME__DATE_AND_TIME := SUB_DT_DT(IN1, IN2);
END_FUNCTION

(********************
*
* This operator produces the subtraction of two DT operands returning TIME
* Panic on overflow
*
******************** *)
{external}
FUNCTION SUB_DT_DT : TIME
VAR_INPUT
    IN1 : DT;
    IN2 : DT;
END_VAR
END_FUNCTION

(********************
*
* This operator produces the subtraction of two LDT operands returning LTIME
* Panic on overflow
*
******************** *)
FUNCTION SUB_LDT_LDT : LTIME
VAR_INPUT
    IN1 : LDT;
    IN2 : LDT;
END_VAR
    SUB_LDT_LDT := SUB_DT_DT(IN1, IN2);
END_FUNCTION

(********************
*
* This operator produces the multiplication of the operands.
*
******************** *)
FUNCTION MUL < T1 : ANY, T2 : ANY > : T1
VAR_INPUT
    IN1 : T1;
    IN2 : T2;
END_VAR
END_FUNCTION

(********************
*
* This operator produces the multiplication of TIME and ANY_NUM.
*
******************** *)
FUNCTION MUL_TIME < T : ANY_NUM > : TIME
VAR_INPUT
    IN1 : TIME;
    IN2 : T;
END_VAR
END_FUNCTION

(********************
*
* This operator produces the multiplication of LTIME and ANY_NUM.
*
******************** *)
FUNCTION MUL_LTIME < T : ANY_NUM > : LTIME
VAR_INPUT
    IN1 : LTIME;
    IN2 : T;
END_VAR
END_FUNCTION

(********************
*
* This operator produces the division of the operands.
*
******************** *)
FUNCTION DIV < T1 : ANY, T2 : ANY > : T1
VAR_INPUT
    IN1 : T1;
    IN2 : T2;
END_VAR
END_FUNCTION

(********************
*
* This operator produces the division of TIME and ANY_NUM.
*
******************** *)
FUNCTION DIV_TIME < T : ANY_NUM > : TIME
VAR_INPUT
    IN1 : TIME;
    IN2 : T;
END_VAR
END_FUNCTION

(********************
*
* This operator produces the division of LTIME and ANY_NUM.
*
******************** *)
FUNCTION DIV_LTIME < T : ANY_NUM > : LTIME
VAR_INPUT
    IN1 : LTIME;
    IN2 : T;
END_VAR
END_FUNCTION (* *******************
*
* Converts LREAL to REAL
*
******************** *)
FUNCTION LREAL_TO_REAL : REAL
VAR_INPUT
    in : LREAL;
END_VAR
    LREAL_TO_REAL := in;
END_FUNCTION

(********************
*
* Converts LREAL to LINT
*
******************** *)
FUNCTION LREAL_TO_LINT : LINT
VAR_INPUT
    in : LREAL;
END_VAR
    LREAL_TO_LINT := ROUND(in);
END_FUNCTION

(********************
*
* Converts LREAL to DINT
*
******************** *)
FUNCTION LREAL_TO_DINT : DINT
VAR_INPUT
    in : LREAL;
END_VAR
    LREAL_TO_DINT := ROUND(in);
END_FUNCTION

(********************
*
* Converts LREAL to INT
*
******************** *)
FUNCTION LREAL_TO_INT : INT
VAR_INPUT
    in : LREAL;
END_VAR
    LREAL_TO_INT := ROUND(in);
END_FUNCTION

(********************
*
* Converts LREAL to SINT
*
******************** *)
FUNCTION LREAL_TO_SINT : SINT
VAR_INPUT
    in : LREAL;
END_VAR
    LREAL_TO_SINT := ROUND(in);
END_FUNCTION

(********************
*
* Converts LREAL to ULINT
*
******************** *)
FUNCTION LREAL_TO_ULINT : ULINT
VAR_INPUT
    in : LREAL;
END_VAR
    LREAL_TO_ULINT := ROUND(in);
END_FUNCTION

(********************
*
* Converts LREAL to UDINT
*
******************** *)
FUNCTION LREAL_TO_UDINT : UDINT
VAR_INPUT
    in : LREAL;
END_VAR
    LREAL_TO_UDINT := ROUND(in);
END_FUNCTION

(********************
*
* Converts LREAL to UINT
*
******************** *)
FUNCTION LREAL_TO_UINT : UINT
VAR_INPUT
    in : LREAL;
END_VAR
    LREAL_TO_UINT := ROUND(in);
END_FUNCTION

(********************
*
* Converts LREAL to USINT
*
******************** *)
FUNCTION LREAL_TO_USINT : USINT
VAR_INPUT
    in : LREAL;
END_VAR
    LREAL_TO_USINT := ROUND(in);
END_FUNCTION

(********************
*
* Converts REAL to LREAL
*
******************** *)
FUNCTION REAL_TO_LREAL : LREAL
VAR_INPUT
    in : REAL;
END_VAR
    REAL_TO_LREAL := in;
END_FUNCTION

(********************
*
* Converts REAL to LINT
*
******************** *)
FUNCTION REAL_TO_LINT : LINT
VAR_INPUT
    in : REAL;
END_VAR
    REAL_TO_LINT := ROUND(in);
END_FUNCTION

(********************
*
* Converts REAL to DINT
*
******************** *)
FUNCTION REAL_TO_DINT : DINT
VAR_INPUT
    in : REAL;
END_VAR
    REAL_TO_DINT := ROUND(in);
END_FUNCTION

(********************
*
* Converts REAL to INT
*
******************** *)
FUNCTION REAL_TO_INT : INT
VAR_INPUT
    in : REAL;
END_VAR
    REAL_TO_INT := ROUND(in);
END_FUNCTION

(********************
*
* Converts REAL to SINT
*
******************** *)
FUNCTION REAL_TO_SINT : SINT
VAR_INPUT
    in : REAL;
END_VAR
    REAL_TO_SINT := ROUND(in);
END_FUNCTION

(********************
*
* Converts REAL to ULINT
*
******************** *)
FUNCTION REAL_TO_ULINT : ULINT
VAR_INPUT
    in : REAL;
END_VAR
    REAL_TO_ULINT := ROUND(in);
END_FUNCTION

(********************
*
* Converts REAL to UDINT
*
******************** *)
FUNCTION REAL_TO_UDINT : UDINT
VAR_INPUT
    in : REAL;
END_VAR
    REAL_TO_UDINT := ROUND(in);
END_FUNCTION

(********************
*
* Converts REAL to UINT
*
******************** *)
FUNCTION REAL_TO_UINT : UINT
VAR_INPUT
    in : REAL;
END_VAR
    REAL_TO_UINT := ROUND(in);
END_FUNCTION

(********************
*
* Converts REAL to USINT
*
******************** *)
FUNCTION REAL_TO_USINT : USINT
VAR_INPUT
    in : REAL;
END_VAR
    REAL_TO_USINT := ROUND(in);
END_FUNCTION

(********************
*
* Converts LINT to LREAL
*
******************** *)
FUNCTION LINT_TO_LREAL : LREAL
VAR_INPUT
    in : LINT;
END_VAR
    LINT_TO_LREAL := in;
END_FUNCTION

(********************
*
* Converts LINT to REAL
*
******************** *)
FUNCTION LINT_TO_REAL : REAL
VAR_INPUT
    in : LINT;
END_VAR
    LINT_TO_REAL := in;
END_FUNCTION

(********************
*
* Converts LINT to DINT
*
******************** *)
FUNCTION LINT_TO_DINT : DINT
VAR_INPUT
    in : LINT;
END_VAR
    LINT_TO_DINT := in;
END_FUNCTION

(********************
*
* Converts LINT to INT
*
******************** *)
FUNCTION LINT_TO_INT : INT
VAR_INPUT
    in : LINT;
END_VAR
    LINT_TO_INT := in;
END_FUNCTION

(********************
*
* Converts LINT to SINT
*
******************** *)
FUNCTION LINT_TO_SINT : SINT
VAR_INPUT
    in : LINT;
END_VAR
    LINT_TO_SINT := in;
END_FUNCTION

(********************
*
* Converts LINT to ULINT
*
******************** *)
FUNCTION LINT_TO_ULINT : ULINT
VAR_INPUT
    in : LINT;
END_VAR
    LINT_TO_ULINT := in;
END_FUNCTION

(********************
*
* Converts LINT to UDINT
*
******************** *)
FUNCTION LINT_TO_UDINT : UDINT
VAR_INPUT
    in : LINT;
END_VAR
    LINT_TO_UDINT := in;
END_FUNCTION

(********************
*
* Converts LINT to UINT
*
******************** *)
FUNCTION LINT_TO_UINT : UINT
VAR_INPUT
    in : LINT;
END_VAR
    LINT_TO_UINT := in;
END_FUNCTION

(********************
*
* Converts LINT to USINT
*
******************** *)
FUNCTION LINT_TO_USINT : USINT
VAR_INPUT
    in : LINT;
END_VAR
    LINT_TO_USINT := in;
END_FUNCTION

(********************
*
* Converts DINT to LREAL
*
******************** *)
FUNCTION DINT_TO_LREAL : LREAL
VAR_INPUT
    in : DINT;
END_VAR
    DINT_TO_LREAL := in;
END_FUNCTION

(********************
*
* Converts DINT to REAL
*
******************** *)
FUNCTION DINT_TO_REAL : REAL
VAR_INPUT
    in : DINT;
END_VAR
    DINT_TO_REAL := in;
END_FUNCTION

(********************
*
* Converts DINT to LINT
*
******************** *)
FUNCTION DINT_TO_LINT : LINT
VAR_INPUT
    in : DINT;
END_VAR
    DINT_TO_LINT := in;
END_FUNCTION

(********************
*
* Converts DINT to INT
*
******************** *)
FUNCTION DINT_TO_INT : INT
VAR_INPUT
    in : DINT;
END_VAR
    DINT_TO_INT := in;
END_FUNCTION

(********************
*
* Converts DINT to SINT
*
******************** *)
FUNCTION DINT_TO_SINT : SINT
VAR_INPUT
    in : DINT;
END_VAR
    DINT_TO_SINT := in;
END_FUNCTION

(********************
*
* Converts DINT to ULINT
*
******************** *)
FUNCTION DINT_TO_ULINT : ULINT
VAR_INPUT
    in : DINT;
END_VAR
    DINT_TO_ULINT := in;
END_FUNCTION

(********************
*
* Converts DINT to UDINT
*
******************** *)
FUNCTION DINT_TO_UDINT : UDINT
VAR_INPUT
    in : DINT;
END_VAR
    DINT_TO_UDINT := in;
END_FUNCTION

(********************
*
* Converts DINT to UINT
*
******************** *)
FUNCTION DINT_TO_UINT : UINT
VAR_INPUT
    in : DINT;
END_VAR
    DINT_TO_UINT := in;
END_FUNCTION

(********************
*
* Converts DINT to USINT
*
******************** *)
FUNCTION DINT_TO_USINT : USINT
VAR_INPUT
    in : DINT;
END_VAR
    DINT_TO_USINT := in;
END_FUNCTION

(********************
*
* Converts INT to LREAL
*
******************** *)
FUNCTION INT_TO_LREAL : LREAL
VAR_INPUT
    in : INT;
END_VAR
    INT_TO_LREAL := in;
END_FUNCTION

(********************
*
* Converts INT to REAL
*
******************** *)
FUNCTION INT_TO_REAL : REAL
VAR_INPUT
    in : INT;
END_VAR
    INT_TO_REAL := in;
END_FUNCTION

(********************
*
* Converts INT to LINT
*
******************** *)
FUNCTION INT_TO_LINT : LINT
VAR_INPUT
    in : INT;
END_VAR
    INT_TO_LINT := in;
END_FUNCTION

(********************
*
* Converts INT to DINT
*
******************** *)
FUNCTION INT_TO_DINT : DINT
VAR_INPUT
    in : INT;
END_VAR
    INT_TO_DINT := in;
END_FUNCTION

(********************
*
* Converts INT to SINT
*
******************** *)
FUNCTION INT_TO_SINT : SINT
VAR_INPUT
    in : INT;
END_VAR
    INT_TO_SINT := in;
END_FUNCTION

(********************
*
* Converts INT to ULINT
*
******************** *)
FUNCTION INT_TO_ULINT : ULINT
VAR_INPUT
    in : INT;
END_VAR
    INT_TO_ULINT := in;
END_FUNCTION

(********************
*
* Converts INT to UDINT
*
******************** *)
FUNCTION INT_TO_UDINT : UDINT
VAR_INPUT
    in : INT;
END_VAR
    INT_TO_UDINT := in;
END_FUNCTION

(********************
*
* Converts INT to UINT
*
******************** *)
FUNCTION INT_TO_UINT : UINT
VAR_INPUT
    in : INT;
END_VAR
    INT_TO_UINT := in;
END_FUNCTION

(********************
*
* Converts INT to USINT
*
******************** *)
FUNCTION INT_TO_USINT : USINT
VAR_INPUT
    in : INT;
END_VAR
    INT_TO_USINT := in;
END_FUNCTION

(********************
*
* Converts SINT to LREAL
*
******************** *)
FUNCTION SINT_TO_LREAL : LREAL
VAR_INPUT
    in : SINT;
END_VAR
    SINT_TO_LREAL := in;
END_FUNCTION

(********************
*
* Converts SINT to REAL
*
******************** *)
FUNCTION SINT_TO_REAL : REAL
VAR_INPUT
    in : SINT;
END_VAR
    SINT_TO_REAL := in;
END_FUNCTION

(********************
*
* Converts SINT to LINT
*
******************** *)
FUNCTION SINT_TO_LINT : LINT
VAR_INPUT
    in : SINT;
END_VAR
    SINT_TO_LINT := in;
END_FUNCTION

(********************
*
* Converts SINT to DINT
*
******************** *)
FUNCTION SINT_TO_DINT : DINT
VAR_INPUT
    in : SINT;
END_VAR
    SINT_TO_DINT := in;
END_FUNCTION

(********************
*
* Converts SINT to INT
*
******************** *)
FUNCTION SINT_TO_INT : INT
VAR_INPUT
    in : SINT;
END_VAR
    SINT_TO_INT := in;
END_FUNCTION

(********************
*
* Converts SINT to ULINT
*
******************** *)
FUNCTION SINT_TO_ULINT : ULINT
VAR_INPUT
    in : SINT;
END_VAR
    SINT_TO_ULINT := in;
END_FUNCTION

(********************
*
* Converts SINT to UDINT
*
******************** *)
FUNCTION SINT_TO_UDINT : UDINT
VAR_INPUT
    in : SINT;
END_VAR
    SINT_TO_UDINT := in;
END_FUNCTION

(********************
*
* Converts SINT to UINT
*
******************** *)
FUNCTION SINT_TO_UINT : UINT
VAR_INPUT
    in : SINT;
END_VAR
    SINT_TO_UINT := in;
END_FUNCTION

(********************
*
* Converts SINT to USINT
*
******************** *)
FUNCTION SINT_TO_USINT : USINT
VAR_INPUT
    in : SINT;
END_VAR
    SINT_TO_USINT := in;
END_FUNCTION

(********************
*
* Converts ULINT to LREAL
*
******************** *)
FUNCTION ULINT_TO_LREAL : LREAL
VAR_INPUT
    in : ULINT;
END_VAR
    ULINT_TO_LREAL := in;
END_FUNCTION

(********************
*
* Converts ULINT to REAL
*
******************** *)
FUNCTION ULINT_TO_REAL : REAL
VAR_INPUT
    in : ULINT;
END_VAR
    ULINT_TO_REAL := in;
END_FUNCTION

(********************
*
* Converts ULINT to LINT
*
******************** *)
FUNCTION ULINT_TO_LINT : LINT
VAR_INPUT
    in : ULINT;
END_VAR
    ULINT_TO_LINT := in;
END_FUNCTION

(********************
*
* Converts ULINT to DINT
*
******************** *)
FUNCTION ULINT_TO_DINT : DINT
VAR_INPUT
    in : ULINT;
END_VAR
    ULINT_TO_DINT := in;
END_FUNCTION

(********************
*
* Converts ULINT to INT
*
******************** *)
FUNCTION ULINT_TO_INT : INT
VAR_INPUT
    in : ULINT;
END_VAR
    ULINT_TO_INT := in;
END_FUNCTION

(********************
*
* Converts ULINT to SINT
*
******************** *)
FUNCTION ULINT_TO_SINT : SINT
VAR_INPUT
    in : ULINT;
END_VAR
    ULINT_TO_SINT := in;
END_FUNCTION

(********************
*
* Converts ULINT to UDINT
*
******************** *)
FUNCTION ULINT_TO_UDINT : UDINT
VAR_INPUT
    in : ULINT;
END_VAR
    ULINT_TO_UDINT := in;
END_FUNCTION

(********************
*
* Converts ULINT to UINT
*
******************** *)
FUNCTION ULINT_TO_UINT : UINT
VAR_INPUT
    in : ULINT;
END_VAR
    ULINT_TO_UINT := in;
END_FUNCTION

(********************
*
* Converts ULINT to USINT
*
******************** *)
FUNCTION ULINT_TO_USINT : USINT
VAR_INPUT
    in : ULINT;
END_VAR
    ULINT_TO_USINT := in;
END_FUNCTION

(********************
*
* Converts UDINT to LREAL
*
******************** *)
FUNCTION UDINT_TO_LREAL : LREAL
VAR_INPUT
    in : UDINT;
END_VAR
    UDINT_TO_LREAL := in;
END_FUNCTION

(********************
*
* Converts UDINT to REAL
*
******************** *)
FUNCTION UDINT_TO_REAL : REAL
VAR_INPUT
    in : UDINT;
END_VAR
    UDINT_TO_REAL := in;
END_FUNCTION

(********************
*
* Converts UDINT to LINT
*
******************** *)
FUNCTION UDINT_TO_LINT : LINT
VAR_INPUT
    in : UDINT;
END_VAR
    UDINT_TO_LINT := in;
END_FUNCTION

(********************
*
* Converts UDINT to DINT
*
******************** *)
FUNCTION UDINT_TO_DINT : DINT
VAR_INPUT
    in : UDINT;
END_VAR
    UDINT_TO_DINT := in;
END_FUNCTION

(********************
*
* Converts UDINT to INT
*
******************** *)
FUNCTION UDINT_TO_INT : INT
VAR_INPUT
    in : UDINT;
END_VAR
    UDINT_TO_INT := in;
END_FUNCTION

(********************
*
* Converts UDINT to SINT
*
******************** *)
FUNCTION UDINT_TO_SINT : SINT
VAR_INPUT
    in : UDINT;
END_VAR
    UDINT_TO_SINT := in;
END_FUNCTION

(********************
*
* Converts UDINT to ULINT
*
******************** *)
FUNCTION UDINT_TO_ULINT : ULINT
VAR_INPUT
    in : UDINT;
END_VAR
    UDINT_TO_ULINT := in;
END_FUNCTION

(********************
*
* Converts UDINT to UINT
*
******************** *)
FUNCTION UDINT_TO_UINT : UINT
VAR_INPUT
    in : UDINT;
END_VAR
    UDINT_TO_UINT := in;
END_FUNCTION

(********************
*
* Converts UDINT to USINT
*
******************** *)
FUNCTION UDINT_TO_USINT : USINT
VAR_INPUT
    in : UDINT;
END_VAR
    UDINT_TO_USINT := in;
END_FUNCTION

(********************
*
* Converts UINT to LREAL
*
******************** *)
FUNCTION UINT_TO_LREAL : LREAL
VAR_INPUT
    in : UINT;
END_VAR
    UINT_TO_LREAL := in;
END_FUNCTION

(********************
*
* Converts UINT to REAL
*
******************** *)
FUNCTION UINT_TO_REAL : REAL
VAR_INPUT
    in : UINT;
END_VAR
    UINT_TO_REAL := in;
END_FUNCTION

(********************
*
* Converts UINT to LINT
*
******************** *)
FUNCTION UINT_TO_LINT : LINT
VAR_INPUT
    in : UINT;
END_VAR
    UINT_TO_LINT := in;
END_FUNCTION

(********************
*
* Converts UINT to DINT
*
******************** *)
FUNCTION UINT_TO_DINT : DINT
VAR_INPUT
    in : UINT;
END_VAR
    UINT_TO_DINT := in;
END_FUNCTION

(********************
*
* Converts UINT to INT
*
******************** *)
FUNCTION UINT_TO_INT : INT
VAR_INPUT
    in : UINT;
END_VAR
    UINT_TO_INT := in;
END_FUNCTION

(********************
*
* Converts UINT to SINT
*
******************** *)
FUNCTION UINT_TO_SINT : SINT
VAR_INPUT
    in : UINT;
END_VAR
    UINT_TO_SINT := in;
END_FUNCTION

(********************
*
* Converts UINT to ULINT
*
******************** *)
FUNCTION UINT_TO_ULINT : ULINT
VAR_INPUT
    in : UINT;
END_VAR
    UINT_TO_ULINT := in;
END_FUNCTION

(********************
*
* Converts UINT to UDINT
*
******************** *)
FUNCTION UINT_TO_UDINT : UDINT
VAR_INPUT
    in : UINT;
END_VAR
    UINT_TO_UDINT := in;
END_FUNCTION

(********************
*
* Converts UINT to USINT
*
******************** *)
FUNCTION UINT_TO_USINT : USINT
VAR_INPUT
    in : UINT;
END_VAR
    UINT_TO_USINT := in;
END_FUNCTION

(********************
*
* Converts USINT to LREAL
*
******************** *)
FUNCTION USINT_TO_LREAL : LREAL
VAR_INPUT
    in : USINT;
END_VAR
    USINT_TO_LREAL := in;
END_FUNCTION

(********************
*
* Converts USINT to REAL
*
******************** *)
FUNCTION USINT_TO_REAL : REAL
VAR_INPUT
    in : USINT;
END_VAR
    USINT_TO_REAL := in;
END_FUNCTION

(********************
*
* Converts USINT to LINT
*
******************** *)
FUNCTION USINT_TO_LINT : LINT
VAR_INPUT
    in : USINT;
END_VAR
    USINT_TO_LINT := in;
END_FUNCTION

(********************
*
* Converts USINT to DINT
*
******************** *)
FUNCTION USINT_TO_DINT : DINT
VAR_INPUT
    in : USINT;
END_VAR
    USINT_TO_DINT := in;
END_FUNCTION

(********************
*
* Converts USINT to INT
*
******************** *)
FUNCTION USINT_TO_INT : INT
VAR_INPUT
    in : USINT;
END_VAR
    USINT_TO_INT := in;
END_FUNCTION

(********************
*
* Converts USINT to SINT
*
******************** *)
FUNCTION USINT_TO_SINT : SINT
VAR_INPUT
    in : USINT;
END_VAR
    USINT_TO_SINT := in;
END_FUNCTION

(********************
*
* Converts USINT to ULINT
*
******************** *)
FUNCTION USINT_TO_ULINT : ULINT
VAR_INPUT
    in : USINT;
END_VAR
    USINT_TO_ULINT := in;
END_FUNCTION

(********************
*
* Converts USINT to UDINT
*
******************** *)
FUNCTION USINT_TO_UDINT : UDINT
VAR_INPUT
    in : USINT;
END_VAR
    USINT_TO_UDINT := in;
END_FUNCTION

(********************
*
* Converts USINT to UINT
*
******************** *)
FUNCTION USINT_TO_UINT : UINT
VAR_INPUT
    in : USINT;
END_VAR
    USINT_TO_UINT := in;
END_FUNCTION (* *****************************************************************************
Description: Rising Edge detection
Input:
  - CLK:  Signal to detect
Output:
  - Q:  Edge detected
Return: Output variable are used for return.
***************************************************************************** *)
{external}
FUNCTION_BLOCK R_TRIG
VAR_INPUT
    CLK : BOOL;
END_VAR
VAR_OUTPUT
    Q : BOOL;
END_VAR
VAR
    M : BOOL;
END_VAR
END_FUNCTION_BLOCK

(******************************************************************************
Description: Falling Edge detection
Input:
  - CLK:  Signal to detect
Output:
  - Q:  Edge detected
Return: Output variable is used for return.
***************************************************************************** *)
{external}
FUNCTION_BLOCK F_TRIG
VAR_INPUT
    CLK : BOOL;
END_VAR
VAR_OUTPUT
    Q : BOOL;
END_VAR
VAR
    M : BOOL;
END_VAR
END_FUNCTION_BLOCK (* *****************************************************************************
*
* Converts BYTE to STRING.
*
***************************************************************************** *)
FUNCTION BYTE_TO_STRING : STRING[__STRING_LENGTH]
VAR_INPUT
    IN : BYTE;
END_VAR
    BYTE_TO_STRING_EXT(IN, BYTE_TO_STRING);
END_FUNCTION

{external}
FUNCTION BYTE_TO_STRING_EXT : DINT
VAR_INPUT
    IN : BYTE;
END_VAR
VAR_IN_OUT
    OUT : STRING[__STRING_LENGTH];
END_VAR
END_FUNCTION

{external}
FUNCTION LWORD_TO_STRING_EXT : DINT
VAR_INPUT
    IN : LWORD;
END_VAR
VAR_IN_OUT
    OUT : STRING[__STRING_LENGTH];
END_VAR
END_FUNCTION

(******************************************************************************
*
* Converts LWORD to STRING.
*
***************************************************************************** *)
FUNCTION LWORD_TO_STRING : STRING[__STRING_LENGTH]
VAR_INPUT
    IN : LWORD;
END_VAR
    LWORD_TO_STRING_EXT(IN, LWORD_TO_STRING);
END_FUNCTION

(******************************************************************************
*
* Converts DWORD to STRING.
*
***************************************************************************** *)
FUNCTION DWORD_TO_STRING : STRING[__STRING_LENGTH]
VAR_INPUT
    IN : DWORD;
END_VAR
    LWORD_TO_STRING_EXT(IN, DWORD_TO_STRING);
END_FUNCTION

{external}
FUNCTION LINT_TO_STRING_EXT : DINT
VAR_INPUT
    IN : LINT;
END_VAR
VAR_IN_OUT
    OUT : STRING[__STRING_LENGTH];
END_VAR
END_FUNCTION

(******************************************************************************
*
* Converts LINT to STRING.
*
***************************************************************************** *)
FUNCTION LINT_TO_STRING : STRING[__STRING_LENGTH]
VAR_INPUT
    IN : LINT;
END_VAR
    LINT_TO_STRING_EXT(IN, LINT_TO_STRING);
END_FUNCTION

(******************************************************************************
*
* Converts DINT to STRING.
*
***************************************************************************** *)
FUNCTION DINT_TO_STRING : STRING[__STRING_LENGTH]
VAR_INPUT
    IN : DINT;
END_VAR
    LINT_TO_STRING_EXT(IN, DINT_TO_STRING);
END_FUNCTION

{external}
FUNCTION LREAL_TO_STRING_EXT : DINT
VAR_INPUT
    IN : LREAL;
END_VAR
VAR_IN_OUT
    OUT : STRING[__STRING_LENGTH];
END_VAR
END_FUNCTION

(******************************************************************************
*
* Converts DT to STRING.
*
***************************************************************************** *)
FUNCTION DT_TO_STRING : STRING[__STRING_LENGTH]
VAR_INPUT
    IN : DT;
END_VAR
    DT_TO_STRING_EXT(IN, DT_TO_STRING);
END_FUNCTION

FUNCTION LDT_TO_STRING : STRING[__STRING_LENGTH]
VAR_INPUT
    IN : DT;
END_VAR
    DT_TO_STRING_EXT(IN, LDT_TO_STRING);
END_FUNCTION

{external}
FUNCTION DT_TO_STRING_EXT : DINT
VAR_INPUT
    IN : DT;
END_VAR
VAR_IN_OUT
    OUT : STRING[__STRING_LENGTH];
END_VAR
END_FUNCTION

(******************************************************************************
*
* Converts DT to WSTRING.
*
***************************************************************************** *)
FUNCTION DT_TO_WSTRING : WSTRING[__STRING_LENGTH]
VAR_INPUT
    IN : DT;
END_VAR
    STRING_TO_WSTRING_EXT(DT_TO_STRING(IN), DT_TO_WSTRING);
END_FUNCTION

FUNCTION LDT_TO_WSTRING : WSTRING[__STRING_LENGTH]
VAR_INPUT
    IN : DT;
END_VAR
    STRING_TO_WSTRING_EXT(DT_TO_STRING(IN), LDT_TO_WSTRING);
END_FUNCTION

(******************************************************************************
*
* Converts TIME to STRING.
*
***************************************************************************** *)
FUNCTION TIME_TO_STRING : STRING[__STRING_LENGTH]
VAR_INPUT
    IN : TIME;
END_VAR
    TIME_TO_STRING_EXT(IN, TIME_TO_STRING);
END_FUNCTION

FUNCTION LTIME_TO_STRING : STRING[__STRING_LENGTH]
VAR_INPUT
    IN : TIME;
END_VAR
    TIME_TO_STRING_EXT(IN, LTIME_TO_STRING);
END_FUNCTION

{external}
FUNCTION TIME_TO_STRING_EXT : DINT
VAR_INPUT
    IN : TIME;
END_VAR
VAR_IN_OUT
    OUT : STRING[__STRING_LENGTH];
END_VAR
END_FUNCTION

(******************************************************************************
*
* Converts TIME to WSTRING.
*
***************************************************************************** *)
FUNCTION TIME_TO_WSTRING : WSTRING[__STRING_LENGTH]
VAR_INPUT
    IN : TIME;
END_VAR
    STRING_TO_WSTRING_EXT(TIME_TO_STRING(IN), TIME_TO_WSTRING);
END_FUNCTION

FUNCTION LTIME_TO_WSTRING : WSTRING[__STRING_LENGTH]
VAR_INPUT
    IN : TIME;
END_VAR
    STRING_TO_WSTRING_EXT(TIME_TO_STRING(IN), LTIME_TO_WSTRING);
END_FUNCTION

(******************************************************************************
*
* Converts DATE to STRING.
*
***************************************************************************** *)
FUNCTION DATE_TO_STRING : STRING[__STRING_LENGTH]
VAR_INPUT
    IN : DATE;
END_VAR
    DATE_TO_STRING_EXT(IN, DATE_TO_STRING);
END_FUNCTION

FUNCTION LDATE_TO_STRING : STRING[__STRING_LENGTH]
VAR_INPUT
    IN : DATE;
END_VAR
    DATE_TO_STRING_EXT(IN, LDATE_TO_STRING);
END_FUNCTION

{external}
FUNCTION DATE_TO_STRING_EXT : DINT
VAR_INPUT
    IN : DATE;
END_VAR
VAR_IN_OUT
    OUT : STRING[__STRING_LENGTH];
END_VAR
END_FUNCTION

(******************************************************************************
*
* Converts DATE to WSTRING.
*
***************************************************************************** *)
FUNCTION DATE_TO_WSTRING : WSTRING[__STRING_LENGTH]
VAR_INPUT
    IN : DATE;
END_VAR
    STRING_TO_WSTRING_EXT(DATE_TO_STRING(IN), DATE_TO_WSTRING);
END_FUNCTION

FUNCTION LDATE_TO_WSTRING : WSTRING[__STRING_LENGTH]
VAR_INPUT
    IN : DATE;
END_VAR
    STRING_TO_WSTRING_EXT(DATE_TO_STRING(IN), LDATE_TO_WSTRING);
END_FUNCTION

(******************************************************************************
*
* Converts TOD to STRING.
*
***************************************************************************** *)
FUNCTION TOD_TO_STRING : STRING[__STRING_LENGTH]
VAR_INPUT
    IN : TOD;
END_VAR
    TOD_TO_STRING_EXT(IN, TOD_TO_STRING);
END_FUNCTION

FUNCTION LTOD_TO_STRING : STRING[__STRING_LENGTH]
VAR_INPUT
    IN : TOD;
END_VAR
    TOD_TO_STRING_EXT(IN, LTOD_TO_STRING);
END_FUNCTION

{external}
FUNCTION TOD_TO_STRING_EXT : DINT
VAR_INPUT
    IN : TOD;
END_VAR
VAR_IN_OUT
    OUT : STRING[__STRING_LENGTH];
END_VAR
END_FUNCTION

(******************************************************************************
*
* Converts TOD to WSTRING.
*
***************************************************************************** *)
FUNCTION TOD_TO_WSTRING : WSTRING[__STRING_LENGTH]
VAR_INPUT
    IN : TOD;
END_VAR
    STRING_TO_WSTRING_EXT(TOD_TO_STRING(IN), TOD_TO_WSTRING);
END_FUNCTION

FUNCTION LTOD_TO_WSTRING : WSTRING[__STRING_LENGTH]
VAR_INPUT
    IN : TOD;
END_VAR
    STRING_TO_WSTRING_EXT(TOD_TO_STRING(IN), LTOD_TO_WSTRING);
END_FUNCTION

(******************************************************************************
*
* Converts BYTE to WSTRING.
*
***************************************************************************** *)
FUNCTION BYTE_TO_WSTRING : WSTRING[__STRING_LENGTH]
VAR_INPUT
    IN : BYTE;
END_VAR
    STRING_TO_WSTRING_EXT(BYTE_TO_STRING(IN), BYTE_TO_WSTRING);
END_FUNCTION

(******************************************************************************
*
* Converts LWORD to WSTRING.
*
***************************************************************************** *)
FUNCTION LWORD_TO_WSTRING : WSTRING[__STRING_LENGTH]
VAR_INPUT
    IN : LWORD;
END_VAR
    STRING_TO_WSTRING_EXT(LWORD_TO_STRING(IN), LWORD_TO_WSTRING);
END_FUNCTION

(******************************************************************************
*
* Converts DWORD to WSTRING.
*
***************************************************************************** *)
FUNCTION DWORD_TO_WSTRING : WSTRING[__STRING_LENGTH]
VAR_INPUT
    IN : DWORD;
END_VAR
    DWORD_TO_WSTRING := LWORD_TO_WSTRING(IN);
END_FUNCTION

(******************************************************************************
*
* Converts LINT to WSTRING.
*
***************************************************************************** *)
FUNCTION LINT_TO_WSTRING : WSTRING[__STRING_LENGTH]
VAR_INPUT
    IN : LINT;
END_VAR
    STRING_TO_WSTRING_EXT(LINT_TO_STRING(IN), LINT_TO_WSTRING);
END_FUNCTION

(******************************************************************************
*
* Converts DINT to WSTRING.
*
***************************************************************************** *)
FUNCTION DINT_TO_WSTRING : WSTRING[__STRING_LENGTH]
VAR_INPUT
    IN : DINT;
END_VAR
    DINT_TO_WSTRING := LINT_TO_WSTRING(IN);
END_FUNCTION

(******************************************************************************
*
* Converts LREAL to STRING.
*
***************************************************************************** *)
FUNCTION LREAL_TO_STRING : STRING[__STRING_LENGTH]
VAR_INPUT
    IN : LREAL;
END_VAR
    LREAL_TO_STRING_EXT(IN, LREAL_TO_STRING);
END_FUNCTION

(******************************************************************************
*
* Converts REAL to STRING.
*
***************************************************************************** *)
FUNCTION REAL_TO_STRING : STRING[__STRING_LENGTH]
VAR_INPUT
    IN : REAL;
END_VAR
    LREAL_TO_STRING_EXT(IN, REAL_TO_STRING);
END_FUNCTION

(******************************************************************************
*
* Converts LREAL to WSTRING.
*
***************************************************************************** *)
FUNCTION LREAL_TO_WSTRING : WSTRING[__STRING_LENGTH]
VAR_INPUT
    IN : LREAL;
END_VAR
    STRING_TO_WSTRING_EXT(LREAL_TO_STRING(IN), LREAL_TO_WSTRING);
END_FUNCTION

(******************************************************************************
*
* Converts REAL to WSTRING.
*
***************************************************************************** *)
FUNCTION REAL_TO_WSTRING : WSTRING[__STRING_LENGTH]
VAR_INPUT
    IN : REAL;
END_VAR
    REAL_TO_WSTRING := LREAL_TO_WSTRING(IN);
END_FUNCTION

(******************************************************************************
*
* Converts TOD to LWORD.
*
***************************************************************************** *)
FUNCTION TOD_TO_LWORD : LWORD
VAR_INPUT
    IN : TOD;
END_VAR
    TOD_TO_LWORD := IN;
END_FUNCTION

FUNCTION LTOD_TO_LWORD : LWORD
VAR_INPUT
    IN : TOD;
END_VAR
    LTOD_TO_LWORD := IN;
END_FUNCTION

(******************************************************************************
*
* Converts DT to LWORD.
*
***************************************************************************** *)
FUNCTION DT_TO_LWORD : LWORD
VAR_INPUT
    IN : DT;
END_VAR
    DT_TO_LWORD := IN;
END_FUNCTION

FUNCTION LDT_TO_LWORD : LWORD
VAR_INPUT
    IN : DT;
END_VAR
    LDT_TO_LWORD := IN;
END_FUNCTION

(******************************************************************************
*
* Converts DATE to LWORD.
*
***************************************************************************** *)
FUNCTION DATE_TO_LWORD : LWORD
VAR_INPUT
    IN : DATE;
END_VAR
    DATE_TO_LWORD := IN;
END_FUNCTION

FUNCTION LDATE_TO_LWORD : LWORD
VAR_INPUT
    IN : DATE;
END_VAR
    LDATE_TO_LWORD := IN;
END_FUNCTION

(******************************************************************************
*
* Converts TIME to LWORD.
*
***************************************************************************** *)
FUNCTION TIME_TO_LWORD : LWORD
VAR_INPUT
    IN : TIME;
END_VAR
    TIME_TO_LWORD := IN;
END_FUNCTION

FUNCTION LTIME_TO_LWORD : LWORD
VAR_INPUT
    IN : TIME;
END_VAR
    LTIME_TO_LWORD := IN;
END_FUNCTION

(******************************************************************************
*
* Converts STRING to LINT.
*
***************************************************************************** *)
{external}
FUNCTION STRING_TO_LINT : LINT
VAR_INPUT {ref}
    IN : STRING;
END_VAR
END_FUNCTION

(******************************************************************************
*
* Converts WSTRING to LINT.
*
***************************************************************************** *)
FUNCTION WSTRING_TO_LINT : LINT
VAR_INPUT {ref}
    IN : WSTRING;
END_VAR
VAR_TEMP
    STR : STRING;
END_VAR
    WSTRING_TO_STRING_EXT(IN, STR);
    WSTRING_TO_LINT := STRING_TO_LINT(STR);
END_FUNCTION

(******************************************************************************
*
* Converts TIME to LINT.
*
***************************************************************************** *)
FUNCTION TIME_TO_LINT : LINT
VAR_INPUT
    IN : TIME;
END_VAR
    TIME_TO_LINT := IN;
END_FUNCTION

FUNCTION LTIME_TO_LINT : LINT
VAR_INPUT
    IN : TIME;
END_VAR
    LTIME_TO_LINT := IN;
END_FUNCTION

(******************************************************************************
*
* Converts TOD to LINT.
*
***************************************************************************** *)
FUNCTION TOD_TO_LINT : LINT
VAR_INPUT
    IN : TOD;
END_VAR
    TOD_TO_LINT := IN;
END_FUNCTION

FUNCTION LTOD_TO_LINT : LINT
VAR_INPUT
    IN : TOD;
END_VAR
    LTOD_TO_LINT := IN;
END_FUNCTION

(******************************************************************************
*
* Converts DT to LINT.
*
***************************************************************************** *)
FUNCTION DT_TO_LINT : LINT
VAR_INPUT
    IN : DT;
END_VAR
    DT_TO_LINT := IN;
END_FUNCTION

FUNCTION LDT_TO_LINT : LINT
VAR_INPUT
    IN : DT;
END_VAR
    LDT_TO_LINT := IN;
END_FUNCTION

(******************************************************************************
*
* Converts DATE to LINT.
*
***************************************************************************** *)
FUNCTION DATE_TO_LINT : LINT
VAR_INPUT
    IN : DATE;
END_VAR
    DATE_TO_LINT := IN;
END_FUNCTION

FUNCTION LDATE_TO_LINT : LINT
VAR_INPUT
    IN : DATE;
END_VAR
    LDATE_TO_LINT := IN;
END_FUNCTION

(******************************************************************************
*
* Converts DT to ULINT.
*
***************************************************************************** *)
FUNCTION DT_TO_ULINT : ULINT
VAR_INPUT
    IN : DT;
END_VAR
    DT_TO_ULINT := IN;
END_FUNCTION

FUNCTION LDT_TO_ULINT : ULINT
VAR_INPUT
    IN : DT;
END_VAR
    LDT_TO_ULINT := IN;
END_FUNCTION

(******************************************************************************
*
* Converts TIME to ULINT.
*
***************************************************************************** *)
FUNCTION TIME_TO_ULINT : ULINT
VAR_INPUT
    IN : TIME;
END_VAR
    TIME_TO_ULINT := IN;
END_FUNCTION

FUNCTION LTIME_TO_ULINT : ULINT
VAR_INPUT
    IN : TIME;
END_VAR
    LTIME_TO_ULINT := IN;
END_FUNCTION

(******************************************************************************
*
* Converts DATE to ULINT.
*
***************************************************************************** *)
FUNCTION DATE_TO_ULINT : ULINT
VAR_INPUT
    IN : DATE;
END_VAR
    DATE_TO_ULINT := IN;
END_FUNCTION

FUNCTION LDATE_TO_ULINT : ULINT
VAR_INPUT
    IN : DATE;
END_VAR
    LDATE_TO_ULINT := IN;
END_FUNCTION

(******************************************************************************
*
* Converts TOD to ULINT.
*
***************************************************************************** *)
FUNCTION TOD_TO_ULINT : ULINT
VAR_INPUT
    IN : TOD;
END_VAR
    TOD_TO_ULINT := IN;
END_FUNCTION

FUNCTION LTOD_TO_ULINT : ULINT
VAR_INPUT
    IN : TOD;
END_VAR
    LTOD_TO_ULINT := IN;
END_FUNCTION

(******************************************************************************
*
* Converts STRING to DINT.
*
***************************************************************************** *)
{external}
FUNCTION STRING_TO_DINT : DINT
VAR_INPUT {ref}
    IN : STRING;
END_VAR
END_FUNCTION

(******************************************************************************
*
* Converts WSTRING to DINT.
*
***************************************************************************** *)
FUNCTION WSTRING_TO_DINT : DINT
VAR_INPUT {ref}
    IN : WSTRING;
END_VAR
VAR_TEMP
    STR : STRING;
END_VAR
    WSTRING_TO_STRING_EXT(IN, STR);
    WSTRING_TO_DINT := STRING_TO_LINT(STR);
END_FUNCTION

(******************************************************************************
*
* Converts STRING to LREAL.
*
***************************************************************************** *)
{external}
FUNCTION STRING_TO_LREAL : LREAL
VAR_INPUT {ref}
    IN : STRING;
END_VAR
END_FUNCTION

(******************************************************************************
*
* Converts STRING to REAL.
*
***************************************************************************** *)
{external}
FUNCTION STRING_TO_REAL : REAL
VAR_INPUT {ref}
    IN : STRING;
END_VAR
END_FUNCTION

(******************************************************************************
*
* Converts WSTRING to LREAL.
*
***************************************************************************** *)
FUNCTION WSTRING_TO_LREAL : LREAL
VAR_INPUT {ref}
    IN : WSTRING;
END_VAR
VAR_TEMP
    STR : STRING;
END_VAR
    WSTRING_TO_STRING_EXT(IN, STR);
    WSTRING_TO_LREAL := STRING_TO_LREAL(STR);
END_FUNCTION

(******************************************************************************
*
* Converts WSTRING to REAL.
*
***************************************************************************** *)
FUNCTION WSTRING_TO_REAL : REAL
VAR_INPUT {ref}
    IN : WSTRING;
END_VAR
VAR_TEMP
    STR : STRING;
END_VAR
    WSTRING_TO_STRING_EXT(IN, STR);
    WSTRING_TO_REAL := STRING_TO_REAL(STR);
END_FUNCTION


(******************************************************************************
*
* Converts TIME to LREAL.
*
***************************************************************************** *)
FUNCTION TIME_TO_LREAL : LREAL
VAR_INPUT
    IN : TIME;
END_VAR
    TIME_TO_LREAL := IN;
END_FUNCTION

FUNCTION LTIME_TO_LREAL : LREAL
VAR_INPUT
    IN : TIME;
END_VAR
    LTIME_TO_LREAL := IN;
END_FUNCTION

(******************************************************************************
*
* Converts LREAL to TIME.
*
***************************************************************************** *)
FUNCTION LREAL_TO_TIME : TIME
VAR_INPUT
    IN : LREAL;
END_VAR
// TODO: seconds.millis? round as nanos?
    LREAL_TO_TIME := ROUND(IN);
END_FUNCTION

FUNCTION LREAL_TO_LTIME : TIME
VAR_INPUT
    IN : LREAL;
END_VAR
    LREAL_TO_LTIME := ROUND(IN);
END_FUNCTION

(******************************************************************************
*
* Converts LWORD to DATE.
*
***************************************************************************** *)
FUNCTION LWORD_TO_DATE : DATE
VAR_INPUT
    IN : LWORD;
END_VAR
    LWORD_TO_DATE := IN;
END_FUNCTION

FUNCTION LWORD_TO_LDATE : DATE
VAR_INPUT
    IN : LWORD;
END_VAR
    LWORD_TO_LDATE := IN;
END_FUNCTION

(******************************************************************************
*
* Converts LWORD to DT.
*
***************************************************************************** *)
FUNCTION LWORD_TO_DT : DT
VAR_INPUT
    IN : LWORD;
END_VAR
    LWORD_TO_DT := IN;
END_FUNCTION

FUNCTION LWORD_TO_LDT : DT
VAR_INPUT
    IN : LWORD;
END_VAR
    LWORD_TO_LDT := IN;
END_FUNCTION

(******************************************************************************
*
* Converts LWORD to TOD.
*
***************************************************************************** *)
FUNCTION LWORD_TO_TOD : TOD
VAR_INPUT
    IN : LWORD;
END_VAR
    LWORD_TO_TOD := IN;
END_FUNCTION

FUNCTION LWORD_TO_LTOD : TOD
VAR_INPUT
    IN : LWORD;
END_VAR
    LWORD_TO_LTOD := IN;
END_FUNCTION

(******************************************************************************
*
* Converts LWORD to TIME.
*
***************************************************************************** *)
FUNCTION LWORD_TO_TIME : TIME
VAR_INPUT
    IN : LWORD;
END_VAR
    LWORD_TO_TIME := IN;
END_FUNCTION

FUNCTION LWORD_TO_LTIME : TIME
VAR_INPUT
    IN : LWORD;
END_VAR
    LWORD_TO_LTIME := IN;
END_FUNCTION

(******************************************************************************
*
* Converts ULINT to DATE.
*
***************************************************************************** *)
FUNCTION ULINT_TO_DATE : DATE
VAR_INPUT
    IN : ULINT;
END_VAR
    ULINT_TO_DATE := IN;
END_FUNCTION

FUNCTION ULINT_TO_LDATE : DATE
VAR_INPUT
    IN : ULINT;
END_VAR
    ULINT_TO_LDATE := IN;
END_FUNCTION

(******************************************************************************
*
* Converts ULINT to DT.
*
***************************************************************************** *)
FUNCTION ULINT_TO_DT : DT
VAR_INPUT
    IN : ULINT;
END_VAR
    ULINT_TO_DT := IN;
END_FUNCTION

FUNCTION ULINT_TO_LDT : DT
VAR_INPUT
    IN : ULINT;
END_VAR
    ULINT_TO_LDT := IN;
END_FUNCTION

(******************************************************************************
*
* Converts ULINT to TOD.
*
***************************************************************************** *)
FUNCTION ULINT_TO_TOD : TOD
VAR_INPUT
    IN : ULINT;
END_VAR
    ULINT_TO_TOD := IN;
END_FUNCTION

FUNCTION ULINT_TO_LTOD : TOD
VAR_INPUT
    IN : ULINT;
END_VAR
    ULINT_TO_LTOD := IN;
END_FUNCTION

(******************************************************************************
*
* Converts ULINT to TIME.
*
***************************************************************************** *)
FUNCTION ULINT_TO_TIME : TIME
VAR_INPUT
    IN : ULINT;
END_VAR
    ULINT_TO_TIME := IN;
END_FUNCTION

FUNCTION ULINT_TO_LTIME : LTIME
VAR_INPUT
    IN : ULINT;
END_VAR
    ULINT_TO_LTIME := IN;
END_FUNCTION

(******************************************************************************
*
* Converts LINT to TIME.
*
***************************************************************************** *)
FUNCTION LINT_TO_TIME : TIME
VAR_INPUT
    IN : LINT;
END_VAR
    LINT_TO_TIME := IN;
END_FUNCTION

FUNCTION LINT_TO_LTIME : TIME
VAR_INPUT
    IN : LINT;
END_VAR
    LINT_TO_LTIME := IN;
END_FUNCTION

(******************************************************************************
*
* Converts LINT to TOD.
*
***************************************************************************** *)
FUNCTION LINT_TO_TOD : TOD
VAR_INPUT
    IN : LINT;
END_VAR
    LINT_TO_TOD := IN;
END_FUNCTION

FUNCTION LINT_TO_LTOD : TOD
VAR_INPUT
    IN : LINT;
END_VAR
    LINT_TO_LTOD := IN;
END_FUNCTION

(******************************************************************************
*
* Converts LINT to DATE.
*
***************************************************************************** *)
FUNCTION LINT_TO_DATE : DATE
VAR_INPUT
    IN : LINT;
END_VAR
    LINT_TO_DATE := IN;
END_FUNCTION

FUNCTION LINT_TO_LDATE : DATE
VAR_INPUT
    IN : LINT;
END_VAR
    LINT_TO_LDATE := IN;
END_FUNCTION

(******************************************************************************
*
* Converts LINT to BOOL.
*
***************************************************************************** *)
FUNCTION LINT_TO_BOOL : BOOL
VAR_INPUT
    IN : LINT;
END_VAR
   IF IN = 0 THEN
    LINT_TO_BOOL := FALSE;
   ELSE
    LINT_TO_BOOL := TRUE;
   END_IF;
END_FUNCTION

(******************************************************************************
*
* Converts DINT to BOOL.
*
***************************************************************************** *)
FUNCTION DINT_TO_BOOL : BOOL
VAR_INPUT
    IN : DINT;
END_VAR
   DINT_TO_BOOL := LINT_TO_BOOL(IN);
END_FUNCTION

(******************************************************************************
*
* Truncate REAL_TYPE to INT_TYPE.
*
***************************************************************************** *)
FUNCTION TRUNC < T : ANY_REAL > : LINT
VAR_INPUT
    IN : T;
END_VAR
END_FUNCTION

FUNCTION TRUNC__REAL : LINT
VAR_INPUT
    IN : REAL;
END_VAR
    TRUNC__REAL := IN;
END_FUNCTION

FUNCTION TRUNC__LREAL : LINT
VAR_INPUT
    IN : LREAL;
END_VAR
    TRUNC__LREAL := IN;
END_FUNCTION

(******************************************************************************
*
* Returns the local TIME since midnight, which can be used make relative measurements
* by calculating the difference between TIME return-values.
*
***************************************************************************** *)
{external}
FUNCTION TIME : TIME END_FUNCTION

(******************************************************************************
*
* Converts USINT to STRING.
*
***************************************************************************** *)
FUNCTION USINT_TO_STRING : STRING[__STRING_LENGTH]
VAR_INPUT
    IN : USINT;
END_VAR
    LWORD_TO_STRING_EXT(IN, USINT_TO_STRING);
END_FUNCTION

(******************************************************************************
*
* Converts UINT to STRING.
*
***************************************************************************** *)
FUNCTION UINT_TO_STRING : STRING[__STRING_LENGTH]
VAR_INPUT
    IN : UINT;
END_VAR
    LWORD_TO_STRING_EXT(IN, UINT_TO_STRING);
END_FUNCTION

(******************************************************************************
*
* Converts UDINT to STRING.
*
***************************************************************************** *)
FUNCTION UDINT_TO_STRING : STRING[__STRING_LENGTH]
VAR_INPUT
    IN : UDINT;
END_VAR
    LWORD_TO_STRING_EXT(IN, UDINT_TO_STRING);
END_FUNCTION

(******************************************************************************
*
* Converts ULINT to STRING.
*
***************************************************************************** *)
FUNCTION ULINT_TO_STRING : STRING[__STRING_LENGTH]
VAR_INPUT
    IN : ULINT;
END_VAR
    LWORD_TO_STRING_EXT(IN, ULINT_TO_STRING);
END_FUNCTION (* *****************************************************************************
Description: Counter up
Input:
  - CU: Count up
  - R: Reset counter to 0
  - PV: Counter limit
Output:
  - Q: Counter reached limit
  - CV: Current counter value
Return: Output is used as return value
***************************************************************************** *)
{external}
FUNCTION_BLOCK CTU
VAR_INPUT
    CU : BOOL;
    R : BOOL;
    PV : INT;
END_VAR
VAR_OUTPUT
    Q : BOOL;
    CV : INT;
END_VAR
VAR
    M : BOOL;
END_VAR
END_FUNCTION_BLOCK

(******************************************************************************
Description: Counter up
Input:
  - CU: Count up
  - R: Reset counter to 0
  - PV: Counter limit
Output:
  - Q: Counter reached limit
  - CV: Current counter value
Return: Output is used as return value
***************************************************************************** *)
{external}
FUNCTION_BLOCK CTU_INT
VAR_INPUT
    CU : BOOL;
    R : BOOL;
    PV : INT;
END_VAR
VAR_OUTPUT
    Q : BOOL;
    CV : INT;
END_VAR
VAR
    M : BOOL;
END_VAR
END_FUNCTION_BLOCK

(******************************************************************************
Description: Counter up
Input:
  - CU: Count up
  - R: Reset counter to 0
  - PV: Counter limit
Output:
  - Q: Counter reached limit
  - CV: Current counter value
Return: Output is used as return value
***************************************************************************** *)
{external}
FUNCTION_BLOCK CTU_DINT
VAR_INPUT
    CU : BOOL;
    R : BOOL;
    PV : DINT;
END_VAR
VAR_OUTPUT
    Q : BOOL;
    CV : DINT;
END_VAR
VAR
    M : BOOL;
END_VAR
END_FUNCTION_BLOCK

(******************************************************************************
Description: Counter up
Input:
  - CU: Count up
  - R: Reset counter to 0
  - PV: Counter limit
Output:
  - Q: Counter reached limit
  - CV: Current counter value
Return: Output is used as return value
***************************************************************************** *)
{external}
FUNCTION_BLOCK CTU_UDINT
VAR_INPUT
    CU : BOOL;
    R : BOOL;
    PV : UDINT;
END_VAR
VAR_OUTPUT
    Q : BOOL;
    CV : UDINT;
END_VAR
VAR
    M : BOOL;
END_VAR
END_FUNCTION_BLOCK

(******************************************************************************
Description: Counter up
Input:
  - CU: Count up
  - R: Reset counter to 0
  - PV: Counter limit
Output:
  - Q: Counter reached limit
  - CV: Current counter value
Return: Output is used as return value
***************************************************************************** *)
{external}
FUNCTION_BLOCK CTU_LINT
VAR_INPUT
    CU : BOOL;
    R : BOOL;
    PV : LINT;
END_VAR
VAR_OUTPUT
    Q : BOOL;
    CV : LINT;
END_VAR
VAR
    M : BOOL;
END_VAR
END_FUNCTION_BLOCK

(******************************************************************************
Description: Counter up
Input:
  - CU: Count up
  - R: Reset counter to 0
  - PV: Counter limit
Output:
  - Q: Counter reached limit
  - CV: Current counter value
Return: Output is used as return value
***************************************************************************** *)
{external}
FUNCTION_BLOCK CTU_ULINT
VAR_INPUT
    CU : BOOL;
    R : BOOL;
    PV : ULINT;
END_VAR
VAR_OUTPUT
    Q : BOOL;
    CV : ULINT;
END_VAR
VAR
    M : BOOL;
END_VAR
END_FUNCTION_BLOCK

(******************************************************************************
Description: Counter down
Input:
  - CD: Count down
  - LD: Load start value
  - PV: Start value
Output:
  - Q: Counter reached 0
  - CV: Current counter value
Return: Output is used as return value
***************************************************************************** *)
{external}
FUNCTION_BLOCK CTD
VAR_INPUT
    CD : BOOL;
    LD : BOOL;
    PV : INT;
END_VAR
VAR_OUTPUT
    Q : BOOL;
    CV : INT;
END_VAR
VAR
    M : BOOL;
END_VAR
END_FUNCTION_BLOCK

(******************************************************************************
Description: Counter down
Input:
  - CD: Count down
  - LD: Load start value
  - PV: Start value
Output:
  - Q: Counter reached 0
  - CV: Current counter value
Return: Output is used as return value
***************************************************************************** *)
{external}
FUNCTION_BLOCK CTD_INT
VAR_INPUT
    CD : BOOL;
    LD : BOOL;
    PV : INT;
END_VAR
VAR_OUTPUT
    Q : BOOL;
    CV : INT;
END_VAR
VAR
    M : BOOL;
END_VAR
END_FUNCTION_BLOCK

(******************************************************************************
Description: Counter down
Input:
  - CD: Count down
  - LD: Load start value
  - PV: Start value
Output:
  - Q: Counter reached 0
  - CV: Current counter value
Return: Output is used as return value
***************************************************************************** *)
{external}
FUNCTION_BLOCK CTD_DINT
VAR_INPUT
    CD : BOOL;
    LD : BOOL;
    PV : DINT;
END_VAR
VAR_OUTPUT
    Q : BOOL;
    CV : DINT;
END_VAR
VAR
    M : BOOL;
END_VAR
END_FUNCTION_BLOCK

(******************************************************************************
Description: Counter down
Input:
  - CD: Count down
  - LD: Load start value
  - PV: Start value
Output:
  - Q: Counter reached 0
  - CV: Current counter value
Return: Output is used as return value
***************************************************************************** *)
{external}
FUNCTION_BLOCK CTD_UDINT
VAR_INPUT
    CD : BOOL;
    LD : BOOL;
    PV : UDINT;
END_VAR
VAR_OUTPUT
    Q : BOOL;
    CV : UDINT;
END_VAR
VAR
    M : BOOL;
END_VAR
END_FUNCTION_BLOCK

(******************************************************************************
Description: Counter down
Input:
  - CD: Count down
  - LD: Load start value
  - PV: Start value
Output:
  - Q: Counter reached 0
  - CV: Current counter value
Return: Output is used as return value
***************************************************************************** *)
{external}
FUNCTION_BLOCK CTD_LINT
VAR_INPUT
    CD : BOOL;
    LD : BOOL;
    PV : LINT;
END_VAR
VAR_OUTPUT
    Q : BOOL;
    CV : LINT;
END_VAR
VAR
    M : BOOL;
END_VAR
END_FUNCTION_BLOCK

(******************************************************************************
Description: Counter down
Input:
  - CD: Count down
  - LD: Load start value
  - PV: Start value
Output:
  - Q: Counter reached 0
  - CV: Current counter value
Return: Output is used as return value
***************************************************************************** *)
{external}
FUNCTION_BLOCK CTD_ULINT
VAR_INPUT
    CD : BOOL;
    LD : BOOL;
    PV : ULINT;
END_VAR
VAR_OUTPUT
    Q : BOOL;
    CV : ULINT;
END_VAR
VAR
    M : BOOL;
END_VAR
END_FUNCTION_BLOCK

(******************************************************************************
Description: Counter up and down
Input:
  - CU: Count up
  - CD: Count down
  - R: Reset counter to 0
  - LD: Load start value
  - PV: Start value / counter limit
Output:
  - QU: Counter reached limit
  - QD: Counter reached  0
  - CV: Current counter value
Return: Output is used as return value
***************************************************************************** *)
{external}
FUNCTION_BLOCK CTUD
VAR_INPUT
    CU : BOOL;
    CD : BOOL;
    R : BOOL;
    LD : BOOL;
    PV : INT;
END_VAR
VAR_OUTPUT
    QU : BOOL;
    QD : BOOL;
    CV : INT;
END_VAR
VAR
    MU : BOOL;
    MD : BOOL;
END_VAR
END_FUNCTION_BLOCK

(******************************************************************************
Description: Counter up and down
Input:
  - CU: Count up
  - CD: Count down
  - R: Reset counter to 0
  - LD: Load start value
  - PV: Start value / counter limit
Output:
  - QU: Counter reached limit
  - QD: Counter reached  0
  - CV: Current counter value
Return: Output is used as return value
***************************************************************************** *)
{external}
FUNCTION_BLOCK CTUD_INT
VAR_INPUT
    CU : BOOL;
    CD : BOOL;
    R : BOOL;
    LD : BOOL;
    PV : INT;
END_VAR
VAR_OUTPUT
    QU : BOOL;
    QD : BOOL;
    CV : INT;
END_VAR
VAR
    MU : BOOL;
    MD : BOOL;
END_VAR
END_FUNCTION_BLOCK

(******************************************************************************
Description: Counter up and down
Input:
  - CU: Count up
  - CD: Count down
  - R: Reset counter to 0
  - LD: Load start value
  - PV: Start value / counter limit
Output:
  - QU: Counter reached limit
  - QD: Counter reached  0
  - CV: Current counter value
Return: Output is used as return value
***************************************************************************** *)
{external}
FUNCTION_BLOCK CTUD_DINT
VAR_INPUT
    CU : BOOL;
    CD : BOOL;
    R : BOOL;
    LD : BOOL;
    PV : DINT;
END_VAR
VAR_OUTPUT
    QU : BOOL;
    QD : BOOL;
    CV : DINT;
END_VAR
VAR
    MU : BOOL;
    MD : BOOL;
END_VAR
END_FUNCTION_BLOCK

(******************************************************************************
Description: Counter up and down
Input:
  - CU: Count up
  - CD: Count down
  - R: Reset counter to 0
  - LD: Load start value
  - PV: Start value / counter limit
Output:
  - QU: Counter reached limit
  - QD: Counter reached  0
  - CV: Current counter value
Return: Output is used as return value
***************************************************************************** *)
{external}
FUNCTION_BLOCK CTUD_UDINT
VAR_INPUT
    CU : BOOL;
    CD : BOOL;
    R : BOOL;
    LD : BOOL;
    PV : UDINT;
END_VAR
VAR_OUTPUT
    QU : BOOL;
    QD : BOOL;
    CV : UDINT;
END_VAR
VAR
    MU : BOOL;
    MD : BOOL;
END_VAR
END_FUNCTION_BLOCK

(******************************************************************************
Description: Counter up and down
Input:
  - CU: Count up
  - CD: Count down
  - R: Reset counter to 0
  - LD: Load start value
  - PV: Start value / counter limit
Output:
  - QU: Counter reached limit
  - QD: Counter reached  0
  - CV: Current counter value
Return: Output is used as return value
***************************************************************************** *)
{external}
FUNCTION_BLOCK CTUD_LINT
VAR_INPUT
    CU : BOOL;
    CD : BOOL;
    R : BOOL;
    LD : BOOL;
    PV : LINT;
END_VAR
VAR_OUTPUT
    QU : BOOL;
    QD : BOOL;
    CV : LINT;
END_VAR
VAR
    MU : BOOL;
    MD : BOOL;
END_VAR
END_FUNCTION_BLOCK

(******************************************************************************
Description: Counter up and down
Input:
  - CU: Count up
  - CD: Count down
  - R: Reset counter to 0
  - LD: Load start value
  - PV: Start value / counter limit
Output:
  - QU: Counter reached limit
  - QD: Counter reached  0
  - CV: Current counter value
Return: Output is used as return value
***************************************************************************** *)
{external}
FUNCTION_BLOCK CTUD_ULINT
VAR_INPUT
    CU : BOOL;
    CD : BOOL;
    R : BOOL;
    LD : BOOL;
    PV : ULINT;
END_VAR
VAR_OUTPUT
    QU : BOOL;
    QD : BOOL;
    CV : ULINT;
END_VAR
VAR
    MU : BOOL;
    MD : BOOL;
END_VAR
END_FUNCTION_BLOCK (* *******************
*
* Converts little endian data format to big endian data format
*
******************** *)
{external}
FUNCTION TO_BIG_ENDIAN < T : ANY > : T
VAR_INPUT
    in : T;
END_VAR
END_FUNCTION

(********************
*
* Converts big endian data format to little endian data format
*
******************** *)
{external}
FUNCTION TO_LITTLE_ENDIAN < T : ANY > : T
VAR_INPUT
    in : T;
END_VAR
END_FUNCTION

(********************
*
* Converts to little endian data format from big endian data format
*
******************** *)
{external}
FUNCTION FROM_BIG_ENDIAN < T : ANY > : T
VAR_INPUT
    in : T;
END_VAR
END_FUNCTION

(********************
*
* Converts to big endian data format from little endian data format
*
******************** *)
{external}
FUNCTION FROM_LITTLE_ENDIAN < T : ANY > : T
VAR_INPUT
    in : T;
END_VAR
END_FUNCTION

(********************
*
* Converts LWORD to LREAL
*
******************** *)
{external}
FUNCTION LWORD_TO_LREAL : LREAL
VAR_INPUT
    in : LWORD;
END_VAR
END_FUNCTION

(********************
*
* Converts LWORD to LINT
*
******************** *)
FUNCTION LWORD_TO_LINT : LINT
VAR_INPUT
    in : LWORD;
END_VAR
    LWORD_TO_LINT := in;
END_FUNCTION

(********************
*
* Converts LWORD to DINT
*
******************** *)
FUNCTION LWORD_TO_DINT : DINT
VAR_INPUT
    in : LWORD;
END_VAR
    LWORD_TO_DINT := in;
END_FUNCTION

(********************
*
* Converts LWORD to INT
*
******************** *)
FUNCTION LWORD_TO_INT : INT
VAR_INPUT
    in : LWORD;
END_VAR
    LWORD_TO_INT := in;
END_FUNCTION

(********************
*
* Converts LWORD to SINT
*
******************** *)
FUNCTION LWORD_TO_SINT : SINT
VAR_INPUT
    in : LWORD;
END_VAR
    LWORD_TO_SINT := in;
END_FUNCTION

(********************
*
* Converts LWORD to ULINT
*
******************** *)
FUNCTION LWORD_TO_ULINT : ULINT
VAR_INPUT
    in : LWORD;
END_VAR
    LWORD_TO_ULINT := in;
END_FUNCTION

(********************
*
* Converts LWORD to UDINT
*
******************** *)
FUNCTION LWORD_TO_UDINT : UDINT
VAR_INPUT
    in : LWORD;
END_VAR
    LWORD_TO_UDINT := in;
END_FUNCTION

(********************
*
* Converts LWORD to UINT
*
******************** *)
FUNCTION LWORD_TO_UINT : UINT
VAR_INPUT
    in : LWORD;
END_VAR
    LWORD_TO_UINT := in;
END_FUNCTION

(********************
*
* Converts LWORD to USINT
*
******************** *)
FUNCTION LWORD_TO_USINT : USINT
VAR_INPUT
    in : LWORD;
END_VAR
    LWORD_TO_USINT := in;
END_FUNCTION

(********************
*
* Converts DWORD to REAL
*
******************** *)
{external}
FUNCTION DWORD_TO_REAL : REAL
VAR_INPUT
    in : DWORD;
END_VAR
END_FUNCTION

(********************
*
* Converts DWORD to LINT
*
******************** *)
FUNCTION DWORD_TO_LINT : LINT
VAR_INPUT
    in : DWORD;
END_VAR
    DWORD_TO_LINT := in;
END_FUNCTION

(********************
*
* Converts DWORD to DINT
*
******************** *)
FUNCTION DWORD_TO_DINT : DINT
VAR_INPUT
    in : DWORD;
END_VAR
    DWORD_TO_DINT := in;
END_FUNCTION

(********************
*
* Converts DWORD to INT
*
******************** *)
FUNCTION DWORD_TO_INT : INT
VAR_INPUT
    in : DWORD;
END_VAR
    DWORD_TO_INT := in;
END_FUNCTION

(********************
*
* Converts DWORD to SINT
*
******************** *)
FUNCTION DWORD_TO_SINT : SINT
VAR_INPUT
    in : DWORD;
END_VAR
    DWORD_TO_SINT := in;
END_FUNCTION

(********************
*
* Converts DWORD to ULINT
*
******************** *)
FUNCTION DWORD_TO_ULINT : ULINT
VAR_INPUT
    in : DWORD;
END_VAR
    DWORD_TO_ULINT := in;
END_FUNCTION

(********************
*
* Converts DWORD to UDINT
*
******************** *)
FUNCTION DWORD_TO_UDINT : UDINT
VAR_INPUT
    in : DWORD;
END_VAR
    DWORD_TO_UDINT := in;
END_FUNCTION

(********************
*
* Converts DWORD to UINT
*
******************** *)
FUNCTION DWORD_TO_UINT : UINT
VAR_INPUT
    in : DWORD;
END_VAR
    DWORD_TO_UINT := in;
END_FUNCTION

(********************
*
* Converts DWORD to USINT
*
******************** *)
FUNCTION DWORD_TO_USINT : USINT
VAR_INPUT
    in : DWORD;
END_VAR
    DWORD_TO_USINT := in;
END_FUNCTION

(********************
*
* Converts WORD to LINT
*
******************** *)
FUNCTION WORD_TO_LINT : LINT
VAR_INPUT
    in : WORD;
END_VAR
    WORD_TO_LINT := in;
END_FUNCTION

(********************
*
* Converts WORD to DINT
*
******************** *)
FUNCTION WORD_TO_DINT : DINT
VAR_INPUT
    in : WORD;
END_VAR
    WORD_TO_DINT := in;
END_FUNCTION

(********************
*
* Converts WORD to INT
*
******************** *)
FUNCTION WORD_TO_INT : INT
VAR_INPUT
    in : WORD;
END_VAR
    WORD_TO_INT := in;
END_FUNCTION

(********************
*
* Converts WORD to SINT
*
******************** *)
FUNCTION WORD_TO_SINT : SINT
VAR_INPUT
    in : WORD;
END_VAR
    WORD_TO_SINT := in;
END_FUNCTION

(********************
*
* Converts WORD to ULINT
*
******************** *)
FUNCTION WORD_TO_ULINT : ULINT
VAR_INPUT
    in : WORD;
END_VAR
    WORD_TO_ULINT := in;
END_FUNCTION

(********************
*
* Converts WORD to UDINT
*
******************** *)
FUNCTION WORD_TO_UDINT : UDINT
VAR_INPUT
    in : WORD;
END_VAR
    WORD_TO_UDINT := in;
END_FUNCTION

(********************
*
* Converts WORD to UINT
*
******************** *)
FUNCTION WORD_TO_UINT : UINT
VAR_INPUT
    in : WORD;
END_VAR
    WORD_TO_UINT := in;
END_FUNCTION

(********************
*
* Converts WORD to USINT
*
******************** *)
FUNCTION WORD_TO_USINT : USINT
VAR_INPUT
    in : WORD;
END_VAR
    WORD_TO_USINT := in;
END_FUNCTION

(********************
*
* Converts BYTE to LINT
*
******************** *)
FUNCTION BYTE_TO_LINT : LINT
VAR_INPUT
    in : BYTE;
END_VAR
    BYTE_TO_LINT := in;
END_FUNCTION

(********************
*
* Converts BYTE to DINT
*
******************** *)
FUNCTION BYTE_TO_DINT : DINT
VAR_INPUT
    in : BYTE;
END_VAR
    BYTE_TO_DINT := in;
END_FUNCTION

(********************
*
* Converts BYTE to INT
*
******************** *)
FUNCTION BYTE_TO_INT : INT
VAR_INPUT
    in : BYTE;
END_VAR
    BYTE_TO_INT := in;
END_FUNCTION

(********************
*
* Converts BYTE to SINT
*
******************** *)
FUNCTION BYTE_TO_SINT : SINT
VAR_INPUT
    in : BYTE;
END_VAR
    BYTE_TO_SINT := in;
END_FUNCTION

(********************
*
* Converts BYTE to ULINT
*
******************** *)
FUNCTION BYTE_TO_ULINT : ULINT
VAR_INPUT
    in : BYTE;
END_VAR
    BYTE_TO_ULINT := in;
END_FUNCTION

(********************
*
* Converts BYTE to UDINT
*
******************** *)
FUNCTION BYTE_TO_UDINT : UDINT
VAR_INPUT
    in : BYTE;
END_VAR
    BYTE_TO_UDINT := in;
END_FUNCTION

(********************
*
* Converts BYTE to UINT
*
******************** *)
FUNCTION BYTE_TO_UINT : UINT
VAR_INPUT
    in : BYTE;
END_VAR
    BYTE_TO_UINT := in;
END_FUNCTION

(********************
*
* Converts BYTE to USINT
*
******************** *)
FUNCTION BYTE_TO_USINT : USINT
VAR_INPUT
    in : BYTE;
END_VAR
    BYTE_TO_USINT := in;
END_FUNCTION

(********************
*
* Converts BOOL to LINT
*
******************** *)
FUNCTION BOOL_TO_LINT : LINT
VAR_INPUT
    in : BOOL;
END_VAR
    BOOL_TO_LINT := in;
END_FUNCTION

(********************
*
* Converts BOOL to DINT
*
******************** *)
FUNCTION BOOL_TO_DINT : DINT
VAR_INPUT
    in : BOOL;
END_VAR
    BOOL_TO_DINT := in;
END_FUNCTION

(********************
*
* Converts BOOL to INT
*
******************** *)
FUNCTION BOOL_TO_INT : INT
VAR_INPUT
    in : BOOL;
END_VAR
    BOOL_TO_INT := in;
END_FUNCTION

(********************
*
* Converts BOOL to SINT
*
******************** *)
FUNCTION BOOL_TO_SINT : SINT
VAR_INPUT
    in : BOOL;
END_VAR
    BOOL_TO_SINT := in;
END_FUNCTION

(********************
*
* Converts BOOL to ULINT
*
******************** *)
FUNCTION BOOL_TO_ULINT : ULINT
VAR_INPUT
    in : BOOL;
END_VAR
    BOOL_TO_ULINT := in;
END_FUNCTION

(********************
*
* Converts BOOL to UDINT
*
******************** *)
FUNCTION BOOL_TO_UDINT : UDINT
VAR_INPUT
    in : BOOL;
END_VAR
    BOOL_TO_UDINT := in;
END_FUNCTION

(********************
*
* Converts BOOL to UINT
*
******************** *)
FUNCTION BOOL_TO_UINT : UINT
VAR_INPUT
    in : BOOL;
END_VAR
    BOOL_TO_UINT := in;
END_FUNCTION

(********************
*
* Converts BOOL to USINT
*
******************** *)
FUNCTION BOOL_TO_USINT : USINT
VAR_INPUT
    in : BOOL;
END_VAR
    BOOL_TO_USINT := in;
END_FUNCTION

(********************
*
* Converts LREAL to LWORD
*
******************** *)
{external}
FUNCTION LREAL_TO_LWORD : LWORD
VAR_INPUT
    in : LREAL;
END_VAR
END_FUNCTION

(********************
*
* Converts REAL to DWORD
*
******************** *)
{external}
FUNCTION REAL_TO_DWORD : DWORD
VAR_INPUT
    in : REAL;
END_VAR
END_FUNCTION

(********************
*
* Converts LINT to LWORD
*
******************** *)
FUNCTION LINT_TO_LWORD : LWORD
VAR_INPUT
    in : LINT;
END_VAR
    LINT_TO_LWORD := in;
END_FUNCTION

(********************
*
* Converts LINT to DWORD
*
******************** *)
FUNCTION LINT_TO_DWORD : DWORD
VAR_INPUT
    in : LINT;
END_VAR
    LINT_TO_DWORD := in;
END_FUNCTION

(********************
*
* Converts LINT to WORD
*
******************** *)
FUNCTION LINT_TO_WORD : WORD
VAR_INPUT
    in : LINT;
END_VAR
    LINT_TO_WORD := in;
END_FUNCTION

(********************
*
* Converts LINT to BYTE
*
******************** *)
FUNCTION LINT_TO_BYTE : BYTE
VAR_INPUT
    in : LINT;
END_VAR
    LINT_TO_BYTE := in;
END_FUNCTION

(********************
*
* Converts DINT to LWORD
*
******************** *)
FUNCTION DINT_TO_LWORD : LWORD
VAR_INPUT
    in : DINT;
END_VAR
    DINT_TO_LWORD := in;
END_FUNCTION

(********************
*
* Converts DINT to DWORD
*
******************** *)
FUNCTION DINT_TO_DWORD : DWORD
VAR_INPUT
    in : DINT;
END_VAR
    DINT_TO_DWORD := in;
END_FUNCTION

(********************
*
* Converts DINT to WORD
*
******************** *)
FUNCTION DINT_TO_WORD : WORD
VAR_INPUT
    in : DINT;
END_VAR
    DINT_TO_WORD := in;
END_FUNCTION

(********************
*
* Converts DINT to BYTE
*
******************** *)
FUNCTION DINT_TO_BYTE : BYTE
VAR_INPUT
    in : DINT;
END_VAR
    DINT_TO_BYTE := in;
END_FUNCTION

(********************
*
* Converts INT to LWORD
*
******************** *)
FUNCTION INT_TO_LWORD : LWORD
VAR_INPUT
    in : INT;
END_VAR
    INT_TO_LWORD := in;
END_FUNCTION

(********************
*
* Converts INT to DWORD
*
******************** *)
FUNCTION INT_TO_DWORD : DWORD
VAR_INPUT
    in : INT;
END_VAR
    INT_TO_DWORD := in;
END_FUNCTION

(********************
*
* Converts INT to WORD
*
******************** *)
FUNCTION INT_TO_WORD : WORD
VAR_INPUT
    in : INT;
END_VAR
    INT_TO_WORD := in;
END_FUNCTION

(********************
*
* Converts INT to BYTE
*
******************** *)
FUNCTION INT_TO_BYTE : BYTE
VAR_INPUT
    in : INT;
END_VAR
    INT_TO_BYTE := in;
END_FUNCTION

(********************
*
* Converts SINT to LWORD
*
******************** *)
FUNCTION SINT_TO_LWORD : LWORD
VAR_INPUT
    in : SINT;
END_VAR
    SINT_TO_LWORD := in;
END_FUNCTION

(********************
*
* Converts SINT to DWORD
*
******************** *)
FUNCTION SINT_TO_DWORD : DWORD
VAR_INPUT
    in : SINT;
END_VAR
    SINT_TO_DWORD := in;
END_FUNCTION

(********************
*
* Converts SINT to WORD
*
******************** *)
FUNCTION SINT_TO_WORD : WORD
VAR_INPUT
    in : SINT;
END_VAR
    SINT_TO_WORD := in;
END_FUNCTION

(********************
*
* Converts SINT to BYTE
*
******************** *)
FUNCTION SINT_TO_BYTE : BYTE
VAR_INPUT
    in : SINT;
END_VAR
    SINT_TO_BYTE := in;
END_FUNCTION

(********************
*
* Converts ULINT to LWORD
*
******************** *)
FUNCTION ULINT_TO_LWORD : LWORD
VAR_INPUT
    in : ULINT;
END_VAR
    ULINT_TO_LWORD := in;
END_FUNCTION

(********************
*
* Converts ULINT to DWORD
*
******************** *)
FUNCTION ULINT_TO_DWORD : DWORD
VAR_INPUT
    in : ULINT;
END_VAR
    ULINT_TO_DWORD := in;
END_FUNCTION

(********************
*
* Converts ULINT to WORD
*
******************** *)
FUNCTION ULINT_TO_WORD : WORD
VAR_INPUT
    in : ULINT;
END_VAR
    ULINT_TO_WORD := in;
END_FUNCTION

(********************
*
* Converts ULINT to BYTE
*
******************** *)
FUNCTION ULINT_TO_BYTE : BYTE
VAR_INPUT
    in : ULINT;
END_VAR
    ULINT_TO_BYTE := in;
END_FUNCTION

(********************
*
* Converts UDINT to LWORD
*
******************** *)
FUNCTION UDINT_TO_LWORD : LWORD
VAR_INPUT
    in : UDINT;
END_VAR
    UDINT_TO_LWORD := in;
END_FUNCTION

(********************
*
* Converts UDINT to DWORD
*
******************** *)
FUNCTION UDINT_TO_DWORD : DWORD
VAR_INPUT
    in : UDINT;
END_VAR
    UDINT_TO_DWORD := in;
END_FUNCTION

(********************
*
* Converts UDINT to WORD
*
******************** *)
FUNCTION UDINT_TO_WORD : WORD
VAR_INPUT
    in : UDINT;
END_VAR
    UDINT_TO_WORD := in;
END_FUNCTION

(********************
*
* Converts UDINT to BYTE
*
******************** *)
FUNCTION UDINT_TO_BYTE : BYTE
VAR_INPUT
    in : UDINT;
END_VAR
    UDINT_TO_BYTE := in;
END_FUNCTION

(********************
*
* Converts UINT to LWORD
*
******************** *)
FUNCTION UINT_TO_LWORD : LWORD
VAR_INPUT
    in : UINT;
END_VAR
    UINT_TO_LWORD := in;
END_FUNCTION

(********************
*
* Converts UINT to DWORD
*
******************** *)
FUNCTION UINT_TO_DWORD : DWORD
VAR_INPUT
    in : UINT;
END_VAR
    UINT_TO_DWORD := in;
END_FUNCTION

(********************
*
* Converts UINT to WORD
*
******************** *)
FUNCTION UINT_TO_WORD : WORD
VAR_INPUT
    in : UINT;
END_VAR
    UINT_TO_WORD := in;
END_FUNCTION

(********************
*
* Converts UINT to BYTE
*
******************** *)
FUNCTION UINT_TO_BYTE : BYTE
VAR_INPUT
    in : UINT;
END_VAR
    UINT_TO_BYTE := in;
END_FUNCTION

(********************
*
* Converts USINT to LWORD
*
******************** *)
FUNCTION USINT_TO_LWORD : LWORD
VAR_INPUT
    in : USINT;
END_VAR
    USINT_TO_LWORD := in;
END_FUNCTION

(********************
*
* Converts USINT to DWORD
*
******************** *)
FUNCTION USINT_TO_DWORD : DWORD
VAR_INPUT
    in : USINT;
END_VAR
    USINT_TO_DWORD := in;
END_FUNCTION

(********************
*
* Converts USINT to WORD
*
******************** *)
FUNCTION USINT_TO_WORD : WORD
VAR_INPUT
    in : USINT;
END_VAR
    USINT_TO_WORD := in;
END_FUNCTION

(********************
*
* Converts USINT to BYTE
*
******************** *)
FUNCTION USINT_TO_BYTE : BYTE
VAR_INPUT
    in : USINT;
END_VAR
    USINT_TO_BYTE := in;
END_FUNCTIONVAR_GLOBAL CONSTANT
    __STRING_LENGTH : DINT := 2048;
END_VAR

(******************************************************************************
Description: String character length
Input:
    - IN:   A character string
Return: String length
***************************************************************************** *)
{external}
FUNCTION LEN < T : ANY_STRING > : DINT
VAR_INPUT {ref}
    IN : T;
END_VAR
END_FUNCTION

(******************************************************************************
Description: Left
Input:
    - IN:   A character string
    - L:    The length of the substring
Return: A substring consisting of the leftmost L characters of IN
***************************************************************************** *)
FUNCTION LEFT < T : ANY_STRING > : T
VAR_INPUT {ref}
    IN : T;
END_VAR
VAR_INPUT
    L  : DINT;
END_VAR
END_FUNCTION

{external}
FUNCTION LEFT_EXT < T : ANY_STRING > : DINT
VAR_INPUT {ref}
    IN : T;
END_VAR
VAR_INPUT
    L  : DINT;
END_VAR
VAR_IN_OUT
    OUT : T;
END_VAR
END_FUNCTION

FUNCTION LEFT__STRING : STRING[__STRING_LENGTH]
VAR_INPUT {ref}
    IN : STRING[__STRING_LENGTH];
END_VAR
VAR_INPUT
    L  : DINT;
END_VAR
    LEFT_EXT(IN, L, LEFT__STRING);
END_FUNCTION

FUNCTION LEFT__WSTRING : WSTRING[__STRING_LENGTH]
VAR_INPUT {ref}
    IN : WSTRING[__STRING_LENGTH];
END_VAR
VAR_INPUT
    L  : DINT;
END_VAR
    LEFT_EXT(IN, L, LEFT__WSTRING);
END_FUNCTION

{external}
FUNCTION LEFT_EXT__STRING : DINT
VAR_INPUT {ref}
    IN : STRING[__STRING_LENGTH];
END_VAR
VAR_INPUT
    L  : DINT;
END_VAR
VAR_IN_OUT
    OUT : STRING[__STRING_LENGTH];
END_VAR
END_FUNCTION

{external}
FUNCTION LEFT_EXT__WSTRING : DINT
VAR_INPUT {ref}
    IN : WSTRING[__STRING_LENGTH];
END_VAR
VAR_INPUT
    L  : DINT;
END_VAR
VAR_IN_OUT
    OUT : WSTRING[__STRING_LENGTH];
END_VAR
END_FUNCTION

(******************************************************************************
Description: Right
Input:
    - IN:   A character string
    - L:    The length of the substring
Return: A substring consisting of the rightmost L characters of IN
***************************************************************************** *)
FUNCTION RIGHT < T : ANY_STRING > : T
VAR_INPUT {ref}
    IN : T;
END_VAR
VAR_INPUT
    L  : DINT;
END_VAR
END_FUNCTION

{external}
FUNCTION RIGHT_EXT < T : ANY_STRING > : DINT
VAR_INPUT {ref}
    IN : T;
END_VAR
VAR_INPUT
    L  : DINT;
END_VAR
VAR_IN_OUT
    OUT : T;
END_VAR
END_FUNCTION

FUNCTION RIGHT__STRING : STRING[__STRING_LENGTH]
VAR_INPUT {ref}
    IN : STRING[__STRING_LENGTH];
END_VAR
VAR_INPUT
    L  : DINT;
END_VAR
    RIGHT_EXT(IN, L, RIGHT__STRING);
END_FUNCTION

FUNCTION RIGHT__WSTRING : WSTRING[__STRING_LENGTH]
VAR_INPUT {ref}
    IN : WSTRING[__STRING_LENGTH];
END_VAR
VAR_INPUT
    L  : DINT;
END_VAR
    RIGHT_EXT(IN, L, RIGHT__WSTRING);
END_FUNCTION

{external}
FUNCTION RIGHT_EXT__STRING : DINT
VAR_INPUT {ref}
    IN : STRING[__STRING_LENGTH];
END_VAR
VAR_INPUT
    L  : DINT;
END_VAR
VAR_IN_OUT
    OUT : STRING[__STRING_LENGTH];
END_VAR
END_FUNCTION

{external}
FUNCTION RIGHT_EXT__WSTRING : DINT
VAR_INPUT {ref}
    IN : WSTRING[__STRING_LENGTH];
END_VAR
VAR_INPUT
    L  : DINT;
END_VAR
VAR_IN_OUT
    OUT : WSTRING[__STRING_LENGTH];
END_VAR
END_FUNCTION

(******************************************************************************
Description: Middle
Input:
    - IN:   A character string
    - L:    The length of the substring
    - P:    The starting index position
Return:
    A substring that contains L characters starting
    from position P in a string.
***************************************************************************** *)
FUNCTION MID < T : ANY_STRING > : T
VAR_INPUT {ref}
    IN : T;
END_VAR
VAR_INPUT
    L  : DINT;
    P  : DINT;
END_VAR
END_FUNCTION

{external}
FUNCTION MID_EXT < T : ANY_STRING > : DINT
VAR_INPUT {ref}
    IN : T;
END_VAR
VAR_INPUT
    L  : DINT;
    P  : DINT;
END_VAR
VAR_IN_OUT
    OUT : T;
END_VAR
END_FUNCTION

FUNCTION MID__STRING : STRING[__STRING_LENGTH]
VAR_INPUT {ref}
    IN : STRING[__STRING_LENGTH];
END_VAR
VAR_INPUT
    L  : DINT;
    P  : DINT;
END_VAR
    MID_EXT(IN, L, P, MID__STRING);
END_FUNCTION

FUNCTION MID__WSTRING : WSTRING[__STRING_LENGTH]
VAR_INPUT {ref}
    IN : WSTRING[__STRING_LENGTH];
END_VAR
VAR_INPUT
    L  : DINT;
    P  : DINT;
END_VAR
    MID_EXT(IN, L, P, MID__WSTRING);
END_FUNCTION

{external}
FUNCTION MID_EXT__STRING : DINT
VAR_INPUT {ref}
    IN : STRING[__STRING_LENGTH];
END_VAR
VAR_INPUT
    L  : DINT;
    P  : DINT;
END_VAR
VAR_IN_OUT
    OUT : STRING[__STRING_LENGTH];
END_VAR
END_FUNCTION

{external}
FUNCTION MID_EXT__WSTRING : DINT
VAR_INPUT {ref}
    IN : WSTRING[__STRING_LENGTH];
END_VAR
VAR_INPUT
    L  : DINT;
    P  : DINT;
END_VAR
VAR_IN_OUT
    OUT : WSTRING[__STRING_LENGTH];
END_VAR
END_FUNCTION


(******************************************************************************
Description: Extensible concatenation
Input:
    - IN:   Two or more comma-separated strings
Return:
    A string combining all given input strings in the same order
    as the given string parameters.
***************************************************************************** *)
{external}
FUNCTION CONCAT__STRING : STRING[2048]
VAR_INPUT {ref}
    args : {sized} STRING...;
END_VAR
END_FUNCTION

{external}
FUNCTION CONCAT__WSTRING : WSTRING[2048]
VAR_INPUT {ref}
    args : {sized} WSTRING...;
END_VAR
END_FUNCTION

FUNCTION CONCAT < T : ANY_STRING > : T
VAR_INPUT {ref}
    args : {sized} T...;
END_VAR
END_FUNCTION

{external}
FUNCTION CONCAT_EXT < T : ANY_STRING > : DINT
VAR_IN_OUT
    OUT : T;
END_VAR
VAR_INPUT {ref}
    args : {sized} T...;
END_VAR
END_FUNCTION

(******************************************************************************
Description: Insert
Input:
    IN1:    The string to insert into
    IN2:    The string to insert
    P:      The position at which to insert
Return:
    A string consisting of IN2 inserted at position P into string IN1
***************************************************************************** *)
FUNCTION INSERT < T : ANY_STRING > : T
VAR_INPUT {ref}
    IN1 : T;
    IN2 : T;
END_VAR
VAR_INPUT
    P   : DINT;
END_VAR
END_FUNCTION

{external}
FUNCTION INSERT_EXT < T : ANY_STRING > : DINT
VAR_INPUT {ref}
    IN1 : T;
    IN2 : T;
END_VAR
VAR_INPUT
    P   : DINT;
END_VAR
VAR_IN_OUT
    OUT : T;
END_VAR
END_FUNCTION

FUNCTION INSERT__STRING : STRING[__STRING_LENGTH]
VAR_INPUT {ref}
    IN1 : STRING[__STRING_LENGTH];
    IN2 : STRING[__STRING_LENGTH];
END_VAR
VAR_INPUT
    P   : DINT;
END_VAR
    INSERT_EXT(IN1, IN2, P, INSERT__STRING);
END_FUNCTION

FUNCTION INSERT__WSTRING : WSTRING[__STRING_LENGTH]
VAR_INPUT {ref}
    IN1 : WSTRING[__STRING_LENGTH];
    IN2 : WSTRING[__STRING_LENGTH];
END_VAR
VAR_INPUT
    P   : DINT;
END_VAR
    INSERT_EXT(IN1, IN2, P, INSERT__WSTRING);
END_FUNCTION

{external}
FUNCTION INSERT_EXT__STRING : DINT
VAR_INPUT {ref}
    IN1 : STRING[__STRING_LENGTH];
    IN2 : STRING[__STRING_LENGTH];
END_VAR
VAR_INPUT
    P   : DINT;
END_VAR
VAR_IN_OUT
    OUT : STRING[__STRING_LENGTH];
END_VAR
END_FUNCTION

{external}
FUNCTION INSERT_EXT__WSTRING : DINT
VAR_INPUT {ref}
    IN1 : WSTRING[__STRING_LENGTH];
    IN2 : WSTRING[__STRING_LENGTH];
END_VAR
VAR_INPUT
    P   : DINT;
END_VAR
VAR_IN_OUT
    OUT : WSTRING[__STRING_LENGTH];
END_VAR
END_FUNCTION

(******************************************************************************
Description: Delete
Input:
    IN: The string to delete characters from
    L:  The amount of characters to delete
    P:  The position at which to start deleting
Return: A new string with L characters deleted from P onwards
***************************************************************************** *)
FUNCTION DELETE < T : ANY_STRING > : T
VAR_INPUT {ref}
    IN : T;
END_VAR
VAR_INPUT
    L  : DINT;
    P  : DINT;
END_VAR
END_FUNCTION

{external}
FUNCTION DELETE_EXT < T : ANY_STRING > : DINT
VAR_INPUT {ref}
    IN : T;
END_VAR
VAR_INPUT
    L  : DINT;
    P  : DINT;
END_VAR
VAR_IN_OUT
    OUT : T;
END_VAR
END_FUNCTION

FUNCTION DELETE__STRING : STRING[__STRING_LENGTH]
VAR_INPUT {ref}
    IN : STRING[__STRING_LENGTH];
END_VAR
VAR_INPUT
    L  : DINT;
    P  : DINT;
END_VAR
    DELETE_EXT(IN, L, P, DELETE__STRING);
END_FUNCTION

FUNCTION DELETE__WSTRING : WSTRING[__STRING_LENGTH]
VAR_INPUT {ref}
    IN : WSTRING[__STRING_LENGTH];
END_VAR
VAR_INPUT
    L  : DINT;
    P  : DINT;
END_VAR
    DELETE_EXT(IN, L, P, DELETE__WSTRING);
END_FUNCTION

{external}
FUNCTION DELETE_EXT__STRING : DINT
VAR_INPUT {ref}
    IN : STRING[__STRING_LENGTH];
END_VAR
VAR_INPUT
    L  : DINT;
    P  : DINT;
END_VAR
VAR_IN_OUT
    OUT : STRING[__STRING_LENGTH];
END_VAR
END_FUNCTION

{external}
FUNCTION DELETE_EXT__WSTRING : DINT
VAR_INPUT {ref}
    IN : WSTRING[__STRING_LENGTH];
END_VAR
VAR_INPUT
    L  : DINT;
    P  : DINT;
END_VAR
VAR_IN_OUT
    OUT : WSTRING[__STRING_LENGTH];
END_VAR
END_FUNCTION
(******************************************************************************
Description: Replace
Input:
    IN1:    The string to replace characters from
    IN2:    The replacement string to be inserted
    L:      The amount of characters to delete
    P:      The position at which to start deleting
Return: A new string which has L characters replaced by IN2 from position P onwards
***************************************************************************** *)
FUNCTION REPLACE < T : ANY_STRING > : T
VAR_INPUT {ref}
    IN1 : T;
    IN2 : T;
END_VAR
VAR_INPUT
    L  : DINT;
    P  : DINT;
END_VAR
END_FUNCTION

{external}
FUNCTION REPLACE_EXT < T : ANY_STRING > : DINT
VAR_INPUT {ref}
    IN1 : T;
    IN2 : T;
END_VAR
VAR_INPUT
    L   : DINT;
    P   : DINT;
END_VAR
VAR_IN_OUT
    OUT : T;
END_VAR
END_FUNCTION

FUNCTION REPLACE__STRING : STRING[__STRING_LENGTH]
VAR_INPUT {ref}
    IN1 : STRING[__STRING_LENGTH];
    IN2 : STRING[__STRING_LENGTH];
END_VAR
VAR_INPUT
    L  : DINT;
    P  : DINT;
END_VAR
    REPLACE_EXT(IN1, IN2, L, P, REPLACE__STRING);
END_FUNCTION

FUNCTION REPLACE__WSTRING : WSTRING[__STRING_LENGTH]
VAR_INPUT {ref}
    IN1 : WSTRING[__STRING_LENGTH];
    IN2 : WSTRING[__STRING_LENGTH];
END_VAR
VAR_INPUT
    L   : DINT;
    P   : DINT;
END_VAR
    REPLACE_EXT(IN1, IN2, L, P, REPLACE__WSTRING);
END_FUNCTION

{external}
FUNCTION REPLACE_EXT__STRING : DINT
VAR_INPUT {ref}
    IN1 : STRING[__STRING_LENGTH];
    IN2 : STRING[__STRING_LENGTH];
END_VAR
VAR_INPUT
    L   : DINT;
    P   : DINT;
END_VAR
VAR_IN_OUT
    OUT : STRING[__STRING_LENGTH];
END_VAR
END_FUNCTION

{external}
FUNCTION REPLACE_EXT__WSTRING : DINT
VAR_INPUT {ref}
    IN1 : WSTRING[__STRING_LENGTH];
    IN2 : WSTRING[__STRING_LENGTH];
END_VAR
VAR_INPUT
    L   : DINT;
    P   : DINT;
END_VAR
VAR_IN_OUT
    OUT : WSTRING[__STRING_LENGTH];
END_VAR
END_FUNCTION


(******************************************************************************
Description: Find
Input:
    IN1:    The string in which to search in
    IN2:    The substring to search for
Return: The character index of the first match. 0 if there are no matches.
***************************************************************************** *)
{external}
FUNCTION FIND < T : ANY_STRING > : DINT
VAR_INPUT {ref}
    IN1 : T;
    IN2 : T;
END_VAR
END_FUNCTION

(******************************************************************************
Description: Decreasing sequence
Input:  The strings to compare, in order. ((IN1>IN2) & (IN2>IN3) & .. & (INn-1>INn))
Return:
    TRUE if codepoints are in decreasing order (alphabetical order: ZYX > YXW > XWV ..)
    FALSE otherwise
***************************************************************************** *)
{external}
FUNCTION GT < T : ANY_STRING > : BOOL
VAR_INPUT {ref}
    IN1 : {sized} T...;
END_VAR
END_FUNCTION

// passing strings by ref causes an error/warning. code still compiles and works correctly
FUNCTION STRING_GREATER : BOOL
VAR_INPUT
    a, b : STRING;
END_VAR
    STRING_GREATER := GT(a, b);
END_FUNCTION

FUNCTION WSTRING_GREATER : BOOL
VAR_INPUT
    a, b : WSTRING;
END_VAR
    WSTRING_GREATER := GT(a, b);
END_FUNCTION
(******************************************************************************
Description: Monotonic sequence
Input:  The strings to compare, in order. ((IN1>=IN2) & (IN2>=IN3) & .. & (INn-1>=INn))
Return:
    TRUE if codepoints are in decreasing order or are equal to adjacent codepoints (alphabetical order: ZYX >= ZYX >= YXW ..)
    FALSE otherwise
***************************************************************************** *)
{external}
FUNCTION GE < T : ANY_STRING > : BOOL
VAR_INPUT {ref}
    IN1 : {sized} T...;
END_VAR
END_FUNCTION

(******************************************************************************
Description: Equality
Input: The strings to compare, in order. ((IN1=IN2) & (IN2=IN3) & .. & (INn-1=INn))
Return:
    TRUE if codepoints are equal to each other
    FALSE otherwise.
***************************************************************************** *)
{external}
FUNCTION EQ < T : ANY_STRING > : BOOL
VAR_INPUT {ref}
    IN1 : {sized} T...;
END_VAR
END_FUNCTION

FUNCTION STRING_EQUAL : BOOL
VAR_INPUT
    a, b : STRING;
END_VAR
    STRING_EQUAL := EQ(a, b);
END_FUNCTION

FUNCTION WSTRING_EQUAL : BOOL
VAR_INPUT
    a, b : WSTRING;
END_VAR
    WSTRING_EQUAL := EQ(a, b);
END_FUNCTION

(******************************************************************************
Description: Monotonic sequence
Input:  The strings to compare, in order. ((IN1<=IN2) & (IN2<=IN3) & .. & (INn-1<=INn))
Return:
    TRUE if codepoints are in increasing order or are equal to adjacent codepoints (alphabetical order: ABC <= ABC <= BCD ..)
    FALSE otherwise
***************************************************************************** *)
{external}
FUNCTION LE < T : ANY_STRING > : BOOL
VAR_INPUT {ref}
    IN1 : {sized} T...;
END_VAR
END_FUNCTION

(******************************************************************************
Description: Increasing sequence
Input:  The strings to compare, in order. ((IN1<IN2) & (IN2<IN3) & .. & (INn-1<INn))
Return:
    TRUE if codepoints are in increasing order (alphabetical order: ABC < BCD < CDE ..)
    FALSE otherwise
***************************************************************************** *)
{external}
FUNCTION LT < T : ANY_STRING > : BOOL
VAR_INPUT {ref}
    IN1 : {sized} T...;
END_VAR
END_FUNCTION

FUNCTION STRING_LESS : BOOL
VAR_INPUT
    a, b : STRING;
END_VAR
    STRING_LESS := LT(a, b);
END_FUNCTION

FUNCTION WSTRING_LESS : BOOL
VAR_INPUT
    a, b : WSTRING;
END_VAR
    WSTRING_LESS := LT(a, b);
END_FUNCTION

(******************************************************************************
Description: Inequality
Input: The strings to compare. (IN1<>IN2)
Return:
    TRUE if strings do not match
    FALSE otherwise
***************************************************************************** *)
{external}
FUNCTION NE < T : ANY_STRING > : BOOL
VAR_INPUT {ref}
    IN1 : T;
    IN2 : T;
END_VAR
END_FUNCTION (* Definitions of arithmetic functions defined by the IEC61131-3 standard *)

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

(* Calculates the square root of a given value *)
{external}
FUNCTION SQRT < T : ANY_REAL > : T
VAR_INPUT
    in : T;
END_VAR
END_FUNCTION

(** Logarithmic functions * *)


(* Calculates the natural logarithm of the given value *)
{external}
FUNCTION LN < T : ANY_REAL > : T
VAR_INPUT
    in : T;
END_VAR
END_FUNCTION


(* Calculates the base 10 logarithm of the given value *)
{external}
FUNCTION LOG < T : ANY_REAL > : T
VAR_INPUT
    in : T;
END_VAR
END_FUNCTION

(* Calculates the natural exporenent of the given value *)
{external}
FUNCTION EXP < T : ANY_REAL > : T
VAR_INPUT
    in : T;
END_VAR
END_FUNCTION

(** Trigonometric functions * *)


(* Calculates the sine of the given value in radiants *)
{external}
FUNCTION SIN < T : ANY_REAL > : T
VAR_INPUT
    rad : T;
END_VAR
END_FUNCTION

(* Calculates the cosine of the given value in radiants *)
{external}
FUNCTION COS < T : ANY_REAL > : T
VAR_INPUT
    rad : T;
END_VAR
END_FUNCTION

(* Calculates the tangent of the given value in radiants *)
{external}
FUNCTION TAN < T : ANY_REAL > : T
VAR_INPUT
    rad : T;
END_VAR
END_FUNCTION

(* Calculates the arc sine of the given value *)
{external}
FUNCTION ASIN < T : ANY_REAL > : T
VAR_INPUT
    in : T;
END_VAR
END_FUNCTION

(* Calculates the arc cosine of the given value *)
{external}
FUNCTION ACOS < T : ANY_REAL > : T
VAR_INPUT
    in : T;
END_VAR
END_FUNCTION

(* Calculates the arc tangent of the given value *)
{external}
FUNCTION ATAN < T : ANY_REAL > : T
VAR_INPUT
    in : T;
END_VAR
END_FUNCTION

(* Calculate the four quadrant arc tangent of the given values *)
{external}
FUNCTION ATAN2 < T : ANY_REAL > : T
VAR_INPUT
    y : T;
    x : T;
END_VAR
END_FUNCTION

{external}
FUNCTION EXPT < T : ANY_REAL, U : ANY_NUM > : T
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
(**************************
 *
 *This operator returns the absolute value of a number.
 *
************************ *)
FUNCTION ABS < T : ANY_NUM > : T
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
        ABS__DINT := - 1 * ABS__DINT;
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
        ABS__LINT := - 1 * ABS__LINT;
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
        ABS__LREAL := - 1.0 * ABS__LREAL;
    END_IF
END_FUNCTION

(**************************
 *
 * Rounds a given ANY_REAL to the nearest integer
 * Rounds half values away from zero
 *
************************ *)
{external}
FUNCTION ROUND < T : ANY_REAL > : T
VAR_INPUT
    in : T;
END_VAR
END_FUNCTION
CLASS MyClass
    VAR
        x, y : INT;
    END_VAR

    METHOD testMethod
        VAR_INPUT myMethodArg : INT; END_VAR
        VAR myMethodLocalVar : INT; END_VAR

        x := myMethodArg;
    END_METHOD
END_CLASSPROGRAM name
(1 + 2) * (3 + 4);
END_PROGRAM
FUNCTION smaller_than_ten : BOOL
    VAR_INPUT
        n : INT;
    END_VAR

    IF n < 10 THEN
        smaller_than_ten := TRUE;
        RETURN;
    END_IF;
    smaller_than_ten := FALSE;
END_FUNCTION
PROGRAM hello
VAR
   x : INT
   y : BOOL;
END_VAR
   IF x = 2 THEN
      x := 3
END_PROGRAM
FUNCTION a : DINT END_FUNCTION
        PROGRAM main
        VAR
            x : ARRAY[0..1] OF INT;
            y : REF_TO INT;
            z : REF_TO ARRAY[0..1] OF INT;
        END_VAR
            x[0] := 1;
            y^ := 2;
            y^.1 := 3;
            z^[0] := 4;
            z^[1].1 := 5;
                        a() := 5;
        END_PROGRAM
@EXTERNAL FUNCTION LOG : DINT
VAR_INPUT
    x : DINT;
END_VAR
END_FUNCTION

@EXTERNAL FUNCTION PRINTF : DINT
VAR_INPUT
    text : STRING;
    value : DINT;
END_VAR
END_FUNCTION


FUNCTION main : DINT
    main := LOG(100);
  PRINTF('Log value %d\n', main);
END_FUNCTION
FUNCTION main : DINT
  main := 1;
  WHILE main < 10 DO
    main := main + 1;
    CONTINUE;
    main := 200;
  END_WHILE
END_FUNCTION
TYPE MyStruct : STRUCT  x : DINT; y : DINT; END_STRUCT END_TYPE

FUNCTION main : DINT
    main := foo();
END_FUNCTION

FUNCTION foo : DINT
VAR
                x : DINT;
                s : MyStruct;
                u, y : REF_TO DINT;
                z : REF_TO REF_TO DINT;

END_VAR
u := NULL;
u := &s.x;
y := u;
z := &y;
s.x := 9;
z^^ := y^ * 2;
y^ := z^^ * 2;

foo := y^;
END_FUNCTION
{external}
FUNCTION puts : DINT
VAR_INPUT {ref}
    text : STRING;
END_VAR
END_FUNCTION

FUNCTION main : DINT
    puts('hello, world!$N');
END_FUNCTION
PROGRAM hello
VAR
   x : INT;
   y : INT;
   z : INT;
END_VAR


IF Condition THEN

ELSIF

END_IF;

1 + 2 + 3 + 4;
END_PROGRAM
PROGRAM exp a AND NOT b OR c XOR d; END_PROGRAMPROGRAM prg
    VAR
            a : INT;
            b : REAL;
    END_VAR
        b := 1.5;
        a := b;
END_PROGRAM
PROGRAM a
(* Comment *)
END_PROGRAM
PROGRAM a
    VAR
        binvar      AT %IX7.8 : BOOL;
        sinvar      AT %IB4.8 : BYTE;
        winvar      AT %IW3.1 : WORD;
        dinvar      AT %ID1.7 : DWORD;
        boutvar     AT %QX7.7 : BOOL;
        soutvar     AT %QB1.6 : BYTE;
        woutvar     AT %QW2.4 : WORD;
        doutvar     AT %QD1.4 : DWORD;
        bmemvar     AT %MX7.8 : BOOL;
        smemvar     AT %MB5.8 : BYTE;
        wmemvar     AT %MW4.2 : WORD;
        dmemvar     AT %MD8.3 : DWORD;
        linvar      AT %IL9.0 : LWORD;
        loutvar     AT %QL4.2 : LWORD;
        lmemvar     AT %ML9.6 : LWORD;
    END_VAR
END_PROGRAM
PROGRAM a
(* Comment *)
END_PROGRAM
FUNCTION myFunc : DINT
VAR_INPUT
     a, b, c : DINT;
END_VAR
END_FUNCTION
FUNCTION foo2 : LINT
    foo2 := CONCAT_DATE(INT#4, SINT#5, SINT#6);
END_FUNCTION{external}
FUNCTION CONCAT_DATE < T : ANY_INT > : DATE
VAR_INPUT
    year : T;
    month : T;
    day : T;
END_VAR
END_FUNCTION

FUNCTION main : INT
VAR_INPUT

END_VAR

VAR

END_VAR
mainProg();
main := mainProg.a;

END_FUNCTION
PROGRAM mainProg
VAR
    a : DINT := 0;
END_VAR
a := 42;
END_PROGRAM
FUNCTION foo1 : LINT
    foo1 := CONCAT_DATE(INT#1, SINT#2, SINT#3);
END_FUNCTIONVAR_GLOBAL CONSTANT
    myValue : BOOL := TRUE;
END_VAR

FUNCTION_BLOCK myFb2
    VAR
        fb : myFb;
    END_VAR
END_FUNCTION_BLOCK
VAR_GLOBAL CONSTANT
    myValue2 : BOOL := TRUE;
END_VAR

VAR_GLOBAL CONSTANT
    myValue : BOOL := TRUE;
END_VAR

FUNCTION_BLOCK myFB
    VAR
        myS : myStruct := (a := 2);
    END_VAR
END_FUNCTION_BLOCK
{external}
FUNCTION func1 : DINT
END_FUNCTION

FUNCTION func2 : DINT
    func1();
END_FUNCTION

FUNCTION relative : DINT
END_FUNCTION
TYPE myStruct : STRUCT
    a : INT := 5;
    b : DINT := 6;
END_STRUCT
END_TYPE
{external}
FUNCTION func2 : DINT
END_FUNCTION

FUNCTION func1 : DINT
    func2();
END_FUNCTION

PROGRAM name
(1 + 2) * (3 + 4);
END_PROGRAM
PROGRAM hello
VAR
   x : INT;
   y : BOOL;
END_VAR
   IF x = 2 THEN
      x := 3;
   END_IF
END_PROGRAM
PROGRAM prg
    VAR
            a : INT;
            b : REAL;
    END_VAR
        b := 1.5;
        a := b;
END_PROGRAM
VAR_GLOBAL
    flags : ARRAY[1..500_000_000] OF BOOL;
    sieve_size : ULINT := 500_000_000;
END_VAR

// en.wikipedia.org/wiki/Sieve_of_Eratosthenes
FUNCTION sieve : DINT
    VAR
        i, j : ULINT;
        primes : ULINT;
    END_VAR

    FOR i := 2 TO sieve_size DO
        IF flags[i] = 0 THEN
            FOR j := i * i TO sieve_size BY i DO
                IF j < sieve_size THEN
                    flags[j] := 1;
                END_IF;
            END_FOR;
        END_IF;
    END_FOR;

    FOR i := 2 TO sieve_size DO
        IF flags[i] = 0 THEN
            primes := primes + 1;
        END_IF;
    END_FOR;

    printf('Primes found: %lld (correct: %d)$N', primes, (primes - 1) = 26_355_867);
END_FUNCTION

FUNCTION main : DINT
    sieve();
END_FUNCTION

{external}
FUNCTION printf : DINT
    VAR_INPUT {ref} format : STRING; END_VAR
    VAR_INPUT args : ...; END_VAR
END_FUNCTIONTYPE struct2 :
STRUCT
    param1 : DINT;
    param2 : DINT;
END_STRUCT
END_TYPE

PROGRAM mainProg
    VAR
        node : ARRAY[0..15] OF struc2 := ((param1 := 1, param2 := 2), (param1 := 3, param2 := 4));
        // node : ARRAY[0..15] OF struct2 := [(param1 := 1, param2 := 2), (param1 := 3, param2 := 4)];
    END_VAR
END_PROGRAM
