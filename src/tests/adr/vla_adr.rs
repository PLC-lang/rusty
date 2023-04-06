use crate::{
    lexer::IdProvider,
    test_utils::tests::{annotate_with_ids, codegen, index_with_ids},
};

/// TODO: explain VLAs in general, i.e. what they do and/or what they're used for
static SOURCE: &str = "
    FUNCTION foo : DINT
        VAR
            arr : ARRAY[*] OF DINT;
        END_VAR
    END_FUNCTION

    FUNCTION main : DINT
        VAR
            local : ARRAY[0..1] OF DINT;
        END_VAR

        foo(local);
    END_FUNCTION
";

/// TODO: Explain how VLAs are internally represented, i.e. expanded into fat-pointer, fields of fat-pointer
#[test]
fn ddeclare() {
    let id_provider = IdProvider::default();
    let (pr, mut index) = index_with_ids(SOURCE, id_provider.clone());
    // process pou and types
    annotate_with_ids(&pr, &mut index, id_provider);

    insta::assert_debug_snapshot!(index.find_effective_type_by_name("__foo_arr").unwrap(), 
    @r###"
        DataType {
            name: "__foo_arr",
            initial_value: None,
            information: Struct {
                name: "__foo_arr",
                members: [
                    VariableIndexEntry {
                        name: "struct_vla_dint_1",
                        qualified_name: "__foo_arr.struct_vla_dint_1",
                        initial_value: None,
                        variable_type: ByVal(
                            Input,
                        ),
                        is_constant: false,
                        data_type_name: "ptr_to___arr_vla_1_dint",
                        location_in_parent: 0,
                        linkage: Internal,
                        binding: None,
                        source_location: SymbolLocation {
                            line_number: 0,
                            source_range: SourceRange {
                                range: 0..0,
                            },
                        },
                        varargs: None,
                    },
                    VariableIndexEntry {
                        name: "dimensions",
                        qualified_name: "__foo_arr.dimensions",
                        initial_value: None,
                        variable_type: ByVal(
                            Input,
                        ),
                        is_constant: false,
                        data_type_name: "n_dims",
                        location_in_parent: 1,
                        linkage: Internal,
                        binding: None,
                        source_location: SymbolLocation {
                            line_number: 0,
                            source_range: SourceRange {
                                range: 0..0,
                            },
                        },
                        varargs: None,
                    },
                ],
                source: Internal(
                    VariableLengthArray {
                        inner_type_name: "DINT",
                        ndims: 1,
                    },
                ),
            },
            nature: Derived,
            location: SymbolLocation {
                line_number: 3,
                source_range: SourceRange {
                    range: 67..83,
                },
            },
        }
    "###);
}

/// TODO: Explain how VLAs are translated from internal representation into LLVM IR
#[test]
fn declare() {
    let src = "
        FUNCTION foo : DINT
            VAR
                arr : ARRAY[*] OF DINT;
            END_VAR
        END_FUNCTION
    ";

    insta::assert_snapshot!(codegen(src), @r###"
        ...
    "###);
}

/// TODO: Explain how local arrays are passed to POUs expecting VLAs, i.e. transformation from array -> vla
#[test]
fn pass() {}

/// TODO: explain read-access
#[test]
fn access_read() {}

/// TODO: explain write-access
#[test]
fn access_write() {}
