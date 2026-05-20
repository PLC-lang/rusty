// RUN: (%COMPILE %s && %RUN) | %CHECK %s
PROGRAM test
    VAR
        fbIntContainer : fbIntContainer;
        fbIntContainer_value_store : REFERENCE TO IntContainer;
        fbIntContainer_struct_store : IntContainer;

        fbStringContainer : fbStringContainer;
        fbStringContainer_value_store : REFERENCE TO StringContainer;
        fbStringContainer_struct_store : StringContainer;
    END_VAR

    fbIntContainer_struct_store.child := 51;
    fbIntContainer_value_store REF= fbIntContainer_struct_store;
    fbIntContainer.value := fbIntContainer_value_store;

    // CHECK: FB1-Value: 51
    printf('FB1-Value: %d$N', fbIntContainer.value.child);

    fbStringContainer_struct_store.child := 'Hello There';
    fbStringContainer_value_store REF= fbStringContainer_struct_store;
    fbStringContainer.value := fbStringContainer_value_store;

    // CHECK: FB2-Value: Hello There
    printf('FB2-Value: %s$N', fbStringContainer.value.child);
END_PROGRAM

// INT Container
FUNCTION_BLOCK fbIntContainer
    VAR
        _value	: IntContainer;
    END_VAR

    PROPERTY_GET value : REFERENCE TO IntContainer
        value REF= _value;
    END_PROPERTY

    PROPERTY_SET value : REFERENCE TO IntContainer
        _value := value;
    END_PROPERTY
END_FUNCTION_BLOCK

// String Container
FUNCTION_BLOCK fbStringContainer
    VAR
        _value	: StringContainer;
    END_VAR

    PROPERTY_GET value : REFERENCE TO StringContainer
        value REF= _value;
    END_PROPERTY

    PROPERTY_SET value : REFERENCE TO StringContainer
        _value := value;
    END_PROPERTY
END_FUNCTION_BLOCK

TYPE IntContainer :
	STRUCT
		child : INT := 42;
	END_STRUCT
END_TYPE

TYPE StringContainer :
	STRUCT
		child : STRING[255] := 'Hello World';
	END_STRUCT
END_TYPE

FUNCTION main
    test();
END_FUNCTION
