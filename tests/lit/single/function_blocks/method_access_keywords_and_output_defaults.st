// RUN: %COMPILE %s && %RUN | %CHECK %s
FUNCTION_BLOCK fb
    VAR
        return_value : INT;
        return_value_2 : INT;
    END_VAR

    METHOD PUBLIC public_method
        VAR_INPUT
            input_var : INT;
            input_var_2 : INT;
        END_VAR
        VAR_OUTPUT
            output_var : INT;
            output_var_2 : INT;
            output_return_sum : INT;
        END_VAR
        output_return_sum := private_method(input_var, input_var_2, output_var, output_var_2);
        PRINTF('PUBLIC-METHOD: private_method.output_var: %d, output_var_2: %d, output_return_sum: %d$N', output_var, output_var_2, output_return_sum);
    END_METHOD
    METHOD PRIVATE private_method : INT
        VAR_INPUT
            input_var : INT;
            input_var_2 : INT;
        END_VAR
        VAR_OUTPUT
            output_var : INT;
            output_var_2 : INT;
        END_VAR
        output_var := input_var;
        output_var_2 := input_var_2;
        private_method := output_var + output_var_2;
    END_METHOD

    METHOD INTERNAL internal_method
        VAR_INPUT
            input_var : INT;
            input_var_2 : INT;
        END_VAR
        VAR_OUTPUT
            output_var : INT;
            output_var_2 : INT;
        END_VAR
        protected_method(input_var := input_var, input_var_2 := input_var_2, output_var => output_var, output_var_2 => output_var_2);
        PRINTF('INTERNAL-METHOD: protected_method.output_var: %d, output_var_2: %d$N', output_var, output_var_2);
    END_METHOD
    METHOD PROTECTED protected_method
        VAR_INPUT
            input_var : INT;
            input_var_2 : INT;
        END_VAR
        VAR_OUTPUT
            output_var : INT;
            output_var_2 : INT;
        END_VAR
        output_var := input_var;
        output_var_2 := input_var_2;
    END_METHOD

    METHOD no_keyword_method
        VAR_INPUT
            input_var : INT;
            input_var_2 : INT;
        END_VAR
        VAR_OUTPUT
            output_var : INT;
            output_var_2 : INT;
        END_VAR
        final_method(output_var_2 => output_var_2, input_var_2 := input_var_2);
        PRINTF('NO-KEYWORD-METHOD: final_method.output_var: %d, output_var_2: %d$N', output_var, output_var_2);
        output_var := input_var;
    END_METHOD
    METHOD FINAL final_method
        VAR_INPUT
            input_var : INT;
            input_var_2 : INT;
        END_VAR
        VAR_OUTPUT
            output_var : INT;
            output_var_2 : INT;
        END_VAR
        output_var := input_var;
        output_var_2 := input_var_2;
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION_BLOCK fb_extended EXTENDS fb
    VAR_OUTPUT
        output_var : INT;
        output_var_2 : INT;
    END_VAR
    METHOD extended_method : INT
        VAR_OUTPUT
            m_output_var : INT;
            m_output_var_2 : INT;
        END_VAR
        public_method(100, 101, m_output_var, m_output_var_2, extended_method);
        PRINTF('EXTENDED-METHOD: public_method.m_output_var: %d, m_output_var_2: %d, extended_method.return_value: %d$N', m_output_var, m_output_var_2, extended_method);
    END_METHOD

    internal_method(input_var := 102, input_var_2 := 103, output_var => output_var, output_var_2 => output_var_2);
    PRINTF('FB-EXTENDED: internal_method.output_var: %d, output_var_2: %d$N', output_var, output_var_2);
    no_keyword_method(input_var := 104, output_var => output_var);
    PRINTF('FB-EXTENDED: no_keyword_method.output_var: %d, output_var_2: %d$N', output_var, output_var_2);

    SUPER^.public_method(105, 106, output_var, output_var_2, return_value);
    PRINTF('FB-EXTENDED: SUPER^.public_method.output_var: %d, output_var_2: %d, return_value: %d$N', output_var, output_var_2, return_value);

    THIS^.public_method(200, 205, output_var, output_var_2, return_value);
    PRINTF('FB-EXTENDED: THIS^.public_method.output_var: %d, output_var_2: %d, return_value: %d$N', output_var, output_var_2, return_value);

    THIS^.internal_method(input_var_2 := 301, output_var_2 => output_var_2);
    PRINTF('FB-EXTENDED: THIS^.internal_method.output_var_2: %d$N', output_var_2);

    SUPER^.no_keyword_method(400, 450, output_var, output_var_2);
    PRINTF('FB-EXTENDED: SUPER^.no_keyword_method.output_var: %d, output_var_2: %d$N', output_var, output_var_2);
END_FUNCTION_BLOCK

FUNCTION main
    VAR
        fb_extended_instance : fb_extended;
        return_value : INT;
    END_VAR

    return_value := fb_extended_instance.extended_method();
    fb_extended_instance();

    //CHECK:PUBLIC-METHOD: private_method.output_var: 100, output_var_2: 101, output_return_sum: 201
    //CHECK-NEXT:EXTENDED-METHOD: public_method.m_output_var: 100, m_output_var_2: 101, extended_method.return_value: 201
    //CHECK-NEXT:INTERNAL-METHOD: protected_method.output_var: 102, output_var_2: 103
    //CHECK-NEXT:FB-EXTENDED: internal_method.output_var: 102, output_var_2: 103
    //CHECK-NEXT:NO-KEYWORD-METHOD: final_method.output_var: 0, output_var_2: 0
    //CHECK-NEXT:FB-EXTENDED: no_keyword_method.output_var: 104, output_var_2: 103
    //CHECK-NEXT:PUBLIC-METHOD: private_method.output_var: 105, output_var_2: 106, output_return_sum: 211
    //CHECK-NEXT:FB-EXTENDED: SUPER^.public_method.output_var: 105, output_var_2: 106, return_value: 211
    //CHECK-NEXT:PUBLIC-METHOD: private_method.output_var: 200, output_var_2: 205, output_return_sum: 405
    //CHECK-NEXT:FB-EXTENDED: THIS^.public_method.output_var: 200, output_var_2: 205, return_value: 405
    //CHECK-NEXT:INTERNAL-METHOD: protected_method.output_var: 0, output_var_2: 301
    //CHECK-NEXT:FB-EXTENDED: THIS^.internal_method.output_var_2: 301
    //CHECK-NEXT:NO-KEYWORD-METHOD: final_method.output_var: 0, output_var_2: 450
    //CHECK-NEXT:FB-EXTENDED: SUPER^.no_keyword_method.output_var: 400, output_var_2: 450
END_FUNCTION
