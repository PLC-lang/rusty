(******************************************************************************
*
* Converts BYTE to STRING.
*
******************************************************************************)
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
******************************************************************************)
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
******************************************************************************)
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
******************************************************************************)
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
******************************************************************************)
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
******************************************************************************)
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
******************************************************************************)
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
******************************************************************************)
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
******************************************************************************)
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
******************************************************************************)
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
******************************************************************************)
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
******************************************************************************)
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
******************************************************************************)
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
******************************************************************************)
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
******************************************************************************)
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
******************************************************************************)
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
******************************************************************************)
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
******************************************************************************)
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
******************************************************************************)
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
******************************************************************************)
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
******************************************************************************)
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
******************************************************************************)
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
******************************************************************************)
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
******************************************************************************)
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
******************************************************************************)
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
******************************************************************************)
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
******************************************************************************)
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
******************************************************************************)
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
******************************************************************************)
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
******************************************************************************)
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
******************************************************************************)
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
******************************************************************************)
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
******************************************************************************)
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
******************************************************************************)
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
******************************************************************************)
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
******************************************************************************)
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
******************************************************************************)
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
******************************************************************************)
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
******************************************************************************)
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
******************************************************************************)
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
******************************************************************************)
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
******************************************************************************)
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
******************************************************************************)
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
******************************************************************************)
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
******************************************************************************)
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
******************************************************************************)
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
******************************************************************************)
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
******************************************************************************)
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
******************************************************************************)
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
******************************************************************************)
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
******************************************************************************)
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
******************************************************************************)
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
******************************************************************************)
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
******************************************************************************)
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
******************************************************************************)
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
******************************************************************************)
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
******************************************************************************)
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
******************************************************************************)
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
******************************************************************************)
{external}
FUNCTION TIME : TIME END_FUNCTION