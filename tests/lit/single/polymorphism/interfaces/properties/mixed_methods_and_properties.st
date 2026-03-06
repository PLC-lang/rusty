// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Scenario: An interface declares both a regular method and a property. Two function blocks
// implement the interface with different behaviors. Both the method and property are dispatched
// through the same interface variable, verifying they coexist cleanly in the itable.

INTERFACE IWidget
    METHOD describe END_METHOD

    PROPERTY label : DINT
        GET END_GET
        SET END_SET
    END_PROPERTY
END_INTERFACE

FUNCTION_BLOCK FbButton IMPLEMENTS IWidget
    VAR
        _label : DINT := 1;
    END_VAR

    METHOD describe
        printf('Button(%d)$N', _label);
    END_METHOD

    PROPERTY label : DINT
        GET
            label := _label;
        END_GET

        SET
            _label := label;
        END_SET
    END_PROPERTY
END_FUNCTION_BLOCK

FUNCTION_BLOCK FbSlider IMPLEMENTS IWidget
    VAR
        _label : DINT := 2;
    END_VAR

    METHOD describe
        printf('Slider(%d)$N', _label);
    END_METHOD

    PROPERTY label : DINT
        GET
            label := _label * 10;
        END_GET

        SET
            _label := label + 100;
        END_SET
    END_PROPERTY
END_FUNCTION_BLOCK

FUNCTION main
    VAR
        button : FbButton;
        slider : FbSlider;
        iface : IWidget;
    END_VAR

    // FbButton
    iface := button;
    iface.describe();                           // CHECK: Button(1)
    printf('label=%d$N', iface.label);          // CHECK-NEXT: label=1
    iface.label := 42;
    iface.describe();                           // CHECK-NEXT: Button(42)
    printf('label=%d$N', iface.label);          // CHECK-NEXT: label=42

    // FbSlider — getter returns _label * 10, setter stores label + 100
    iface := slider;
    iface.describe();                           // CHECK-NEXT: Slider(2)
    printf('label=%d$N', iface.label);          // CHECK-NEXT: label=20
    iface.label := 99;
    iface.describe();                           // CHECK-NEXT: Slider(199)
    printf('label=%d$N', iface.label);          // CHECK-NEXT: label=1990
END_FUNCTION
