FUNCTION_BLOCK Grid
VAR
    cells : ARRAY[0..1] OF Cell;
END_VAR
    METHOD fill
        cells[0].value := 8;
        cells[1].value := 9;
    END_METHOD

    METHOD sum : DINT
        sum := cells[0].get() + cells[1].get();
    END_METHOD
END_FUNCTION_BLOCK
