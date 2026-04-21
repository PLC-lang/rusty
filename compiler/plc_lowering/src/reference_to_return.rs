//! # Reference To Return Lowering
//!
//! This module handles the lowering of functions that return a "REFERENCE TO" to hide additional processing
//! that creates a temporary variable and assigns the result to that instead so that no values are assigned
//! directly from the stack.
//!
//! The lowering mechanism works as follows:
//! 1. Functions with a `REFERENCE TO` return type will be lowered to a `VOID` return type.
//! 2. Functions with a `REFERENCE TO` return type will be given a new `VAR_INPUT` variable:
//!    `__{function_name}_return_val` that any return value of the function will be assigned to.
//! 3. Any POU that invokes a function with a `REFERENCE TO` return type will receive a new temporary
//!    variable that is then passed as the first parameter to the function.
//! 4. Any POU that invokes a function with a `REFERENCE TO` return type will then lower the call
//!    to that function from:
//!     ```ST
//!         refVal REF= exampleFunc(refVal);
//!     ```
//!     to the following:
//!     ```ST
//!         __exampleFunc_return_val_1 REF= __exampleFunc_return_val_store_1;
//!         exampleFunc(__exampleFunc_return_val_1, refVal);
//!         refVal REF= __exampleFunc_return_val_1;
//!     ```

use plc_ast::{
    ast::{
        AccessModifier, ArgumentProperty, AstFactory, AstNode, AstStatement, AutoDerefType, CallStatement,
        CompilationUnit, DataType, DataTypeDeclaration, Implementation, LinkageType, Pou, PouType,
        UserTypeDeclaration, Variable, VariableBlock,
    },
    mut_visitor::{AstVisitorMut, WalkerMut},
    provider::IdProvider,
};
use plc_source::source_location::SourceLocation;

pub struct ReferenceToReturnParticipant {
    pub ids: IdProvider,
}

impl ReferenceToReturnParticipant {
    pub fn new(ids: IdProvider) -> Self {
        Self { ids }
    }

    pub fn lower_reference_to_return(&mut self, units: &mut [CompilationUnit]) {
        // Pre-populate context with gathered setup information
        let mut context_gatherer = ReferenceToReturnContextGatherer::new();
        for unit in &mut *units {
            context_gatherer.visit_compilation_unit(unit);
        }

        // Perform the lowering process
        let mut lowerer = ReferenceToReturnLowerer::new(
            self.ids.clone(),
            context_gatherer.pou_context,
            context_gatherer.implementation_context,
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
}

pub struct ReferenceToReturnLowerer {
    ids: IdProvider,
    pou_context: Vec<ReferenceToReturnPouContext>,
    implementation_context: Vec<ReferenceToReturnImplementationContext>,
    new_user_types: Vec<UserTypeDeclaration>,
    current_implementation_call_context: ReferenceToReturnImplementationCallContext,
    current_implementation_pou_context: ReferenceToReturnImplementationPouContext,
}

struct ReferenceToReturnContextGatherer {
    pou_context: Vec<ReferenceToReturnPouContext>,
    implementation_context: Vec<ReferenceToReturnImplementationContext>,
    current_implementation_context: Option<ReferenceToReturnImplementationContext>,
}

impl ReferenceToReturnContextGatherer {
    pub fn new() -> Self {
        Self {
            pou_context: Vec::new(),
            implementation_context: Vec::new(),
            current_implementation_context: None,
        }
    }
}

impl Default for ReferenceToReturnContextGatherer {
    fn default() -> Self {
        Self::new()
    }
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
    calls_to_pous_with_reference_to_return: Vec<ReferenceToReturnImplementationContextCalls>,
}

#[derive(Clone)]
pub struct ReferenceToReturnImplementationContextCalls {
    name: String,
    count: usize,
}

pub struct ReferenceToReturnImplementationCallContext {
    left_side_assignment: Option<AstNode>,
    pre_statements: Vec<AstNode>,
    post_statements: Vec<AstNode>,
    in_context: bool,
    in_call_stack: bool,
    in_deep_call_stack: bool,
    used_call_states: Vec<ReferenceToReturnImplementationContextCalls>,
}

impl ReferenceToReturnImplementationCallContext {
    pub fn new() -> Self {
        Self {
            left_side_assignment: None,
            pre_statements: Vec::new(),
            post_statements: Vec::new(),
            in_context: false,
            in_call_stack: false,
            in_deep_call_stack: false,
            used_call_states: Vec::new(),
        }
    }
}

impl Default for ReferenceToReturnImplementationCallContext {
    fn default() -> Self {
        Self::new()
    }
}

pub struct ReferenceToReturnImplementationPouContext {
    in_context: bool,
}

impl ReferenceToReturnImplementationPouContext {
    pub fn new() -> Self {
        Self { in_context: false }
    }
}

impl Default for ReferenceToReturnImplementationPouContext {
    fn default() -> Self {
        Self::new()
    }
}

impl AstVisitorMut for ReferenceToReturnContextGatherer {
    fn visit_compilation_unit(&mut self, unit: &mut CompilationUnit) {
        unit.walk(self);
    }

    fn visit_pou(&mut self, pou: &mut Pou) {
        pou.walk(self);

        if pou.kind == PouType::Function
            && pou.return_type.as_ref().is_some_and(|return_type| match return_type {
                DataTypeDeclaration::Definition { data_type, .. } => {
                    matches!(
                        data_type.as_ref(),
                        DataType::PointerType { auto_deref: Some(AutoDerefType::Reference), .. }
                    )
                }
                _ => false,
            })
        {
            let return_type = pou.return_type.clone().expect("return type is evaluated by the check above");
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

    fn visit_implementation(&mut self, implementation: &mut Implementation) {
        self.current_implementation_context = Some(ReferenceToReturnImplementationContext {
            implementation_name: implementation.name.to_string(),
            calls_to_pous_with_reference_to_return: Vec::new(),
        });

        implementation.walk(self);

        if let Some(current_implementation_context) = &self.current_implementation_context {
            if !current_implementation_context.calls_to_pous_with_reference_to_return.is_empty() {
                self.implementation_context.push(current_implementation_context.clone());
            }
        }

        self.current_implementation_context = None;
    }

    fn visit_call_statement(&mut self, node: &mut AstNode) {
        if let AstStatement::CallStatement(call_statement) = &mut node.stmt {
            if let Some(call_statement_name) = call_statement.operator.get_flat_reference_name() {
                if self.pou_context.iter().any(|it| it.pou_name == call_statement_name) {
                    if let Some(current_implementation_context) = &mut self.current_implementation_context {
                        let call_statement_name = call_statement_name.to_string();
                        let element = current_implementation_context
                            .calls_to_pous_with_reference_to_return
                            .iter_mut()
                            .find(|it| it.name == call_statement_name);

                        if let Some(element) = element {
                            element.count += 1;
                        } else {
                            current_implementation_context.calls_to_pous_with_reference_to_return.push(
                                ReferenceToReturnImplementationContextCalls {
                                    name: call_statement_name,
                                    count: 1,
                                },
                            );
                        }
                    };
                }
            }

            call_statement.operator.walk(self);
            if let Some(call_statement_parameters) = &mut call_statement.parameters {
                call_statement_parameters.walk(self);
            }
        }
    }
}

impl AstVisitorMut for ReferenceToReturnLowerer {
    fn visit_compilation_unit(&mut self, unit: &mut CompilationUnit) {
        unit.walk(self);
    }

    fn visit_pou(&mut self, pou: &mut Pou) {
        pou.walk(self);

        if self.pou_original_return_type_is_reference_to(&pou.name) {
            // Steal the return type
            let cloned_return_type = pou
                .return_type
                .clone()
                .expect("this absolutely must be present to enter this conditional block");
            pou.return_type = None;

            let location = SourceLocation::from(&cloned_return_type);

            // Construct the by reference variable used to fetch the return data
            let current_context = self
                .pou_context
                .iter()
                .find(|it| it.pou_name == pou.name)
                .expect("context is required for this action");
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
            for call_context in calls {
                let call = call_context.name;
                let count = call_context.count;

                let current_context =
                    self.pou_context.iter().find(|it| it.pou_name == call).expect("this context must exist");
                let return_type_for_call = current_context.return_type.clone();
                let return_type_inner_type_for_call = current_context.return_type_inner_type.clone();
                let return_type_for_call_location = return_type_for_call.get_location().clone();

                let mut reference_variables = Vec::new();
                let mut reference_store_variables = Vec::new();

                // Construct the by reference variable used to fetch the return data
                for i in 1..(count + 1) {
                    let new_data_type_name =
                        format!("__{}{}_{i}", pou.name, self.get_return_variable_name(&call));
                    let new_data_type = DataTypeDeclaration::Reference {
                        referenced_type: new_data_type_name.clone(),
                        location: return_type_for_call_location.clone(),
                    };

                    let new_user_type = UserTypeDeclaration {
                        data_type: DataType::PointerType {
                            name: Some(new_data_type_name.clone()),
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

                    reference_variables.push(Variable {
                        name: format!("{}_{i}", self.get_return_variable_name(&call)),
                        data_type_declaration: new_data_type.clone(),
                        initializer: None,
                        address: None,
                        location: return_type_for_call_location.clone(),
                    });

                    reference_store_variables.push(Variable {
                        name: format!("{}_{i}", self.get_return_variable_name_store(&call)),
                        data_type_declaration: return_type_inner_type_for_call.clone(),
                        initializer: None,
                        address: None,
                        location: return_type_for_call_location.clone(),
                    });
                }

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
                        block.variables.append(&mut reference_variables);
                        block.variables.append(&mut reference_store_variables);
                    }
                });
            }
        }
    }

    fn visit_implementation(&mut self, implementation: &mut Implementation) {
        // Enter context
        self.current_implementation_pou_context.in_context =
            self.pou_original_return_type_is_reference_to(&implementation.name);

        implementation.walk(self);

        // Exit context
        self.current_implementation_pou_context.in_context = false;
        self.current_implementation_call_context.used_call_states = Vec::new();
    }

    fn visit_statement_list(&mut self, stmts: &mut Vec<AstNode>) {
        // Handle the implementation changes for implementations that invoke "reference to" functions
        for node in stmts {
            self.current_implementation_call_context.in_context = true;

            node.walk(self);

            if !self.current_implementation_call_context.pre_statements.is_empty()
                || !self.current_implementation_call_context.post_statements.is_empty()
            {
                let stolen_node = match &node.stmt {
                    AstStatement::RefAssignment(assignment) => (*assignment.right).clone(),
                    _ => node.clone(),
                };

                let mut combined_expressions = Vec::new();
                combined_expressions.append(&mut self.current_implementation_call_context.pre_statements);
                combined_expressions.push(stolen_node);
                combined_expressions.append(&mut self.current_implementation_call_context.post_statements);

                // Need to assign a new id if we're stealing this node
                node.id = self.ids.next_id();
                node.stmt = AstStatement::ExpressionList(combined_expressions);
            }

            // Clean up
            self.current_implementation_call_context.in_context = false;
            self.current_implementation_call_context.left_side_assignment = None;
            self.current_implementation_call_context.pre_statements = Vec::new();
            self.current_implementation_call_context.post_statements = Vec::new();
        }
    }

    fn visit_call_statement(&mut self, node: &mut AstNode) {
        // We need to split the assignment process into three statements and return a list statement
        let mut replacement_call_statement: Option<AstStatement> = None;

        if let AstStatement::CallStatement(call_statement) = &mut node.stmt {
            let previous_in_call_stack = self.current_implementation_call_context.in_call_stack;
            let previous_in_deep_call_stack = self.current_implementation_call_context.in_deep_call_stack;

            if self.current_implementation_call_context.in_context
                && self.call_statement_referenced_pou_is_reference_to_return(call_statement)
            {
                // Enter sub-call context
                if previous_in_call_stack {
                    self.current_implementation_call_context.in_deep_call_stack = true;
                }

                let call_statement_name = call_statement
                    .operator
                    .get_flat_reference_name()
                    .expect("this must have a name")
                    .to_string();

                let current_index = self.get_and_bump_index_for_current_call_state(call_statement);
                let return_variable_name =
                    format!("{}_{}", self.get_return_variable_name(&call_statement_name), current_index);
                let return_variable_name_store = format!(
                    "{}_{}",
                    self.get_return_variable_name_store(&call_statement_name),
                    current_index
                );

                // Construct the new parameter
                let param = AstFactory::create_member_reference(
                    AstFactory::create_identifier(
                        return_variable_name.to_string(),
                        node.location.clone(),
                        self.ids.next_id(),
                    ),
                    None,
                    self.ids.next_id(),
                );

                // Construct the temp variable assignment
                let left = AstFactory::create_member_reference(
                    AstFactory::create_identifier(
                        return_variable_name.to_string(),
                        node.location.clone(),
                        self.ids.next_id(),
                    ),
                    None,
                    self.ids.next_id(),
                );

                let right = AstFactory::create_member_reference(
                    AstFactory::create_identifier(
                        return_variable_name_store,
                        node.location.clone(),
                        self.ids.next_id(),
                    ),
                    None,
                    self.ids.next_id(),
                );

                let ref_assignment = AstFactory::create_ref_assignment(left, right, self.ids.next_id());
                self.current_implementation_call_context.pre_statements.push(ref_assignment);

                // We edit the call statement in place
                replacement_call_statement = {
                    let mut operator = *call_statement.operator.clone();
                    let mut parameters = if let Some(expression_list) = &call_statement.parameters {
                        let expression_list = expression_list.clone();
                        match expression_list.stmt {
                            AstStatement::ExpressionList(mut expressions) => {
                                expressions.insert(0, param);
                                AstFactory::create_expression_list(
                                    expressions,
                                    node.location.clone(),
                                    self.ids.next_id(),
                                )
                            }
                            _ => AstFactory::create_expression_list(
                                vec![param, *expression_list],
                                node.location.clone(),
                                self.ids.next_id(),
                            ),
                        }
                    } else {
                        AstFactory::create_expression_list(
                            vec![param],
                            node.location.clone(),
                            self.ids.next_id(),
                        )
                    };

                    // Enter call context (only if we are not in a sub-call)
                    if !self.current_implementation_call_context.in_deep_call_stack {
                        self.current_implementation_call_context.in_call_stack = true;
                    }

                    // Ensure we check the operator and parameters for further possible lowering
                    operator.walk(self);
                    parameters.walk(self);

                    // Exit call context (only if we are not in a sub-call)
                    if !self.current_implementation_call_context.in_deep_call_stack {
                        self.current_implementation_call_context.in_call_stack = false;
                    }

                    if !self.current_implementation_call_context.in_call_stack {
                        Some(AstStatement::CallStatement(CallStatement {
                            operator: Box::new(operator),
                            parameters: Some(Box::new(parameters)),
                        }))
                    } else {
                        /* ------------
                            FIXME (anbt): We should perform this lowering at an earlier stage to simplify this code and remove
                            the unnecessary stack management that occurs here.

                            Example:
                            ```ST
                                foo(bar(baz()));

                                // Should be lowered to:
                                __baz := baz();
                                __bar := bar(__baz);
                                foo(__bar);
                            ```
                        ------------ */

                        // If we're currently in a sub-call (i.e. call nested in call) then we need to handle this differently

                        // Push the call statement to pre-statements
                        self.current_implementation_call_context.pre_statements.push(
                            AstFactory::create_call_statement(
                                operator,
                                Some(parameters),
                                self.ids.next_id(),
                                node.location.clone(),
                            ),
                        );

                        // Return the member reference to the pointer in place of the function
                        Some(
                            AstFactory::create_member_reference(
                                AstFactory::create_identifier(
                                    return_variable_name.to_string(),
                                    node.location.clone(),
                                    self.ids.next_id(),
                                ),
                                None,
                                self.ids.next_id(),
                            )
                            .stmt,
                        )
                    }
                };

                // We only push the follow up assignment if it was assigned to in the first place
                // and if this is not a sub call (there will be no ref assignments for sub-calls)
                if !self.current_implementation_call_context.in_call_stack {
                    if let Some(return_assignment) =
                        &self.current_implementation_call_context.left_side_assignment
                    {
                        let right = AstFactory::create_member_reference(
                            AstFactory::create_identifier(
                                return_variable_name,
                                node.location.clone(),
                                self.ids.next_id(),
                            ),
                            None,
                            self.ids.next_id(),
                        );

                        let ref_assignment = AstFactory::create_ref_assignment(
                            return_assignment.clone(),
                            right,
                            self.ids.next_id(),
                        );

                        self.current_implementation_call_context.post_statements.push(ref_assignment);
                    }
                }

                if self.current_implementation_call_context.in_call_stack {
                    self.current_implementation_call_context.in_deep_call_stack = false;
                }
            }

            self.current_implementation_call_context.in_call_stack = previous_in_call_stack;
            self.current_implementation_call_context.in_deep_call_stack = previous_in_deep_call_stack;
        }

        if let Some(replacement_call_statement) = replacement_call_statement {
            node.stmt = replacement_call_statement;
        }
    }

    fn visit_ref_assignment(&mut self, node: &mut AstNode) {
        let mut replacement_statement: Option<AstStatement> = None;

        if let AstStatement::RefAssignment(assignment) = &mut node.stmt {
            if self.current_implementation_call_context.in_context
                && self.node_is_call_statement_with_referenced_pou_that_is_reference_to_return(
                    assignment.right.get_node_peeled(),
                )
            {
                self.current_implementation_call_context.left_side_assignment =
                    Some(assignment.left.as_ref().clone());
            } else if self.current_implementation_pou_context.in_context {
                if let Some(name) = assignment.left.get_flat_reference_name() {
                    if self.pou_original_return_type_is_reference_to(name) {
                        let mut replacement_assignment = assignment.clone();

                        let member_reference = AstFactory::create_member_reference(
                            AstFactory::create_identifier(
                                self.get_return_variable_name(name),
                                assignment.left.location.clone(),
                                self.ids.next_id(),
                            ),
                            None,
                            self.ids.next_id(),
                        );

                        replacement_assignment.left = Box::new(member_reference);
                        replacement_statement = Some(AstStatement::Assignment(replacement_assignment));
                    }
                }
            }

            assignment.left.walk(self);
            assignment.right.walk(self);
        }

        if let Some(replacement_statement) = replacement_statement {
            node.stmt = replacement_statement;
        }
    }
}

impl ReferenceToReturnLowerer {
    pub fn new(
        ids: IdProvider,
        pou_context: Vec<ReferenceToReturnPouContext>,
        implementation_context: Vec<ReferenceToReturnImplementationContext>,
    ) -> Self {
        Self {
            ids,
            pou_context,
            implementation_context,
            new_user_types: Vec::new(),
            current_implementation_call_context: ReferenceToReturnImplementationCallContext::new(),
            current_implementation_pou_context: ReferenceToReturnImplementationPouContext::new(),
        }
    }

    fn pou_original_return_type_is_reference_to(&self, name: &str) -> bool {
        self.pou_context.iter().any(|it| it.pou_name == name)
    }

    fn get_all_pous_that_return_reference_to_for_implementation_of_pou(
        &self,
        pou: &Pou,
    ) -> Option<Vec<ReferenceToReturnImplementationContextCalls>> {
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

    fn get_return_variable_name(&self, name: &str) -> String {
        format!("__{}_return_val", name)
    }

    fn get_return_variable_name_store(&self, name: &str) -> String {
        format!("{}_store", self.get_return_variable_name(name))
    }

    fn node_is_call_statement_with_referenced_pou_that_is_reference_to_return(&self, node: &AstNode) -> bool {
        match &node.stmt {
            AstStatement::CallStatement(call_statement) => {
                self.call_statement_referenced_pou_is_reference_to_return(call_statement)
            }
            _ => false,
        }
    }

    fn call_statement_referenced_pou_is_reference_to_return(&self, call_statement: &CallStatement) -> bool {
        if let Some(call_statement_name) = call_statement.operator.get_flat_reference_name() {
            if self.pou_original_return_type_is_reference_to(call_statement_name) {
                return true;
            }
        }

        false
    }

    fn get_and_bump_index_for_current_call_state(&mut self, call_statement: &CallStatement) -> usize {
        if let Some(call_statement_name) = call_statement.operator.get_flat_reference_name() {
            if self.pou_original_return_type_is_reference_to(call_statement_name) {
                let element = self
                    .current_implementation_call_context
                    .used_call_states
                    .iter_mut()
                    .find(|it| it.name == call_statement_name);

                if let Some(element) = element {
                    element.count += 1;

                    return element.count;
                } else {
                    self.current_implementation_call_context.used_call_states.push(
                        ReferenceToReturnImplementationContextCalls {
                            name: call_statement_name.to_string(),
                            count: 1,
                        },
                    );

                    return 1;
                }
            }
        }

        unreachable!("This is being called from somewhere it shouldn't be called from!")
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

        let (_, project) = parse_and_annotate("test", vec![src]).expect("must parse and annotation");
        let unit = &project.units[0].get_unit();
        let pous = &unit.pous;
        let implementations = &unit.implementations;

        // 1. Function "referenceFunc" should now have no return type after lowering
        let ref_func_pou =
            pous.iter().find(|i| i.name == "referenceFunc").expect("referenceFunc pou should exist");

        assert_eq!(ref_func_pou.return_type, None);

        // 2. Function "referenceFunc" should have a new `VAR_INPUT` variable: `__referenceFunc_return_val`
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
            __referenceFunc_return_val_1 : REFERENCE TO INT;
            __referenceFunc_return_val_store_1 : INT;
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
        __main__referenceFunc_return_val_1__ctor(__referenceFunc_return_val_1);
        tmpVal := 11;
        refVal REF= tmpVal;
        __referenceFunc_return_val_1 REF= __referenceFunc_return_val_store_1;
        referenceFunc(__referenceFunc_return_val_1, refVal);
        refVal REF= __referenceFunc_return_val_1;
        conVal := refVal;
        ");
    }

    #[test]
    fn reference_to_return_called_twice() {
        let src: SourceCode = r#"
            FUNCTION referenceFunc : REFERENCE TO INT
                VAR_INPUT
                    in	: REFERENCE TO INT;
                END_VAR

                in := in + 1;
                referenceFunc REF= in;
            END_FUNCTION

            FUNCTION main : DINT
                VAR
                    refA   : REFERENCE TO INT;
                    refB   : REFERENCE TO INT;
                    valA   : INT;
                    valB   : INT;
                    tmpA   : INT;
                    tmpB   : INT;
                END_VAR

                tmpA := 10;
                tmpB := 20;
                refA REF= tmpA;
                refB REF= tmpB;

                refA REF= referenceFunc(refA);
                refB REF= referenceFunc(refB);

                valA := refA;
                valB := refB;
            END_FUNCTION
            "#
        .into();

        let (_, project) = parse_and_annotate("test", vec![src]).expect("must parse and annotation");
        let unit = &project.units[0].get_unit();
        let implementations = &unit.implementations;

        let main_implementation =
            implementations.iter().find(|i| i.name == "main").expect("main implementation should exist");

        let ref_eq_call_func_statement = &main_implementation.statements;
        assert_snapshot!(AstSerializer::format_nodes(ref_eq_call_func_statement), @"
        __main_refA__ctor(refA);
        __main_refB__ctor(refB);
        __main__referenceFunc_return_val_1__ctor(__referenceFunc_return_val_1);
        __main__referenceFunc_return_val_2__ctor(__referenceFunc_return_val_2);
        tmpA := 10;
        tmpB := 20;
        refA REF= tmpA;
        refB REF= tmpB;
        __referenceFunc_return_val_1 REF= __referenceFunc_return_val_store_1;
        referenceFunc(__referenceFunc_return_val_1, refA);
        refA REF= __referenceFunc_return_val_1;
        __referenceFunc_return_val_2 REF= __referenceFunc_return_val_store_2;
        referenceFunc(__referenceFunc_return_val_2, refB);
        refB REF= __referenceFunc_return_val_2;
        valA := refA;
        valB := refB;
        ");
    }

    #[test]
    fn reference_to_return_inside_case() {
        let src: SourceCode = r#"
            FUNCTION referenceFunc : REFERENCE TO INT
                VAR_INPUT
                    in	: REFERENCE TO INT;
                END_VAR

                in := in + 1;
                referenceFunc REF= in;
            END_FUNCTION

            FUNCTION main : DINT
                VAR
                    refVal : REFERENCE TO INT;
                    conVal : INT;
                    tmpVal : INT;
                    sel    : INT := 1;
                END_VAR

                tmpVal := 10;
                refVal REF= tmpVal;

                CASE sel OF
                    1:
                        refVal REF= referenceFunc(refVal);
                    2:
                        tmpVal := 99;
                END_CASE;

                conVal := refVal;
            END_FUNCTION
            "#
        .into();

        let (_, project) = parse_and_annotate("test", vec![src]).expect("must parse and annotation");
        let unit = &project.units[0].get_unit();
        let implementations = &unit.implementations;

        let main_implementation =
            implementations.iter().find(|i| i.name == "main").expect("main implementation should exist");

        let ref_eq_call_func_statement = &main_implementation.statements;
        assert_snapshot!(AstSerializer::format_nodes(ref_eq_call_func_statement), @"
        __main_refVal__ctor(refVal);
        sel := 1;
        __main__referenceFunc_return_val_1__ctor(__referenceFunc_return_val_1);
        tmpVal := 10;
        refVal REF= tmpVal;
        CASE sel OF
            1:
                __referenceFunc_return_val_1 REF= __referenceFunc_return_val_store_1;
                referenceFunc(__referenceFunc_return_val_1, refVal);
                refVal REF= __referenceFunc_return_val_1;
            2:
                tmpVal := 99
        END_CASE;
        conVal := refVal;
        ");
    }

    #[test]
    fn reference_to_return_inside_for_loop() {
        let src: SourceCode = r#"
            FUNCTION referenceFunc : REFERENCE TO INT
                VAR_INPUT
                    in	: REFERENCE TO INT;
                END_VAR

                in := in + 1;
                referenceFunc REF= in;
            END_FUNCTION

            FUNCTION main : DINT
                VAR
                    refVal : REFERENCE TO INT;
                    conVal : INT;
                    tmpVal : INT;
                    i      : INT;
                END_VAR

                tmpVal := 0;
                refVal REF= tmpVal;

                FOR i := 1 TO 5 DO
                    refVal REF= referenceFunc(refVal);
                END_FOR;

                conVal := refVal;
            END_FUNCTION
            "#
        .into();

        let (_, project) = parse_and_annotate("test", vec![src]).expect("must parse and annotation");
        let unit = &project.units[0].get_unit();
        let implementations = &unit.implementations;

        let main_implementation =
            implementations.iter().find(|i| i.name == "main").expect("main implementation should exist");

        let ref_eq_call_func_statement = &main_implementation.statements;
        assert_snapshot!(AstSerializer::format_nodes(ref_eq_call_func_statement), @"
        __main_refVal__ctor(refVal);
        __main__referenceFunc_return_val_1__ctor(__referenceFunc_return_val_1);
        tmpVal := 0;
        refVal REF= tmpVal;
        alloca ran_once_0: BOOL;
        alloca is_incrementing_0: BOOL;
        i := 1;
        is_incrementing_0 := TRUE;
        WHILE TRUE DO
            IF ran_once_0 THEN
                i := i + 1
            END_IF
            ran_once_0 := TRUE
            IF is_incrementing_0 THEN
                IF i > 5 THEN
                    EXIT;
                END_IF
            ELSE
                IF i < 5 THEN
                    EXIT;
                END_IF
            END_IF
            __referenceFunc_return_val_1 REF= __referenceFunc_return_val_store_1;
            referenceFunc(__referenceFunc_return_val_1, refVal);
            refVal REF= __referenceFunc_return_val_1;
        END_WHILE;
        conVal := refVal;
        ");
    }

    #[test]
    fn reference_to_return_inside_if() {
        let src: SourceCode = r#"
            FUNCTION referenceFunc : REFERENCE TO INT
                VAR_INPUT
                    in	: REFERENCE TO INT;
                END_VAR

                in := in + 1;
                referenceFunc REF= in;
            END_FUNCTION

            FUNCTION main : DINT
                VAR
                    refVal : REFERENCE TO INT;
                    conVal : INT;
                    tmpVal : INT;
                    flag   : BOOL := TRUE;
                END_VAR

                tmpVal := 11;
                refVal REF= tmpVal;

                IF flag THEN
                    refVal REF= referenceFunc(refVal);
                END_IF;

                conVal := refVal;
            END_FUNCTION
            "#
        .into();

        let (_, project) = parse_and_annotate("test", vec![src]).expect("must parse and annotation");
        let unit = &project.units[0].get_unit();
        let implementations = &unit.implementations;

        let main_implementation =
            implementations.iter().find(|i| i.name == "main").expect("main implementation should exist");

        let ref_eq_call_func_statement = &main_implementation.statements;
        assert_snapshot!(AstSerializer::format_nodes(ref_eq_call_func_statement), @"
        __main_refVal__ctor(refVal);
        flag := TRUE;
        __main__referenceFunc_return_val_1__ctor(__referenceFunc_return_val_1);
        tmpVal := 11;
        refVal REF= tmpVal;
        IF flag THEN
            __referenceFunc_return_val_1 REF= __referenceFunc_return_val_store_1;
            referenceFunc(__referenceFunc_return_val_1, refVal);
            refVal REF= __referenceFunc_return_val_1;
        END_IF;
        conVal := refVal;
        ");
    }

    #[test]
    fn reference_to_return_inside_while_loop() {
        let src: SourceCode = r#"
            FUNCTION referenceFunc : REFERENCE TO INT
                VAR_INPUT
                    in	: REFERENCE TO INT;
                END_VAR

                in := in + 1;
                referenceFunc REF= in;
            END_FUNCTION

            FUNCTION main : DINT
                VAR
                    refVal : REFERENCE TO INT;
                    conVal : INT;
                    tmpVal : INT;
                END_VAR

                tmpVal := 0;
                refVal REF= tmpVal;

                WHILE tmpVal < 3 DO
                    refVal REF= referenceFunc(refVal);
                END_WHILE;

                conVal := refVal;
            END_FUNCTION
            "#
        .into();

        let (_, project) = parse_and_annotate("test", vec![src]).expect("must parse and annotation");
        let unit = &project.units[0].get_unit();
        let implementations = &unit.implementations;

        let main_implementation =
            implementations.iter().find(|i| i.name == "main").expect("main implementation should exist");

        let ref_eq_call_func_statement = &main_implementation.statements;
        assert_snapshot!(AstSerializer::format_nodes(ref_eq_call_func_statement), @"
        __main_refVal__ctor(refVal);
        __main__referenceFunc_return_val_1__ctor(__referenceFunc_return_val_1);
        tmpVal := 0;
        refVal REF= tmpVal;
        WHILE TRUE DO
            IF NOT tmpVal < 3 THEN
                EXIT;
            END_IF
            __referenceFunc_return_val_1 REF= __referenceFunc_return_val_store_1;
            referenceFunc(__referenceFunc_return_val_1, refVal);
            refVal REF= __referenceFunc_return_val_1;
        END_WHILE;
        conVal := refVal;
        ");
    }

    #[test]
    fn reference_to_return_multiple_different_functions() {
        let src: SourceCode = r#"
            FUNCTION addOne : REFERENCE TO INT
                VAR_INPUT
                    in	: REFERENCE TO INT;
                END_VAR

                in := in + 1;
                addOne REF= in;
            END_FUNCTION

            FUNCTION doubleIt : REFERENCE TO INT
                VAR_INPUT
                    in	: REFERENCE TO INT;
                END_VAR

                in := in * 2;
                doubleIt REF= in;
            END_FUNCTION

            FUNCTION main : DINT
                VAR
                    refVal : REFERENCE TO INT;
                    conVal : INT;
                    tmpVal : INT;
                END_VAR

                tmpVal := 5;
                refVal REF= tmpVal;
                refVal REF= addOne(refVal);
                refVal REF= doubleIt(refVal);
                conVal := refVal;
            END_FUNCTION
            "#
        .into();

        let (_, project) = parse_and_annotate("test", vec![src]).expect("must parse and annotation");
        let unit = &project.units[0].get_unit();
        let implementations = &unit.implementations;

        let main_implementation =
            implementations.iter().find(|i| i.name == "main").expect("main implementation should exist");

        let ref_eq_call_func_statement = &main_implementation.statements;
        assert_snapshot!(AstSerializer::format_nodes(ref_eq_call_func_statement), @"
        __main_refVal__ctor(refVal);
        __main__addOne_return_val_1__ctor(__addOne_return_val_1);
        __main__doubleIt_return_val_1__ctor(__doubleIt_return_val_1);
        tmpVal := 5;
        refVal REF= tmpVal;
        __addOne_return_val_1 REF= __addOne_return_val_store_1;
        addOne(__addOne_return_val_1, refVal);
        refVal REF= __addOne_return_val_1;
        __doubleIt_return_val_1 REF= __doubleIt_return_val_store_1;
        doubleIt(__doubleIt_return_val_1, refVal);
        refVal REF= __doubleIt_return_val_1;
        conVal := refVal;
        ");
    }

    #[test]
    fn reference_to_return_nested_call() {
        let src: SourceCode = r#"
            FUNCTION addOne : REFERENCE TO INT
                VAR_INPUT
                    in	: REFERENCE TO INT;
                END_VAR

                in := in + 1;
                addOne REF= in;
            END_FUNCTION

            FUNCTION doubleIt : REFERENCE TO INT
                VAR_INPUT
                    in	: REFERENCE TO INT;
                END_VAR

                in := in * 2;
                doubleIt REF= in;
            END_FUNCTION

            FUNCTION main : DINT
                VAR
                    refVal : REFERENCE TO INT;
                    conVal : INT;
                    tmpVal : INT;
                END_VAR

                tmpVal := 5;
                refVal REF= tmpVal;
                refVal REF= doubleIt(addOne(refVal));
                conVal := refVal;
            END_FUNCTION
            "#
        .into();

        let (_, project) = parse_and_annotate("test", vec![src]).expect("must parse and annotation");
        let unit = &project.units[0].get_unit();
        let implementations = &unit.implementations;

        let main_implementation =
            implementations.iter().find(|i| i.name == "main").expect("main implementation should exist");

        let ref_eq_call_func_statement = &main_implementation.statements;
        assert_snapshot!(AstSerializer::format_nodes(ref_eq_call_func_statement), @"
        __main_refVal__ctor(refVal);
        __main__doubleIt_return_val_1__ctor(__doubleIt_return_val_1);
        __main__addOne_return_val_1__ctor(__addOne_return_val_1);
        tmpVal := 5;
        refVal REF= tmpVal;
        __doubleIt_return_val_1 REF= __doubleIt_return_val_store_1;
        __addOne_return_val_1 REF= __addOne_return_val_store_1;
        addOne(__addOne_return_val_1, refVal);
        doubleIt(__doubleIt_return_val_1, __addOne_return_val_1);
        refVal REF= __doubleIt_return_val_1;
        conVal := refVal;
        ");
    }

    #[test]
    fn reference_to_return_double_nested_call() {
        let src: SourceCode = r#"
            FUNCTION addOne : REFERENCE TO INT
                VAR_INPUT
                    in	: REFERENCE TO INT;
                END_VAR

                in := in + 1;
                addOne REF= in;
            END_FUNCTION

            FUNCTION doubleIt : REFERENCE TO INT
                VAR_INPUT
                    in	: REFERENCE TO INT;
                END_VAR

                in := in * 2;
                doubleIt REF= in;
            END_FUNCTION

            FUNCTION tripleIt : REFERENCE TO INT
                VAR_INPUT
                    in	: REFERENCE TO INT;
                END_VAR

                in := in * 3;
                tripleIt REF= in;
            END_FUNCTION

            FUNCTION main : DINT
                VAR
                    refVal : REFERENCE TO INT;
                    conVal : INT;
                    tmpVal : INT;
                END_VAR

                tmpVal := 5;
                refVal REF= tmpVal;
                refVal REF= tripleIt(doubleIt(addOne(refVal)));
                conVal := refVal;
            END_FUNCTION
            "#
        .into();

        let (_, project) = parse_and_annotate("test", vec![src]).expect("must parse and annotation");
        let unit = &project.units[0].get_unit();
        let implementations = &unit.implementations;

        let main_implementation =
            implementations.iter().find(|i| i.name == "main").expect("main implementation should exist");

        let ref_eq_call_func_statement = &main_implementation.statements;
        assert_snapshot!(AstSerializer::format_nodes(ref_eq_call_func_statement), @"
        __main_refVal__ctor(refVal);
        __main__tripleIt_return_val_1__ctor(__tripleIt_return_val_1);
        __main__doubleIt_return_val_1__ctor(__doubleIt_return_val_1);
        __main__addOne_return_val_1__ctor(__addOne_return_val_1);
        tmpVal := 5;
        refVal REF= tmpVal;
        __tripleIt_return_val_1 REF= __tripleIt_return_val_store_1;
        __doubleIt_return_val_1 REF= __doubleIt_return_val_store_1;
        __addOne_return_val_1 REF= __addOne_return_val_store_1;
        addOne(__addOne_return_val_1, refVal);
        doubleIt(__doubleIt_return_val_1, __addOne_return_val_1);
        tripleIt(__tripleIt_return_val_1, __doubleIt_return_val_1);
        refVal REF= __tripleIt_return_val_1;
        conVal := refVal;
        ");
    }

    #[test]
    fn reference_to_return_not_last_statement() {
        let src: SourceCode = r#"
            FUNCTION referenceFunc : REFERENCE TO INT
                VAR_INPUT
                    in	: REFERENCE TO INT;
                END_VAR
                VAR
                    dummy : INT;
                END_VAR

                in := in + 1;
                referenceFunc REF= in;
                dummy := 0; // this is the last statement, so the REF= above won't be lowered
            END_FUNCTION

            FUNCTION main : DINT
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

        let (_, project) = parse_and_annotate("test", vec![src]).expect("must parse and annotation");
        let unit = &project.units[0].get_unit();
        let implementations = &unit.implementations;

        let main_implementation =
            implementations.iter().find(|i| i.name == "main").expect("main implementation should exist");

        let ref_eq_call_func_statement = &main_implementation.statements;
        assert_snapshot!(AstSerializer::format_nodes(ref_eq_call_func_statement), @"
        __main_refVal__ctor(refVal);
        __main__referenceFunc_return_val_1__ctor(__referenceFunc_return_val_1);
        tmpVal := 11;
        refVal REF= tmpVal;
        __referenceFunc_return_val_1 REF= __referenceFunc_return_val_store_1;
        referenceFunc(__referenceFunc_return_val_1, refVal);
        refVal REF= __referenceFunc_return_val_1;
        conVal := refVal;
        ");
    }

    #[test]
    fn reference_to_return_standalone_call() {
        let src: SourceCode = r#"
            FUNCTION referenceFunc : REFERENCE TO INT
                VAR_INPUT
                    in	: REFERENCE TO INT;
                END_VAR

                in := in + 1;
                referenceFunc REF= in;
            END_FUNCTION

            FUNCTION main : DINT
                VAR
                    refVal : REFERENCE TO INT;
                    conVal : INT;
                    tmpVal : INT;
                END_VAR

                tmpVal := 11;
                refVal REF= tmpVal;

                // Call without capturing the return - should still compile & run with the
                // rewritten void signature (the side effect on `in` still matters)
                referenceFunc(refVal);

                conVal := refVal;
            END_FUNCTION
            "#
        .into();

        let (_, project) = parse_and_annotate("test", vec![src]).expect("must parse and annotation");
        let unit = &project.units[0].get_unit();
        let implementations = &unit.implementations;

        let main_implementation =
            implementations.iter().find(|i| i.name == "main").expect("main implementation should exist");

        let ref_eq_call_func_statement = &main_implementation.statements;
        assert_snapshot!(AstSerializer::format_nodes(ref_eq_call_func_statement), @"
        __main_refVal__ctor(refVal);
        __main__referenceFunc_return_val_1__ctor(__referenceFunc_return_val_1);
        tmpVal := 11;
        refVal REF= tmpVal;
        __referenceFunc_return_val_1 REF= __referenceFunc_return_val_store_1;
        referenceFunc(__referenceFunc_return_val_1, refVal);
        conVal := refVal;
        ");
    }
}
