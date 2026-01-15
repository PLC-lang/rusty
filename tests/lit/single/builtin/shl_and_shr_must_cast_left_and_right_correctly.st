// RUN: (%COMPILE %s && %RUN) | %CHECK %s

// SHL and SHR must cast left and right correctly
FUNCTION main
VAR
    value : BYTE;
END_VAR

value := BIT_LOAD_B2(BYTE#255, TRUE, 2, 2);

// CHECK: 255
printf('%d$N', value);

value := BIT_LOAD_B2(BYTE#255, FALSE, 2, 2);

// CHECK: 243
printf('%d$N', value);

END_FUNCTION

FUNCTION BIT_LOAD_B2 : BYTE
VAR_INPUT
	I : BYTE;
	D : BOOL;
	P : INT;
	N : INT;
END_VAR
VAR
    result : BYTE;
END_VAR

IF D THEN
	BIT_LOAD_B2 := ROL(SHR(BYTE#255, 8 - N) OR ROR(I, P), P);
ELSE
	BIT_LOAD_B2 := ROL(SHL(BYTE#255, N) AND ROR(I, P), P);
END_IF;

END_FUNCTION
