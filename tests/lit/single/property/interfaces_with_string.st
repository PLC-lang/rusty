// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// XFAIL: *
// TODO: Disabled because it doesnt work yet since an INTERFACE
//       with a PROPERTY of type STRING/STRUCT fails to compile
//       atm.
TYPE MyStruct : STRUCT
  ArrayOfStrings : ARRAY[0..100] OF STRING[10];
END_STRUCT
END_TYPE
INTERFACE myInterface
    PROPERTY foo : STRING
        GET
        END_GET
        SET END_SET
    END_PROPERTY
END_INTERFACE
FUNCTION_BLOCK fb2 implements myInterface
    VAR
        waldo : ARRAY[0..100] OF MyStruct;
    END_VAR
    PROPERTY foo : STRING
        GET
            foo := waldo[42].ArrayOfStrings[88];
        END_GET
        SET
            waldo[42].ArrayOfStrings[88] := foo;
        END_SET
    END_PROPERTY
END_FUNCTION_BLOCK
FUNCTION_BLOCK fb implements myInterface
    VAR
        fubo : fb2;
    END_VAR
    PROPERTY foo : STRING
        GET
            foo := fubo.foo;
        END_GET
        SET
          fubo.foo := foo;
        END_SET
    END_PROPERTY
END_FUNCTION_BLOCK

FUNCTION main
    VAR
        instanceA : fb;
    END_VAR
    instanceA.fubo.foo := 'This is quite some text!';
    printf('instanceA.fubo.foo = %s$n', instanceA.fubo.foo); // CHECK: instanceA.fubo.baz = This is quite some text!
END_FUNCTION
