// RUN: (%COMPILE %s && %RUN) | %CHECK %s

TYPE User:
    STRUCT
        id: DINT;
        name: STRING;
    END_STRUCT
END_TYPE

INTERFACE A
    METHOD returnString: STRING
    END_METHOD

    METHOD returnArray: ARRAY[1..5] OF DINT
    END_METHOD

    METHOD returnStruct: User 
    END_METHOD
END_INTERFACE

FUNCTION_BLOCK FbA IMPLEMENTS A
    METHOD returnString: STRING
        returnString := 'FbA';
    END_METHOD

    METHOD returnArray: ARRAY[1..5] OF DINT
        returnArray := [1, 2, 3, 4, 5];
    END_METHOD

    METHOD returnStruct: User 
        VAR
            result: User := (id := 42, name := 'FbA::User');
        END_VAR

        returnStruct := result;
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION_BLOCK FbB IMPLEMENTS A
    METHOD returnString: STRING
        returnString := 'FbB';
    END_METHOD

    METHOD returnArray: ARRAY[1..5] OF DINT
        returnArray := [6, 7, 8, 9, 10];
    END_METHOD

    METHOD returnStruct: User 
        VAR
            result: User := (id := 43, name := 'FbB::User');
        END_VAR

        returnStruct := result;
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION main
    VAR
        refA: A;
        instanceFbA: FbA;
        instanceFbB: FbB;

        resultReturnString: STRING;
        resultReturnArray: ARRAY[1..5] OF DINT;
        resultReturnStruct: User;
    END_VAR

    // ~~ InstanceFbA ~~
    refA := instanceFbA;
    resultReturnString := refA.returnString();
    resultReturnArray := refA.returnArray();
    resultReturnStruct := refA.returnStruct();

    // CHECK: resultReturnString = FbA
    // CHECK: resultReturnArray = [1, 2, 3, 4, 5]
    // CHECK: resultReturnStruct = User { id = 42, name = FbA::User }
    printf('resultReturnString = %s$N', resultReturnString);
    printf('resultReturnArray = [%d, %d, %d, %d, %d]$N', resultReturnArray[1], resultReturnArray[2], resultReturnArray[3], resultReturnArray[4], resultReturnArray[5]);
    printf('resultReturnStruct = User { id = %d, name = %s }$N', resultReturnStruct.id, resultReturnStruct.name);

    // ~~ InstanceFbB ~~
    refA := instanceFbB;
    resultReturnString := refA.returnString();
    resultReturnArray := refA.returnArray();
    resultReturnStruct := refA.returnStruct();

    // CHECK: resultReturnString = FbB
    // CHECK: resultReturnArray = [6, 7, 8, 9, 10]
    // CHECK: resultReturnStruct = User { id = 43, name = FbB::User }
    printf('resultReturnString = %s$N', resultReturnString);
    printf('resultReturnArray = [%d, %d, %d, %d, %d]$N', resultReturnArray[1], resultReturnArray[2], resultReturnArray[3], resultReturnArray[4], resultReturnArray[5]);
    printf('resultReturnStruct = User { id = %d, name = %s }$N', resultReturnStruct.id, resultReturnStruct.name);
END_FUNCTION