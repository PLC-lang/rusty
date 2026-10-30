// RUN: rm -f %t.out
// RUN: not %COMPILE --error-format clang %s 2>&1 | %CHECK --implicit-check-not='{{.*}}__FATPOINTER{{.*}}' %s
// RUN: test ! -e %t.out
// Scenario: a function block instance passed to a function VAR_OUTPUT of interface type is
// rejected with a type mismatch naming the interface; the output needs an IElement variable.

INTERFACE IElement
    METHOD ElementData : POINTER TO BYTE
    END_METHOD
    METHOD ElementSize : ULINT
    END_METHOD
END_INTERFACE

FUNCTION_BLOCK COLLECTIONS_Element_LREAL IMPLEMENTS IElement
    VAR_INPUT
        Value : LREAL;
    END_VAR
    METHOD ElementData : POINTER TO BYTE
        ElementData := ADR(Value);
    END_METHOD
    METHOD ElementSize : ULINT
        ElementSize := SIZEOF(Value);
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION GetElement : DINT
    VAR_OUTPUT
        // CHECK-DAG: {{.*}}by_ref_argument_reproducer.st:[[@LINE+1]]:9:{{.*}}info[E159]: Parameter 'Element' is an output and needs a variable of type 'IElement'
        Element : IElement;
    END_VAR
END_FUNCTION

PROGRAM mainProg
    VAR
        element : COLLECTIONS_Element_LREAL;
    END_VAR
    // CHECK-DAG: {{.*}}by_ref_argument_reproducer.st:[[@LINE+1]]:16:{{.*}}error[E126]: Type mismatch: expected type 'IElement' but received 'COLLECTIONS_Element_LREAL'; pass a variable of type 'IElement' to 'GetElement'
    GetElement(element);
END_PROGRAM
// CHECK: error: Compilation aborted due to critical errors
