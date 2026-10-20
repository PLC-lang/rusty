FUNCTION remainderReal : REAL
VAR_INPUT
    value, range : REAL;
END_VAR
    remainderReal := value MOD range;
END_FUNCTION

FUNCTION remainderLreal : LREAL
VAR_INPUT
    value, range : LREAL;
END_VAR
    remainderLreal := value MOD range;
END_FUNCTION
