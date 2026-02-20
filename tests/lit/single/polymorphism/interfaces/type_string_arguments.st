// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Scenario: Ensure STRING arguments are passed correctly through interface dispatch for
// VAR_INPUT, VAR_OUTPUT, VAR_IN_OUT and mixed signatures.

INTERFACE A
    METHOD input
        VAR_INPUT
            in_literal: STRING;
            in_variable: STRING;
        END_VAR
    END_METHOD

    METHOD output
        VAR_OUTPUT
            out: STRING;
        END_VAR
    END_METHOD

    METHOD inout
        VAR_IN_OUT
            inout: STRING;
        END_VAR
    END_METHOD

    METHOD in_out_and_inout
        VAR_INPUT
            in_literal: STRING;
            in_variable: STRING;
        END_VAR

        VAR_OUTPUT
            out: STRING;
        END_VAR

        VAR_IN_OUT
            inout: STRING;
        END_VAR
    END_METHOD
END_INTERFACE

FUNCTION_BLOCK FbA IMPLEMENTS A
    METHOD input
        VAR_INPUT
            in_literal: STRING;
            in_variable: STRING;
        END_VAR

        printf('FbA::input, in_literal = %s$N', in_literal);
        printf('FbA::input, in_variable = %s$N', in_variable);
    END_METHOD

    METHOD output
        VAR_OUTPUT
            out: STRING;
        END_VAR

        printf('FbA::output, out = %s$N', out);
        out := 'FbA::output, dirty';
    END_METHOD

    METHOD inout
        VAR_IN_OUT
            inout: STRING;
        END_VAR

        printf('FbA::inout, inout = %s$N', inout);
        inout := 'FbA::inout, dirty';
    END_METHOD

    METHOD in_out_and_inout
        VAR_INPUT
            in_literal: STRING;
            in_variable: STRING;
        END_VAR

        VAR_OUTPUT
            out: STRING;
        END_VAR

        VAR_IN_OUT
            inout: STRING;
        END_VAR

        printf('FbA::in_out_and_inout, in_literal = %s$N', in_literal);
        printf('FbA::in_out_and_inout, in_variable = %s$N', in_variable);
        printf('FbA::in_out_and_inout, out = %s$N', out);
        printf('FbA::in_out_and_inout, inout = %s$N', inout);

        out := 'FbA::in_out_and_inout, out dirty';
        inout := 'FbA::in_out_and_inout, inout dirty';
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION main
    VAR
        instance: FbA;
        reference: A;

        localInputVariable: STRING := 'main, clean';
        localOutputVariable: STRING := 'main, clean';
        localInOutVariable: STRING := 'main, clean';
    END_VAR

    reference := instance;

    // CHECK: FbA::input, in_literal = input, literal
    // CHECK-NEXT: FbA::input, in_variable = main, clean
    reference.input('input, literal', localInputVariable);

    // CHECK-NEXT: FbA::output, out = main, clean
    // CHECK-NEXT: FbA::output, dirty
    // CHECK-NEXT: main, clean
    reference.output(localOutputVariable);
    printf('%s$N', localOutputVariable);
    localOutputVariable := 'main, clean';
    printf('%s$N', localOutputVariable);

    // CHECK-NEXT: FbA::inout, inout = main, clean
    // CHECK-NEXT: FbA::inout, dirty
    // CHECK-NEXT: main, clean
    reference.inout(localInOutVariable);
    printf('%s$N', localInOutVariable);
    localInOutVariable := 'main, clean';
    printf('%s$N', localInOutVariable);

    // CHECK-NEXT: FbA::in_out_and_inout, in_literal = in_out_and_inout, literal
    // CHECK-NEXT: FbA::in_out_and_inout, in_variable = main, clean
    // CHECK-NEXT: FbA::in_out_and_inout, out = main, clean
    // CHECK-NEXT: FbA::in_out_and_inout, inout = main, clean
    // CHECK-NEXT: out = FbA::in_out_and_inout, out dirty
    // CHECK-NEXT: inout = FbA::in_out_and_inout, inout dirty
    reference.in_out_and_inout('in_out_and_inout, literal', localInputVariable, localOutputVariable, localInOutVariable);
    printf('out = %s$N', localOutputVariable);
    printf('inout = %s$N', localInOutVariable);
END_FUNCTION
