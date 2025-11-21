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

use std::{any::Any, default};

use plc::{
    index::{FxIndexMap, Index},
    lowering::helper::{create_assignment, create_call_statement},
};
use plc_ast::{
    ast::{AstFactory, AstNode, CompilationUnit, PouType, VariableBlockType},
    provider::IdProvider,
    visitor::{AstVisitor, Walker},
};
use plc_source::source_location::SourceLocation;

#[derive(Debug, PartialEq)]
enum Body {
    Internal(Vec<AstNode>),
    External,
    None,
}

pub struct Initializer<'idx> {
    pub id_provider: IdProvider,
    index: Option<&'idx Index>,
    /// Stateful constructor per POU/struct
    constructors: FxIndexMap<String, Body>,
    /// User defined constructor names per POU/datatype
    user_defined_constructors: FxIndexMap<String, String>,
    /// Constructors for temp and stack variables per POU
    stack_constructor: FxIndexMap<String, Body>,
    /// Global constructor statements
    global_constructor: (Vec<AstNode>, Vec<AstNode>),
    context: Context,
}

#[derive(Default)]
struct Context {
    pub current_pou: Option<String>,
    pub current_datatype: Option<String>,
    pub current_variable_block: Option<VariableBlockType>,
}

impl Context {
    fn enter_pou(&mut self, pou: &str) {
        self.current_pou = Some(pou.to_string());
    }
    fn exit_pou(&mut self) {
        self.current_pou = None;
    }
    fn enter_datatype(&mut self, datatype: &str) {
        self.current_datatype = Some(datatype.to_string());
    }
    fn exit_datatype(&mut self) {
        self.current_datatype = None;
    }
    fn enter_variable_block(&mut self, block: &plc_ast::ast::VariableBlock) {
        self.current_variable_block = Some(block.kind);
    }
    fn exit_variable_block(&mut self) {
        self.current_variable_block = None;
    }
}

//TODO: might need to be a mutable ast visitor
impl AstVisitor for Initializer<'_> {
    fn visit_pou(&mut self, pou: &plc_ast::ast::Pou) {
        self.context.enter_pou(pou.name.as_str());
        self.user_defined_constructors.insert(pou.name.clone(), "FB_INIT".to_string());
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
        pou.walk(self);

        self.constructors.insert(pou.name.clone(), Body::Internal(vec![]));
        self.stack_constructor.insert(pou.name.clone(), Body::Internal(vec![]));
        self.context.exit_pou();
    }

    fn visit_variable_block(&mut self, block: &plc_ast::ast::VariableBlock) {
        self.context.enter_variable_block(block);
        block.walk(self);
        self.context.exit_variable_block();
    }

    fn visit_variable(&mut self, variable: &plc_ast::ast::Variable) {
        // grab the index
        let index = self.index.expect("index is set at this stage");
        let variable_block_type =
            self.context.current_variable_block.expect("variable block is set at this stage");
        // Find if the parent is stateful
        let is_stateful = if let Some(pou_name) = &self.context.current_pou {
            index.find_pou(pou_name).is_some_and(|it| it.is_stateful())
        } else {
            true
        };
        if let Some(initializer) = &variable.initializer {
            // Create a call to the type constructor
            // Create a call to the type's user defined constructor
            // Create an assignment "self.<variable.name> := <initializer>"
            let assignment =
                create_assignment(variable.get_name(), Some("self"), initializer, self.id_provider.clone());
            if variable_block_type.is_temp() || (variable_block_type.is_local() && is_stateful) {
                self.add_to_current_stack_constructor(assignment);
            } else {
                self.add_to_current_constructor(assignment);
            }
        }
    }

    fn visit_user_type_declaration(&mut self, user_type: &plc_ast::ast::UserTypeDeclaration) {
        let name = user_type.data_type.get_name().expect("name is set at this stage").to_string();
        match user_type.linkage {
            plc_ast::ast::LinkageType::External => {
                self.constructors.insert(name, Body::External);
                return;
            }
            plc_ast::ast::LinkageType::BuiltIn => {
                self.constructors.insert(name, Body::None);
                return;
            }
            _ => {}
        };

        let mut constructor = vec![];
        if let plc_ast::ast::DataType::StructType { variables, .. } = &user_type.data_type {
            for variable in variables.iter() {
                if let Some(initializer) = &variable.initializer {
                    // Create an assignment "self.<variable.name> := <initializer>"
                    let assignment = create_assignment(
                        variable.get_name(),
                        Some("self"),
                        initializer,
                        self.id_provider.clone(),
                    );
                    constructor.push(assignment);
                }
            }
        }
        // Insert the initializer as the assignment to self
        if let Some(initializer) = &user_type.initializer {
            let assignment = create_assignment("self", None, initializer, self.id_provider.clone());
            constructor.push(assignment);
        }
        self.constructors.insert(name.to_string(), Body::Internal(constructor));
    }
}

impl<'idx> Initializer<'idx> {
    pub fn new(id_provider: IdProvider) -> Initializer<'idx> {
        Initializer {
            id_provider,
            index: None,
            constructors: Default::default(),
            user_defined_constructors: Default::default(),
            stack_constructor: Default::default(),
            global_constructor: Default::default(),
            context: Default::default(),
        }
    }

    pub fn apply_initialization(&mut self, unit: &mut CompilationUnit, index: &'idx Index) {
        // Set the index
        self.index = Some(index);
        // Visit the unit and prepare constructors
        // Add each constructor function to the unit as a new function
        // Add the construction calls for stack variables to each function
        // Remove the index
        self.index = None
    }

    fn add_to_current_stack_constructor(&mut self, node: Vec<AstNode>) {
        if let Some(current_pou) = self.context.current_pou.as_ref() {
            if let Some(body) = self.stack_constructor.get_mut(current_pou) {
                match body {
                    Body::Internal(nodes) => nodes.extend(node),
                    _ => {}
                }
            }
        }
    }

    fn add_to_current_constructor(&mut self, node: Vec<AstNode>) {
        if let Some(current_struct) =
            self.context.current_pou.as_ref().or_else(|| self.context.current_datatype.as_ref())
        {
            if let Some(body) = self.constructors.get_mut(current_struct) {
                match body {
                    Body::Internal(nodes) => nodes.extend(node),
                    _ => {}
                }
            }
        }
    }

    fn get_constructor_call(&self, type_name: &str, var_name: &str) -> Option<AstNode> {
        if self.constructors.contains_key(type_name) {
            let call = create_call_statement(
                &format!("{}_ctor", type_name),
                var_name,
                None,
                self.id_provider.clone(),
                &SourceLocation::internal(),
            );
            Some(call)
        } else {
            None
        }
    }

    fn get_user_defined_constructor_call(&self, type_name: &str, var_name: &str) -> Option<AstNode> {
        if let Some(index) = self.index {
            // Search for the user defined constructor for the given struct
            None
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use plc_ast::{ast::AstNode, visitor::AstVisitor};
    use plc_diagnostics::diagnostician::Diagnostician;
    use plc_driver::pipelines::{AnnotatedProject, BuildPipeline};
    use plc_source::SourceCode;

    use crate::initializer::Initializer;

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

    fn parse_and_init(src: &str) -> Initializer {
        let src: SourceCode = src.into();
        let diagnostician = Diagnostician::buffered();
        let mut pipeline = BuildPipeline::from_sources("test.st", vec![(src)], diagnostician).unwrap();
        let AnnotatedProject { units, .. } = pipeline.parse_and_annotate().unwrap();
        // Visit the AST with the Initializer
        let mut initializer = super::Initializer::new(pipeline.context.provider());
        for unit in units {
            initializer.visit_compilation_unit(unit.get_unit());
        }
        initializer
    }

    #[test]
    fn struct_gets_imlicit_initializer_and_constructor() {
        let src = r#"
        TYPE MyStruct : STRUCT
            a : INT := 5;
            b : REAL := 3.14;
            c : BOOL := TRUE;
        END_STRUCT
        END_TYPE
        "#;

        let initializer = parse_and_init(src);
        // Expecting a function declaration: void MyStruct_ctor(MyStruct* self)
        // Expecting assignments inside the constructor: self->a = 5; self->b = 3.14; self->c = 1;
        insta::assert_snapshot!(print_body_to_string(initializer.constructors.get("MyStruct").unwrap()), @r"
        self.a := 5
        self.b := 3.14
        self.c := TRUE
        ");
    }

    #[test]
    fn nested_structs_get_initializers_and_constructors() {
        let src = r#"
        TYPE InnerStruct : STRUCT
            x : INT := 10;
            y : INT := 20;
        END_STRUCT
        END_TYPE

        TYPE OuterStruct : STRUCT
            inner : InnerStruct;
            z : REAL := 2.71;
        END_STRUCT
        END_TYPE
        "#;

        let initializer = parse_and_init(src);
        // Check for constructors
        // Expecting a function declaration: void InnerStruct_ctor(InnerStruct* self)
        // Expecting assignments inside the constructor: self->x = 10; self->y = 20;
        insta::assert_snapshot!(print_body_to_string(initializer.constructors.get("InnerStruct").unwrap()), @r"
        self.x := 10
        self.y := 20
        ");
        // Expecting a function declaration: void OuterStruct_ctor(OuterStruct* self)
        // Expecting a call to InnerStruct_ctor(&self->inner);
        // Expecting an assignment: self->z = 2.71;
        insta::assert_snapshot!(print_body_to_string(initializer.constructors.get("OuterStruct").unwrap()), @"self.z := 2.71");
    }

    #[test]
    fn nested_structs_with_values_get_initializers_and_constructors() {
        let src = r#"
        TYPE InnerStruct : STRUCT
            x : INT := 10;
            y : INT := 20;
        END_STRUCT
        END_TYPE

        TYPE OuterStruct : STRUCT
            inner : InnerStruct := (x := 1, y := 2);
            inner2 : InnerStruct := (y := 3);
            z : REAL := 2.71;
        END_STRUCT
        END_TYPE
        "#;

        let initializer = parse_and_init(src);
        // Check for constructors
        // Expecting a function declaration: void InnerStruct_ctor(InnerStruct* self)
        // Expecting assignments inside the constructor: self->x = 10; self->y = 20;
        insta::assert_snapshot!(print_body_to_string(initializer.constructors.get("InnerStruct").unwrap()), @r"
        self.x := 10
        self.y := 20
        ");
        // Expecting a function declaration: void OuterStruct_ctor(OuterStruct* self)
        // Expecting a call to InnerStruct_ctor(&self->inner);
        // Expecting a call to InnerStruct_ctor(&self->inner2);
        // Expecting assignments: self->inner.x = 1; self->inner.y = 2;
        // Expecting assignments: self->inner2.y = 3;
        // Expecting an assignment: self->z = 2.71;
        insta::assert_snapshot!(print_body_to_string(initializer.constructors.get("OuterStruct").unwrap()), @r"
        self.inner := (x := 1, y := 2)
        self.inner2 := (y := 3)
        self.z := 2.71
        ");
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
        END_TYPE
        "#;

        let initializer = parse_and_init(src);
        // Check for constructor
        // Expecting a function declaration: void MyStruct_ctor(MyStruct* self)
        // Expecting assignments inside the constructor: self->a = 5; self->c = 1;
        // Expecting an assignment: self->b = &gVar;
        insta::assert_snapshot!(print_body_to_string(initializer.constructors.get("MyStruct").unwrap()), @r"
        self.a := 5
        self.b := ADR(gVar)
        self.c := TRUE
        ");
    }

    #[test]
    fn enum_default_values_in_struct() {
        let src = r#"
        TYPE MyEnum : (Option1, Option2, Option3) := Option3; END_TYPE
        TYPE MyStruct : STRUCT
            e : MyEnum := Option2;
            n : INT := 42;
        END_STRUCT
        END_TYPE
        "#;

        let initializer = parse_and_init(src);
        // Expecting a function declaration: void MyEnum_ctor(MyEnum* self)
        // Expecting an assignment inside the constructor: *self = 2;
        insta::assert_snapshot!(print_body_to_string(initializer.constructors.get("MyEnum").unwrap()), @"self := Option3");
        // Expecting a function declaration: void MyStruct_ctor(MyStruct* self)
        // Expecting a call to MyEnum_ctor(&self->e);
        // Expecting an assignment: self->e = 1;
        // Expecting an assignment: self->n = 42;
        insta::assert_snapshot!(print_body_to_string(initializer.constructors.get("MyStruct").unwrap()), @r"
        self.e := Option2
        self.n := 42
        ");
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
        END_TYPE

        TYPE InnerStruct2 : STRUCT
            c : INT := 4;
            d : INT := 5;
            inner : InnerStruct := (a := 6);
            inner2 : InnerStruct := (b := ADR(gVar));
        END_STRUCT
        END_TYPE

        TYPE OuterStruct : STRUCT
            e : INT := 0;
            inner : InnerStruct2 := (a := 1, b := 2, inner := (a := 3));
            inner2 : InnerStruct2 := (d := 8, inner := (b := ADR(gVar)));
            inner3 : InnerStruct2 := (inner (a := 9));
        END_STRUCT
        END_TYPE
        "#;

        let initializer = parse_and_init(src);
        // Check for constructors
        // Expecting a function declaration: void InnerStruct_ctor(InnerStruct* self)
        // Expecting assignments inside the constructor: self->a = 1; self->b = &gVar;
        insta::assert_snapshot!(print_body_to_string(initializer.constructors.get("InnerStruct").unwrap()), @r"
        self.a := 1
        self.b := ADR(gVar)
        ");
        // Expecting a function declaration: void InnerStruct2_ctor(InnerStruct2* self)
        // Expecting a call to InnerStruct_ctor(&self->inner);
        // Expecting a call to InnerStruct_ctor(&self->inner2);
        // Expecting assignments: self->c = 4; self->d = 5;
        // Expecting assignments: self->inner.a = 6; self->inner2.b = &gvar;
        insta::assert_snapshot!(print_body_to_string(initializer.constructors.get("InnerStruct2").unwrap()), @r"
        self.c := 4
        self.d := 5
        self.inner := (a := 6)
        self.inner2 := (b := ADR(gVar))
        ");
        // Expecting a function declaration: void OuterStruct_ctor(OuterStruct* self)
        // Expecting a call to InnerStruct2_ctor(&self->inner);
        // Expecting a call to InnerStruct2_ctor(&self->inner2);
        // Expecting a call to InnerStruct2_ctor(&self->inner3);
        // Expecting an assignment: self->e = 0;
        // Expecting assignments: self->inner.a = 1; self->inner.b = 2; self->inner.inner.a = 3;
        // Expecting assignments: self->inner2.d = 8; self->inner2.inner.b = &gVar;
        // Expecting assignments: self->inner3.inner.a = 9;
        insta::assert_snapshot!(print_body_to_string(initializer.constructors.get("OuterStruct").unwrap()), @r"
        self.e := 0
        self.inner := (a := 1, b := 2, inner := (a := 3))
        self.inner2 := (d := 8, inner := (b := ADR(gVar)))
        self.inner3 := (inner(a := 9))
        ");
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
        END_TYPE
        "#;

        let initializer = parse_and_init(src);
        // Check for global constructor
        // Expecting a call to MyStruct_ctor(&gStructVar);
        insta::assert_snapshot!(print_to_string(&initializer.global_constructor), @r#""#);
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
        END_TYPE
        "#;

        let initializer = parse_and_init(src);
        // Check for function constructor on the stack
        // Expecting a call to MyStruct_ctor(&localStruct);
        insta::assert_snapshot!(print_body_to_string(initializer.stack_constructor.get("MyFunction").unwrap()), @r#""#);
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
        END_PROGRAM

        TYPE MyStruct : STRUCT
            a : INT := 5;
            b : BOOL := TRUE;
        END_STRUCT
        END_TYPE
        "#;

        let initializer = parse_and_init(src);
        // Check for program constructor on the stack
        // Expecting a call to MyStruct_ctor(&localStruct);
        insta::assert_snapshot!(print_body_to_string(initializer.stack_constructor.get("MyProgram").unwrap()), @r#""#);
        // Check for program constructor
        insta::assert_snapshot!(print_body_to_string(initializer.constructors.get("MyProgram").unwrap()), @r#""#);
    }

    #[test]
    fn programs_are_globals() {
        let src = r#"
        PROGRAM MyProgram
        VAR
        END_VAR
        END_PROGRAM
        "#;

        let initializer = parse_and_init(src);
        // Check for program constructor
        // Expecting a call to MyProgram_ctor(&progStruct);
        insta::assert_snapshot!(print_to_string(&initializer.global_constructor), @r#""#);
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
        END_TYPE
        "#;

        let initializer = parse_and_init(src);
        // Check for internal var assignment in global constructor
        // Expecting a call to MyExtStruct_ctor(&internalVar);
        // No call to MyExtStruct_ctor(&extVar);
        insta::assert_snapshot!(print_to_string(&initializer.global_constructor), @r#""#);
        // Check that no constructor is generated for MyExtStruct
        insta::assert_snapshot!(print_body_to_string(initializer.constructors.get("MyExtStruct").unwrap()), @"extern");
    }
}
