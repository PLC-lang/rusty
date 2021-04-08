// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
mod code_gen_tests;
mod codegen_error_messages_tests;
mod typesystem_test;

#[macro_export]
macro_rules! codegen_wihout_unwrap {
    ($code:tt) => {{
        let lexer = crate::lexer::lex($code);
        let (mut ast, _) = crate::parser::parse(lexer).unwrap();

        let context = inkwell::context::Context::create();
        crate::ast::pre_process(&mut ast);
        let index = crate::index::visitor::visit(&ast);
        let code_generator = crate::codegen::CodeGen::new(&context, "main");
        code_generator.generate(ast, &index)
    }};
}

#[macro_export]
macro_rules! codegen {
    ($code:tt) => {{
        let lexer = crate::lexer::lex($code);
        let (mut ast, _) = crate::parser::parse(lexer).unwrap();

        let context = inkwell::context::Context::create();
        crate::ast::pre_process(&mut ast);
        let index = crate::index::visitor::visit(&ast);
        let code_generator = crate::codegen::CodeGen::new(&context, "main");
        code_generator.generate(ast, &index).unwrap()
    }};
}

#[macro_export]
macro_rules! generate_with_empty_program {
    ($code:tt) => {{
        let source = format!("{} {}", "PROGRAM main END_PROGRAM", $code);
        let str_source = source.as_str();
        codegen!(str_source)
    }};
}

fn generate_program_boiler_plate(
    pou_name: &str,
    type_list: &[(&str, &str)],
    return_type: &str,
    thread_mode: &str,
    global_variables: &str,
    body: &str,
) -> String {
    let mut interface: String = type_list
        .iter()
        .map(|(t, _)| *t)
        .collect::<Vec<&str>>()
        .join(", ");
    if !interface.is_empty() {
        interface = format!(" {} ", interface);
    }

    let mut type_references: Vec<String> = vec![];

    for (i, t) in type_list.iter().enumerate() {
        let type_def = format!("  %{var_name} = getelementptr inbounds %{pou_name}_interface, %{pou_name}_interface* %0, i32 0, i32 {index}",
    var_name = t.1,
    index = i,
    pou_name = pou_name,
  );
        type_references.push(type_def);
    }

    if return_type != "void" {
        type_references.push(format!(
            "  %{pou_name} = alloca {return_type}",
            pou_name = pou_name,
            return_type = return_type
        ))
    }

    if !type_references.is_empty() {
        type_references.push("  ".to_string());
    }

    format!(
r#"; ModuleID = 'main'
source_filename = "main"

%{pou_name}_interface = type {{{type}}}
{global_variables}
@{pou_name}_instance = {thread_mode}global %{pou_name}_interface zeroinitializer

define {return_type} @{pou_name}(%{pou_name}_interface* %0) {{
entry:
{type_references}{body}}}
"#,
    pou_name = pou_name,
    type = interface,
    return_type = return_type,
    thread_mode = thread_mode,
        type_references = type_references.join(
r#"
"#
        ),
    body = body,
    global_variables = global_variables
    )
}

fn generate_program_boiler_plate_globals(global_variables: &str) -> String {
    generate_program_boiler_plate(
        "main",
        &[],
        "void",
        "",
        global_variables,
        "  ret void
",
    )
}
