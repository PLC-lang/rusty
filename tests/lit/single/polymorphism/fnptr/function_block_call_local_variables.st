// RUN: (%COMPILE %s && %RUN) | %CHECK %s

TYPE Position: STRUCT
        x: DINT;
        y: DINT;
    END_STRUCT
END_TYPE

FUNCTION_BLOCK A
    VAR
        localStateInt: DINT;
        localStateString: STRING;
        localStatePosition: Position;
        localStateStringArray: ARRAY[1..2] OF STRING;
        localStatePositionArray: ARRAY[1..2] OF Position;
    END_VAR

    METHOD printState
        printf('A::printState: localStateInt = %d$N', localStateInt);
        printf('A::printState: localStateString = %s$N', ADR(localStateString));
        printf('A::printState: localStatePosition = (%d, %d)$N', localStatePosition.x, localStatePosition.y);
        printf('A::printState: localStateStringArray[1] = %s$N', ADR(localStateStringArray[1]));
        printf('A::printState: localStateStringArray[2] = %s$N', ADR(localStateStringArray[2]));
        printf('A::printState: localStatePositionArray[1] = (%d, %d)$N', localStatePositionArray[1].x, localStatePositionArray[1].y);
        printf('A::printState: localStatePositionArray[2] = (%d, %d)$N', localStatePositionArray[2].x, localStatePositionArray[2].y);
    END_METHOD

    METHOD reset
        THIS^();
    END_METHOD

    // Init
    localStateInt := 5;
    localStateString := 'ten';
    localStatePosition.x := 15;
    localStatePosition.y := 20;
    localStateStringArray[1] := 'twenty-five';
    localStateStringArray[2] := 'thirty';
    localStatePositionArray[1].x := 35;
    localStatePositionArray[1].y := 40;
    localStatePositionArray[2].x := 45;
    localStatePositionArray[2].y := 50;
END_FUNCTION_BLOCK

FUNCTION main
    VAR
        instanceA: A;
        fnPtrBody: FNPTR A := ADR(A);
    END_VAR

    // CHECK: A::printState: localStateInt = 0
    // CHECK: A::printState: localStateString =
    // CHECK: A::printState: localStatePosition = (0, 0)
    // CHECK: A::printState: localStateStringArray[1] =
    // CHECK: A::printState: localStateStringArray[2] =
    // CHECK: A::printState: localStatePositionArray[1] = (0, 0)
    // CHECK: A::printState: localStatePositionArray[2] = (0, 0)
    instanceA.printState();

    // CHECK: A::printState: localStateInt = 5
    // CHECK: A::printState: localStateString = ten
    // CHECK: A::printState: localStatePosition = (15, 20)
    // CHECK: A::printState: localStateStringArray[1] = twenty-five
    // CHECK: A::printState: localStateStringArray[2] = thirty
    // CHECK: A::printState: localStatePositionArray[1] = (35, 40)
    // CHECK: A::printState: localStatePositionArray[2] = (45, 50)
    fnPtrBody^(instanceA);
    instanceA.printState();
END_FUNCTION