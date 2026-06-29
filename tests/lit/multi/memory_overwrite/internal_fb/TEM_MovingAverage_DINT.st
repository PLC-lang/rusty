(******************************************************************************

	Descripsion:
			This functionblock build an moving average of the 
			input value depending to the cyles

	Input:	
			diInputValue	 : DINT				// Input Value 
			diCycles		 : DINT				// Number of Cycles

	Output:
			diAverageValue  : DINT				// Averaged Value 



	date	: 06.2026 
	auhtor	: axwe bachmann electronic GmbH

*******************************************************************************)
FUNCTION_BLOCK TEM_MovingAverage_DINT
VAR_INPUT
	diInputValue : DINT;
	udiCycles	 : UDINT;
END_VAR

VAR_OUTPUT
	diAverageValue	: DINT;
END_VAR

VAR
	diDividend 	: DINT;
	diDivisor	: DINT;
	diQuotient	: DINT;
	diDivRest	: DINT;

END_VAR
(* moving average *)
IF udiCycles <= 1 THEN
	diAverageValue := diInputValue;
ELSE
	diDividend := (diInputValue-diAverageValue)+diDivRest;
	diDivisor := UDINT_TO_DINT(udiCycles);

	(* Division dint *)
	IF diDivisor <> 0 THEN
		IF diDivisor = 0 THEN
			diDivRest := diDividend-(diQuotient*diDivisor);
		ELSE
			diQuotient := diDividend/diDivisor;
			diDivRest := diDividend-(diQuotient*diDivisor);
		END_IF
	ELSE
		IF diDividend >= 0 THEN
			diQuotient := 2147483647;
			diDivRest := 0;
		ELSE
			diQuotient := -2147483648;
			diDivRest := 0;
		END_IF
	END_IF
	(* End Division dint *)

	diAverageValue := diAverageValue+diQuotient;
END_IF

END_FUNCTION_BLOCK