FUNCTION main
    VAR
        fb1: function_block_1;
        arr: ARRAY[0..14, -2..1] OF INT;
        arr2: ARRAY[0..14] OF INT;
        arr3: ARRAY[0..14, -2..1] OF INT;
        arr4: ARRAY[0..14, -2..1] OF INT;
    END_VAR

    fb1(arr);
    function_1(arr2, arr3, arr4);
END_FUNCTION
