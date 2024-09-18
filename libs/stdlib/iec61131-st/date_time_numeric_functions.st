(* Specialized implementation of ADD for TIME *)
FUNCTION ADD__TIME__TIME: TIME
VAR_INPUT
    IN1: TIME;
    IN2: TIME;
END_VAR
    ADD__TIME__TIME := ADD_TIME(IN1, IN2);
END_FUNCTION

(********************
*
* This operator returns the value of adding up two TIME operands.
*
*********************)
{external}
FUNCTION ADD_TIME: TIME
VAR_INPUT
    IN1: TIME;
    IN2: TIME;
END_VAR
END_FUNCTION

(********************
*
* This operator returns the value of adding up two LTIME operands.
*
*********************)
FUNCTION ADD_LTIME: LTIME
VAR_INPUT
    IN1: LTIME;
    IN2: LTIME;
END_VAR
    ADD_LTIME := ADD_TIME(IN1, IN2);
END_FUNCTION

(* Specialized implementation of ADD for TOD *)
FUNCTION ADD__TIME_OF_DAY__TIME: TOD
VAR_INPUT
    IN1: TOD;
    IN2: TIME;
END_VAR
    ADD__TIME_OF_DAY__TIME := ADD_TOD_TIME(IN1, IN2);
END_FUNCTION

(********************
*
* This operator returns the value of adding up TOD and TIME.
* Panic on overflow
*
*********************)
{external}
FUNCTION ADD_TOD_TIME: TOD
VAR_INPUT
    IN1: TOD;
    IN2: TIME;
END_VAR
END_FUNCTION

(********************
*
* This operator returns the value of adding up LTOD and LTIME.
* Panic on overflow
*
*********************)
FUNCTION ADD_LTOD_LTIME: LTOD
VAR_INPUT
    IN1: LTOD;
    IN2: LTIME;
END_VAR
    ADD_LTOD_LTIME := ADD_TOD_TIME(IN1, IN2);
END_FUNCTION

(* Specialized implementation of ADD for DT *)
FUNCTION ADD__DATE_AND_TIME__TIME: DT
VAR_INPUT
    IN1: DT;
    IN2: TIME;
END_VAR
    ADD__DATE_AND_TIME__TIME := ADD_DT_TIME(IN1, IN2);
END_FUNCTION

(********************
*
* This operator returns the value of adding up DT and TIME.
* Panic on overflow
*
*********************)
{external}
FUNCTION ADD_DT_TIME: DT
VAR_INPUT
    IN1: DT;
    IN2: TIME;
END_VAR
END_FUNCTION

(********************
*
* This operator returns the value of adding up LDT and LTIME.
* Panic on overflow
*
*********************)
FUNCTION ADD_LDT_LTIME: LDT
VAR_INPUT
    IN1: LDT;
    IN2: LTIME;
END_VAR
    ADD_LDT_LTIME := ADD_DT_TIME(IN1, IN2);
END_FUNCTION

(* Specialized implementation of SUB for TIME *)
FUNCTION SUB__TIME__TIME: TIME
VAR_INPUT
    IN1: TIME;
    IN2: TIME;
END_VAR
    SUB__TIME__TIME := SUB_TIME(IN1, IN2);
END_FUNCTION

(********************
*
* This operator produces the subtraction of two TIME operands
*
*********************)
{external}
FUNCTION SUB_TIME: TIME
VAR_INPUT
    IN1: TIME;
    IN2: TIME;
END_VAR
END_FUNCTION

(********************
*
* This operator produces the subtraction of two LTIME operands
*
*********************)
FUNCTION SUB_LTIME: LTIME
VAR_INPUT
    IN1: LTIME;
    IN2: LTIME;
END_VAR
    SUB_LTIME := SUB_TIME(IN1, IN2);
END_FUNCTION

(* Specialized implementation of SUB for DATE *)
FUNCTION SUB__DATE__DATE: TIME
VAR_INPUT
    IN1: DATE;
    IN2: DATE;
END_VAR
    SUB__DATE__DATE := SUB_DATE_DATE(IN1, IN2);
END_FUNCTION

(********************
*
* This operator produces the subtraction of two DATE operands returning TIME
* Panic on overflow
*
*********************)
{external}
FUNCTION SUB_DATE_DATE: TIME
VAR_INPUT
    IN1: DATE;
    IN2: DATE;
END_VAR
END_FUNCTION

(********************
*
* This operator produces the subtraction of two LDATE operands returning LTIME
* Panic on overflow
*
*********************)
FUNCTION SUB_LDATE_LDATE: LTIME
VAR_INPUT
    IN1: LDATE;
    IN2: LDATE;
END_VAR
    SUB_LDATE_LDATE := SUB_DATE_DATE(IN1, IN2);
END_FUNCTION

(* Specialized implementation of SUB for TOD and TIME *)
FUNCTION SUB__TIME_OF_DAY__TIME: TOD
VAR_INPUT
    IN1: TOD;
    IN2: TIME;
END_VAR
    SUB__TIME_OF_DAY__TIME := SUB_TOD_TIME(IN1, IN2);
END_FUNCTION

(********************
*
* This operator produces the subtraction of TOD and TIME returning TOD
* Panic on overflow
*
*********************)
{external}
FUNCTION SUB_TOD_TIME: TOD
VAR_INPUT
    IN1: TOD;
    IN2: TIME;
END_VAR
END_FUNCTION

(********************
*
* This operator produces the subtraction of LTOD and LTIME returning LTOD
* Panic on overflow
*
*********************)
FUNCTION SUB_LTOD_LTIME: LTOD
VAR_INPUT
    IN1: LTOD;
    IN2: LTIME;
END_VAR
    SUB_LTOD_LTIME := SUB_TOD_TIME(IN1, IN2);
END_FUNCTION

(* Specialized implementation of SUB for TOD *)
FUNCTION SUB__TIME_OF_DAY__TIME_OF_DAY: TIME
VAR_INPUT
    IN1: TOD;
    IN2: TOD;
END_VAR
    SUB__TIME_OF_DAY__TIME_OF_DAY := SUB_TOD_TOD(IN1, IN2);
END_FUNCTION

(********************
*
* This operator produces the subtraction of two TOD operands returning TIME
* Panic on overflow
*
*********************)
{external}
FUNCTION SUB_TOD_TOD: TIME
VAR_INPUT
    IN1: TOD;
    IN2: TOD;
END_VAR
END_FUNCTION

(********************
*
* This operator produces the subtraction of two LTOD operands returning LTIME
* Panic on overflow
*
*********************)
FUNCTION SUB_LTOD_LTOD: LTIME
VAR_INPUT
    IN1: LTOD;
    IN2: LTOD;
END_VAR
    SUB_LTOD_LTOD := SUB_TOD_TOD(IN1, IN2);
END_FUNCTION

(* Specialized implementation of SUB for DT and TIME *)
FUNCTION SUB__DATE_AND_TIME__TIME: DT
VAR_INPUT
    IN1: DT;
    IN2: TIME;
END_VAR
    SUB__DATE_AND_TIME__TIME := SUB_DT_TIME(IN1, IN2);
END_FUNCTION

(********************
*
* This operator produces the subtraction of DT and TIME returning DT
* Panic on overflow
*
*********************)
{external}
FUNCTION SUB_DT_TIME: DT
VAR_INPUT
    IN1: DT;
    IN2: TIME;
END_VAR
END_FUNCTION

(********************
*
* This operator produces the subtraction of LDT and LTIME returning LDT
* Panic on overflow
*
*********************)
FUNCTION SUB_LDT_LTIME: LDT
VAR_INPUT
    IN1: LDT;
    IN2: LTIME;
END_VAR
    SUB_LDT_LTIME := SUB_DT_TIME(IN1, IN2);
END_FUNCTION

(* Specialized implementation of SUB for DT *)
FUNCTION SUB__DATE_AND_TIME__DATE_AND_TIME: TIME
VAR_INPUT
    IN1: DT;
    IN2: DT;
END_VAR
    SUB__DATE_AND_TIME__DATE_AND_TIME := SUB_DT_DT(IN1, IN2);
END_FUNCTION

(********************
*
* This operator produces the subtraction of two DT operands returning TIME
* Panic on overflow
*
*********************)
{external}
FUNCTION SUB_DT_DT: TIME
VAR_INPUT
    IN1: DT;
    IN2: DT;
END_VAR
END_FUNCTION

(********************
*
* This operator produces the subtraction of two LDT operands returning LTIME
* Panic on overflow
*
*********************)
FUNCTION SUB_LDT_LDT: LTIME
VAR_INPUT
    IN1: LDT;
    IN2: LDT;
END_VAR
    SUB_LDT_LDT := SUB_DT_DT(IN1, IN2);
END_FUNCTION

(********************
*
* This operator produces the multiplication of TIME and ANY_NUM.
*
*********************)
FUNCTION MUL_TIME < T: ANY_NUM >: TIME
VAR_INPUT
    IN1: TIME;
    IN2: T;
END_VAR
END_FUNCTION

(********************
*
* This operator produces the multiplication of LTIME and ANY_NUM.
*
*********************)
FUNCTION MUL_LTIME < T: ANY_NUM >: LTIME
VAR_INPUT
    IN1: LTIME;
    IN2: T;
END_VAR
END_FUNCTION

(********************
*
* This operator produces the division of TIME and ANY_NUM.
*
*********************)
FUNCTION DIV_TIME < T: ANY_NUM >: TIME
VAR_INPUT
    IN1: TIME;
    IN2: T;
END_VAR
END_FUNCTION

(********************
*
* This operator produces the division of LTIME and ANY_NUM.
*
*********************)
FUNCTION DIV_LTIME < T: ANY_NUM >: LTIME
VAR_INPUT
    IN1: LTIME;
    IN2: T;
END_VAR
END_FUNCTION
