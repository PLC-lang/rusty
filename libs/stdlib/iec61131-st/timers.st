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
******************************************************************************)
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
******************************************************************************)
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
******************************************************************************)
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
******************************************************************************)
FUNCTION_BLOCK TON
VAR_INPUT
	IN: BOOL;
	PT: TIME;
END_VAR
VAR_OUTPUT
	Q: BOOL;
	ET: TIME;
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
******************************************************************************)
FUNCTION_BLOCK TON_TIME
VAR_INPUT
	IN: BOOL;
	PT: TIME;
END_VAR
VAR_OUTPUT
	Q: BOOL;
	ET: TIME;
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
******************************************************************************)
FUNCTION_BLOCK TON_LTIME
VAR_INPUT
	IN: BOOL;
	PT: LTIME;
END_VAR
VAR_OUTPUT
	Q: BOOL;
	ET: TIME;
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
******************************************************************************)
FUNCTION_BLOCK TOF
VAR_INPUT
	IN: BOOL;
	PT:TIME;
END_VAR
VAR_OUTPUT
	Q: BOOL;
	ET: TIME;
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
******************************************************************************)
FUNCTION_BLOCK TOF_TIME
VAR_INPUT
	IN: BOOL;
	PT:TIME;
END_VAR
VAR_OUTPUT
	Q: BOOL;
	ET: TIME;
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
******************************************************************************)
FUNCTION_BLOCK TOF_LTIME
VAR_INPUT
	IN: BOOL;
	PT:LTIME;
END_VAR
VAR_OUTPUT
	Q: BOOL;
	ET: LTIME;
END_VAR
VAR
  __signal__ : BOOL; (* Value representing the internal signal *)
  __BUFFER__ : ARRAY[1..24] OF BYTE; (* Buffer used for internal implementation *)
END_VAR
END_FUNCTION_BLOCK