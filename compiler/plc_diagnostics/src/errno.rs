use std::fmt::Display;

#[allow(non_camel_case_types)]
#[derive(Default, PartialEq, Eq, Debug, Clone, Copy)]
pub enum ErrNo {
    #[default]
    undefined,

    //general
    general__err,
    general__io_err,
    general__param_err,
    duplicate_symbol,
    llvm_error,

    //syntax
    syntax__generic_error,
    syntax__missing_token,
    syntax__unexpected_token,
    syntax__invalid_range,
    syntax__mismatched_parans,
    syntax__time_missing_unit,
    syntax__invalid_time_literal,
    syntax__invalid_number,
    syntax__missing_case_contition,
    syntax_keywords_with_underscore,

    //semantic
    semantic__wrong_string_parans,
    semantic__pointer_not_standard,
    semnatic__return_default_value,
    semantic__class_no_implementation,
    semantic__duplicate_label,
    semantic__class_with_var_in_out,
    semnatic__class_no_return,
    semantic__pou_cannot_extend,
    semantic__actions_container_name,
    semantic__statement_has_no_effect,

    // pgramas
    pragma_invalid_location,

    // pou related
    pou__missing_return_type,
    pou__unexpected_return_type,
    pou__unsupported_return_type,
    pou__empty_variable_block,
    pou__missing_action_container,
    pou__recursive_data_structure,
    pou__missing_inout_parameter,

    // call
    call__invalid_parameter_type,
    call__invalid_parameter_count,

    //variable related
    var__unresolved_constant,
    var__invalid_constant_block,
    var__invalid_constant,
    var__cannot_assign_to_const,
    var__invalid_assignment,
    var__missing_type,
    var__assigning_to_var_input_ref,
    var__overflow,
    var__invalid_enum_variant,
    var__initializer,
    var__ref_assignment,

    //array related
    arr__invalid_array_assignment,

    // VLA related
    vla__invalid_container,
    vla__invalid_array_access,
    vla__dimension_idx_out_of_bounds,
    vla__always_byref,

    //reference related
    reference__unresolved,
    reference__illegal_access,
    reference__expected,

    //type related
    type__cast_error,
    type__unknown_type,
    type__invalid_type,
    type__literal_out_of_range,
    type__incompatible_literal_cast,
    type__incompatible_directaccess,
    type__incompatible_directaccess_variable,
    type__incompatible_directaccess_range,
    type__incompatible_arrayaccess_range,
    type__incompatible_arrayaccess_variable,
    type__incompatible_arrayaccess_type,
    type__directaccess_global_var,
    type__expected_literal,
    type__invalid_nature,
    type__unknown_nature,
    type__unresolved_generic,
    type__incompatible_size,
    type__invalid_operation,
    type__invalid_name,
    type__implicit_typecast,
    type__array_access_to_non_array_value,
    type__pointer_deref_required_pointer,
    type__address_of_required_value,

    //codegen related
    codegen__general,
    codegen__missing_function,
    codegen__missing_compare_function,
    codegen__string_literal,
    codegen__initial_values_not_generated,

    //Debug code
    debug_general,
    //linker
    linker__generic_error,

    //switch case
    case__duplicate_condition,
    case__case_condition_outside_case_statement,
    case__invalid_case_condition,

    // CFC related
    cfc__empty_control_statement,
    cfc__undefined_node,
    cfc__unexpected_node,
    cfc__unconnected_source,
    cfc__cyclic_connection,
    cfc__no_associated_connector,
    cfc__unnamed_control,

    // Project description file
    plc_json__invalid,
}

impl Display for ErrNo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let desc = match self {
            Self::general__err => "General Error".into(),
            Self::codegen__general => "Codegen Error".into(),
            Self::codegen__missing_compare_function => "Codegen Error: No compare function".into(),
            Self::codegen__missing_function => "Codegen Error: Missing Function".into(),
            _ => format!("{self:#?}"),
        };

        write!(f, "{desc}")
    }
}
