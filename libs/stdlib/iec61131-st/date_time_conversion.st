(********************
*
* Converts DT/LDT to DATE
*
*********************)
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
*********************)
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
*********************)
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
*********************)
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
*********************)
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
*********************)
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
*********************)
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
*********************)
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
*********************)
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
*********************)
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
*********************)
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
*********************)
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
*********************)
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
*********************)
FUNCTION TOD_TO_LTOD : LTOD
VAR_INPUT
	in : TOD;
END_VAR
	TOD_TO_LTOD := in;
END_FUNCTION