use insta::assert_snapshot;

use crate::test_utils::tests::codegen_with_debug as codegen;
#[test]
fn test_global_var_int_added_to_debug_info() {
    let codegen = codegen(r#"
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
    "#);

    assert_snapshot!(codegen)
}

#[test]
fn test_global_var_byteseq_added_to_debug_info() {
    let codegen = codegen(r#"
    VAR_GLOBAL
        a : BYTE; //8bit
        b : WORD; //16bit
        c : DWORD; //32bit
        d : LWORD; //64bit
    END_VAR
    "#);

    assert_snapshot!(codegen)
}

#[test]
fn test_global_var_enum_added_to_debug_info() {
    //Multiple types
    let codegen = codegen(r#"
    TYPE en1 : (a,b,c); END_TYPE
    TYPE en2 : BYTE (d,e,f); END_TYPE
    VAR_GLOBAL
        en3 : LINT (a,b,c);
    END_VAR
    "#);

    assert_snapshot!(codegen)
}


#[test]
fn test_global_var_bool_added_to_debug_info() {

}


#[test]
fn test_global_var_float_added_to_debug_info() {

}