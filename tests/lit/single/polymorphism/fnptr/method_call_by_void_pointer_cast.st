// RUN: (%COMPILE %s && %RUN) | %CHECK %s

FUNCTION_BLOCK A
    METHOD printArgs
        VAR_INPUT
            message: STRING;
            value: DINT;
        END_VAR

        printf('%s %d$N', ADR(message), value);
    END_METHOD
END_FUNCTION_BLOCK

TYPE FnTable:
    STRUCT
        printArgs: POINTER TO A.printArgs := ADR(A.printArgs);
    END_STRUCT
END_TYPE

VAR_GLOBAL
    fnTableInstance: FnTable;
END_VAR

FUNCTION main
    VAR
        instanceA: A;
        fnTableRef: POINTER TO __VOID := ADR(fnTableInstance);
    END_VAR

    // Not a direct cast of a void pointer function but rather its virtual table, basically how the virtual
    // table is also accessed in desugared code (though the access happens in the POU rather than a local
    // variable in the main function)
    // CHECK: value = 5
    // CHECK: value = 5
    FnTable#(fnTableRef^).printArgs^(instanceA, 'value =', 5);
    FnTable#(fnTableRef^).printArgs^(instanceA, message := 'value =', value := 5);
END_FUNCTION