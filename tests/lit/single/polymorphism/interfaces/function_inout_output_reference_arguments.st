// RUN: %COMPILE %s && %RUN | %CHECK %s
// Scenario: A FUNCTION that declares an interface as VAR_IN_OUT, VAR_OUTPUT or REFERENCE TO must accept
// a function block instance implementing that interface at the call site, just like VAR_INPUT does, and
// dispatch through it. Previously the call failed with
//   error[E037]: Invalid assignment: cannot assign 'COLLECTIONS_Element_LREAL' to '__FATPOINTER'

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

VAR_GLOBAL
    gElement : COLLECTIONS_Element_LREAL := (Value := 2.5);
END_VAR

FUNCTION ElementGet_Raw : DINT
    VAR_OUTPUT
        buffer : POINTER TO BYTE;
    END_VAR
    VAR_INPUT
        size : ULINT;
    END_VAR

    ElementGet_Raw := ULINT_TO_DINT(size);
END_FUNCTION

// Interface passed as VAR_IN_OUT: the callee dispatches on the caller's instance.
FUNCTION GetElementInOut : DINT
    VAR_IN_OUT
        Element : IElement;
    END_VAR
    VAR
        data : POINTER TO BYTE;
        value : POINTER TO LREAL;
    END_VAR

    data := Element.ElementData();
    value := data;
    GetElementInOut := ElementGet_Raw(buffer => data, Element.ElementSize());
    printf('GetElementInOut: size = %d, value = %.1f$N', GetElementInOut, value^);
END_FUNCTION

// Interface passed as REFERENCE TO: the callee dispatches on the referenced instance.
FUNCTION GetElementRef : DINT
    VAR_INPUT
        Element : REFERENCE TO IElement;
    END_VAR
    VAR
        data : POINTER TO BYTE;
        value : POINTER TO LREAL;
    END_VAR

    data := Element.ElementData();
    value := data;
    GetElementRef := ElementGet_Raw(buffer => data, Element.ElementSize());
    printf('GetElementRef: size = %d, value = %.1f$N', GetElementRef, value^);
END_FUNCTION

// Interface returned through VAR_OUTPUT: the callee hands an instance back to the caller.
FUNCTION GetElementOutput : DINT
    VAR_OUTPUT
        Element : IElement;
    END_VAR

    Element := gElement;
    GetElementOutput := ULINT_TO_DINT(Element.ElementSize());
END_FUNCTION

FUNCTION main : DINT
    VAR
        element : COLLECTIONS_Element_LREAL := (Value := 1.5);
        returned : IElement;
        value : POINTER TO LREAL;
        result : DINT;
    END_VAR

    // CHECK: GetElementInOut: size = 8, value = 1.5
    // CHECK-NEXT: main: inout result = 8
    result := GetElementInOut(element);
    printf('main: inout result = %d$N', result);

    // CHECK-NEXT: GetElementRef: size = 8, value = 1.5
    // CHECK-NEXT: main: ref result = 8
    result := GetElementRef(element);
    printf('main: ref result = %d$N', result);

    // CHECK-NEXT: main: output result = 8, size = 8, value = 2.5
    result := GetElementOutput(returned);
    value := returned.ElementData();
    printf('main: output result = %d, size = %llu, value = %.1f$N', result, returned.ElementSize(), value^);

    // A concrete instance is accepted as the output argument too. The callee writes into a temporary
    // fat pointer that nothing copies back, so only the return value is observable.
    // CHECK-NEXT: main: output into instance result = 8, value = 1.5
    result := GetElementOutput(element);
    printf('main: output into instance result = %d, value = %.1f$N', result, element.Value);
END_FUNCTION
