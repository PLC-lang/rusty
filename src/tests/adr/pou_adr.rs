use crate::test_utils::tests::{annotate, codegen, index};

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
/// POU like the name of the struct-type carrying the program's state, the names of all members, vararg and generic
/// information etc.

#[test]
fn programs_state_is_stored_in_a_struct() {
    // parse and index
    let (pr, mut index) = index(DEFAULT_PRG);
    // process pou and types
    annotate(&pr, &mut index);

    let pou_struct = index
        .find_pou("main_prg")
        .and_then(|pou| pou.find_instance_struct_type(&index));

    insta::assert_debug_snapshot!(pou_struct, @r###"
    Some(
        DataType {
            name: "main_prg",
            initial_value: None,
            information: Struct {
                name: "main_prg_interface",
                member_names: [
                    "i",
                    "io",
                    "o",
                    "v",
                    "vt",
                ],
                source: Pou(
                    Program,
                ),
            },
            nature: Any,
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

    %main_prg_interface = type { i16, i16*, i16, i16 }

    @main_prg_instance = global %main_prg_interface zeroinitializer

    define void @main_prg(%main_prg_interface* %0) {
    entry:
      %i = getelementptr inbounds %main_prg_interface, %main_prg_interface* %0, i32 0, i32 0
      %io = getelementptr inbounds %main_prg_interface, %main_prg_interface* %0, i32 0, i32 1
      %o = getelementptr inbounds %main_prg_interface, %main_prg_interface* %0, i32 0, i32 2
      %v = getelementptr inbounds %main_prg_interface, %main_prg_interface* %0, i32 0, i32 3
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

    %main_prg_interface = type { i16, i16*, i16, i16 }

    @main_prg_instance = global %main_prg_interface zeroinitializer

    define i16 @foo() {
    entry:
      %x = alloca i16, align 2
      %y = alloca i16, align 2
      %foo = alloca i16, align 2
      store i16 0, i16* %x, align 2
      store i16 0, i16* %y, align 2
      store i16 0, i16* %foo, align 2
      store i16 1, i16* getelementptr inbounds (%main_prg_interface, %main_prg_interface* @main_prg_instance, i32 0, i32 0), align 2
      store i16* %y, i16** getelementptr inbounds (%main_prg_interface, %main_prg_interface* @main_prg_instance, i32 0, i32 1), align 8
      call void @main_prg(%main_prg_interface* @main_prg_instance)
      %0 = load i16, i16* getelementptr inbounds (%main_prg_interface, %main_prg_interface* @main_prg_instance, i32 0, i32 2), align 2
      store i16 %0, i16* %x, align 2
      %foo_ret = load i16, i16* %foo, align 2
      ret i16 %foo_ret
    }

    define void @main_prg(%main_prg_interface* %0) {
    entry:
      %i = getelementptr inbounds %main_prg_interface, %main_prg_interface* %0, i32 0, i32 0
      %io = getelementptr inbounds %main_prg_interface, %main_prg_interface* %0, i32 0, i32 1
      %o = getelementptr inbounds %main_prg_interface, %main_prg_interface* %0, i32 0, i32 2
      %v = getelementptr inbounds %main_prg_interface, %main_prg_interface* %0, i32 0, i32 3
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

    %main_fb_interface = type { i16, i16*, i16, i16 }

    @main_fb__init = unnamed_addr constant %main_fb_interface { i16 6, i16* null, i16 0, i16 1 }

    define void @main_fb(%main_fb_interface* %0) {
    entry:
      %i = getelementptr inbounds %main_fb_interface, %main_fb_interface* %0, i32 0, i32 0
      %io = getelementptr inbounds %main_fb_interface, %main_fb_interface* %0, i32 0, i32 1
      %o = getelementptr inbounds %main_fb_interface, %main_fb_interface* %0, i32 0, i32 2
      %v = getelementptr inbounds %main_fb_interface, %main_fb_interface* %0, i32 0, i32 3
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

    %foo_interface = type { i16, i16, %main_fb_interface }
    %main_fb_interface = type { i16, i16*, i16, i16 }

    @foo_instance = global %foo_interface { i16 0, i16 0, %main_fb_interface { i16 6, i16* null, i16 0, i16 1 } }
    @main_fb__init = unnamed_addr constant %main_fb_interface { i16 6, i16* null, i16 0, i16 1 }

    define void @foo(%foo_interface* %0) {
    entry:
      %x = getelementptr inbounds %foo_interface, %foo_interface* %0, i32 0, i32 0
      %y = getelementptr inbounds %foo_interface, %foo_interface* %0, i32 0, i32 1
      %fb = getelementptr inbounds %foo_interface, %foo_interface* %0, i32 0, i32 2
      %1 = getelementptr inbounds %main_fb_interface, %main_fb_interface* %fb, i32 0, i32 0
      store i16 1, i16* %1, align 2
      %2 = getelementptr inbounds %main_fb_interface, %main_fb_interface* %fb, i32 0, i32 1
      store i16* %y, i16** %2, align 8
      call void @main_fb(%main_fb_interface* %fb)
      %3 = getelementptr inbounds %main_fb_interface, %main_fb_interface* %fb, i32 0, i32 2
      %4 = load i16, i16* %3, align 2
      store i16 %4, i16* %x, align 2
      ret void
    }

    define void @main_fb(%main_fb_interface* %0) {
    entry:
      %i = getelementptr inbounds %main_fb_interface, %main_fb_interface* %0, i32 0, i32 0
      %io = getelementptr inbounds %main_fb_interface, %main_fb_interface* %0, i32 0, i32 1
      %o = getelementptr inbounds %main_fb_interface, %main_fb_interface* %0, i32 0, i32 2
      %v = getelementptr inbounds %main_fb_interface, %main_fb_interface* %0, i32 0, i32 3
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
      %i = alloca i16, align 2
      store i16 %0, i16* %i, align 2
      %io = alloca i8*, align 8
      store i8* %1, i8** %io, align 8
      %o = alloca i64*, align 8
      store i64* %2, i64** %o, align 8
      %v = alloca i16, align 2
      %vt = alloca i16, align 2
      %main_fun = alloca i32, align 4
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

    %prg_interface = type { i16, i8 }

    @prg_instance = global %prg_interface zeroinitializer

    define void @prg(%prg_interface* %0) {
    entry:
      %x = getelementptr inbounds %prg_interface, %prg_interface* %0, i32 0, i32 0
      %z = getelementptr inbounds %prg_interface, %prg_interface* %0, i32 0, i32 1
      %load_x = load i16, i16* %x, align 2
      %1 = alloca i64, align 8
      %call = call i32 @main_fun(i16 %load_x, i8* %z, i64* %1)
      ret void
    }

    define i32 @main_fun(i16 %0, i8* %1, i64* %2) {
    entry:
      %i = alloca i16, align 2
      store i16 %0, i16* %i, align 2
      %io = alloca i8*, align 8
      store i8* %1, i8** %io, align 8
      %o = alloca i64*, align 8
      store i64* %2, i64** %o, align 8
      %v = alloca i16, align 2
      %vt = alloca i16, align 2
      %main_fun = alloca i32, align 4
      store i16 1, i16* %v, align 2
      store i16 2, i16* %vt, align 2
      store i32 0, i32* %main_fun, align 4
      %main_fun_ret = load i32, i32* %main_fun, align 4
      ret i32 %main_fun_ret
    }
    "###);
}
