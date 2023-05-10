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
******************************************************************************)
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
******************************************************************************)
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
******************************************************************************)
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
******************************************************************************)
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
******************************************************************************)
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
******************************************************************************)
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
******************************************************************************)
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
******************************************************************************)
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
******************************************************************************)
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
******************************************************************************)
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
******************************************************************************)
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
******************************************************************************)
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
******************************************************************************)
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
******************************************************************************)
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
******************************************************************************)
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
******************************************************************************)
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
******************************************************************************)
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
******************************************************************************)
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
END_FUNCTION_BLOCK