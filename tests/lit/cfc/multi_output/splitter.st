// A function block with two distinct outputs, to exercise multi-output `=>` wiring.
FUNCTION_BLOCK Splitter
VAR_INPUT
    n : DINT;
END_VAR
VAR_OUTPUT
    lo : DINT;
    hi : DINT;
END_VAR
    lo := n;
    hi := n + 100;
END_FUNCTION_BLOCK
