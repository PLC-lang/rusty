FUNCTION_BLOCK child EXTENDS baseFb
    THIS^.x := This^.x + 5; // x is now 16
    THIS^.y := THIS^.y + 2; // y is now 18
END_FUNCTION_BLOCK
