// RUN: (%COMPILE %s && %RUN) | %CHECK %s

TYPE PositionWithExtraMetadata:
    STRUCT
        x: REAL;
        y: REAL;
        data: ARRAY[1..100] OF STRING;
    END_STRUCT
END_TYPE

FUNCTION_BLOCK FbA
    METHOD foo
        VAR_OUTPUT
            outString: STRING;
            outStruct: PositionWithExtraMetadata;
            outStructArray: ARRAY[1..5] OF PositionWithExtraMetadata;
        END_VAR

        outString := 'FbA::foo result';
        outStruct.x := 1.0;
        outStruct.y := 2.0;
        outStruct.data[1] := 'Data 1';
        outStruct.data[2] := 'Data 2';
        outStruct.data[3] := 'Data 3';
        outStructArray[1].x := 3.0;
        outStructArray[1].y := 4.0;
        outStructArray[1].data[1] := 'Data 4';
        outStructArray[1].data[2] := 'Data 5';
        outStructArray[1].data[3] := 'Data 6';
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION main
    VAR
        fooPtr: POINTER TO FbA.foo := ADR(FbA.foo);
        instanceA: FbA;
        localOutString: STRING;
        localOutStruct: PositionWithExtraMetadata;
        localOutStructArray: ARRAY[1..5] OF PositionWithExtraMetadata;
    END_VAR

    // CHECK: localOutString = FbA::foo result
    // CHECK: localOutStruct.x = 1.000000
    // CHECK: localOutStruct.y = 2.000000
    // CHECK: localOutStruct.data[1] = Data 1
    // CHECK: localOutStruct.data[2] = Data 2
    // CHECK: localOutStruct.data[3] = Data 3
    // CHECK: localOutStructArray[1].x = 3.000000
    // CHECK: localOutStructArray[1].y = 4.000000
    // CHECK: localOutStructArray[1].data[1] = Data 4
    // CHECK: localOutStructArray[1].data[2] = Data 5
    // CHECK: localOutStructArray[1].data[3] = Data 6
    fooPtr^(instanceA, outStructArray => localOutStructArray, outString => localOutString, outStruct => localOutStruct );
    printf('localOutString = %s$N', ADR(localOutString));
    printf('localOutStruct.x = %f$N', localOutStruct.x);
    printf('localOutStruct.y = %f$N', localOutStruct.y);
    printf('localOutStruct.data[1] = %s$N', ADR(localOutStruct.data[1]));
    printf('localOutStruct.data[2] = %s$N', ADR(localOutStruct.data[2]));
    printf('localOutStruct.data[3] = %s$N', ADR(localOutStruct.data[3]));
    printf('localOutStructArray[1].x = %f$N', localOutStructArray[1].x);
    printf('localOutStructArray[1].y = %f$N', localOutStructArray[1].y);
    printf('localOutStructArray[1].data[1] = %s$N', ADR(localOutStructArray[1].data[1]));
    printf('localOutStructArray[1].data[2] = %s$N', ADR(localOutStructArray[1].data[2]));
    printf('localOutStructArray[1].data[3] = %s$N', ADR(localOutStructArray[1].data[3]));
END_FUNCTION