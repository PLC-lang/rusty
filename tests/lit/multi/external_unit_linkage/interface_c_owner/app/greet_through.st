FUNCTION greet_through : DINT
VAR_INPUT
    greeter : IGreet;
END_VAR
VAR
    weights : ARRAY[0..1] OF DINT := [3, 4];
END_VAR
    greet_through := greeter.greet('hello', weights);
END_FUNCTION
