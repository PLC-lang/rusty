FUNCTION greet_through : DINT
VAR_INPUT
    greeter : IGreet;
END_VAR
VAR
    weights : ARRAY[0..1] OF DINT := [3, 4];
END_VAR
    greet_through := greeter.greet('hello', weights);
END_FUNCTION

FUNCTION main : DINT
VAR
    greeter : CGreeter;
    weights : ARRAY[0..1] OF DINT := [1, 2];
END_VAR
    printf('%d$N', greeter.greet('hi', weights));
    printf('%d$N', greet_through(greeter));
END_FUNCTION
