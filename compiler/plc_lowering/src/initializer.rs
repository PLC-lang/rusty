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
//!   constructor.

use std::{any::Any, rc::Rc};

use plc::{
    index::{FxIndexMap, Index},
    lowering::helper::{
        create_assignment, create_assignment, create_call_statement, create_call_statement,
        create_member_reference, create_member_reference, get_unit_name, new_constructor, new_constructor,
        new_global_constructor, new_unit_constructor,
    },
};
use plc_ast::{
    ast::{AstFactory, AstNode, CompilationUnit, LinkageType, PouType, VariableBlockType},
    provider::IdProvider,
    visitor::{AstVisitor, Walker},
};
use plc_source::source_location::SourceLocation;

#[derive(Debug, PartialEq)]
enum Body {
    Internal(Vec<AstNode>),
    External(Vec<AstNode>),
    None,
}

#[derive(Debug)]
pub struct Initializer {
    pub id_provider: IdProvider,
    index: Option<Rc<Index>>,
    /// Stateful constructor per POU/struct
    constructors: FxIndexMap<String, Body>,
    /// User defined constructor names per POU/datatype
    user_defined_constructors: FxIndexMap<String, String>,
    /// Constructors for temp and stack variables per POU
    stack_constructor: FxIndexMap<String, Body>,
    /// Global constructor statements
    global_constructor: Vec<AstNode>,
    context: Context,
}

#[derive(Default, Debug)]
struct Context {
    current_pou: Option<String>,
    current_datatype: Option<String>,
    current_variable_block: Option<VariableBlockType>,
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

impl AstVisitor for Initializer {
    fn visit_pou(&mut self, pou: &plc_ast::ast::Pou) {
        self.context.enter_pou(pou.name.as_str());
        self.user_defined_constructors.insert(pou.name.clone(), "FB_INIT".to_string());
        match pou.linkage {
            plc_ast::ast::LinkageType::Internal => {
                if pou.is_stateful() {
                    self.constructors.insert(pou.name.clone(), Body::Internal(vec![]));
                }
                self.stack_constructor.insert(pou.name.clone(), Body::Internal(vec![]));
            }
            plc_ast::ast::LinkageType::External => {
                if pou.is_stateful() {
                    self.constructors.insert(pou.name.clone(), Body::External(vec![]));
                }
                self.stack_constructor.insert(pou.name.clone(), Body::External(vec![]));
            }
            plc_ast::ast::LinkageType::BuiltIn => {
                self.constructors.insert(pou.name.clone(), Body::None);
                return;
            }
        };
        // If the POU is a function block or a class, add an assignment to the vtable
        if pou.is_function_block() || pou.is_class() {
            let vtable_assignment = create_assignment(
                "self.__vtable",
                None,
                &create_member_reference(
                    &format!("ADR(__vtable_{}_instance)", pou.name),
                    self.id_provider.clone(),
                    None,
                ),
                self.id_provider.clone(),
            );
            self.add_to_current_constructor(vec![vtable_assignment]);
        }
        pou.walk(self);
        // call the user defined constructor here
        if let Some(user_defined_ctor_call) = self.get_user_defined_constructor_call(&pou.name, "self") {
            self.add_to_current_constructor(vec![user_defined_ctor_call]);
        }
        // If a program, add a constructor call to the global variables
        self.context.exit_pou();
    }

    fn visit_variable_block(&mut self, block: &plc_ast::ast::VariableBlock) {
        self.context.enter_variable_block(block);
        block.walk(self);
        self.context.exit_variable_block();
    }

    fn visit_variable(&mut self, variable: &plc_ast::ast::Variable) {
        // grab the index
        let index = self.index.as_ref().expect("index is set at this stage");
        let variable_block_type =
            self.context.current_variable_block.expect("variable block is set at this stage");
        // Find if the parent is stateful
        let is_stateful = if let Some(pou_name) = &self.context.current_pou {
            index.find_pou(pou_name).is_some_and(|it| it.is_stateful())
        } else {
            true
        };
        if let Some(initializer) = &variable.initializer {
            let mut stmts = vec![];
            // Create a call to the type constructor
            if let Some(constructor) = variable
                .data_type_declaration
                .get_referenced_type()
                .and_then(|it| self.get_constructor_call(it, variable.get_name()))
            {
                stmts.push(constructor);
            }
            // Create an assignment "self.<variable.name> := <initializer>"
            let assignment =
                create_assignment(variable.get_name(), Some("self"), initializer, self.id_provider.clone());
            stmts.push(assignment);
            if variable_block_type.is_temp() || (variable_block_type.is_local() && !is_stateful) {
                self.add_to_current_stack_constructor(stmts);
            } else {
                self.add_to_current_constructor(stmts);
            }
        }
    }

    fn visit_user_type_declaration(&mut self, user_type: &plc_ast::ast::UserTypeDeclaration) {
        let name = user_type.data_type.get_name().expect("name is set at this stage");
        self.context.enter_datatype(name);
        match user_type.linkage {
            plc_ast::ast::LinkageType::Internal => {
                self.constructors.insert(name.to_string(), Body::Internal(vec![]));
            }
            plc_ast::ast::LinkageType::External => {
                self.constructors.insert(name.to_string(), Body::External(vec![]));
            }
            plc_ast::ast::LinkageType::BuiltIn => {
                self.constructors.insert(name.to_string(), Body::None);
                return;
            }
        };
        self.visit_data_type(&user_type.data_type);
        let mut stmts = vec![];
        // TODO: call the user defined constructor here, don't know yet how we find it
        // if let Some(user_defined_ctor_call) = self.get_user_defined_constructor_call(name, "self") {
        //     stmts.push(user_defined_ctor_call);
        // }

        // Explicitly add the initializer call here
        if let Some(initializer) = &user_type.initializer {
            let assignment = create_assignment("self", None, initializer, self.id_provider.clone());
            stmts.push(assignment);
        }
        self.add_to_current_constructor(stmts);
        self.context.exit_datatype();
    }

    fn visit_data_type(&mut self, data_type: &plc_ast::ast::DataType) {
        // Only structs get constructors
        if let plc_ast::ast::DataType::StructType { variables, .. } = data_type {
            let mut constructor = vec![];
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
            self.add_to_current_constructor(constructor);
        }
    }
}

impl Initializer {
    pub fn new(id_provider: IdProvider) -> Initializer {
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

    /// Visits the given unit and collects all initialization logic, then applies it to the unit
    /// Adds a new constructor for each struct/POU with initialization logic
    /// Adds constructor calls for stack variables to each function
    /// Adds a global constructor function with global variable initializations
    pub fn apply_initialization(mut self, mut unit: CompilationUnit, index: Rc<Index>) -> CompilationUnit {
        // Set the index
        self.index = Some(index);
        self.visit_compilation_unit(&unit);
        self.index = None;
        // Add each constructor function to the unit as a new function
        for (name, body) in self.constructors {
            match body {
                Body::Internal(nodes) => {
                    let (pou, implementation) = new_constructor(
                        &name,
                        LinkageType::Internal,
                        PouType::Init,
                        nodes,
                        self.id_provider.clone(),
                    );
                    unit.pous.push(pou);
                    unit.implementations.push(implementation);
                }
                Body::External(nodes) => {
                    let (pou, implementation) = new_constructor(
                        &name,
                        LinkageType::External,
                        PouType::Init,
                        nodes,
                        self.id_provider.clone(),
                    );
                    unit.pous.push(pou);
                    unit.implementations.push(implementation);
                }
                Body::None => {}
            }
        }
        // Add stack constructor calls to each function and pou (VAR_TEMP)
        for (pou_name, body) in self.stack_constructor {
            match body {
                Body::Internal(mut nodes) | Body::External(mut nodes) => {
                    if let Some(implementation) =
                        unit.implementations.iter_mut().find(|it| it.name == pou_name)
                    {
                        implementation.statements.splice(0..0, nodes);
                    }
                }
                Body::None => {}
            }
        }
        // Add a global constructor function with the global constructor calls
        if !self.global_constructor.is_empty() {
            let unit_name = get_unit_name(&unit);
            let (pou, implementation) =
                new_unit_constructor(&unit_name, self.global_constructor, self.id_provider.clone());
            unit.pous.push(pou);
            unit.implementations.push(implementation);
        }
        unit
    }

    fn add_to_current_stack_constructor(&mut self, node: Vec<AstNode>) {
        if let Some(current_pou) = self.context.current_pou.as_ref() {
            if let Some(Body::Internal(nodes) | Body::External(nodes)) =
                self.stack_constructor.get_mut(current_pou)
            {
                nodes.extend(node);
            }
        }
    }

    fn add_to_current_constructor(&mut self, node: Vec<AstNode>) {
        if let Some(current_struct) =
            self.context.current_pou.as_ref().or(self.context.current_datatype.as_ref())
        {
            if let Some(Body::Internal(nodes) | Body::External(nodes)) =
                self.constructors.get_mut(current_struct)
            {
                nodes.extend(node)
            }
        } else if self.context.current_variable_block.is_some_and(|it| it.is_global()) {
            // Global constructor
            self.global_constructor.extend(node);
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

    fn get_user_defined_constructor_call(&mut self, type_name: &str, var_name: &str) -> Option<AstNode> {
        //TODO: datatypes work differently if they support user defined init
        if let Some(index) = self.index.as_ref() {
            // Search for the user defined constructor for the given struct
            if let Some(user_defined_ctor_name) = self.user_defined_constructors.get(type_name) {
                if let Some(pou) = index.find_method(type_name, user_defined_ctor_name) {
                    let op = create_member_reference(
                        &format!("{var_name}.{user_defined_ctor_name}"),
                        self.id_provider.clone(),
                        None,
                    );
                    let call = AstFactory::create_call_statement(
                        op,
                        None,
                        self.id_provider.next_id(),
                        SourceLocation::internal(),
                    );
                    return Some(call);
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use plc::lowering::vtable::VirtualTableGenerator;
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
            super::Body::Internal(nodes) => {
                if nodes.is_empty() {
                    "".into()
                } else {
                    format!(
                        r"intern:
{}",
                        print_to_string(nodes)
                    )
                }
            }
            super::Body::External(nodes) => {
                if nodes.is_empty() {
                    "".into()
                } else {
                    format!(
                        r"extern:
{}",
                        print_to_string(nodes)
                    )
                }
            }
            super::Body::None => "none".to_string(),
        }
    }

    fn parse_and_init(src: &str) -> Initializer {
        let src: SourceCode = src.into();
        let diagnostician = Diagnostician::buffered();
        let mut pipeline = BuildPipeline::from_sources("test.st", vec![(src)], diagnostician).unwrap();
        pipeline.register_mut_participants(vec![Box::new(VirtualTableGenerator::new(
            pipeline.context.provider(),
        ))]);
        let AnnotatedProject { units, index, .. } = pipeline.parse_and_annotate().unwrap();
        // Visit the AST with the Initializer
        let mut initializer = super::Initializer::new(pipeline.context.provider());
        initializer.index = Some(Rc::new(index));
        for unit in units {
            initializer.visit_compilation_unit(unit.get_unit());
        }
        initializer.index = None;
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
        intern:
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
        intern:
        self.x := 10
        self.y := 20
        ");
        // Expecting a function declaration: void OuterStruct_ctor(OuterStruct* self)
        // Expecting a call to InnerStruct_ctor(&self->inner);
        // Expecting an assignment: self->z = 2.71;
        insta::assert_snapshot!(print_body_to_string(initializer.constructors.get("OuterStruct").unwrap()), @r"
        intern:
        self.z := 2.71
        ");
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
        intern:
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
        intern:
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
        intern:
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
        insta::assert_snapshot!(print_body_to_string(initializer.constructors.get("MyEnum").unwrap()), @r"
        intern:
        self := Option3
        ");
        // Expecting a function declaration: void MyStruct_ctor(MyStruct* self)
        // Expecting a call to MyEnum_ctor(&self->e);
        // Expecting an assignment: self->e = 1;
        // Expecting an assignment: self->n = 42;
        insta::assert_snapshot!(print_body_to_string(initializer.constructors.get("MyStruct").unwrap()), @r"
        intern:
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
        intern:
        self.a := 1
        self.b := ADR(gVar)
        ");
        // Expecting a function declaration: void InnerStruct2_ctor(InnerStruct2* self)
        // Expecting a call to InnerStruct_ctor(&self->inner);
        // Expecting a call to InnerStruct_ctor(&self->inner2);
        // Expecting assignments: self->c = 4; self->d = 5;
        // Expecting assignments: self->inner.a = 6; self->inner2.b = &gvar;
        insta::assert_snapshot!(print_body_to_string(initializer.constructors.get("InnerStruct2").unwrap()), @r"
        intern:
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
        intern:
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
        insta::assert_snapshot!(print_to_string(&initializer.global_constructor), @"self.gVar1 := 10");
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
        insta::assert_snapshot!(print_body_to_string(initializer.stack_constructor.get("MyFunction").unwrap()), @"");
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
        insta::assert_snapshot!(print_body_to_string(initializer.stack_constructor.get("MyProgram").unwrap()), @"");
        // Check for program constructor
        insta::assert_snapshot!(print_body_to_string(initializer.constructors.get("MyProgram").unwrap()), @"");
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
        insta::assert_snapshot!(print_body_to_string(initializer.constructors.get("MyExtStruct").unwrap()), @r"
        extern:
        self.a := 5
        self.b := TRUE
        ");
    }

    #[test]
    fn function_block_fb_init_called_in_constructor() {
        let src = r#"
        FUNCTION_BLOCK MyFB
        VAR
            fbVar : INT;
        END_VAR
        METHOD FB_INIT : VOID
            fbVar := 1;
        END_METHOD
        END_FUNCTION_BLOCK
        "#;
        let initializer = parse_and_init(src);
        // Check for FB_INIT call in constructor
        // Expecting a call to MyFB.FB_INIT(&self);
        insta::assert_snapshot!(print_body_to_string(initializer.constructors.get("MyFB").unwrap()), @r"
        intern:
        self.FB_INIT()
        ");
    }

    #[test]
    fn function_block_inheritance_chain_called_in_constructor() {
        let src = r#"
        FUNCTION_BLOCK MyBaseFB
        VAR
            baseVar : INT := 5;
        END_VAR
        METHOD baseMeth
        END_METHOD
        METHOD overrideMeth
        END_METHOD
        END_FUNCTION_BLOCK
        FUNCTION_BLOCK MyFB EXTENDS MyBaseFB
        VAR
            fbVar : INT := 10;
        END_VAR
        METHOD FB_INIT : VOID
            fbVar := 1;
        END_METHOD
        METHOD overrideMeth
        END_METHOD
        METHOD localMeth
        END_METHOD
        END_FUNCTION_BLOCK
        "#;
        let initializer = parse_and_init(src);
        // Check that there's a constructor function for MyBaseFB
        insta::assert_snapshot!(print_body_to_string(initializer.constructors.get("MyBaseFB").unwrap()), @r"
        intern:
        self.baseVar := 5
        ");
        // Check that the constructor for MyFB calls the base FB constructor
        insta::assert_snapshot!(print_body_to_string(initializer.constructors.get("MyFB").unwrap()), @r"
        intern:
        self.fbVar := 10
        self.FB_INIT()
        ");
        // Check that there's a constructor function for the vtables
        insta::assert_snapshot!(print_body_to_string(initializer.constructors.get("__vtable_MyBaseFB").unwrap()), @r"
        intern:
        self.__body := ADR(MyBaseFB)
        self.baseMeth := ADR(MyBaseFB.baseMeth)
        self.overrideMeth := ADR(MyBaseFB.overrideMeth)
        ");
        insta::assert_snapshot!(print_body_to_string(initializer.constructors.get("__vtable_MyFB").unwrap()), @r"
        intern:
        self.__body := ADR(MyFB)
        self.baseMeth := ADR(MyBaseFB.baseMeth)
        self.overrideMeth := ADR(MyFB.overrideMeth)
        self.FB_INIT := ADR(MyFB.FB_INIT)
        self.localMeth := ADR(MyFB.localMeth)
        ");
    }
}
