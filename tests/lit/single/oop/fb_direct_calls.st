// RUN: (%COMPILE %s && %RUN) | %CHECK %s

TYPE Position2D:
    STRUCT
        x: DINT;
        y: DINT;
    END_STRUCT
END_TYPE

TYPE Position3D:
    STRUCT
        x: DINT;
        y: DINT;
        z: DINT;
    END_STRUCT
END_TYPE

FUNCTION_BLOCK FbA
    VAR
        placeholderA: STRING;
    END_VAR

    VAR_INPUT
        x: DINT;
        y: DINT;
    END_VAR

    VAR_OUTPUT
        result2D: Position2D;
    END_VAR

    result2D.x := x;
    result2D.y := y;
END_FUNCTION_BLOCK

FUNCTION_BLOCK FbB EXTENDS FbA
    VAR_INPUT
        z: DINT;
    END_VAR

    VAR_OUTPUT
        result3D: Position3D;
    END_VAR

    // using the values from FbA
    result3D.x := x;
    result3D.y := y;
    result3D.z := z;
END_FUNCTION_BLOCK

FUNCTION_BLOCK FbC EXTENDS FbB
    VAR_INPUT
        multiplier2D: DINT;
        multiplier3D: DINT;
    END_VAR

    VAR
        placeholderC1: STRING;
        placeholderC2: INT;
    END_VAR

    VAR_IN_OUT
        resultMultiplied2D: Position2D;
        resultMultiplied3D: Position3D;
    END_VAR

    resultMultiplied2D.x := x * multiplier2D;
    resultMultiplied2D.y := y * multiplier2D;

    resultMultiplied3D.x := x * multiplier3D;
    resultMultiplied3D.y := y * multiplier3D;
    resultMultiplied3D.z := z * multiplier3D;
END_FUNCTION_BLOCK

FUNCTION main
    VAR
        instanceA: FbA;
        instanceB: FbB;
        instanceC: FbC;

        pos2D: Position2D;
        pos3D: Position3D;
    END_VAR

    // CHECK: Position2D(10, 20)
    instanceA(
        x := 10,
        result2D => pos2D,
        y := 20,
    );
    printf('Position2D(%d, %d)$N', pos2D.x, pos2D.y);

    // CHECK-NEXT: Position3D(40, 50, 60)
    instanceB(
        z := 60,
        y := 50,
        result3D => pos3D,
        x := 40,
    );
    printf('Position3D(%d, %d, %d)$N', pos3D.x, pos3D.y, pos3D.z);

    // CHECK-NEXT: Position2D(140, 160)
    // CHECK-NEXT: Position3D(280, 320, 360)
    instanceC(
        multiplier2D := 2,
        z := 90,
        x := 70,
        resultMultiplied3D := pos3D,
        y := 80,
        multiplier3D := 4,
        resultMultiplied2D := pos2D,
    );
    printf('Position2D(%d, %d)$N', pos2D.x, pos2D.y);
    printf('Position3D(%d, %d, %d)$N', pos3D.x, pos3D.y, pos3D.z);
END_FUNCTION