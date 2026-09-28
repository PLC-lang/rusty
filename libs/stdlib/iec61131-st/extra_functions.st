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
    IN : LTOD;
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
    IN : LDT;
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
    IN : LDATE;
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
    IN : LTIME;
END_VAR
    LTIME_TO_LWORD := IN;
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
    IN : LTIME;
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
    IN : LTOD;
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
    IN : LDT;
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
    IN : LDATE;
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
    IN : LDT;
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
    IN : LTIME;
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
    IN : LDATE;
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
    IN : LTOD;
END_VAR
    LTOD_TO_ULINT := IN;
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
    IN : LTIME;
END_VAR
    LTIME_TO_LREAL := IN;
END_FUNCTION

(******************************************************************************
*
* Converts LREAL to TIME.
*
******************************************************************************)
{external}
FUNCTION LREAL_TO_TIME : TIME
VAR_INPUT
    IN : LREAL;
END_VAR
END_FUNCTION

{external}
FUNCTION LREAL_TO_LTIME : LTIME
VAR_INPUT
    IN : LREAL;
END_VAR
END_FUNCTION

(******************************************************************************
*
* Converts REAL to TIME.
*
******************************************************************************)
FUNCTION REAL_TO_TIME : TIME
VAR_INPUT
    IN : REAL;
END_VAR
    REAL_TO_TIME := LREAL_TO_TIME(IN);
END_FUNCTION

FUNCTION REAL_TO_LTIME : LTIME
VAR_INPUT
    IN : REAL;
END_VAR
    REAL_TO_LTIME := LREAL_TO_LTIME(IN);
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

FUNCTION LWORD_TO_LDATE : LDATE
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

FUNCTION LWORD_TO_LDT : LDT
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

FUNCTION LWORD_TO_LTOD : LTOD
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

FUNCTION LWORD_TO_LTIME : LTIME
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

FUNCTION ULINT_TO_LDATE : LDATE
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

FUNCTION ULINT_TO_LDT : LDT
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

FUNCTION ULINT_TO_LTOD : LTOD
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

FUNCTION LINT_TO_LTIME : LTIME
VAR_INPUT
    IN : LINT;
END_VAR
    LINT_TO_LTIME := IN;
END_FUNCTION

(******************************************************************************
*
* Converts BOOL to TIME.
*
******************************************************************************)
FUNCTION BOOL_TO_TIME : TIME
VAR_INPUT
    IN : BOOL;
END_VAR
    BOOL_TO_TIME := IN;
END_FUNCTION

FUNCTION BOOL_TO_LTIME : LTIME
VAR_INPUT
    IN : BOOL;
END_VAR
    BOOL_TO_LTIME := IN;
END_FUNCTION

(******************************************************************************
*
* Converts BYTE to TIME.
*
******************************************************************************)
FUNCTION BYTE_TO_TIME : TIME
VAR_INPUT
    IN : BYTE;
END_VAR
    BYTE_TO_TIME := IN;
END_FUNCTION

FUNCTION BYTE_TO_LTIME : LTIME
VAR_INPUT
    IN : BYTE;
END_VAR
    BYTE_TO_LTIME := IN;
END_FUNCTION

(******************************************************************************
*
* Converts WORD to TIME.
*
******************************************************************************)
FUNCTION WORD_TO_TIME : TIME
VAR_INPUT
    IN : WORD;
END_VAR
    WORD_TO_TIME := IN;
END_FUNCTION

FUNCTION WORD_TO_LTIME : LTIME
VAR_INPUT
    IN : WORD;
END_VAR
    WORD_TO_LTIME := IN;
END_FUNCTION

(******************************************************************************
*
* Converts DWORD to TIME.
*
******************************************************************************)
FUNCTION DWORD_TO_TIME : TIME
VAR_INPUT
    IN : DWORD;
END_VAR
    DWORD_TO_TIME := IN;
END_FUNCTION

FUNCTION DWORD_TO_LTIME : LTIME
VAR_INPUT
    IN : DWORD;
END_VAR
    DWORD_TO_LTIME := IN;
END_FUNCTION

(******************************************************************************
*
* Converts SINT to TIME.
*
******************************************************************************)
FUNCTION SINT_TO_TIME : TIME
VAR_INPUT
    IN : SINT;
END_VAR
    SINT_TO_TIME := IN;
END_FUNCTION

FUNCTION SINT_TO_LTIME : LTIME
VAR_INPUT
    IN : SINT;
END_VAR
    SINT_TO_LTIME := IN;
END_FUNCTION

(******************************************************************************
*
* Converts USINT to TIME.
*
******************************************************************************)
FUNCTION USINT_TO_TIME : TIME
VAR_INPUT
    IN : USINT;
END_VAR
    USINT_TO_TIME := IN;
END_FUNCTION

FUNCTION USINT_TO_LTIME : LTIME
VAR_INPUT
    IN : USINT;
END_VAR
    USINT_TO_LTIME := IN;
END_FUNCTION

(******************************************************************************
*
* Converts INT to TIME.
*
******************************************************************************)
FUNCTION INT_TO_TIME : TIME
VAR_INPUT
    IN : INT;
END_VAR
    INT_TO_TIME := IN;
END_FUNCTION

FUNCTION INT_TO_LTIME : LTIME
VAR_INPUT
    IN : INT;
END_VAR
    INT_TO_LTIME := IN;
END_FUNCTION

(******************************************************************************
*
* Converts UINT to TIME.
*
******************************************************************************)
FUNCTION UINT_TO_TIME : TIME
VAR_INPUT
    IN : UINT;
END_VAR
    UINT_TO_TIME := IN;
END_FUNCTION

FUNCTION UINT_TO_LTIME : LTIME
VAR_INPUT
    IN : UINT;
END_VAR
    UINT_TO_LTIME := IN;
END_FUNCTION

(******************************************************************************
*
* Converts DINT to TIME.
*
******************************************************************************)
FUNCTION DINT_TO_TIME : TIME
VAR_INPUT
    IN : DINT;
END_VAR
    DINT_TO_TIME := IN;
END_FUNCTION

FUNCTION DINT_TO_LTIME : LTIME
VAR_INPUT
    IN : DINT;
END_VAR
    DINT_TO_LTIME := IN;
END_FUNCTION

(******************************************************************************
*
* Converts UDINT to TIME.
*
******************************************************************************)
FUNCTION UDINT_TO_TIME : TIME
VAR_INPUT
    IN : UDINT;
END_VAR
    UDINT_TO_TIME := IN;
END_FUNCTION

FUNCTION UDINT_TO_LTIME : LTIME
VAR_INPUT
    IN : UDINT;
END_VAR
    UDINT_TO_LTIME := IN;
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

FUNCTION LINT_TO_LTOD : LTOD
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

{external}
FUNCTION LTIME : LTIME END_FUNCTION
