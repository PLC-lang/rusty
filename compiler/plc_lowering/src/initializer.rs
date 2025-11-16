//! The initializer lowering module is responsible for additing initialization logic
//! to PLC AST nodes. This includes generating default values for variables, handling
//! constant expressions, and ensuring that all necessary initializations are present
//! before code generation. The module traverses the AST and modifies nodes as needed
//! to include initialization code, making sure that the resulting AST is ready for
//! further compilation stages.
//! Initialization logic is as follows:
//! - Every struct(and POU) has a constructor for fields with constant or pointer initializers.
//!    - The name for this constructor is `<StructName>_ctor`
//!    - The constructor is always public
//! - Variables of the struct are initialized by calling the constructor
//! - Global variables are initialized in a global constructor function called `__global_ctor`
//!   - This function is called per module inside the static initialization code
//!   - The function is private to the module
//! - Stateless POUs (functions and methods) are initialized during their call.
//!     - Variables of a stateless POU of a struct type are initialized using the constructor call.
//! - External POUs and struct constructors are marked as `extern` and have no body.
//! - External variables are not re-initialized in the global constructor, they are assumed to be
//!   initialized externally.
//! - Built-in types and variables are not re-initialized in the global
//! constructor.

use std::any::Any;

use plc::{
    index::{FxIndexMap, Index},
    lowering::helper::create_assignment,
};
use plc_ast::{
    ast::{AstFactory, AstNode, CompilationUnit},
    mut_visitor::AstVisitorMut,
    provider::IdProvider,
    visitor::AstVisitor,
};

#[derive(Debug, PartialEq)]
enum Body {
    Internal(Vec<AstNode>),
    External,
    None,
}

pub struct Initializer<'idx> {
    id_provider: IdProvider,
    index: &'idx Index,
    /// Stateful constructor per POU/struct
    constructors: FxIndexMap<String, Body>,
    /// Constructors for temp and stack variables per POU
    stack_constructor: FxIndexMap<String, Body>,
    /// Global constructor statements
    global_constructor: Vec<AstNode>,
}

//TODO: might need to be a mutable ast visitor
impl AstVisitor for Initializer<'_> {
    fn visit_pou(&mut self, pou: &plc_ast::ast::Pou) {
        match pou.linkage {
            plc_ast::ast::LinkageType::External => {
                self.constructors.insert(pou.name.clone(), Body::External);
                return;
            }
            plc_ast::ast::LinkageType::BuiltIn => {
                self.constructors.insert(pou.name.clone(), Body::None);
                return;
            }
            _ => {}
        };

        let mut constructor = vec![];
        let mut stack_constructor = vec![];

        // Collect variable initializers
        for var in pou.variable_blocks.iter() {
            for variable in var.variables.iter() {
                if let Some(initializer) = &variable.initializer {
                    // Create an assignment "self.<variable.name> := <initializer>"
                    let assignment = create_assignment(
                        variable.get_name(),
                        Some("self"),
                        initializer,
                        self.id_provider.clone(),
                    );
                    if var.is_temp() || (var.is_local() && !pou.is_stateful()) {
                        stack_constructor.push(assignment);
                    } else {
                        constructor.push(assignment);
                    }
                }
            }
        }

        self.constructors.insert(pou.name.clone(), Body::Internal(constructor));
        self.stack_constructor.insert(pou.name.clone(), Body::Internal(stack_constructor));
    }

    fn visit_user_type_declaration(&mut self, user_type: &plc_ast::ast::UserTypeDeclaration) {}

    fn visit_variable_block(&mut self, var_block: &plc_ast::ast::VariableBlock) {}
}

impl Initializer<'_> {
    pub fn new(id_provider: IdProvider, index: &Index) -> Initializer<'_> {
        Initializer {
            id_provider,
            index,
            constructors: FxIndexMap::default(),
            stack_constructor: FxIndexMap::default(),
            global_constructor: Vec::new(),
        }
    }
}

mod tests {
    use plc_ast::{ast::AstNode, visitor::AstVisitor};
    use plc_diagnostics::diagnostician::Diagnostician;

    fn print_to_string(nodes: &[AstNode]) -> String {
        nodes.iter().map(|it| it.as_string()).collect::<Vec<_>>().join("\n")
    }

    fn print_body_to_string(body: &super::Body) -> String {
        match body {
            super::Body::Internal(nodes) => print_to_string(nodes),
            super::Body::External => "extern".to_string(),
            super::Body::None => "none".to_string(),
        }
    }

    #[test]
    fn struct_gets_imlicit_initializer_and_constructor() {
        let src = r#"
        TYPE MyStruct : STRUCT
            a : INT := 5;
            b : REAL := 3.14;
            c : BOOL := TRUE;
        END_STRUCT
        "#;

        let diagnostician = Diagnostician::buffered();
        let mut pipeline =
            plc_driver::pipelines::BuildPipeline::from_sources("test.st", vec![(src)], diagnostician)
                .unwrap();
        let project = pipeline.parse_and_annotate().unwrap();
        // Visit the AST with the Initializer
        let mut initializer = super::Initializer::new(pipeline.context.provider(), &project.index);
        for unit in &project.units {
            initializer.visit_compilation_unit(unit.get_unit());
        }
        // Expecting a function declaration: void MyStruct_ctor(MyStruct* self)
        // Expecting assignments inside the constructor: self->a = 5; self->b = 3.14; self->c = 1;
        insta::assert_debug_snapshot!(print_body_to_string(initializer.constructors.get("MyStruct").unwrap()), @r#""#);
    }

    #[test]
    fn nested_structs_get_initializers_and_constructors() {
        let src = r#"
        TYPE InnerStruct : STRUCT
            x : INT := 10;
            y : INT := 20;
        END_STRUCT

        TYPE OuterStruct : STRUCT
            inner : InnerStruct;
            z : REAL := 2.71;
        END_STRUCT
        "#;

        let diagnostician = Diagnostician::buffered();
        let mut pipeline =
            plc_driver::pipelines::BuildPipeline::from_sources("test.st", vec![(src)], diagnostician)
                .unwrap();
        let project = pipeline.parse_and_annotate().unwrap();
        // Visit the AST with the Initializer
        let mut initializer = super::Initializer::new(pipeline.context.provider(), &project.index);
        for unit in &project.units {
            initializer.visit_compilation_unit(unit.get_unit());
        }
        // Check for constructors
        // Expecting a function declaration: void InnerStruct_ctor(InnerStruct* self)
        // Expecting assignments inside the constructor: self->x = 10; self->y = 20;
        insta::assert_debug_snapshot!(print_body_to_string(initializer.constructors.get("InnerStruct").unwrap()), @r#""#);
        // Expecting a function declaration: void OuterStruct_ctor(OuterStruct* self)
        // Expecting a call to InnerStruct_ctor(&self->inner);
        // Expecting an assignment: self->z = 2.71;
        insta::assert_debug_snapshot!(print_body_to_string(initializer.constructors.get("OuterStruct").unwrap()), @r#""#);
    }

    #[test]
    fn nested_structs_with_values_get_initializers_and_constructors() {
        let src = r#"
        TYPE InnerStruct : STRUCT
            x : INT := 10;
            y : INT := 20;
        END_STRUCT

        TYPE OuterStruct : STRUCT
            inner : InnerStruct := (x := 1, y := 2);
            inner2 : InnerStruct := (y := 3);
            z : REAL := 2.71;
        END_STRUCT
        "#;

        let diagnostician = Diagnostician::buffered();
        let mut pipeline =
            plc_driver::pipelines::BuildPipeline::from_sources("test.st", vec![(src)], diagnostician)
                .unwrap();
        let project = pipeline.parse_and_annotate().unwrap();
        // Visit the AST with the Initializer
        let mut initializer = super::Initializer::new(pipeline.context.provider(), &project.index);
        for unit in &project.units {
            initializer.visit_compilation_unit(unit.get_unit());
        }
        // Check for constructors
        // Expecting a function declaration: void InnerStruct_ctor(InnerStruct* self)
        // Expecting assignments inside the constructor: self->x = 10; self->y = 20;
        insta::assert_debug_snapshot!(print_body_to_string(initializer.constructors.get("InnerStruct").unwrap()), @r#""#);
        // Expecting a function declaration: void OuterStruct_ctor(OuterStruct* self)
        // Expecting a call to InnerStruct_ctor(&self->inner);
        // Expecting a call to InnerStruct_ctor(&self->inner2);
        // Expecting assignments: self->inner.x = 1; self->inner.y = 2;
        // Expecting assignments: self->inner2.y = 3;
        // Expecting an assignment: self->z = 2.71;
        insta::assert_debug_snapshot!(print_body_to_string(initializer.constructors.get("OuterStruct").unwrap()), @r#""#);
    }

    #[test]
    fn variable_with_pointer_initializer_is_added_to_constructor() {
        let src = r#"
        VAR_GLOBAL
            gVar : INT;
            myStructVar : MyStruct;
        END_VAR
        TYPE MyStruct : STRUCT
            a : INT := 5;
            b : POINTER TO INT := ADR(gVar);
            c : BOOL := TRUE;
        END_STRUCT
        "#;

        let diagnostician = Diagnostician::buffered();
        let mut pipeline =
            plc_driver::pipelines::BuildPipeline::from_sources("test.st", vec![(src)], diagnostician)
                .unwrap();
        let project = pipeline.parse_and_annotate().unwrap();
        // Visit the AST with the Initializer
        let mut initializer = super::Initializer::new(pipeline.context.provider(), &project.index);
        for unit in &project.units {
            initializer.visit_compilation_unit(unit.get_unit());
        }
        // Check for constructor
        // Expecting a function declaration: void MyStruct_ctor(MyStruct* self)
        // Expecting assignments inside the constructor: self->a = 5; self->c = 1;
        // Expecting an assignment: self->b = &gVar;
        insta::assert_debug_snapshot!(print_body_to_string(initializer.constructors.get("MyStruct").unwrap()), @r#""#);
    }

    #[test]
    fn enum_default_values_in_struct() {
        let src = r#"
        TYPE MyEnum : (Option1, Option2, Option3) := Option3;
        TYPE MyStruct : STRUCT
            e : MyEnum := Option2;
            n : INT := 42;
        END_STRUCT
        "#;

        let diagnostician = Diagnostician::buffered();
        let mut pipeline =
            plc_driver::pipelines::BuildPipeline::from_sources("test.st", vec![(src)], diagnostician)
                .unwrap();
        let project = pipeline.parse_and_annotate().unwrap();
        // Visit the AST with the Initializer
        let mut initializer = super::Initializer::new(pipeline.context.provider(), &project.index);
        for unit in &project.units {
            initializer.visit_compilation_unit(unit.get_unit());
        }
        // Expecting a function declaration: void MyEnum_ctor(MyEnum* self)
        // Expecting an assignment inside the constructor: *self = 2;
        insta::assert_debug_snapshot!(print_body_to_string(initializer.constructors.get("MyEnum").unwrap()), @r#""#);
        // Expecting a function declaration: void MyStruct_ctor(MyStruct* self)
        // Expecting a call to MyEnum_ctor(&self->e);
        // Expecting an assignment: self->e = 1;
        // Expecting an assignment: self->n = 42;
        insta::assert_debug_snapshot!(print_body_to_string(initializer.constructors.get("MyStruct").unwrap()), @r#""#);
    }

    #[test]
    fn nested_struct_with_different_default_values() {
        let src = r#"
        VAR_GLOBAL
            gVar : INT := 10;
        END_VAR

        TYPE InnerStruct : STRUCT
            a : INT := 1;
            b : POINTER TO INT := ADR(gVar);
        END_STRUCT

        TYPE InnerStruct2 : STRUCT
            c : INT := 4;
            d : INT := 5;
            inner : InnerStruct := (a := 6);
            inner2 : InnerStruct := (b := ADR(gVar));
        END_STRUCT

        TYPE OuterStruct : STRUCT
            e : INT := 0;
            inner : InnerStruct2 := (a := 1, b := 2, inner := (a := 3));
            inner2 : InnerStruct2 := (d := 8, inner := (b := ADR(gVar)));
            inner3 : InnerStruct2 := (inner (a := 9));
        END_STRUCT
        "#;

        let diagnostician = Diagnostician::buffered();
        let mut pipeline =
            plc_driver::pipelines::BuildPipeline::from_sources("test.st", vec![(src)], diagnostician)
                .unwrap();
        let project = pipeline.parse_and_annotate().unwrap();
        // Visit the AST with the Initializer
        let mut initializer = super::Initializer::new(pipeline.context.provider(), &project.index);
        for unit in &project.units {
            initializer.visit_compilation_unit(unit.get_unit());
        }
        // Check for constructors
        // Expecting a function declaration: void InnerStruct_ctor(InnerStruct* self)
        // Expecting assignments inside the constructor: self->a = 1; self->b = &gVar;
        insta::assert_debug_snapshot!(print_body_to_string(initializer.constructors.get("InnerStruct").unwrap()), @r#""#);
        // Expecting a function declaration: void InnerStruct2_ctor(InnerStruct2* self)
        // Expecting a call to InnerStruct_ctor(&self->inner);
        // Expecting a call to InnerStruct_ctor(&self->inner2);
        // Expecting assignments: self->c = 4; self->d = 5;
        // Expecting assignments: self->inner.a = 6; self->inner2.b = &gvar;
        insta::assert_debug_snapshot!(print_body_to_string(initializer.constructors.get("InnerStruct2").unwrap()), @r#""#);
        // Expecting a function declaration: void OuterStruct_ctor(OuterStruct* self)
        // Expecting a call to InnerStruct2_ctor(&self->inner);
        // Expecting a call to InnerStruct2_ctor(&self->inner2);
        // Expecting a call to InnerStruct2_ctor(&self->inner3);
        // Expecting an assignment: self->e = 0;
        // Expecting assignments: self->inner.a = 1; self->inner.b = 2; self->inner.inner.a = 3;
        // Expecting assignments: self->inner2.d = 8; self->inner2.inner.b = &gVar;
        // Expecting assignments: self->inner3.inner.a = 9;
        insta::assert_debug_snapshot!(print_body_to_string(initializer.constructors.get("OuterStruct").unwrap()), @r#""#);
    }

    #[test]
    fn global_constructor_is_generated() {
        let src = r#"
        VAR_GLOBAL
            gVar1 : INT := 10;
            gVar2 : REAL;
            gStructVar : MyStruct;
        END_VAR

        TYPE MyStruct : STRUCT
            a : INT := 5;
            b : POINTER TO INT := ADR(gVar1);
            c : BOOL := TRUE;
        END_STRUCT
        "#;

        let diagnostician = Diagnostician::buffered();
        let mut pipeline =
            plc_driver::pipelines::BuildPipeline::from_sources("test.st", vec![(src)], diagnostician)
                .unwrap();
        let project = pipeline.parse_and_annotate().unwrap();
        // Visit the AST with the Initializer
        let mut initializer = super::Initializer::new(pipeline.context.provider(), &project.index);
        for unit in &project.units {
            initializer.visit_compilation_unit(unit.get_unit());
        }
        // Check for global constructor
        // Expecting a call to MyStruct_ctor(&gStructVar);
        insta::assert_debug_snapshot!(print_to_string(&initializer.global_constructor), @r#""#);
    }

    #[test]
    fn function_constructors_have_the_function_name() {
        let src = r#"
        FUNCTION MyFunction : INT
        VAR
            localStruct : MyStruct;
        END_VAR
        END_FUNCTION

        TYPE MyStruct : STRUCT
            a : INT := 5;
            b : BOOL := TRUE;
        END_STRUCT
        "#;

        let diagnostician = Diagnostician::buffered();
        let mut pipeline =
            plc_driver::pipelines::BuildPipeline::from_sources("test.st", vec![(src)], diagnostician)
                .unwrap();
        let project = pipeline.parse_and_annotate().unwrap();
        // Visit the AST with the Initializer
        let mut initializer = super::Initializer::new(pipeline.context.provider(), &project.index);
        for unit in &project.units {
            initializer.visit_compilation_unit(unit.get_unit());
        }
        // Check for function constructor on the stack
        // Expecting a call to MyStruct_ctor(&localStruct);
        insta::assert_debug_snapshot!(print_body_to_string(initializer.stack_constructor.get("MyFunction").unwrap()), @r#""#);
    }

    #[test]
    fn program_temp_variables_are_in_stack_constructor() {
        let src = r#"
        PROGRAM MyProgram
        VAR_TEMP
            tempStruct : MyStruct;
        END_VAR
        VAR
            localStruct : MyStruct;
        END_VAR

        TYPE MyStruct : STRUCT
            a : INT := 5;
            b : BOOL := TRUE;
        END_STRUCT
        "#;

        let diagnostician = Diagnostician::buffered();
        let mut pipeline =
            plc_driver::pipelines::BuildPipeline::from_sources("test.st", vec![(src)], diagnostician)
                .unwrap();
        let project = pipeline.parse_and_annotate().unwrap();
        // Visit the AST with the Initializer
        let mut initializer = super::Initializer::new(pipeline.context.provider(), &project.index);
        for unit in &project.units {
            initializer.visit_compilation_unit(unit.get_unit());
        }
        // Check for program constructor on the stack
        // Expecting a call to MyStruct_ctor(&localStruct);
        insta::assert_debug_snapshot!(print_body_to_string(initializer.stack_constructor.get("MyProgram").unwrap()), @r#""#);
        // Check for program constructor
        insta::assert_debug_snapshot!(print_body_to_string(initializer.constructors.get("MyProgram").unwrap()), @r#""#);
    }

    #[test]
    fn programs_are_globals() {
        let src = r#"
        PROGRAM MyProgram
        VAR
        END_VAR
        END_PROGRAM
        "#;

        let diagnostician = Diagnostician::buffered();
        let mut pipeline =
            plc_driver::pipelines::BuildPipeline::from_sources("test.st", vec![(src)], diagnostician)
                .unwrap();
        let project = pipeline.parse_and_annotate().unwrap();
        // Visit the AST with the Initializer
        let mut initializer = super::Initializer::new(pipeline.context.provider(), &project.index);
        for unit in &project.units {
            initializer.visit_compilation_unit(unit.get_unit());
        }
        // Check for program constructor
        // Expecting a call to MyProgram_ctor(&progStruct);
        insta::assert_debug_snapshot!(print_to_string(&initializer.global_constructor), @r#""#);
    }

    #[test]
    fn external_structs_and_variables_are_not_initialized() {
        let src = r#"
        VAR_GLOBAL
            internalVar : MyExtStruct;
        END_VAR

        {external}
        VAR_GLOBAL
            extVar : MyExtStruct;
        END_VAR

        {external}
        TYPE MyExtStruct : STRUCT
            a : INT := 5;
            b : BOOL := TRUE;
        END_STRUCT
        "#;

        let diagnostician = Diagnostician::buffered();
        let mut pipeline =
            plc_driver::pipelines::BuildPipeline::from_sources("test.st", vec![(src)], diagnostician)
                .unwrap();
        let project = pipeline.parse_and_annotate().unwrap();
        // Visit the AST with the Initializer
        let mut initializer = super::Initializer::new(pipeline.context.provider(), &project.index);
        for unit in &project.units {
            initializer.visit_compilation_unit(unit.get_unit());
        }
        // Check for internal var assignment in global constructor
        // Expecting a call to MyExtStruct_ctor(&internalVar);
        // No call to MyExtStruct_ctor(&extVar);
        insta::assert_debug_snapshot!(print_to_string(&initializer.global_constructor), @r#""#);
        // Check that no constructor is generated for MyExtStruct
        insta::assert_debug_snapshot!(print_body_to_string(initializer.constructors.get("MyExtStruct").unwrap()), @r#""#);
    }
}
