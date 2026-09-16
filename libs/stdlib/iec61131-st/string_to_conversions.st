(******************************************************************************
*
* Converts STRING and WSTRING values to native values.
*
******************************************************************************)

(******************************************************************************
*
* Converts STRING to BOOL. Never faults; a rejected input yields FALSE.
*
******************************************************************************)
{external}
FUNCTION STRING_TO_BOOL : BOOL
VAR_INPUT {ref}
    IN : STRING;
END_VAR
END_FUNCTION

(******************************************************************************
*
* Converts STRING to BYTE. Never faults; a rejected input yields 0.
*
******************************************************************************)
{external}
FUNCTION STRING_TO_BYTE : BYTE
VAR_INPUT {ref}
    IN : STRING;
END_VAR
END_FUNCTION

(******************************************************************************
*
* Converts STRING to WORD. Never faults; a rejected input yields 0.
*
******************************************************************************)
{external}
FUNCTION STRING_TO_WORD : WORD
VAR_INPUT {ref}
    IN : STRING;
END_VAR
END_FUNCTION

(******************************************************************************
*
* Converts STRING to DWORD. Never faults; a rejected input yields 0.
*
******************************************************************************)
{external}
FUNCTION STRING_TO_DWORD : DWORD
VAR_INPUT {ref}
    IN : STRING;
END_VAR
END_FUNCTION

(******************************************************************************
*
* Converts STRING to LWORD. Never faults; a rejected input yields 0.
*
******************************************************************************)
{external}
FUNCTION STRING_TO_LWORD : LWORD
VAR_INPUT {ref}
    IN : STRING;
END_VAR
END_FUNCTION

(******************************************************************************
*
* Converts STRING to SINT. Never faults; a rejected input yields 0.
*
******************************************************************************)
{external}
FUNCTION STRING_TO_SINT : SINT
VAR_INPUT {ref}
    IN : STRING;
END_VAR
END_FUNCTION

(******************************************************************************
*
* Converts STRING to USINT. Never faults; a rejected input yields 0.
*
******************************************************************************)
{external}
FUNCTION STRING_TO_USINT : USINT
VAR_INPUT {ref}
    IN : STRING;
END_VAR
END_FUNCTION

(******************************************************************************
*
* Converts STRING to INT. Never faults; a rejected input yields 0.
*
******************************************************************************)
{external}
FUNCTION STRING_TO_INT : INT
VAR_INPUT {ref}
    IN : STRING;
END_VAR
END_FUNCTION

(******************************************************************************
*
* Converts STRING to UINT. Never faults; a rejected input yields 0.
*
******************************************************************************)
{external}
FUNCTION STRING_TO_UINT : UINT
VAR_INPUT {ref}
    IN : STRING;
END_VAR
END_FUNCTION

(******************************************************************************
*
* Converts STRING to DINT. Never faults; a rejected input yields 0.
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
* Converts STRING to UDINT. Never faults; a rejected input yields 0.
*
******************************************************************************)
{external}
FUNCTION STRING_TO_UDINT : UDINT
VAR_INPUT {ref}
    IN : STRING;
END_VAR
END_FUNCTION

(******************************************************************************
*
* Converts STRING to LINT. Never faults; a rejected input yields 0.
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
* Converts STRING to ULINT. Never faults; a rejected input yields 0.
*
******************************************************************************)
{external}
FUNCTION STRING_TO_ULINT : ULINT
VAR_INPUT {ref}
    IN : STRING;
END_VAR
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
* Converts STRING to TIME. Requires a T#/TIME# prefix; never faults, a rejected or
* negative duration yields T#0s.
*
******************************************************************************)
{external}
FUNCTION STRING_TO_TIME : TIME
VAR_INPUT {ref}
    IN : STRING;
END_VAR
END_FUNCTION

(******************************************************************************
*
* Converts STRING to LTIME. Requires a LT#/LTIME# prefix; never faults, a rejected or
* negative duration yields LTIME#0s.
*
******************************************************************************)
{external}
FUNCTION STRING_TO_LTIME : LTIME
VAR_INPUT {ref}
    IN : STRING;
END_VAR
END_FUNCTION

(******************************************************************************
*
* Converts STRING to DATE. Requires a D#/DATE# prefix. Never faults; an impossible or
* unrepresentable date yields D#1970-01-01.
*
******************************************************************************)
{external}
FUNCTION STRING_TO_DATE : DATE
VAR_INPUT {ref}
    IN : STRING;
END_VAR
END_FUNCTION

(******************************************************************************
*
* Converts STRING to DT. Requires a DT#/DATE_AND_TIME# prefix. Never faults; an
* impossible or unrepresentable date-time yields DT#1970-01-01-00:00:00.
*
******************************************************************************)
{external}
FUNCTION STRING_TO_DT : DT
VAR_INPUT {ref}
    IN : STRING;
END_VAR
END_FUNCTION

(******************************************************************************
*
* Converts STRING to TOD. Requires a TOD#/TIME_OF_DAY# prefix. Never faults; an
* impossible time yields TOD#00:00:00.
*
******************************************************************************)
{external}
FUNCTION STRING_TO_TOD : TOD
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
    WSTRING_TO_DINT := STRING_TO_DINT(STR);
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
