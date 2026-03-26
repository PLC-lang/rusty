//! # Reference To Return Lowering
//!
//! This module handles the lowering of functions that return a "REFERENCE TO" to hide additional processing
//! that creates a temporary variable and assigns the result to that instead so that no values are assigned
//! directly from the stack.

use plc_ast::{
    ast::{
        AccessModifier, ArgumentProperty, AstFactory, AstNode, AstStatement, AutoDerefType, CompilationUnit,
        DataType, DataTypeDeclaration, Implementation, LinkageType, Pou, PouType, ReferenceAccess,
        UserTypeDeclaration, Variable, VariableBlock,
    },
    control_statements::AstControlStatement,
    mut_visitor::{AstVisitorMut, WalkerMut},
    provider::IdProvider,
};
use plc_source::source_location::SourceLocation;

pub struct ReferenceToReturnParticipant {
    pub ids: IdProvider,
    pub pou_context: Vec<ReferenceToReturnPouContext>,
    pub implementation_context: Vec<ReferenceToReturnImplementationContext>,
}

impl ReferenceToReturnParticipant {
    pub fn new(ids: IdProvider) -> Self {
        Self { ids, pou_context: Vec::new(), implementation_context: Vec::new() }
    }

    pub fn lower_reference_to_return(&mut self, units: &mut [CompilationUnit]) {
        self.populate_pou_context(units);
        self.populate_implementation_context(units);

        let mut lowerer = ReferenceToReturnLowerer::new(
            self.ids.clone(),
            self.pou_context.clone(),
            self.implementation_context.clone(),
        );

        for unit in units {
            lowerer.visit_compilation_unit(unit);

            let mut new_user_types = vec![];
            for new_user_type in lowerer.new_user_types.drain(..) {
                new_user_types.push(new_user_type);
            }

            unit.user_types.append(&mut new_user_types);
        }

        self.ids = lowerer.ids.clone();
    }

    fn populate_pou_context(&mut self, units: &mut [CompilationUnit]) {
        for unit in units {
            for pou in &unit.pous {
                // For now only functions are supported
                if pou.kind == PouType::Function
                    && pou.return_type.as_ref().is_some_and(|return_type| match return_type {
                        DataTypeDeclaration::Definition { data_type, .. } => {
                            matches!(data_type.as_ref(), DataType::PointerType { .. })
                        }
                        _ => false,
                    })
                {
                    // Unwrap is safe as return type is evaluated by the check above
                    let return_type = pou.return_type.clone().unwrap();
                    let return_type_inner_type = match &return_type {
                        DataTypeDeclaration::Definition { data_type, .. } => match data_type.as_ref() {
                            DataType::PointerType { referenced_type, .. } => *referenced_type.clone(),
                            _ => unreachable!("must be a pointer type"),
                        },
                        _ => unreachable!("must be reference"),
                    };

                    self.pou_context.push(ReferenceToReturnPouContext {
                        pou_name: pou.name.to_string(),
                        return_type,
                        return_type_inner_type,
                    });
                }
            }
        }
    }

    fn populate_implementation_context(&mut self, units: &mut [CompilationUnit]) {
        for unit in units {
            for implementation in &unit.implementations {
                if let Some(calls) =
                    self.get_implementation_calls_to_pou_with_reference_to_return(implementation)
                {
                    let mut distinct_calls = Vec::new();
                    for call in calls {
                        if !distinct_calls.contains(&call) {
                            distinct_calls.push(call.to_string());
                        }
                    }

                    self.implementation_context.push(ReferenceToReturnImplementationContext {
                        implementation_name: implementation.name.to_string(),
                        calls_to_pous_with_reference_to_return: distinct_calls,
                    });
                }
            }
        }
    }

    fn get_implementation_calls_to_pou_with_reference_to_return(
        &self,
        implementation: &Implementation,
    ) -> Option<Vec<String>> {
        let mut calls: Vec<String> = Vec::new();
        for node in &implementation.statements {
            if let Some(result) = self.get_node_call_to_pou_with_reference_to_return(node) {
                calls.append(&mut result.clone());
            }
        }

        if calls.is_empty() {
            return None;
        }

        Some(calls)
    }

    fn get_node_call_to_pou_with_reference_to_return(&self, node: &AstNode) -> Option<Vec<String>> {
        let mut calls: Vec<String> = Vec::new();

        match &node.stmt {
            AstStatement::CallStatement(call_statement) => {
                if let Some(call_statement_name) = call_statement.operator.get_flat_reference_name() {
                    if self.pou_context.iter().any(|it| it.pou_name == call_statement_name) {
                        calls.push(call_statement_name.to_string());
                    }
                }
            }
            AstStatement::MultipliedStatement(multiplied_statement) => {
                if let Some(result) =
                    self.get_node_call_to_pou_with_reference_to_return(&multiplied_statement.element)
                {
                    calls.append(&mut result.clone());
                }
            }
            AstStatement::ReferenceExpr(reference_expr) => {
                match &reference_expr.access {
                    ReferenceAccess::Global(ast_node)
                    | ReferenceAccess::Member(ast_node)
                    | ReferenceAccess::Index(ast_node)
                    | ReferenceAccess::Cast(ast_node) => {
                        if let Some(result) = self.get_node_call_to_pou_with_reference_to_return(ast_node) {
                            calls.append(&mut result.clone());
                        }
                    }
                    _ => (),
                }

                if let Some(base) = &reference_expr.base {
                    if let Some(result) = self.get_node_call_to_pou_with_reference_to_return(base) {
                        calls.append(&mut result.clone());
                    }
                }
            }
            AstStatement::DirectAccess(direct_access) => {
                if let Some(result) = self.get_node_call_to_pou_with_reference_to_return(&direct_access.index)
                {
                    calls.append(&mut result.clone());
                }
            }
            AstStatement::HardwareAccess(hardware_access) => {
                for ast_node in &hardware_access.address {
                    if let Some(result) = self.get_node_call_to_pou_with_reference_to_return(ast_node) {
                        calls.append(&mut result.clone());
                    }
                }
            }
            AstStatement::BinaryExpression(binary_expression) => {
                if let Some(result) =
                    self.get_node_call_to_pou_with_reference_to_return(&binary_expression.left)
                {
                    calls.append(&mut result.clone());
                }

                if let Some(result) =
                    self.get_node_call_to_pou_with_reference_to_return(&binary_expression.right)
                {
                    calls.append(&mut result.clone());
                }
            }
            AstStatement::UnaryExpression(unary_expression) => {
                if let Some(result) =
                    self.get_node_call_to_pou_with_reference_to_return(&unary_expression.value)
                {
                    calls.append(&mut result.clone());
                }
            }
            AstStatement::ExpressionList(ast_nodes) => {
                for ast_node in ast_nodes {
                    if let Some(result) = self.get_node_call_to_pou_with_reference_to_return(ast_node) {
                        calls.append(&mut result.clone());
                    }
                }
            }
            AstStatement::ParenExpression(ast_node) => {
                if let Some(result) = self.get_node_call_to_pou_with_reference_to_return(ast_node) {
                    calls.append(&mut result.clone());
                }
            }
            AstStatement::RangeStatement(range_statement) => {
                if let Some(result) =
                    self.get_node_call_to_pou_with_reference_to_return(&range_statement.start)
                {
                    calls.append(&mut result.clone());
                }

                if let Some(result) = self.get_node_call_to_pou_with_reference_to_return(&range_statement.end)
                {
                    calls.append(&mut result.clone());
                }
            }
            AstStatement::Assignment(assignment)
            | AstStatement::OutputAssignment(assignment)
            | AstStatement::RefAssignment(assignment) => {
                if let Some(result) = self.get_node_call_to_pou_with_reference_to_return(&assignment.left) {
                    calls.append(&mut result.clone());
                }

                if let Some(result) = self.get_node_call_to_pou_with_reference_to_return(&assignment.right) {
                    calls.append(&mut result.clone());
                }
            }
            AstStatement::ControlStatement(ast_control_statement) => match ast_control_statement {
                AstControlStatement::If(if_statement) => {
                    for block in &if_statement.blocks {
                        if let Some(result) =
                            self.get_node_call_to_pou_with_reference_to_return(&block.condition)
                        {
                            calls.append(&mut result.clone());
                        }

                        for body_node in &block.body {
                            if let Some(result) =
                                self.get_node_call_to_pou_with_reference_to_return(body_node)
                            {
                                calls.append(&mut result.clone());
                            }
                        }
                    }

                    for else_node in &if_statement.else_block {
                        if let Some(result) = self.get_node_call_to_pou_with_reference_to_return(else_node) {
                            calls.append(&mut result.clone());
                        }
                    }
                }
                AstControlStatement::ForLoop(for_loop_statement) => {
                    if let Some(result) =
                        self.get_node_call_to_pou_with_reference_to_return(&for_loop_statement.counter)
                    {
                        calls.append(&mut result.clone());
                    }

                    if let Some(result) =
                        self.get_node_call_to_pou_with_reference_to_return(&for_loop_statement.start)
                    {
                        calls.append(&mut result.clone());
                    }

                    if let Some(result) =
                        self.get_node_call_to_pou_with_reference_to_return(&for_loop_statement.end)
                    {
                        calls.append(&mut result.clone());
                    }

                    if let Some(by_step) = &for_loop_statement.by_step {
                        if let Some(result) = self.get_node_call_to_pou_with_reference_to_return(by_step) {
                            calls.append(&mut result.clone());
                        }
                    }

                    for body_node in &for_loop_statement.body {
                        if let Some(result) = self.get_node_call_to_pou_with_reference_to_return(body_node) {
                            calls.append(&mut result.clone());
                        }
                    }
                }
                AstControlStatement::WhileLoop(loop_statement)
                | AstControlStatement::RepeatLoop(loop_statement) => {
                    if let Some(result) =
                        self.get_node_call_to_pou_with_reference_to_return(&loop_statement.condition)
                    {
                        calls.append(&mut result.clone());
                    }

                    for body_node in &loop_statement.body {
                        if let Some(result) = self.get_node_call_to_pou_with_reference_to_return(body_node) {
                            calls.append(&mut result.clone());
                        }
                    }
                }
                AstControlStatement::Case(case_statement) => {
                    if let Some(result) =
                        self.get_node_call_to_pou_with_reference_to_return(&case_statement.selector)
                    {
                        calls.append(&mut result.clone());
                    }

                    for block in &case_statement.case_blocks {
                        if let Some(result) =
                            self.get_node_call_to_pou_with_reference_to_return(&block.condition)
                        {
                            calls.append(&mut result.clone());
                        }

                        for body_node in &block.body {
                            if let Some(result) =
                                self.get_node_call_to_pou_with_reference_to_return(body_node)
                            {
                                calls.append(&mut result.clone());
                            }
                        }
                    }

                    for else_node in &case_statement.else_block {
                        if let Some(result) = self.get_node_call_to_pou_with_reference_to_return(else_node) {
                            calls.append(&mut result.clone());
                        }
                    }
                }
            },
            AstStatement::CaseCondition(ast_node) => {
                if let Some(result) = self.get_node_call_to_pou_with_reference_to_return(ast_node) {
                    calls.append(&mut result.clone());
                }
            }
            AstStatement::ReturnStatement(return_statement) => {
                if let Some(ast_node) = &return_statement.condition {
                    if let Some(result) = self.get_node_call_to_pou_with_reference_to_return(ast_node) {
                        calls.append(&mut result.clone());
                    }
                }
            }
            AstStatement::JumpStatement(jump_statement) => {
                if let Some(result) =
                    self.get_node_call_to_pou_with_reference_to_return(&jump_statement.condition)
                {
                    calls.append(&mut result.clone());
                }

                if let Some(result) =
                    self.get_node_call_to_pou_with_reference_to_return(&jump_statement.target)
                {
                    calls.append(&mut result.clone());
                }
            }
            _ => (),
        }

        if calls.is_empty() {
            return None;
        }

        Some(calls)
    }
}

pub struct ReferenceToReturnLowerer {
    ids: IdProvider,
    pou_context: Vec<ReferenceToReturnPouContext>,
    implementation_context: Vec<ReferenceToReturnImplementationContext>,
    new_user_types: Vec<UserTypeDeclaration>,
}

#[derive(Clone)]
pub struct ReferenceToReturnPouContext {
    pou_name: String,
    return_type: DataTypeDeclaration,
    return_type_inner_type: DataTypeDeclaration,
}

#[derive(Clone)]
pub struct ReferenceToReturnImplementationContext {
    implementation_name: String,
    calls_to_pous_with_reference_to_return: Vec<String>,
}

impl AstVisitorMut for ReferenceToReturnLowerer {
    fn visit_compilation_unit(&mut self, unit: &mut CompilationUnit) {
        unit.walk(self);
    }

    fn visit_pou(&mut self, pou: &mut Pou) {
        if self.pou_original_return_type_is_reference_to(&pou.name) {
            // Steal the return type (unwrap is safe because it absolutely must be present to enter this conditional block)
            let cloned_return_type = pou.return_type.clone().unwrap();
            pou.return_type = None;

            let location = SourceLocation::from(&cloned_return_type);

            // Construct the by reference variable used to fetch the return data
            let current_context = self.pou_context.iter().find(|it| it.pou_name == pou.name).unwrap();
            let return_type_inner_type = current_context.return_type_inner_type.clone();

            let new_data_type_name = format!("__{}{}", pou.name, self.get_return_variable_name_for_pou(pou));
            let new_data_type = DataTypeDeclaration::Reference {
                referenced_type: new_data_type_name.clone(),
                location: location.clone(),
            };

            let new_user_type = UserTypeDeclaration {
                data_type: DataType::PointerType {
                    name: Some(new_data_type_name),
                    referenced_type: Box::new(return_type_inner_type.clone()),
                    auto_deref: Some(AutoDerefType::Reference),
                    type_safe: true,
                    is_function: false,
                },
                initializer: None,
                location: location.clone(),
                scope: None,
                linkage: LinkageType::Internal,
            };

            self.new_user_types.push(new_user_type);

            // Construct the by reference variable used to return data
            let reference_variable = Variable {
                name: self.get_return_variable_name_for_pou(pou),
                data_type_declaration: new_data_type,
                initializer: None,
                address: None,
                location: location.clone(),
            };

            // If there is no input block by val
            if !pou.variable_blocks.iter().any(|it| it.is_input_by_val()) {
                // then we need to add one
                pou.variable_blocks.push(VariableBlock {
                    variables: Vec::new(),
                    kind: plc_ast::ast::VariableBlockType::Input(ArgumentProperty::ByVal),
                    constant: false,
                    retain: false,
                    linkage: plc_ast::ast::LinkageType::Internal,
                    location,
                    access: AccessModifier::Public,
                });
            }

            pou.variable_blocks.iter_mut().for_each(|block| {
                // Assumption is made that only one of these blocks exists
                if block.is_input_by_val() {
                    block.variables.insert(0, reference_variable.clone());
                }
            });
        }

        if let Some(calls) = self.get_all_pous_that_return_reference_to_for_implementation_of_pou(pou) {
            for call in calls {
                // Unwrap is safe, this context must exist
                let current_context = self.pou_context.iter().find(|it| it.pou_name == call).unwrap();
                let return_type_for_call = current_context.return_type.clone();
                let return_type_inner_type_for_call = current_context.return_type_inner_type.clone();
                let return_type_for_call_location = return_type_for_call.get_location().clone();

                // Construct the by reference variable used to fetch the return data
                let new_data_type_name = format!("__{}{}", pou.name, self.get_return_variable_name(&call));
                let new_data_type = DataTypeDeclaration::Reference {
                    referenced_type: new_data_type_name.clone(),
                    location: return_type_for_call_location.clone(),
                };

                let new_user_type = UserTypeDeclaration {
                    data_type: DataType::PointerType {
                        name: Some(new_data_type_name),
                        referenced_type: Box::new(return_type_inner_type_for_call.clone()),
                        auto_deref: Some(AutoDerefType::Reference),
                        type_safe: true,
                        is_function: false,
                    },
                    initializer: None,
                    location: return_type_for_call_location.clone(),
                    scope: None,
                    linkage: LinkageType::Internal,
                };

                self.new_user_types.push(new_user_type);

                let reference_variable = Variable {
                    name: self.get_return_variable_name(&call),
                    data_type_declaration: new_data_type,
                    initializer: None,
                    address: None,
                    location: return_type_for_call_location.clone(),
                };

                let reference_variable_store = Variable {
                    name: self.get_return_variable_name_store(&call),
                    data_type_declaration: return_type_inner_type_for_call,
                    initializer: None,
                    address: None,
                    location: return_type_for_call_location.clone(),
                };

                // If there is no input block by val
                if !pou.variable_blocks.iter().any(|it| it.is_temp()) {
                    // then we need to add one
                    pou.variable_blocks.push(VariableBlock {
                        variables: Vec::new(),
                        kind: plc_ast::ast::VariableBlockType::Temp,
                        constant: false,
                        retain: false,
                        linkage: plc_ast::ast::LinkageType::Internal,
                        location: return_type_for_call_location.clone(),
                        access: AccessModifier::Private,
                    });
                }

                pou.variable_blocks.iter_mut().for_each(|block| {
                    // Assumption is made that only one of these blocks exists
                    if block.is_temp() {
                        block.variables.push(reference_variable.clone());
                        block.variables.push(reference_variable_store.clone());
                    }
                });
            }
        }

        pou.walk(self);
    }

    fn visit_implementation(&mut self, implementation: &mut Implementation) {
        // Handle the implementation changes for the function with "reference to" as return
        if self.pou_original_return_type_is_reference_to(&implementation.name)
            && !implementation.statements.is_empty()
        {
            let last_index = implementation.statements.len() - 1;
            let last_statement = &implementation.statements[last_index];
            let (replacement_statement, should_replace) = match &last_statement.stmt {
                AstStatement::RefAssignment(assignment) => {
                    let mut replacement_statement = last_statement.clone();
                    let mut replacement_assignment = assignment.clone();

                    let member_reference = AstFactory::create_member_reference(
                        AstFactory::create_identifier(
                            self.get_return_variable_name_for_implementation(implementation),
                            assignment.left.location.clone(),
                            self.ids.next_id(),
                        ),
                        None,
                        self.ids.next_id(),
                    );

                    replacement_assignment.left = Box::new(member_reference);
                    replacement_statement.stmt = AstStatement::Assignment(replacement_assignment);

                    (replacement_statement, true)
                }
                _ => {
                    // TODO: If the last statement is not a reference assignment
                    // then we _probably_ need to construct the reference assignment based on
                    // the last assigned value
                    (last_statement.clone(), false)
                }
            };

            if should_replace {
                implementation.statements[last_index] = replacement_statement;
            }
            // TODO: Push the last statement if it isn't a replacement
            // else {
            //     implementation.statements.push(replacement_statement);
            // }
        }

        // Handle the implementation changes for implementations that invoke "reference to" functions
        let mut replacement_statements_container: Vec<(usize, Vec<AstNode>)> = Vec::new();
        for (index, node) in implementation.statements.iter().enumerate() {
            let (return_assignment, call_statement) = match &node.stmt {
                AstStatement::RefAssignment(assignment) => {
                    // We only care about call statements that reference a function contained in the contextual list
                    let call_statement = self
                        .get_call_statement_clone_if_referenced_pou_is_reference_to_return(
                            assignment.right.get_node_peeled(),
                        );
                    let left_side_assignment = assignment.left.as_ref().clone();
                    (Some(left_side_assignment), call_statement)
                }
                // TODO: Call statements that don't double back for references should still do this, otherwise the function signatures wont match
                AstStatement::CallStatement(..) => {
                    let call_statement =
                        self.get_call_statement_clone_if_referenced_pou_is_reference_to_return(node);
                    (None, call_statement)
                }
                _ => (None, None),
            };

            if let Some(call_statement) = call_statement {
                // Unwrap is safe, this must have a name
                let call_statement_name = call_statement
                    .get_call_operator()
                    .unwrap()
                    .get_flat_reference_name()
                    .unwrap()
                    .to_string();

                // Construct the new parameter
                let param = AstFactory::create_member_reference(
                    AstFactory::create_identifier(
                        self.get_return_variable_name(&call_statement_name),
                        node.location.clone(),
                        self.ids.next_id(),
                    ),
                    None,
                    self.ids.next_id(),
                );

                let call_statement = match call_statement.stmt {
                    AstStatement::CallStatement(temp_call_statement) => {
                        let parameters = if let Some(expression_list) = temp_call_statement.parameters {
                            match expression_list.stmt {
                                AstStatement::ExpressionList(mut expressions) => {
                                    expressions.insert(0, param);
                                    AstFactory::create_expression_list(
                                        expressions,
                                        call_statement.location.clone(),
                                        self.ids.next_id(),
                                    )
                                }
                                _ => AstFactory::create_expression_list(
                                    vec![param, *expression_list],
                                    call_statement.location.clone(),
                                    self.ids.next_id(),
                                ),
                            }
                        } else {
                            AstFactory::create_expression_list(
                                vec![param],
                                call_statement.location.clone(),
                                self.ids.next_id(),
                            )
                        };

                        AstFactory::create_call_statement(
                            *temp_call_statement.operator,
                            Some(parameters),
                            call_statement.id,
                            call_statement.location,
                        )
                    }
                    _ => unreachable!("this must be a call statement"),
                };

                // Split the assignment process into three statements and return a list statement
                let mut expressions = Vec::new();

                // Construct the temp variable assignment
                let left = AstFactory::create_member_reference(
                    AstFactory::create_identifier(
                        self.get_return_variable_name(&call_statement_name),
                        node.location.clone(),
                        self.ids.next_id(),
                    ),
                    None,
                    self.ids.next_id(),
                );

                let right = AstFactory::create_member_reference(
                    AstFactory::create_identifier(
                        self.get_return_variable_name_store(&call_statement_name),
                        node.location.clone(),
                        self.ids.next_id(),
                    ),
                    None,
                    self.ids.next_id(),
                );

                let ref_assignment = AstFactory::create_ref_assignment(left, right, self.ids.next_id());
                expressions.push(ref_assignment);
                expressions.push(call_statement);

                // We only push the follow up assignment if it was assigned to in the first place
                if let Some(return_assignment) = return_assignment {
                    let right = AstFactory::create_member_reference(
                        AstFactory::create_identifier(
                            self.get_return_variable_name(&call_statement_name),
                            node.location.clone(),
                            self.ids.next_id(),
                        ),
                        None,
                        self.ids.next_id(),
                    );

                    let ref_assignment =
                        AstFactory::create_ref_assignment(return_assignment, right, self.ids.next_id());
                    expressions.push(ref_assignment);
                }

                replacement_statements_container.push((index, expressions));
            }
        }

        let mut new_statements = Vec::new();
        for (index, statement) in implementation.statements.iter().enumerate() {
            if let Some((_, replacement_statements)) =
                replacement_statements_container.iter().find(|it| it.0 == index)
            {
                for replacement_statement in replacement_statements {
                    new_statements.push(replacement_statement.clone());
                }
            } else {
                new_statements.push(statement.clone());
            }
        }
        implementation.statements = new_statements;

        implementation.walk(self);
    }
}

impl ReferenceToReturnLowerer {
    pub fn new(
        ids: IdProvider,
        pou_context: Vec<ReferenceToReturnPouContext>,
        implementation_context: Vec<ReferenceToReturnImplementationContext>,
    ) -> Self {
        Self { ids, pou_context, implementation_context, new_user_types: Vec::new() }
    }

    fn pou_original_return_type_is_reference_to(&self, name: &str) -> bool {
        self.pou_context.iter().any(|it| it.pou_name == name)
    }

    fn get_all_pous_that_return_reference_to_for_implementation_of_pou(
        &mut self,
        pou: &Pou,
    ) -> Option<Vec<String>> {
        if let Some(implementation_with_to_reference_to_pous) =
            self.implementation_context.iter().find(|it| it.implementation_name == pou.name)
        {
            return Some(
                implementation_with_to_reference_to_pous.calls_to_pous_with_reference_to_return.clone(),
            );
        }

        None
    }

    fn get_return_variable_name_for_pou(&self, pou: &Pou) -> String {
        self.get_return_variable_name(&pou.name)
    }

    fn get_return_variable_name_for_implementation(&self, implementation: &Implementation) -> String {
        self.get_return_variable_name(&implementation.name)
    }

    fn get_return_variable_name(&self, name: &str) -> String {
        format!("__{}_return_val", name)
    }

    fn get_return_variable_name_store(&self, name: &str) -> String {
        format!("{}_store", self.get_return_variable_name(name))
    }

    fn get_call_statement_clone_if_referenced_pou_is_reference_to_return(
        &self,
        node: &AstNode,
    ) -> Option<AstNode> {
        match &node.stmt {
            AstStatement::CallStatement(call_statement) => {
                if let Some(call_statement_name) = call_statement.operator.get_flat_reference_name() {
                    if self.pou_original_return_type_is_reference_to(call_statement_name) {
                        return Some(node.clone());
                    }
                }

                None
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use insta::assert_snapshot;
    use plc_ast::{
        ast::{ArgumentProperty, VariableBlockType},
        ser::AstSerializer,
    };
    use plc_driver::parse_and_annotate;
    use plc_source::SourceCode;

    #[test]
    fn reference_to_function_is_lowered_to_void_with_temporary_return() {
        let src: SourceCode = r#"
            FUNCTION referenceFunc : REFERENCE TO INT
                VAR_INPUT
                    in	: REFERENCE TO INT;
                END_VAR
                in := in + 1;
                referenceFunc REF= in;
            END_FUNCTION

            FUNCTION main
                VAR
                    refVal : REFERENCE TO INT;
                    conVal : INT;
                    tmpVal : INT;
                END_VAR
                tmpVal := 11;
                refVal REF= tmpVal;
                refVal REF= referenceFunc(refVal);
                conVal := refVal;
            END_FUNCTION
            "#
        .into();

        /*
            Lowered code:
            ```ST
            FUNCTION referenceFunc
                VAR_INPUT
                    __referenceFunc_return_val : REFERENCE TO INT;
                    in	: REFERENCE TO INT;
                END_VAR
                in := in + 1;
                __referenceFunc_return_val := in;
            END_FUNCTION

            FUNCTION main
                VAR
                    refVal : REFERENCE TO INT;
                    conVal : INT;
                    tmpVal : INT;
                END_VAR
                VAR_TEMP
                    __referenceFunc_return_val : REFERENCE TO INT;
                    __referenceFunc_return_val_store : INT;
                END_VAR
                tmpVal := 11;
                refVal REF= tmpVal;
                __referenceFunc_return_val REF= __referenceFunc_return_val_store;
                referenceFunc(__referenceFunc_return_val, refVal);
                refVal REF= __referenceFunc_return_val;
                conVal := refVal;
            END_FUNCTION
            ```
        */

        let (_, project) = parse_and_annotate("test", vec![src]).unwrap();
        let unit = &project.units[0].get_unit();
        let pous = &unit.pous;
        let implementations = &unit.implementations;

        // 1. Function "referenceFunc" should now have no return type after lowering
        let ref_func_pou =
            pous.iter().find(|i| i.name == "referenceFunc").expect("referenceFunc pou should exist");

        assert_eq!(ref_func_pou.return_type, None);

        // 2. Function "referenceFunc" should have a new `VAR_IN_OUT` variable: `__referenceFunc_return_val`
        let ref_func_var_in_out_block = ref_func_pou
            .variable_blocks
            .iter()
            .find(|p| p.kind == VariableBlockType::Input(ArgumentProperty::ByVal))
            .expect("referenceFunc inout block should exist");

        assert_snapshot!(AstSerializer::format_variable_block(ref_func_var_in_out_block, unit), @"
        VAR_INPUT
            __referenceFunc_return_val : REFERENCE TO INT;
            in : REFERENCE TO INT;
        END_VAR
        ");

        // 3. Function "referenceFunc" should now assign the return value to `__referenceFunc_return_val` as the final statement
        let ref_func_implementation = implementations
            .iter()
            .find(|i| i.name == "referenceFunc")
            .expect("referenceFunc implementation should exist");

        let ref_func_ret_statement =
            &ref_func_implementation.statements[ref_func_implementation.statements.len() - 1];
        assert_snapshot!(AstSerializer::format(ref_func_ret_statement), @"__referenceFunc_return_val := in");

        // 4. Function "main" should now have a new `VAR_TEMP` variable: `__referenceFunc_return_val`
        let main_pou = pous.iter().find(|i| i.name == "main").expect("main pou should exist");

        let main_temp_block = main_pou
            .variable_blocks
            .iter()
            .find(|p| p.kind == VariableBlockType::Temp)
            .expect("main temp block should exist");

        assert_snapshot!(AstSerializer::format_variable_block(main_temp_block, unit), @"
        VAR_TEMP
            __referenceFunc_return_val : REFERENCE TO INT;
            __referenceFunc_return_val_store : INT;
        END_VAR
        ");

        // 5. Function "main" should lower `refVal REF= referenceFunc(refVal);`
        // Into three statements:
        //   `__referenceFunc_return_val REF= __referenceFunc_return_val_store;`
        //   `referenceFunc(__referenceFunc_return_val, refVal);`
        //   `refVal REF= __referenceFunc_return_val;`
        let main_implementation =
            implementations.iter().find(|i| i.name == "main").expect("main implementation should exist");

        let ref_eq_call_func_statement = &main_implementation.statements;
        assert_snapshot!(AstSerializer::format_nodes(ref_eq_call_func_statement), @"
        __main_refVal__ctor(refVal);
        __main__referenceFunc_return_val__ctor(__referenceFunc_return_val);
        tmpVal := 11;
        refVal REF= tmpVal;
        __referenceFunc_return_val REF= __referenceFunc_return_val_store;
        referenceFunc(__referenceFunc_return_val, refVal);
        refVal REF= __referenceFunc_return_val;
        conVal := refVal;
        ");
    }
}
