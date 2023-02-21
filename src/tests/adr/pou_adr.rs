use crate::{
    lexer::IdProvider,
    test_utils::tests::{annotate_with_ids, codegen, index_with_ids},
};

/// # Architecture Design Record: POUs
///
/// POU = Program Organisation Unit
///
/// A Program Organisation Unit is a container for executable code. It is either a PROGRAM,
/// a FUNCTION_BLOCK, a FUNCTION or a CLASS.
/// On the top level we distingish two different types of POUs. There are stateful, and
/// stateless POUs. Stateful POUs are POUs that have a memory across different calls to it. This means
/// that (some of) the variables of the POU keep their value's across different calls. We say that
/// there are `instances` of a POU. Stateless POU start with a fresh set of variables on every call.
///
/// Statefull (instantiatable) POUs are:
/// - Programs
/// - Functionblocks
/// - Classes
///
/// Stateless POUs are:
/// - Functions
///
/// # PROGRAMS
///
/// Programs are POUs with exactly one (static) instance. Programs have a persistent state for VAR, VAR_INPUT,
/// VAR_OUTPUT and VAR_IN_OUT variables. The instance is statically available and behaves as if there is a callable
/// global variable withe the program's name. When calling a program all parameters, except IN_OUT parameters, are optional.

const DEFAULT_PRG: &str = r#"
    PROGRAM main_prg 
        VAR_INPUT   i   : INT   END_VAR
        VAR_IN_OUT  io  : INT   END_VAR
        VAR_OUTPUT  o   : INT;  END_VAR
        VAR         v   : INT;  END_VAR
        VAR_TEMP    vt  : INT;  END_VAR
    END_PROGRAM
"#;

/// The state (the memory, the stateful variables) are intnerally saved in a Struct-Type generated for this program.
/// Programs register a PouType in the index using the POU's name. The PouType offers information about the
/// POU like the name of the struct-type carrying the program's state, all members, vararg and generic
/// information etc.

#[test]
fn programs_state_is_stored_in_a_struct() {
    // parse and index
    let id_provider = IdProvider::default();
    let (pr, mut index) = index_with_ids(DEFAULT_PRG, id_provider.clone());
    // process pou and types
    annotate_with_ids(&pr, &mut index, id_provider);

    let pou_struct = index.find_pou("main_prg").and_then(|pou| pou.find_instance_struct_type(&index));

    insta::assert_debug_snapshot!(pou_struct, @r###"
    Some(
        DataType {
            name: "main_prg",
            initial_value: None,
            information: Struct {
                name: "main_prg",
                members: [
                    VariableIndexEntry {
                        name: "i",
                        qualified_name: "main_prg.i",
                        initial_value: None,
                        variable_type: ByVal(
                            Input,
                        ),
                        is_constant: false,
                        data_type_name: "INT",
                        location_in_parent: 0,
                        linkage: Internal,
                        binding: None,
                        source_location: SymbolLocation {
                            line_number: 2,
                            source_range: SourceRange {
                                range: 43..44,
                            },
                        },
                        varargs: None,
                    },
                    VariableIndexEntry {
                        name: "io",
                        qualified_name: "main_prg.io",
                        initial_value: None,
                        variable_type: ByRef(
                            InOut,
                        ),
                        is_constant: false,
                        data_type_name: "__auto_pointer_to_INT",
                        location_in_parent: 1,
                        linkage: Internal,
                        binding: None,
                        source_location: SymbolLocation {
                            line_number: 3,
                            source_range: SourceRange {
                                range: 83..85,
                            },
                        },
                        varargs: None,
                    },
                    VariableIndexEntry {
                        name: "o",
                        qualified_name: "main_prg.o",
                        initial_value: None,
                        variable_type: ByVal(
                            Output,
                        ),
                        is_constant: false,
                        data_type_name: "INT",
                        location_in_parent: 2,
                        linkage: Internal,
                        binding: None,
                        source_location: SymbolLocation {
                            line_number: 4,
                            source_range: SourceRange {
                                range: 123..124,
                            },
                        },
                        varargs: None,
                    },
                    VariableIndexEntry {
                        name: "v",
                        qualified_name: "main_prg.v",
                        initial_value: None,
                        variable_type: ByVal(
                            Local,
                        ),
                        is_constant: false,
                        data_type_name: "INT",
                        location_in_parent: 3,
                        linkage: Internal,
                        binding: None,
                        source_location: SymbolLocation {
                            line_number: 5,
                            source_range: SourceRange {
                                range: 163..164,
                            },
                        },
                        varargs: None,
                    },
                    VariableIndexEntry {
                        name: "vt",
                        qualified_name: "main_prg.vt",
                        initial_value: None,
                        variable_type: ByVal(
                            Temp,
                        ),
                        is_constant: false,
                        data_type_name: "INT",
                        location_in_parent: 4,
                        linkage: Internal,
                        binding: None,
                        source_location: SymbolLocation {
                            line_number: 6,
                            source_range: SourceRange {
                                range: 203..205,
                            },
                        },
                        varargs: None,
                    },
                ],
                source: Pou(
                    Program,
                ),
            },
            nature: Any,
            location: SymbolLocation {
                line_number: 1,
                source_range: SourceRange {
                    range: 13..21,
                },
            },
        },
    )
    "###);
}

/// Code-Generating a program does ...
///  ... generate a struct-type with all persistent fields (`main_prg_interface`)
///  ... generate a global intance variable (`main_prg_instance`) of that type
///  ... generate the body of the program (`@main_prg`). All state variables are
///      auto-loaded into local variables. VAR_TEMPs (`vt`) get allocated and initialized on the stack.

#[test]
fn codegen_of_a_program_pou() {
    insta::assert_snapshot!(codegen(DEFAULT_PRG),@r###"
    ; ModuleID = 'main'
    source_filename = "main"

    %main_prg = type { i16, i16*, i16, i16 }

    @main_prg_instance = global %main_prg zeroinitializer

    define void @main_prg(%main_prg* %0) {
    entry:
      %i = getelementptr inbounds %main_prg, %main_prg* %0, i32 0, i32 0
      %io = getelementptr inbounds %main_prg, %main_prg* %0, i32 0, i32 1
      %o = getelementptr inbounds %main_prg, %main_prg* %0, i32 0, i32 2
      %v = getelementptr inbounds %main_prg, %main_prg* %0, i32 0, i32 3
      %vt = alloca i16, align 2
      store i16 0, i16* %vt, align 2
      ret void
    }
    "###);
}

/// Calling a program works like this:
///  ... the parameters are assigned to the global instance variable (`main_prg_instance`)
///  ... parameters that are not defined keep their last assigned value
///  ... the program's method (body) is called, the global instance is passed as a parameter (`void @main_prg`)
///  ... note that VAR_OUT & VAR_IN_OUT parameters are stored as pointers (`i16*`)
#[test]
fn calling_a_program() {
    let calling_prg = format!(
        r#"
        FUNCTION foo : INT
            VAR x, y : INT; END_VAR
            main_prg(i := 1, io := y, o => x); 
        END_FUNCTION

        {DEFAULT_PRG}
    "#
    );
    insta::assert_snapshot!(codegen(calling_prg.as_str()), @r###"
    ; ModuleID = 'main'
    source_filename = "main"

    %main_prg = type { i16, i16*, i16, i16 }

    @main_prg_instance = global %main_prg zeroinitializer

    define i16 @foo() {
    entry:
      %foo = alloca i16, align 2
      %x = alloca i16, align 2
      %y = alloca i16, align 2
      store i16 0, i16* %x, align 2
      store i16 0, i16* %y, align 2
      store i16 0, i16* %foo, align 2
      store i16 1, i16* getelementptr inbounds (%main_prg, %main_prg* @main_prg_instance, i32 0, i32 0), align 2
      store i16* %y, i16** getelementptr inbounds (%main_prg, %main_prg* @main_prg_instance, i32 0, i32 1), align 8
      call void @main_prg(%main_prg* @main_prg_instance)
      %0 = load i16, i16* getelementptr inbounds (%main_prg, %main_prg* @main_prg_instance, i32 0, i32 2), align 2
      store i16 %0, i16* %x, align 2
      %foo_ret = load i16, i16* %foo, align 2
      ret i16 %foo_ret
    }

    define void @main_prg(%main_prg* %0) {
    entry:
      %i = getelementptr inbounds %main_prg, %main_prg* %0, i32 0, i32 0
      %io = getelementptr inbounds %main_prg, %main_prg* %0, i32 0, i32 1
      %o = getelementptr inbounds %main_prg, %main_prg* %0, i32 0, i32 2
      %v = getelementptr inbounds %main_prg, %main_prg* %0, i32 0, i32 3
      %vt = alloca i16, align 2
      store i16 0, i16* %vt, align 2
      ret void
    }
    "###);
}

/// # FUNCTION BLOCK
///
/// FunctionBlock are instantiable Blocks that behave very similar to programs. The biggest difference is
/// that you can have mulitple instances of a FunctionBlock. Therefore a FunctionBlocks instance variable is not
/// auto-generated as a global variable as it would be for a program. FB-instances can be declared as part of your
/// code. A FunctionBlock acts automatically as a DataType.

const DEFAULT_FB: &str = r#"
    FUNCTION_BLOCK main_fb
        VAR_INPUT   i    : INT := 6 END_VAR
        VAR_IN_OUT  io  : INT       END_VAR
        VAR_OUTPUT  o   : INT;      END_VAR
        VAR         v   : INT := 1; END_VAR
        VAR_TEMP    vt  : INT := 2; END_VAR
    END_FUNCTION_BLOCK
"#;

/// Code-Generating a function_block does ...
///  ... generate a struct-type with all persistent fields (`main_fb_interface`)
///  ... generate the body of the function_block (`void @main_fb`). All state variables are auto-loaded into
///         local variables. VAR_TEMPs (`vt`) get allocated and initialized on the stack.
///  ... a global variable with the FB's default values (`main_fb__init`), used to initialize function block instances.
///  ... note that no global instance is created (as you would get for PROGRAMs)

#[test]
fn function_blocks_get_a_method_with_a_self_parameter() {
    insta::assert_snapshot!(codegen(DEFAULT_FB), @r###"
    ; ModuleID = 'main'
    source_filename = "main"

    %main_fb = type { i16, i16*, i16, i16 }

    @__main_fb__init = unnamed_addr constant %main_fb { i16 6, i16* null, i16 0, i16 1 }

    define void @main_fb(%main_fb* %0) {
    entry:
      %i = getelementptr inbounds %main_fb, %main_fb* %0, i32 0, i32 0
      %io = getelementptr inbounds %main_fb, %main_fb* %0, i32 0, i32 1
      %o = getelementptr inbounds %main_fb, %main_fb* %0, i32 0, i32 2
      %v = getelementptr inbounds %main_fb, %main_fb* %0, i32 0, i32 3
      %vt = alloca i16, align 2
      store i16 2, i16* %vt, align 2
      ret void
    }
    "###);
}

/// Calling a function block works like this:
///  ... the parameters are assigned to the called fb instance variable (`%fb`)
///  ... the function_block's method is called, the called instance is passed as a parameter (`void @main_fb`)
///  ... note that VAR_OUT & VAR_IN_OUT parameters are stored as pointers (`i16*`)
#[test]
fn calling_a_function_block() {
    let calling_prg = format!(
        r#"
        PROGRAM foo
            VAR x, y    : INT;      END_VAR
            VAR fb      : main_fb;  END_VAR
            
            fb(i := 1, io := y, o => x); 
        END_PROGRAM

        {DEFAULT_FB}
    "#
    );
    insta::assert_snapshot!(codegen(calling_prg.as_str()), @r###"
    ; ModuleID = 'main'
    source_filename = "main"

    %foo = type { i16, i16, %main_fb }
    %main_fb = type { i16, i16*, i16, i16 }

    @foo_instance = global %foo { i16 0, i16 0, %main_fb { i16 6, i16* null, i16 0, i16 1 } }
    @__main_fb__init = unnamed_addr constant %main_fb { i16 6, i16* null, i16 0, i16 1 }

    define void @foo(%foo* %0) {
    entry:
      %x = getelementptr inbounds %foo, %foo* %0, i32 0, i32 0
      %y = getelementptr inbounds %foo, %foo* %0, i32 0, i32 1
      %fb = getelementptr inbounds %foo, %foo* %0, i32 0, i32 2
      %1 = getelementptr inbounds %main_fb, %main_fb* %fb, i32 0, i32 0
      store i16 1, i16* %1, align 2
      %2 = getelementptr inbounds %main_fb, %main_fb* %fb, i32 0, i32 1
      store i16* %y, i16** %2, align 8
      call void @main_fb(%main_fb* %fb)
      %3 = getelementptr inbounds %main_fb, %main_fb* %fb, i32 0, i32 2
      %4 = load i16, i16* %3, align 2
      store i16 %4, i16* %x, align 2
      ret void
    }

    define void @main_fb(%main_fb* %0) {
    entry:
      %i = getelementptr inbounds %main_fb, %main_fb* %0, i32 0, i32 0
      %io = getelementptr inbounds %main_fb, %main_fb* %0, i32 0, i32 1
      %o = getelementptr inbounds %main_fb, %main_fb* %0, i32 0, i32 2
      %v = getelementptr inbounds %main_fb, %main_fb* %0, i32 0, i32 3
      %vt = alloca i16, align 2
      store i16 2, i16* %vt, align 2
      ret void
    }
    "###);
}

/// # FUNCTION
///
/// Functions are stateless methods. They dont have an instance-struct or instance variables. Functions
/// take all their parameters passed one by one to the function.

const DEFAULT_FUNC: &str = r#"
    FUNCTION main_fun : DINT
        VAR_INPUT   i   : INT; END_VAR
        VAR_IN_OUT  io  : SINT;      END_VAR
        VAR_OUTPUT  o   : LINT;      END_VAR
        VAR         v   : INT := 1; END_VAR
        VAR_TEMP    vt  : INT := 2; END_VAR
    END_FUNCTION
"#;

/// Code-Generating a function does ...
///  ... generate the body of the function_block (@main_fun). All passed variables are allocated on the stack.
///  ... a return variable is allocated on the stack and returned at the end of the function
#[test]
fn function_get_a_method_with_by_ref_parameters() {
    insta::assert_snapshot!(codegen(DEFAULT_FUNC), @r###"
    ; ModuleID = 'main'
    source_filename = "main"

    define i32 @main_fun(i16 %0, i8* %1, i64* %2) {
    entry:
      %main_fun = alloca i32, align 4
      %i = alloca i16, align 2
      store i16 %0, i16* %i, align 2
      %io = alloca i8*, align 8
      store i8* %1, i8** %io, align 8
      %o = alloca i64*, align 8
      store i64* %2, i64** %o, align 8
      %v = alloca i16, align 2
      %vt = alloca i16, align 2
      store i16 1, i16* %v, align 2
      store i16 2, i16* %vt, align 2
      store i32 0, i32* %main_fun, align 4
      %main_fun_ret = load i32, i32* %main_fun, align 4
      ret i32 %main_fun_ret
    }
    "###);
}

/// Calling a function works like this:
///  ... the function is called and all parameters are passed (`i32 @main_fun`)
///  ... note that VAR_OUT & VAR_IN_OUT parameters are passed as pointers (`i16*`)
#[test]
fn calling_a_function() {
    let calling_prg = format!(
        r#"
        PROGRAM prg
        VAR
			x : INT;
			z : SINT;
		END_VAR
            main_fun(x, z); 
        END_FUNCTION

        {DEFAULT_FUNC}
    "#
    );
    insta::assert_snapshot!(codegen(calling_prg.as_str()), @r###"
    ; ModuleID = 'main'
    source_filename = "main"

    %prg = type { i16, i8 }

    @prg_instance = global %prg zeroinitializer

    define void @prg(%prg* %0) {
    entry:
      %x = getelementptr inbounds %prg, %prg* %0, i32 0, i32 0
      %z = getelementptr inbounds %prg, %prg* %0, i32 0, i32 1
      %load_x = load i16, i16* %x, align 2
      %1 = alloca i64, align 8
      %call = call i32 @main_fun(i16 %load_x, i8* %z, i64* %1)
      ret void
    }

    define i32 @main_fun(i16 %0, i8* %1, i64* %2) {
    entry:
      %main_fun = alloca i32, align 4
      %i = alloca i16, align 2
      store i16 %0, i16* %i, align 2
      %io = alloca i8*, align 8
      store i8* %1, i8** %io, align 8
      %o = alloca i64*, align 8
      store i64* %2, i64** %o, align 8
      %v = alloca i16, align 2
      %vt = alloca i16, align 2
      store i16 1, i16* %v, align 2
      store i16 2, i16* %vt, align 2
      store i32 0, i32* %main_fun, align 4
      %main_fun_ret = load i32, i32* %main_fun, align 4
      ret i32 %main_fun_ret
    }
    "###);
}

/// Returning complex/aggregate types (string, array, struct) from a function cost a lot of compile-performance. complex types
/// can be returned much faster if the caller allocates the result on its own stack and passes a reference to it. That reference
/// needs to be treated exactly like a VAR_IN_OUT member.
/// ```
/// result := foo(a, b, c);
/// ```
/// where result is a complex/aggregate type is internally handled like
/// ```
/// foo(&result, a, b, c);
/// ```
#[test]
fn return_a_complex_type_from_function() {
    let returning_string = r#"
        FUNCTION foo : STRING
            foo := 'hello world!';
        END_FUNCTION

        PROGRAM prg
            VAR
                s : STRING; 
            END_VAR
            s := foo(); 
        END_FUNCTION
    "#;
    insta::assert_snapshot!(codegen(returning_string), @r###"
    ; ModuleID = 'main'
    source_filename = "main"

    %prg = type { [81 x i8] }

    @prg_instance = global %prg zeroinitializer
    @utf08_literal_0 = unnamed_addr constant [13 x i8] c"hello world!\00"

    define void @foo([81 x i8]* %0) {
    entry:
      %foo = alloca [81 x i8]*, align 8
      store [81 x i8]* %0, [81 x i8]** %foo, align 8
      %deref = load [81 x i8]*, [81 x i8]** %foo, align 8
      %1 = bitcast [81 x i8]* %deref to i8*
      call void @llvm.memset.p0i8.i64(i8* align 1 %1, i8 0, i64 ptrtoint ([81 x i8]* getelementptr ([81 x i8], [81 x i8]* null, i32 1) to i64), i1 false)
      %deref1 = load [81 x i8]*, [81 x i8]** %foo, align 8
      %2 = bitcast [81 x i8]* %deref1 to i8*
      call void @llvm.memcpy.p0i8.p0i8.i32(i8* align 1 %2, i8* align 1 getelementptr inbounds ([13 x i8], [13 x i8]* @utf08_literal_0, i32 0, i32 0), i32 13, i1 false)
      ret void
    }

    define void @prg(%prg* %0) {
    entry:
      %s = getelementptr inbounds %prg, %prg* %0, i32 0, i32 0
      %1 = alloca [81 x i8], align 1
      call void @foo([81 x i8]* %1)
      %2 = bitcast [81 x i8]* %s to i8*
      %3 = bitcast [81 x i8]* %1 to i8*
      call void @llvm.memcpy.p0i8.p0i8.i32(i8* align 1 %2, i8* align 1 %3, i32 80, i1 false)
      ret void
    }

    ; Function Attrs: argmemonly nofree nounwind willreturn writeonly
    declare void @llvm.memset.p0i8.i64(i8* nocapture writeonly, i8, i64, i1 immarg) #0

    ; Function Attrs: argmemonly nofree nounwind willreturn
    declare void @llvm.memcpy.p0i8.p0i8.i32(i8* noalias nocapture writeonly, i8* noalias nocapture readonly, i32, i1 immarg) #1

    attributes #0 = { argmemonly nofree nounwind willreturn writeonly }
    attributes #1 = { argmemonly nofree nounwind willreturn }
    "###);
}

/// Passing aggregate types to a function is an expensive operation, this is why the compiler offers
/// a {ref} macro that turns a block of variables into by-ref parameters without changing the
/// functions interface during compile-time.
///  ... all variables of a block marked by {ref} will be passed by-reference
///  ... the function's signature does not change, a caller passes the variable still the same way.
///      Although it looks like a value is passed by value, it is internally treated as a by-ref parameter
///  ... this allows more efficient function calls if the function can assure that it will not create any
///      side-effects regarding the passed values (e.g. change the variable).
#[test]
fn passing_by_ref_to_functions() {
    let src = r###"
        FUNCTION StrEqual : BOOL
          VAR_INPUT {ref}
            o1: STRING;
            o2: STRING;
          END_VAR
            // ...
        END_FUNCTION
        PROGRAM main
            VAR 
                str1, str2 : STRING; 
            END_VAR
            StrEqual(str1, str2); //looks like pass by-val
        END_PROGRAM
    "###;

    //internally we pass the two strings str1, and str2 as pointers to StrEqual because of the {ref}
    insta::assert_snapshot!(codegen(src), @r###"
    ; ModuleID = 'main'
    source_filename = "main"

    %main = type { [81 x i8], [81 x i8] }

    @main_instance = global %main zeroinitializer

    define i8 @StrEqual(i8* %0, i8* %1) {
    entry:
      %StrEqual = alloca i8, align 1
      %o1 = alloca i8*, align 8
      store i8* %0, i8** %o1, align 8
      %o2 = alloca i8*, align 8
      store i8* %1, i8** %o2, align 8
      store i8 0, i8* %StrEqual, align 1
      %StrEqual_ret = load i8, i8* %StrEqual, align 1
      ret i8 %StrEqual_ret
    }

    define void @main(%main* %0) {
    entry:
      %str1 = getelementptr inbounds %main, %main* %0, i32 0, i32 0
      %str2 = getelementptr inbounds %main, %main* %0, i32 0, i32 1
      %1 = bitcast [81 x i8]* %str1 to i8*
      %2 = bitcast [81 x i8]* %str2 to i8*
      %call = call i8 @StrEqual(i8* %1, i8* %2)
      ret void
    }
    "###);
}
