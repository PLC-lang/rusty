use itertools::Itertools;
use plc_ast::ast::{
    AstId, Identifier, Implementation, Interface, LinkageType, Pou, PouType, VariableBlockType,
};
use plc_diagnostics::diagnostics::Diagnostic;
use plc_source::source_location::SourceLocation;
use signature_validation::validate_method_signature;

use super::{
    property, statement::visit_statement, variable::visit_variable_block, ValidationContext, Validator,
    Validators,
};
use crate::resolver::{AnnotationMap, StatementAnnotation};

pub fn visit_pou<T: AnnotationMap>(validator: &mut Validator, pou: &Pou, context: &ValidationContext<'_, T>) {
    if pou.linkage != LinkageType::External {
        validate_pou(validator, pou);
        validate_interface_impl(validator, context, pou);
        validate_base_class(validator, context, pou);
        validate_methods_overrides(validator, context, pou.id, &pou.name, &pou.name_location);
        if let PouType::Method { .. } = pou.kind {
            validate_method(validator, pou, context);
        }

        for block in &pou.variable_blocks {
            visit_variable_block(validator, Some(pou), block, context);
        }
    }
}

fn validate_methods_overrides<T: AnnotationMap>(
    validator: &mut Validator,
    context: &ValidationContext<'_, T>,
    ast_id: AstId,
    container_name: &str,
    primary_location: &SourceLocation,
) {
    let Some(StatementAnnotation::MethodDeclarations { declarations }) =
        context.annotations.get_with_id(ast_id)
    else {
        return;
    };
    declarations.iter().for_each(|(_, decl)| {
        // validate that abstract signatures all match
        // Concrete to abstract methods are checked at a different stage
        let methods = decl
            .iter()
            .filter(|it| it.is_abstract())
            .flat_map(|it| context.index.find_pou(it.get_qualified_name()))
            // We already have a specialized validation for properties and would like to avoid duplicates here
            .filter(|it| !it.is_property());
        // XXX(ghha) should this not be combinations instead of tuple_windows?
        for (method1, method2) in methods.tuple_windows() {
            let diagnostics = validate_method_signature(context.index, method1, method2, primary_location);
            if !diagnostics.is_empty() {
                validator.push_diagnostic(
                    Diagnostic::new(format!(
                        "{} in `{}` is declared with conflicting signatures in `{}` and `{}`",
                        method1.get_method_name().expect("must be a method"),
                        container_name,
                        method1.get_parent_pou_name().unwrap(),
                        method2.get_parent_pou_name().unwrap()
                    ))
                    .with_error_code("E111")
                    .with_location(primary_location)
                    .with_secondary_location(method1.get_location())
                    .with_secondary_location(method2.get_location())
                    .with_sub_diagnostics(diagnostics),
                );
                // We want to early return here otherwise we could spam the user with lots of (valid) but identical
                // diagnostics reported earlier.
                return;
            }
        }

        // we only want to validate existing concrete implementations for POUs, not interfaces
        if context.index.find_pou(container_name).is_some() {
            // validate that each abstract method has at least one concrete implementation
            let abstracts = decl
                .iter()
                .filter(|it| it.is_abstract())
                .map(|it| (it.get_qualifier(), context.index.find_pou(it.get_qualified_name()).unwrap()));

            // Expecting only one concrete implementation
            let concrete = decl
                .iter()
                .filter(|it| it.is_concrete())
                .map(|it| context.index.find_pou(it.get_qualified_name()).unwrap())
                .next();
            // Validate that each concrete method which has an abstract counterpart has the same signature
            if let Some(method_impl) = concrete {
                abstracts.for_each(|(_, method_ref)| {
                    let diagnostics =
                        validate_method_signature(context.index, method_ref, method_impl, primary_location);
                    for diagnostic in diagnostics {
                        validator.push_diagnostic(diagnostic);
                    }
                });
            } else {
                abstracts.for_each(|(name, intf)| {
                    validator.push_diagnostic(
                        Diagnostic::new(format!(
                            "{} defined in interface `{}` is missing in POU `{}`",
                            intf.get_method_name().expect("must be a method"),
                            name,
                            container_name
                        ))
                        .with_error_code("E112")
                        .with_location(primary_location)
                        .with_secondary_location(intf.get_location()),
                    );
                })
            };
        }
    });
}

fn validate_method<T: AnnotationMap>(
    validator: &mut Validator<'_>,
    pou: &Pou,
    context: &ValidationContext<'_, T>,
) {
    let Some(StatementAnnotation::Override { definitions }) = context.annotations.get_with_id(pou.id) else {
        // No override
        return;
    };
    let Some(method_impl) = context.index.find_pou(&pou.name) else {
        // Method does not exist
        return;
    };

    //Only validate methods that are not abstract, abstract methods are validated in validate_implemented_methods
    let overridden_concrete_methods = definitions
        .iter()
        .filter(|it| it.is_concrete())
        .flat_map(|it| context.index.find_pou(it.get_qualified_name()))
        .collect::<Vec<_>>();
    overridden_concrete_methods.iter().for_each(|method_ref| {
        let diagnostics =
            validate_method_signature(context.index, method_ref, method_impl, &pou.name_location);
        for diagnostic in diagnostics {
            validator.push_diagnostic(diagnostic);
        }
    })
}

fn validate_base_class<T: AnnotationMap>(
    validator: &mut Validator<'_>,
    context: &ValidationContext<'_, T>,
    pou: &Pou,
) {
    if let Some(Identifier { name, location }) = &pou.super_class {
        // Check if the interfaces are implemented on the correct POU types
        if !matches!(pou.kind, PouType::FunctionBlock | PouType::Class) {
            validator.push_diagnostic(
                Diagnostic::new("Subclassing is only allowed in `CLASS` and `FUNCTION_BLOCK`")
                    .with_error_code("E110")
                    .with_location(&pou.name_location),
            );
        }

        if context.index.find_pou(name).is_none() {
            validator.push_diagnostic(
                Diagnostic::new(format!("Base `{}` does not exist", name))
                    .with_error_code("E048")
                    .with_location(location),
            );
        }
    };
}

fn validate_interface_impl<T>(validator: &mut Validator, ctxt: &ValidationContext<'_, T>, pou: &Pou)
where
    T: AnnotationMap,
{
    // No interfaces declared to implement
    if pou.interfaces.is_empty() {
        return;
    }

    // Check if the interfaces are implemented on the correct POU types
    if !matches!(pou.kind, PouType::FunctionBlock | PouType::Class) {
        let location = {
            let location_first = pou.interfaces.first().unwrap();
            let location_last = pou.interfaces.last().unwrap();

            location_first.location.span(&location_last.location)
        };

        validator.push_diagnostic(
            Diagnostic::new("Interfaces can only be implemented by `CLASS` or `FUNCTION_BLOCK`")
                .with_error_code("E110")
                .with_location(location),
        );
    }

    // Check if the declared interfaces exist, i.e. the comma seperated identifiers after `[...] IMPLEMENTS`
    for declaration in &pou.interfaces {
        if ctxt.index.find_interface(&declaration.name).is_none() {
            validator.push_diagnostic(
                Diagnostic::new(format!("Interface `{}` does not exist", declaration.name))
                    .with_error_code("E048")
                    .with_location(&declaration.location),
            );
        }
    }
}

pub fn visit_implementation<T: AnnotationMap>(
    validator: &mut Validator,
    implementation: &Implementation,
    context: &ValidationContext<'_, T>,
) {
    if implementation.pou_type == PouType::Class && !implementation.statements.is_empty() {
        validator.push_diagnostic(
            Diagnostic::new("A class cannot have an implementation")
                .with_error_code("E017")
                .with_location(&implementation.location),
        );
    }
    if implementation.linkage != LinkageType::External {
        validate_action_container(validator, implementation);
        // Validate the label uniqueness

        if let Some(labels) = context.index.get_labels(&implementation.name) {
            for (_, labels) in labels.entries() {
                let mut label_iter = labels.iter();
                if let Some(first) = label_iter.next() {
                    if let Some(second) = label_iter.next() {
                        //Collect remaining
                        let mut locations: Vec<_> = label_iter.map(|it| it.location.clone()).collect();
                        locations.push(first.location.clone());
                        locations.push(second.location.clone());
                        validator.push_diagnostic(
                            Diagnostic::new(format!("{}: Duplicate label.", &first.name))
                                .with_error_code("E018")
                                .with_location(&first.location)
                                .with_secondary_locations(locations),
                        );
                    }
                }
            }
        }
        implementation.statements.iter().for_each(|s| {
            visit_statement(validator, s, &context.with_qualifier(implementation.name.as_str()))
        });
    }
}

fn validate_pou(validator: &mut Validator, pou: &Pou) {
    if pou.kind == PouType::Class {
        validate_class(validator, pou);
    };

    // If the POU is not a function or method, it cannot have a return type
    if !matches!(pou.kind, PouType::Function | PouType::Method { .. }) {
        if let Some(start_return_type) = &pou.return_type {
            validator.push_diagnostic(
                Diagnostic::new(format!("POU Type {:?} does not support a return type", pou.kind))
                    .with_error_code("E026")
                    .with_location(start_return_type.get_location()),
            )
        }
    }
}

fn validate_class(validator: &mut Validator, pou: &Pou) {
    // var in/out/inout blocks are not allowed inside of class declaration
    // TODO: This should be on each block
    if pou.variable_blocks.iter().any(|it| {
        matches!(it.kind, VariableBlockType::InOut | VariableBlockType::Input(_) | VariableBlockType::Output)
    }) {
        validator.push_diagnostic(
            Diagnostic::new("A class cannot contain `VAR_INPUT`, `VAR_IN_OUT`, or `VAR_OUTPUT` blocks")
                .with_error_code("E019")
                .with_location(&pou.name_location),
        );
    }
}

pub fn validate_action_container(validator: &mut Validator, implementation: &Implementation) {
    if implementation.pou_type == PouType::Action && implementation.type_name == "__unknown__" {
        validator.push_diagnostic(
            Diagnostic::new("Missing Actions Container Name")
                .with_error_code("E022")
                .with_location(&implementation.location),
        );
    }
}

pub fn visit_interface<T: AnnotationMap>(
    validator: &mut Validator,
    interface: &Interface,
    context: &ValidationContext<'_, T>,
) {
    let Identifier { name, location } = &interface.ident;
    let entry = context.index.find_interface(name).expect("must exist");
    entry.get_extensions().iter().for_each(|declaration| {
        if context.index.find_interface(&declaration.name).is_none() {
            validator.push_diagnostic(
                Diagnostic::new(format!("Interface `{}` does not exist", declaration.name))
                    .with_error_code("E048")
                    .with_location(&declaration.location),
            );
        }
    });
    validate_methods_overrides(validator, context, interface.id, name, location);
    property::validate_properties_in_interfaces(validator, context, interface);
}

pub(super) mod signature_validation {
    use itertools::Itertools;
    use plc_diagnostics::diagnostics::Diagnostic;
    use plc_source::source_location::SourceLocation;

    use crate::{
        index::{Index, PouIndexEntry},
        typesystem::{DataType, DataTypeInformation},
    };

    pub fn validate_method_signature(
        index: &Index,
        method_ref: &PouIndexEntry,
        method_impl: &PouIndexEntry,
        primary_location: &SourceLocation,
    ) -> Vec<Diagnostic> {
        let ctxt = Context::new(index, method_ref, method_impl, primary_location);
        let mut validator = SignatureValidator::new(&ctxt);
        validator.validate();
        validator.diagnostics
    }

    struct Context<'idx> {
        index: &'idx Index,
        method_ref: &'idx PouIndexEntry,
        method_impl: &'idx PouIndexEntry,
        primary_location: &'idx SourceLocation,
    }

    impl<'idx> Context<'idx> {
        fn new(
            index: &'idx Index,
            method_ref: &'idx PouIndexEntry,
            method_impl: &'idx PouIndexEntry,
            primary_location: &'idx SourceLocation,
        ) -> Self {
            Self { index, method_ref, method_impl, primary_location }
        }

        /// Returns a tuple of the return [DataType]s of the method reference and the method implementation
        fn get_return_types(&self) -> (&DataType, &DataType) {
            let method_ref_return_type_name = self.method_ref.get_return_type().unwrap_or_default();
            let method_impl_return_type_name = self.method_impl.get_return_type().unwrap_or_default();
            let return_type_ref = self.index.get_effective_type_or_void_by_name(method_ref_return_type_name);
            let return_type_impl =
                self.index.get_effective_type_or_void_by_name(method_impl_return_type_name);

            (return_type_ref, return_type_impl)
        }
    }

    struct SignatureValidator<'b, 'idx> {
        context: &'b Context<'idx>,
        diagnostics: Vec<Diagnostic>,
    }

    impl<'b, 'idx> SignatureValidator<'b, 'idx> {
        fn new(context: &'b Context<'idx>) -> Self {
            Self { context, diagnostics: Vec::new() }
        }

        fn validate(&mut self) {
            // Check if the return types match
            self.validate_return_types();

            // Check if the parameters match; note that the order of the parameters is important due to implicit calls.
            self.validate_parameters();
        }

        fn validate_return_types(&mut self) {
            let (return_type_ref, return_type_impl) = self.context.get_return_types();
            if let Some(sub_diagnostics) = self.validate_types(return_type_impl, return_type_ref) {
                self.diagnostics.push(
                    Diagnostic::new(
                        "Derived methods with conflicting signatures, return types do not match:",
                    )
                    .with_location(self.context.primary_location)
                    .with_error_code("E112")
                    .with_sub_diagnostics(sub_diagnostics),
                );
            }
        }

        fn validate_parameters(&mut self) {
            let context = self.context;
            let method_impl = context.method_impl;
            let method_ref = context.method_ref;
            let method_name = context.method_ref.get_call_name();
            let parameters_ref = context.index.get_available_parameters(method_ref.get_name());
            let parameters_impl = context.index.get_available_parameters(method_impl.get_name());
            let mut diagnostics = vec![];

            // Conditionally skip the first parameter if the return type is aggregate.
            // Return types have already been validated and we don't want to show errors
            // for internally modified code.
            let (return_type_ref, return_type_impl) = context.get_return_types();
            parameters_ref
                .iter()
                .skip(return_type_ref.get_type_information().is_aggregate() as usize)
                .zip_longest(parameters_impl.iter().skip(return_type_impl.get_type_information().is_aggregate() as usize))
                .for_each(|pair|
            {
                match pair {
                    itertools::EitherOrBoth::Both(parameter_ref, parameter_impl) => {
                        // Name
                        if parameter_impl.get_name() != parameter_ref.get_name() {
                            diagnostics.push(
                                Diagnostic::new(format!(
                                    "Expected parameter `{}` but got `{}`",
                                    parameter_ref.get_name(),
                                    parameter_impl.get_name()
                                ))
                                .with_error_code("E118")
                                .with_secondary_location(&parameter_ref.source_location)
                                .with_secondary_location(&parameter_impl.source_location),
                            );
                        }

                        // Type
                        let impl_ty = context
                            .index
                            .get_effective_type_or_void_by_name(parameter_impl.get_type_name());
                        let ref_ty = context
                            .index
                            .get_effective_type_or_void_by_name(parameter_ref.get_type_name());

                        if let Some(sub_diagnostics) = self.validate_types(impl_ty, ref_ty) {
                            diagnostics.push(
                                Diagnostic::new(format!(
                                    "Parameter `{}` has conflicting type declarations:",
                                    parameter_ref.get_name(),
                                ))
                                .with_error_code("E118")
                                .with_sub_diagnostics(sub_diagnostics)
                                .with_secondary_location(method_impl)
                                .with_secondary_location(&parameter_ref.source_location)
                            );
                        }

                        // Declaration Type (VAR_INPUT, VAR_OUTPUT, VAR_IN_OUT)
                        if parameter_impl.get_declaration_type() != parameter_ref.get_declaration_type() {
                            diagnostics.push(
                                Diagnostic::new(format!(
                                    "Expected parameter `{}` to have `{}` as its declaration type but got `{}`",
                                    parameter_impl.get_name(),
                                    parameter_ref.get_declaration_type().get_inner(),
                                    parameter_impl.get_declaration_type().get_inner(),
                                ))
                                .with_error_code("E118")
                                .with_secondary_location(method_impl)
                                .with_secondary_location(&parameter_ref.source_location),
                            );
                        }
                    }

                    itertools::EitherOrBoth::Left(parameter_ref) => {
                        diagnostics.push(
                            Diagnostic::new(format!(
                                "Parameter `{} : {}` missing in method `{}`",
                                parameter_ref.get_name(),
                                parameter_ref.get_type_name(),
                                method_name,
                            ))
                            .with_error_code("E118")
                            .with_secondary_location(method_impl)
                            .with_secondary_location(&parameter_ref.source_location),
                        );
                    }

                    // Exceeding parameters in the POU, which we did not catch in the for loop above because we were only
                    // iterating over the interface parameters; anyhow any exceeding parameter is considered an error because
                    // the function signature no longer holds
                    itertools::EitherOrBoth::Right(parameter_impl) => {
                        diagnostics.push(
                            Diagnostic::new(format!(
                                "`{}` has more parameters than the method defined in `{}`",
                                method_name,
                                method_ref.get_parent_pou_name().unwrap(),
                            ))
                            .with_error_code("E118")
                            .with_secondary_location(&parameter_impl.source_location)
                            .with_secondary_location(method_ref),
                        );
                    }
                }
            });

            if !diagnostics.is_empty() {
                self.diagnostics.push(
                    Diagnostic::new("Derived methods with conflicting signatures, parameters do not match:")
                        .with_error_code("E112")
                        .with_location(self.context.primary_location)
                        .with_sub_diagnostics(diagnostics),
                );
            }
        }

        fn validate_types(&self, left: &DataType, right: &DataType) -> Option<Vec<Diagnostic>> {
            let l_type_info = left.get_type_information();
            let r_type_info = right.get_type_information();
            if l_type_info == r_type_info {
                return None;
            }
            let context = self.context;
            match (l_type_info, r_type_info) {
                (DataTypeInformation::Array { .. }, DataTypeInformation::Array { .. }) => {
                    self.validate_array_types(left, right)
                }
                (
                    DataTypeInformation::Pointer { inner_type_name: l_inner, auto_deref: l_auto, .. },
                    DataTypeInformation::Pointer { inner_type_name: r_inner, auto_deref: r_auto, .. },
                ) => {
                    if l_auto.is_some() || r_auto.is_some() {
                        let left = l_auto
                            .map(|_| context.index.get_effective_type_or_void_by_name(l_inner))
                            .unwrap_or(left);
                        let right = r_auto
                            .map(|_| context.index.get_effective_type_or_void_by_name(r_inner))
                            .unwrap_or(right);
                        return self.validate_types(left, right);
                    }
                    self.validate_types(
                        context.index.get_effective_type_or_void_by_name(l_inner),
                        context.index.get_effective_type_or_void_by_name(r_inner),
                    )
                }
                (DataTypeInformation::Pointer { inner_type_name, auto_deref: Some(_), .. }, _) => {
                    let inner_type = context.index.get_effective_type_or_void_by_name(inner_type_name);
                    self.validate_types(inner_type, right)
                }
                (_, DataTypeInformation::Pointer { inner_type_name, auto_deref: Some(_), .. }) => {
                    let inner_type = context.index.get_effective_type_or_void_by_name(inner_type_name);
                    self.validate_types(left, inner_type)
                }
                (DataTypeInformation::String { .. }, DataTypeInformation::String { .. }) => {
                    self.validate_string_types(l_type_info, r_type_info)
                }
                (
                    DataTypeInformation::SubRange { sub_range: _l_range, .. },
                    DataTypeInformation::SubRange { sub_range: _r_range, .. },
                ) => {
                    self.validate_types(
                        context.index.get_intrinsic_type(left),
                        context.index.get_intrinsic_type(right),
                    )

                    // FIXME: properly validate the ranges (folded constants are problematic)
                }
                _ => Some(vec![self.create_diagnostic(l_type_info.get_name(), r_type_info.get_name())]),
            }
        }

        fn create_diagnostic(&self, left: &str, right: &str) -> Diagnostic {
            Diagnostic::new(format!(
                "Type `{right}` declared in `{}` but `{}` declared type `{left}`",
                self.context.method_ref.get_name(),
                self.context.method_impl.get_name()
            ))
            .with_error_code("E118")
            .with_secondary_location(self.context.method_impl)
            .with_secondary_location(self.context.method_ref)
        }

        fn validate_array_types(&self, left: &DataType, right: &DataType) -> Option<Vec<Diagnostic>> {
            let context = self.context;
            let method_impl = context.method_impl;
            let method_ref = context.method_ref;

            let mut sub_diagnostics = vec![];
            let mut left_type = left;
            let mut right_type = right;

            while let (Some(l_inner), Some(r_inner)) = (
                left_type.get_type_information().get_inner_array_type_name(),
                right_type.get_type_information().get_inner_array_type_name(),
            ) {
                // we found two inner types, so the previous iteration was array types. check their dimensions
                sub_diagnostics.extend(self.validate_array_dimensions(left_type, right));
                left_type = context.index.get_effective_type_or_void_by_name(l_inner);
                right_type = context.index.get_effective_type_or_void_by_name(r_inner);
            }

            let left_name = left_type.get_name();
            let right_name = right_type.get_name();
            if left_name != right_name {
                sub_diagnostics.push(
                    Diagnostic::new(format!(
                        "Expected array of type `{}` but got `{}`",
                        right_name, left_name
                    ))
                    .with_error_code("E118")
                    .with_secondary_location(method_impl)
                    .with_secondary_location(method_ref),
                )
            };

            (!sub_diagnostics.is_empty()).then_some(sub_diagnostics)
        }

        fn validate_array_dimensions(&self, left: &DataType, right: &DataType) -> Vec<Diagnostic> {
            let context = self.context;
            let left_type = left.get_type_information();
            let right_type = right.get_type_information();
            let l_dims = left_type.get_dimensions().expect("Array type without dimensions");
            let r_dims = right_type.get_dimensions().expect("Array type without dimensions");

            l_dims
                .iter()
                .zip_longest(r_dims.iter())
                .filter_map(|pair| {
                    match pair {
                        itertools::EitherOrBoth::Both(l, r) => {
                            match (l.get_range(context.index), r.get_range(context.index)) {
                                (Ok(l_range), Ok(r_range)) => (l_range != r_range).then_some(
                                    Diagnostic::new(format!(
                                        "Array range declared as `[{}..{}]` but implemented as `[{}..{}]`",
                                        r_range.start, r_range.end, l_range.start, l_range.end
                                    ))
                                    .with_error_code("E118")
                                    .with_secondary_location(left.location.clone())
                                    .with_secondary_location(right.location.clone()),
                                ),
                                _ => {
                                    // Expression in array dimension could not be evaluated. this should already have raised an error elsewhere
                                    None
                                }
                            }
                        }
                        itertools::EitherOrBoth::Left(_) | itertools::EitherOrBoth::Right(_) => Some(
                            Diagnostic::new(format!(
                                "Array declared with `{}` dimension{} but implemented with `{}`",
                                l_dims.len(),
                                if l_dims.len() == 1 { "" } else { "s" },
                                r_dims.len()
                            ))
                            .with_error_code("E118")
                            .with_secondary_location(right.location.clone())
                            .with_secondary_location(context.method_impl)
                            .with_secondary_location(left.location.clone())
                            .with_secondary_location(context.method_ref),
                        ),
                    }
                })
                .collect()
        }

        fn validate_string_types(
            &self,
            left: &DataTypeInformation,
            right: &DataTypeInformation,
        ) -> Option<Vec<Diagnostic>> {
            let ctxt = self.context;
            let method_impl = ctxt.method_impl;
            let method_ref = ctxt.method_ref;
            //FIXME: this is not accurate, we should not do our own data layout calculations
            let l_encoding = left.get_string_character_width(ctxt.index);
            let r_encoding = right.get_string_character_width(ctxt.index);
            let left_length = left.get_size_in_bits(ctxt.index).unwrap() / l_encoding.bits();
            let right_length = right.get_size_in_bits(ctxt.index).unwrap() / r_encoding.bits();
            let mut sub_diagnostics = vec![];
            (left_length != right_length).then(|| {
                sub_diagnostics.push(
                    Diagnostic::new(format!(
                        "Expected string of length `{}` but got string of length `{}`",
                        right_length, left_length
                    ))
                    .with_error_code("E118")
                    .with_secondary_location(method_impl)
                    .with_secondary_location(method_ref),
                );
            });
            (l_encoding != r_encoding).then(|| {
                sub_diagnostics.push(self.create_diagnostic(left.get_name(), right.get_name()));
            });

            (!sub_diagnostics.is_empty()).then_some(sub_diagnostics)
        }
    }
}
