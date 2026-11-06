FUNCTION main : DINT
VAR
    greeter : CGreeter;
    weights : ARRAY[0..1] OF DINT := [1, 2];
END_VAR
    printf('%d$N', greeter.greet('hi', weights));
    printf('%d$N', greet_through(greeter));
END_FUNCTION
