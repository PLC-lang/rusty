use insta::assert_snapshot;

mod expression_debugging;

use crate::test_utils::tests::codegen_with_debug as codegen;
#[test]
fn test_global_var_int_added_to_debug_info() {
    let codegen = codegen(
        r#"
    VAR_GLOBAL
        a : SINT; //8bit
        b : USINT; //8bit
        c : INT; //16bit
        d : UINT; //16bit
        e : DINT; //32bit
        f : UDINT; //32bit
        g : LINT; //64bit
        h : ULINT; //64bit
    END_VAR
    "#,
    );

    assert_snapshot!(codegen)
}

#[test]
fn test_global_var_byteseq_added_to_debug_info() {
    let codegen = codegen(
        r#"
    VAR_GLOBAL
        a : BOOL; //8bit DW_ATE_boolean
        b : BYTE; //8bit
        c : WORD; //16bit
        d : DWORD; //32bit
        e : LWORD; //64bit
    END_VAR
    "#,
    );

    assert_snapshot!(codegen)
}

#[test]
fn test_global_var_enum_added_to_debug_info() {
    //Multiple types
    let codegen = codegen(
        r#"
    TYPE en1 : (a,b,c); END_TYPE
    TYPE en2 : BYTE (d,e,f); END_TYPE
    VAR_GLOBAL
        en3 : LINT (a,b,c);
    END_VAR
    "#,
    );

    assert_snapshot!(codegen)
}

#[test]
fn test_global_var_float_added_to_debug_info() {
    let codegen = codegen(
        r#"
    VAR_GLOBAL
        a : REAL; 
        b : LREAL; 
    END_VAR
    "#,
    );

    assert_snapshot!(codegen)
}

#[test]
fn test_global_var_array_added_to_debug_info() {
    let codegen = codegen(
        r#"
    VAR_GLOBAL
        a : ARRAY[0..10] OF DINT; 
        b : ARRAY[0..10, 11..20] OF DINT; 
        c : ARRAY[0..10] OF ARRAY[11..20] OF DINT; 
    END_VAR
    "#,
    );
    assert_snapshot!(codegen)
}

#[test]
fn test_global_var_pointer_added_to_debug_info() {
    let codegen = codegen(
        r#"
    VAR_GLOBAL
        a : REF_TO DINT; 
        b : REF_TO ARRAY[0..10] DINT; 
    END_VAR
    "#,
    );
    assert_snapshot!(codegen)
}

#[test]
fn test_global_var_string_added_to_debug_info() {
    let codegen = codegen(
        r#"
    VAR_GLOBAL
        a : STRING;
        b : WSTRING;
    END_VAR
    "#,
    );
    assert_snapshot!(codegen)
}

#[test]
fn test_global_var_struct_added_to_debug_info() {
    let codegen = codegen(
        r#"
    TYPE myStruct : STRUCT
        a : DINT;
        b : LREAL;
        c : ARRAY[0..10] OF DINT;
        // d : STRING;
    END_STRUCT
    END_TYPE

    VAR_GLOBAL
        gStruct : myStruct;
        b : ARRAY[0..10] OF myStruct;
    END_VAR
    "#,
    );
    assert_snapshot!(codegen)
}

#[test]
fn test_global_var_nested_struct_added_to_debug_info() {
    let codegen = codegen(
        r#"
    TYPE myStruct : STRUCT
        a : DINT;
        b : myStruct2;
    END_STRUCT
    END_TYPE

    TYPE myStruct2 : STRUCT
        a : DINT;
        b : LREAL;
    END_STRUCT
    END_TYPE

    VAR_GLOBAL
        gStruct : myStruct;
    END_VAR
    "#,
    );
    assert_snapshot!(codegen)
}

#[test]
fn test_global_alias_type() {
    let codegen = codegen(
        r#"
    TYPE myInt : DINT; END_TYPE

    VAR_GLOBAL
        gInt : myInt;
    END_VAR
    "#,
    );

    assert_snapshot!(codegen)
}
