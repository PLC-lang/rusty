FUNCTION main : DINT
    function_chain.seed := 3;
    function_chain.k := 4;
    function_chain();
    // chain: (seed + k) then (+ k) => (3 + 4) + 4
    // CHECK: result = 11
    printf('result = %d$N', function_chain.result);
END_FUNCTION
