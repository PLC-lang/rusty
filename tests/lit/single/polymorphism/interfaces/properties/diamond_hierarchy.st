// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Scenario: Classic diamond pattern with a property at the root.
//     IA (property value)
//    /  \
//   IB   IC
//    \  /
//     ID
// IB adds method bar, IC adds method baz, ID adds method qux. A single function block
// FbImpl implements ID (and thus transitively IA, IB, IC). Dispatch through IA, IB, IC
// and ID variables should all resolve the property correctly, and each interface level
// should expose its respective methods.

INTERFACE IA
    PROPERTY value : DINT
        PROPERTY_GET END_GET
        PROPERTY_SET END_SET
    END_PROPERTY
END_INTERFACE

INTERFACE IB EXTENDS IA
    METHOD bar END_METHOD
END_INTERFACE

INTERFACE IC EXTENDS IA
    METHOD baz END_METHOD
END_INTERFACE

INTERFACE ID EXTENDS IB, IC
    METHOD qux END_METHOD
END_INTERFACE

FUNCTION_BLOCK FbImpl IMPLEMENTS ID
    VAR
        _val : DINT := 10;
    END_VAR

    PROPERTY value : DINT
        PROPERTY_GET
            value := _val;
        END_GET

        PROPERTY_SET
            _val := value;
        END_SET
    END_PROPERTY

    METHOD bar
        printf('bar(%d)$N', _val);
    END_METHOD

    METHOD baz
        printf('baz(%d)$N', _val);
    END_METHOD

    METHOD qux
        printf('qux(%d)$N', _val);
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION main
    VAR
        instA : FbImpl;
        instB : FbImpl;
        instC : FbImpl;
        instD : FbImpl;
        refA : IA;
        refB : IB;
        refC : IC;
        refD : ID;
    END_VAR

    // Through IA — property only
    refA := instA;
    printf('value=%d$N', refA.value);           // CHECK: value=10
    refA.value := 42;
    printf('value=%d$N', refA.value);           // CHECK-NEXT: value=42

    // Through IB — property + bar
    refB := instB;
    printf('value=%d$N', refB.value);           // CHECK-NEXT: value=10
    refB.bar();                                 // CHECK-NEXT: bar(10)
    refB.value := 42;
    refB.bar();                                 // CHECK-NEXT: bar(42)
    printf('value=%d$N', refB.value);           // CHECK-NEXT: value=42

    // Through IC — property + baz
    refC := instC;
    printf('value=%d$N', refC.value);           // CHECK-NEXT: value=10
    refC.baz();                                 // CHECK-NEXT: baz(10)
    refC.value := 42;
    refC.baz();                                 // CHECK-NEXT: baz(42)
    printf('value=%d$N', refC.value);           // CHECK-NEXT: value=42

    // Through ID — property + bar + baz + qux
    refD := instD;
    printf('value=%d$N', refD.value);           // CHECK-NEXT: value=10
    refD.bar();                                 // CHECK-NEXT: bar(10)
    refD.baz();                                 // CHECK-NEXT: baz(10)
    refD.qux();                                 // CHECK-NEXT: qux(10)
    refD.value := 99;
    printf('value=%d$N', refD.value);           // CHECK-NEXT: value=99
    refD.bar();                                 // CHECK-NEXT: bar(99)
    refD.baz();                                 // CHECK-NEXT: baz(99)
    refD.qux();                                 // CHECK-NEXT: qux(99)
END_FUNCTION
