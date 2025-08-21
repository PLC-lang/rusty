// RUN: (%COMPILE %s && %RUN) | %CHECK %s

FUNCTION_BLOCK A
    VAR
        nameA: STRING := 'FUNCTION_BLOCK A';
    END_VAR

    METHOD printName
        printf('name = %s$N', ADR(nameA));
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION_BLOCK B EXTENDS A
    VAR
        nameB: STRING := 'FUNCTION_BLOCK B';
    END_VAR

    METHOD printName
        printf('name = %s$N', ADR(nameB));
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION_BLOCK C
    VAR
        nameC: STRING := 'FUNCTION_BLOCK C';
    END_VAR

    METHOD printName
        printf('name = %s$N', ADR(nameC));
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION main
    VAR
        instanceA: A;
        instanceB: B;
        instanceC: C;
        fnPtrPrintName: FNPTR A.printName := ADR(A.printName);
    END_VAR

    // Ensure that child POUs can be passed to function pointers of both its own and parent POUs. When doing
    // so we expect a bitcast on these passed arguments to the (in this case) parent POU.
    // CHECK: name = FUNCTION_BLOCK A
    // CHECK: name = FUNCTION_BLOCK A
    fnPtrPrintName^(instanceA);
    fnPtrPrintName^(instanceB); // <- fnPtrPrintName points to A.printName but we pass instanceB which should be fine and result in a bitcast

    // To showcase, we could pass an UNRELATED instance to the function pointer. Because it expects something
    // along `void (%A)` in LLVM IR (no return type, no parameters) passing `instanceC` should result in a
    // bitcast to `A`. The function (printName) should then GEP the first member field because `name` is 
    // positioned there, resulting in access onto `nameC` rather than `nameA` and therefore a complete
    // different result when compared with a child instance. Illustrated (without size information)
    // `type A = { string }`    -- expanded --> `type A = { nameA }`
    // `type B = { A, string }` -- expanded --> `type B = { { nameA }, nameB }`
    // `type C = { string}      -- expanded --> `type C = { nameC }`
    // That should explain why we get `FUNCTION_BLOCK A` when passing `instanceB` and also why we get 
    // `FUNCTION_BLOCK C` when passing `instanceC`
    // CHECK: name = FUNCTION_BLOCK C
    fnPtrPrintName^(instanceC);
END_FUNCTION