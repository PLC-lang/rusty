// RUN: (%COMPILE %s && %RUN) | %CHECK %s

TYPE PositionWithExtraMetadata:
    STRUCT
        x: REAL;
        y: REAL;
        data: ARRAY[1..100] OF STRING;
    END_STRUCT
END_TYPE

FUNCTION_BLOCK FbA
    VAR
        localStruct: PositionWithExtraMetadata := (x := 1.0, y := 2.0, data := ['Data 1', 'Data 2', 'Data 3']);
        localStringArray: ARRAY[1..3] OF STRING := ['Value 1', 'Value 2', 'Value 3'];
        localStructArray: ARRAY[1..3] OF PositionWithExtraMetadata := [
            (x := 3.0, y := 4.0, data := ['Data 4', 'Data 5', 'Data 6']),
            (x := 5.0, y := 6.0, data := ['Data 7', 'Data 8', 'Data 9']),
            (x := 7.0, y := 8.0, data := ['Data 10', 'Data 11', 'Data 12'])
        ];
    END_VAR

    METHOD returnString: STRING
        returnString := 'FbA::returnString';
    END_METHOD

    METHOD returnStruct: PositionWithExtraMetadata
        returnStruct := localStruct;
    END_METHOD

    METHOD returnStringArray: ARRAY[1..3] OF STRING
        returnStringArray := localStringArray;
    END_METHOD

    METHOD returnStructArray: ARRAY[1..3] OF PositionWithExtraMetadata
        returnStructArray := localStructArray;
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION main
    VAR
        instanceA: FbA;
        returnStringPtr: POINTER TO FbA.returnString := ADR(instanceA.returnString);
        returnStructPtr: POINTER TO FbA.returnStruct := ADR(instanceA.returnStruct);
        returnStringArrayPtr: POINTER TO FbA.returnStringArray := ADR(instanceA.returnStringArray);
        returnStructArrayPtr: POINTER TO FbA.returnStructArray := ADR(instanceA.returnStructArray);

        localReturnString: STRING;
        localReturnStruct: PositionWithExtraMetadata;
        localReturnStringArray: ARRAY[1..3] OF STRING;
        localReturnStructArray: ARRAY[1..3] OF PositionWithExtraMetadata;
    END_VAR

    // localReturnString := instanceA.returnString();
    // CHECK: localReturnString = FbA::returnString
    localReturnString := returnStringPtr^(instanceA);
    printf('localReturnString = %s$N', ADR(localReturnString));

    // localReturnStruct := instanceA.returnStruct();
    // CHECK: localReturnStruct.x = 1.000000
    // CHECK: localReturnStruct.y = 2.000000
    // CHECK: localReturnStruct.data[1] = Data 1
    // CHECK: localReturnStruct.data[2] = Data 2
    // CHECK: localReturnStruct.data[3] = Data 3
    localReturnStruct := returnStructPtr^(instanceA);
    printf('localReturnStruct.x = %f$N', localReturnStruct.x);
    printf('localReturnStruct.y = %f$N', localReturnStruct.y);
    printf('localReturnStruct.data[1] = %s$N', ADR(localReturnStruct.data[1]));
    printf('localReturnStruct.data[2] = %s$N', ADR(localReturnStruct.data[2]));
    printf('localReturnStruct.data[3] = %s$N', ADR(localReturnStruct.data[3]));

    // localReturnStringArray := instanceA.returnStringArray();
    // CHECK: localReturnStringArray[1] = Value 1
    // CHECK: localReturnStringArray[2] = Value 2
    // CHECK: localReturnStringArray[3] = Value 3
    localReturnStringArray := returnStringArrayPtr^(instanceA);
    printf('localReturnStringArray[1] = %s$N', ADR(localReturnStringArray[1]));
    printf('localReturnStringArray[2] = %s$N', ADR(localReturnStringArray[2]));
    printf('localReturnStringArray[3] = %s$N', ADR(localReturnStringArray[3]));

    // localReturnStructArray := instanceA.returnStructArray();
    // CHECK: localReturnStructArray[1].x = 3.000000
    // CHECK: localReturnStructArray[1].y = 4.000000
    // CHECK: localReturnStructArray[1].data[1] = Data 4
    // CHECK: localReturnStructArray[1].data[2] = Data 5
    // CHECK: localReturnStructArray[1].data[3] = Data 6
    // CHECK: localReturnStructArray[2].x = 5.000000
    // CHECK: localReturnStructArray[2].y = 6.000000
    // CHECK: localReturnStructArray[2].data[1] = Data 7
    // CHECK: localReturnStructArray[2].data[2] = Data 8
    // CHECK: localReturnStructArray[2].data[3] = Data 9
    // CHECK: localReturnStructArray[3].x = 7.000000
    // CHECK: localReturnStructArray[3].y = 8.000000
    // CHECK: localReturnStructArray[3].data[1] = Data 10
    // CHECK: localReturnStructArray[3].data[2] = Data 11
    // CHECK: localReturnStructArray[3].data[3] = Data 12
    localReturnStructArray := returnStructArrayPtr^(instanceA);
    printf('localReturnStructArray[1].x = %f$N', localReturnStructArray[1].x);
    printf('localReturnStructArray[1].y = %f$N', localReturnStructArray[1].y);
    printf('localReturnStructArray[1].data[1] = %s$N', ADR(localReturnStructArray[1].data[1]));
    printf('localReturnStructArray[1].data[2] = %s$N', ADR(localReturnStructArray[1].data[2]));
    printf('localReturnStructArray[1].data[3] = %s$N', ADR(localReturnStructArray[1].data[3]));
    printf('localReturnStructArray[2].x = %f$N', localReturnStructArray[2].x);
    printf('localReturnStructArray[2].y = %f$N', localReturnStructArray[2].y);
    printf('localReturnStructArray[2].data[1] = %s$N', ADR(localReturnStructArray[2].data[1]));
    printf('localReturnStructArray[2].data[2] = %s$N', ADR(localReturnStructArray[2].data[2]));
    printf('localReturnStructArray[2].data[3] = %s$N', ADR(localReturnStructArray[2].data[3]));
    printf('localReturnStructArray[3].x = %f$N', localReturnStructArray[3].x);
    printf('localReturnStructArray[3].y = %f$N', localReturnStructArray[3].y);
    printf('localReturnStructArray[3].data[1] = %s$N', ADR(localReturnStructArray[3].data[1]));
    printf('localReturnStructArray[3].data[2] = %s$N', ADR(localReturnStructArray[3].data[2]));
    printf('localReturnStructArray[3].data[3] = %s$N', ADR(localReturnStructArray[3].data[3]));
END_FUNCTION