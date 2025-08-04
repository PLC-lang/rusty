// RUN: (%COMPILE %s && %RUN) | %CHECK %s

VAR_GLOBAL
    instanceVirtuaTableA: VirtualTableA;
    instanceVirtuaTableB: VirtualTableB;
    instanceVirtuaTableC: VirtualTableC;
END_VAR

TYPE VirtualTableA:
    STRUCT
        getName: POINTER TO A.getName := ADR(A.getName);
        printName: POINTER TO A.printName := ADR(A.printName);
    END_STRUCT
END_TYPE

TYPE VirtualTableB:
    STRUCT
        getName: POINTER TO A.getName := ADR(A.getName);
        printName: POINTER TO A.printName := ADR(A.printName);
    END_STRUCT
END_TYPE

TYPE VirtualTableC:
    STRUCT
        getName: POINTER TO C.getName := ADR(C.getName);
        printName: POINTER TO A.printName := ADR(A.printName);
    END_STRUCT
END_TYPE

FUNCTION_BLOCK A
    VAR
        vt: POINTER TO __VOID;
    END_VAR

    METHOD getName: STRING
        getName := 'A';
    END_METHOD

    METHOD printName
        VAR
            result: STRING;
        END_VAR

        printf('name = %s$N', ADR(VirtualTableA#(vt^).getName^(THIS^)));
    END_METHOD
END_FUNCTION_BLOCK

// Inherits A as is
FUNCTION_BLOCK B EXTENDS A
END_FUNCTION_BLOCK

// Overrides the `getName` method
FUNCTION_BLOCK C EXTENDS A
    METHOD getName: STRING
        getName := 'C';
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION main
    VAR
        instanceA: A;
        instanceB: B;
        instanceC: C;
    END_VAR

    instanceA.vt := ADR(instanceVirtuaTableA);
    instanceB.vt := ADR(instanceVirtuaTableB);
    instanceC.vt := ADR(instanceVirtuaTableC);

    // The `printName` call will use the virtual table to call the correct `getName` method
    // CHECK: name = A
    // CHECK: name = A
    // CHECK: name = C
    instanceA.printName(); // base method, hence A
    instanceB.printName(); // B inherits A without overriding any methods, hence A
    instanceC.printName(); // C inherits A overriding the `getName` method, hence C
END_FUNCTION