//! # Reference To Return Lowering
//!
//! This module handles the lowering of functions that return a "REFERENCE TO" to hide additional processing
//! that creates a temporary variable and assigns the result to that instead so that no values are assigned
//! directly from the stack.

#[cfg(test)]
mod tests {
    use insta::assert_snapshot;
    use plc::typesystem::VOID_TYPE;
    use plc_ast::{ast::VariableBlockType, ser::AstSerializer};
    use plc_driver::parse_and_annotate;
    use plc_source::SourceCode;

    #[test]
    fn reference_to_function_is_lowered_to_void_with_temporary_return() {
        let src: SourceCode = r#"
            FUNCTION referenceFunc : REFERENCE TO INT
                VAR_INPUT
                    in	: REFERENCE TO INT;
                END_VAR
                referenceFunc REF= in;
            END_FUNCTION

            FUNCTION main
                VAR
                    refVal : REFERENCE TO INT;
                    conVal : INT;
                END_VAR

                refVal := 11;
                refVal REF= referenceFunc(refVal);
                conVal := refVal;
            END_FUNCTION
            "#
        .into();

        /*
            Lowered code:
            ```ST
            FUNCTION referenceFunc
                VAR_IN_OUT
                    __referenceFunc_return_val : REFERENCE TO INT;
                END_VAR
                VAR_INPUT
                    in	: REFERENCE TO INT;
                END_VAR
                __referenceFunc_return_val REF= in;
            END_FUNCTION

            FUNCTION main
                VAR
                    refVal : REFERENCE TO INT;
                    conVal : INT;
                    __referenceFunc_return_val : REFERENCE TO INT;
                END_VAR

                refVal := 11;
                referenceFunc(__referenceFunc_return_val, refVal);
                refVal REF= __referenceFunc_return_val;
                conVal := refVal;
            END_FUNCTION
            ```
        */

        let (_, project) = parse_and_annotate("test", vec![src]).unwrap();
        let unit = &project.units[0].get_unit();
        let pous = &unit.pous;
        let implementations = &unit.implementations;

        // 1. Function "referenceFunc" should now have a `VOID` return type after lowering
        let ref_func_implementation =
            implementations.iter().find(|i| i.name == "referenceFunc").expect("referenceFunc implementation should exist");

        assert_eq!(ref_func_implementation.type_name, VOID_TYPE);

        // 2. Function "referenceFunc" should have a new `VAR_IN_OUT` variable: `__referenceFunc_return_val`
        let ref_func_pou =
            pous.iter().find(|i| i.name == "referenceFunc").expect("referenceFunc pou should exist");

        let ref_func_var_in_out_block = ref_func_pou
            .variable_blocks
            .iter()
            .find(|p| p.kind == VariableBlockType::InOut)
            .expect("referenceFunc inout block should exist");

        assert_snapshot!(AstSerializer::format_variable_block(ref_func_var_in_out_block), @"
        VAR_IN_OUT
            __referenceFunc_return_val : REFERENCE TO INT;
        END_VAR
        ");

        // 3. Function "referenceFunc" should now assign the return value to `__referenceFunc_return_val` as the final statement
        let ref_func_ret_statement = &ref_func_implementation.statements[ref_func_implementation.statements.len() - 1];
        assert_snapshot!(AstSerializer::format(ref_func_ret_statement), @"
        __referenceFunc_return_val REF= in;
        ");

        // 4. Function "main" should now have a new `VAR_TEMP` variable: `__referenceFunc_return_val`
        let main_pou =
            pous.iter().find(|i| i.name == "main").expect("main pou should exist");

        let main_temp_block = main_pou
            .variable_blocks
            .iter()
            .find(|p| p.kind == VariableBlockType::Temp)
            .expect("main temp block should exist");

        assert_snapshot!(AstSerializer::format_variable_block(main_temp_block), @"
        VAR_TEMP
            __referenceFunc_return_val : REFERENCE TO INT;
        END_VAR
        ");

        // 5. Function "main" should lower `refVal REF= referenceFunc(refVal);`
        // Into two statements:
        //   `referenceFunc(__referenceFunc_return_val, refVal);`
        //   `refVal REF= __referenceFunc_return_val;`
        let main_implementation =
            implementations.iter().find(|i| i.name == "main").expect("main implementation should exist");

        let ref_eq_call_func_statement = &main_implementation.statements[1];
        assert_snapshot!(AstSerializer::format(ref_eq_call_func_statement), @"
        referenceFunc(__referenceFunc_return_val, refVal);
        refVal REF= __referenceFunc_return_val;
        ");
    }
}
