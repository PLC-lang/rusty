//! ST supports the concept of Variable Length Arrays (VLA), allowing users to pass array arguments of
//! different sizes to a function, function-block or method POU. These VLAs can be declared using an asterisk
//! instead of a range statement, such that `arr : ARRAY[*] OF DINT` is a valid declaration. Any POU with
//! either a INPUT, OUTPUT or INOUT variable block thereby accepts a 1D array of type DINT as an argument.

use crate::{
    lexer::IdProvider,
    resolver::AnnotationMap,
    test_utils::tests::{annotate_with_ids, codegen, index_with_ids},
    tests::adr::util_macros::{annotate, deconstruct_call_statement},
};

/// While declared as an array, VLAs are internally expanded into so called fat-pointer structs. These structs
/// carry two metadata fields, namely (1) a pointer to an array and (2) an array with information about the
/// pointed array's dimensions and sizes. These fields and how they're populated will be explained in the
/// [`pass`] test, here we just want to showcase how these VLAs are represented internally in the index.
#[test]
fn representation() {
    let (_, _, index) = annotate!(
        r"
        FUNCTION foo : DINT
            VAR_INPUT {ref}
                arr : ARRAY[*] OF DINT;
            END_VAR
        END_FUNCTION
        "
    );

    // The probably most interesting entry here is the `source` field, indicating that the given struct is a
    // VLA with one dimension of type DINT.
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
                    range: 79..95,
                },
            },
        }
    "###);

    // Pointer to `__arr_vla_1_dint`, which translates to...
    insta::assert_debug_snapshot!(index.find_effective_type_by_name("ptr_to___arr_vla_1_dint").unwrap(), 
    @r###"
        DataType {
            name: "ptr_to___arr_vla_1_dint",
            initial_value: None,
            information: Pointer {
                name: "ptr_to___arr_vla_1_dint",
                inner_type_name: "__arr_vla_1_dint",
                auto_deref: false,
            },
            nature: Any,
            location: SymbolLocation {
                line_number: 0,
                source_range: SourceRange {
                    range: 0..0,
                },
            },
        }
    "###);

    // ...an array of type DINT with its dimensions unknown at compile time
    insta::assert_debug_snapshot!(index.find_effective_type_by_name("__arr_vla_1_dint").unwrap(),
    @r###"
    DataType {
        name: "__arr_vla_1_dint",
        initial_value: None,
        information: Array {
            name: "__arr_vla_1_dint",
            inner_type_name: "DINT",
            dimensions: [
                Dimension {
                    start_offset: Undetermined,
                    end_offset: Undetermined,
                },
            ],
        },
        nature: Any,
        location: SymbolLocation {
            line_number: 4294967295,
            source_range: SourceRange {
                range: 0..0,
            },
        },
    }
    "###);

    // Finally the dimensions array, which is being populated at runtime; see [`pass`]
    insta::assert_debug_snapshot!(index.find_effective_type_by_name("n_dims").unwrap(), 
    @r###"
        DataType {
            name: "n_dims",
            initial_value: None,
            information: Array {
                name: "n_dims",
                inner_type_name: "DINT",
                dimensions: [
                    Dimension {
                        start_offset: ConstExpression(
                            Index {
                                index: 0,
                                generation: 0,
                            },
                        ),
                        end_offset: ConstExpression(
                            Index {
                                index: 1,
                                generation: 0,
                            },
                        ),
                    },
                ],
            },
            nature: Any,
            location: SymbolLocation {
                line_number: 0,
                source_range: SourceRange {
                    range: 0..0,
                },
            },
        }
    "###);
}

/// Because VLAs are internally handled as structs, they'll naturally also translate into LLVM structs.
#[test]
fn declare() {
    let src = "
        FUNCTION foo : DINT
            VAR_INPUT {ref}
                arr : ARRAY[*] OF DINT;
            END_VAR
        END_FUNCTION
    ";

    assert!(codegen(src).contains("%__foo_arr = type { i32*, [2 x i32] }"));
}

/// VLAs (in RuSTy) are defined to be always by-ref, meaning POUs accepting VLAs expect a pointer to a struct.
/// To satisfy that constraint, any array passed to such a POU needs to be first wrapped inside a struct
/// with their fields populated. Therefore, whenever we encounter such situations we first stack-allocate
/// a struct then pass (1) the array's address and (2) an array consisting of the dimensions'
/// {start,end}-offset.
/// XXX: It would be ideal to only stack allocate once instead of allocating with every POU call
#[test]
fn pass() {
    let src = "
        FUNCTION main : DINT
            VAR
                local : ARRAY[0..5] OF DINT;
            END_VAR

            foo(local);
        END_FUNCTION

        FUNCTION foo : DINT
            VAR_INPUT {ref}
                arr : ARRAY[*] OF DINT;
            END_VAR
        END_FUNCTION
    ";

    let (statements, annotations, index) = annotate!(src);
    let (_, local) = deconstruct_call_statement!(&statements[0]);

    // `local` is defined as an array of type DINT...
    insta::assert_debug_snapshot!(annotations.get_type(&local[0], &index).unwrap(), 
    @r###"
    DataType {
        name: "__main_local",
        initial_value: None,
        information: Array {
            name: "__main_local",
            inner_type_name: "DINT",
            dimensions: [
                Dimension {
                    start_offset: ConstExpression(
                        Index {
                            index: 0,
                            generation: 0,
                        },
                    ),
                    end_offset: ConstExpression(
                        Index {
                            index: 1,
                            generation: 0,
                        },
                    ),
                },
            ],
        },
        nature: Any,
        location: SymbolLocation {
            line_number: 3,
            source_range: SourceRange {
                range: 70..89,
            },
        },
    }
    "###);

    // ...but their type-hint indicates it should be VLA / fat-pointer struct. Such type-mismatches (for VLAs)
    // result in wrapping arrays into structs.
    let hint = annotations.get_type_hint(&local[0], &index).unwrap();
    insta::assert_debug_snapshot!(index.find_elementary_pointer_type(&hint.information), 
    @r###"
    Struct {
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
    }
    "###);

    // Finally here's the codegen for populating the struct, where we
    // 1. Stack-allocate a struct
    // 2. GEP the structs array and dimension field
    // 3. Populate them based on the information we have on `local`, i.e. 1D and (start, end)-offset = (0, 5)
    insta::assert_snapshot!(codegen(src), 
    @r###"
    ; ModuleID = 'main'
    source_filename = "main"
    
    %__foo_arr = type { i32*, [2 x i32] }
    
    @____foo_arr__init = unnamed_addr constant %__foo_arr zeroinitializer
    
    define i32 @main() {
    entry:
      %main = alloca i32, align 4
      %local = alloca [6 x i32], align 4
      %0 = bitcast [6 x i32]* %local to i8*
      call void @llvm.memset.p0i8.i64(i8* align 1 %0, i8 0, i64 ptrtoint ([6 x i32]* getelementptr ([6 x i32], [6 x i32]* null, i32 1) to i64), i1 false)
      store i32 0, i32* %main, align 4
      %auto_deref = load [6 x i32], [6 x i32]* %local, align 4
      %1 = bitcast [6 x i32]* %local to i32*
      %vla_struct = alloca %__foo_arr, align 8
      %vla_array_gep = getelementptr inbounds %__foo_arr, %__foo_arr* %vla_struct, i32 0, i32 0
      %vla_dimensions_gep = getelementptr inbounds %__foo_arr, %__foo_arr* %vla_struct, i32 0, i32 1
      %2 = getelementptr inbounds [2 x i32], [2 x i32]* %vla_dimensions_gep, i32 0, i32 0
      store i32 0, i32* %2, align 4
      %3 = getelementptr inbounds [2 x i32], [2 x i32]* %vla_dimensions_gep, i32 0, i32 1
      store i32 5, i32* %3, align 4
      store i32* %1, i32** %vla_array_gep, align 8
      %4 = load %__foo_arr, %__foo_arr* %vla_struct, align 8
      %vla_struct_ptr = alloca %__foo_arr, align 8
      store %__foo_arr %4, %__foo_arr* %vla_struct_ptr, align 8
      %call = call i32 @foo(%__foo_arr* %vla_struct_ptr)
      %main_ret = load i32, i32* %main, align 4
      ret i32 %main_ret
    }
    
    define i32 @foo(%__foo_arr* %0) {
    entry:
      %foo = alloca i32, align 4
      %arr = alloca %__foo_arr*, align 8
      store %__foo_arr* %0, %__foo_arr** %arr, align 8
      store i32 0, i32* %foo, align 4
      %foo_ret = load i32, i32* %foo, align 4
      ret i32 %foo_ret
    }
    
    ; Function Attrs: argmemonly nofree nounwind willreturn writeonly
    declare void @llvm.memset.p0i8.i64(i8* nocapture writeonly, i8, i64, i1 immarg) #0

    attributes #0 = { argmemonly nofree nounwind willreturn writeonly }
    "###);
}

/// Accessing arrays for read- / write-operations works by gepping the structs array and dimension fields,
/// similar to how the codegen looks like in [`pass`]. However, because arrays in ST are not always zero-indexed
/// but rather their offsets can be any number such as `foo : ARRAY[90..100] OF DINT;`, which we have to do
/// adjust for LLVM. For example accessing the 6th element in ST would work with `foo[95]`, but
/// for LLVM to not segfault, we have to start at zero i.e. `foo[95 - 90]`. This is done at run-time, as can
/// be seen in the codegen.
#[test]
fn access() {
    let src = "
        FUNCTION foo : DINT
            VAR_INPUT {ref}
                arr : ARRAY[*] OF DINT;
            END_VAR

            arr[0] := 12345;
        END_FUNCTION
    ";

    insta::assert_snapshot!(codegen(src), 
    @r###"
    ; ModuleID = 'main'
    source_filename = "main"

    %__foo_arr = type { i32*, [2 x i32] }

    @____foo_arr__init = unnamed_addr constant %__foo_arr zeroinitializer

    define i32 @foo(%__foo_arr* %0) {
    entry:
      %foo = alloca i32, align 4
      %arr = alloca %__foo_arr*, align 8
      store %__foo_arr* %0, %__foo_arr** %arr, align 8
      store i32 0, i32* %foo, align 4
      %auto_deref = load %__foo_arr*, %__foo_arr** %arr, align 8
      %vla_arr_gep = getelementptr inbounds %__foo_arr, %__foo_arr* %auto_deref, i32 0, i32 0
      %vla_arr_ptr = load i32*, i32** %vla_arr_gep, align 8
      %dim_arr = getelementptr inbounds %__foo_arr, %__foo_arr* %auto_deref, i32 0, i32 1
      %start_idx_ptr = getelementptr inbounds [2 x i32], [2 x i32]* %dim_arr, i32 0, i32 0
      %end_idx_ptr = getelementptr inbounds [2 x i32], [2 x i32]* %dim_arr, i32 0, i32 1
      %start_offset = load i32, i32* %start_idx_ptr, align 4
      %tmpVar = sub i32 0, %start_offset
      %arr_val = getelementptr inbounds i32, i32* %vla_arr_ptr, i32 %tmpVar
      store i32 12345, i32* %arr_val, align 4
      %foo_ret = load i32, i32* %foo, align 4
      ret i32 %foo_ret
    }
    "###);
}
