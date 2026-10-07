FUNCTION increment : REFERENCE TO DINT
VAR_INPUT
    value : REFERENCE TO DINT;
END_VAR
    value := value + 1;
    increment REF= value;
END_FUNCTION

FUNCTION increment_twice : DINT
VAR
    x : DINT := 5;
    r : REFERENCE TO DINT;
END_VAR
    r REF= x;
    r REF= increment(r);
    r REF= increment(r);
    increment_twice := r;
END_FUNCTION
