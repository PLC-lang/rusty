use std::collections::HashMap;

use crate::{
    ast::{self, AstStatement, GenericBinding, LinkageType, TypeNature},
    builtins,
    index::{Index, PouIndexEntry, VariableIndexEntry},
    resolver::AnnotationMap,
    typesystem::{self, DataType, DataTypeInformation},
};

use super::{AnnotationMapImpl, StatementAnnotation, TypeAnnotator, VisitorContext};

// Utility methods handling generic resolution
impl<'i> TypeAnnotator<'i> {
    /// determines a possible generic for the current statement
    /// returns a pair with the possible generics symbol and the real datatype
    /// e.g. `( "T", "INT" )`
    pub(super) fn get_generic_candidate<'idx>(
        index: &'idx Index,
        annotation_map: &'idx AnnotationMapImpl,
        type_name: &str,
        statement: &AstStatement,
    ) -> Option<(&'idx str, &'idx str)> {
        //find inner type if this was turned into an array or pointer (if this is `POINTER TO T` lets find out what T is)
        let effective_type = index.find_effective_type_info(type_name);
        let candidate = match effective_type {
            Some(DataTypeInformation::Pointer {
                inner_type_name, ..
            })
            | Some(DataTypeInformation::Array {
                inner_type_name, ..
            }) => index.find_effective_type_info(inner_type_name),
            _ => effective_type,
        };

        //If generic add a generic annotation
        if let Some(DataTypeInformation::Generic { generic_symbol, .. }) = candidate {
            let statement = match statement {
                //The right side of the assignment is the source of truth
                AstStatement::Assignment { right, .. } => right,
                _ => statement,
            };
            //Find the statement's type
            annotation_map
                .get_type(statement, index)
                .map(|it| (generic_symbol.as_str(), it.get_name()))
        } else {
            None
        }
    }

    /// Updates the generic information of a function call
    /// It collects all candidates for a generic function
    /// Then chooses the best fitting function signature
    /// And reannotates the function with the found information
    pub(crate) fn update_generic_call_statement(
        &mut self,
        generics_candidates: HashMap<String, Vec<String>>,
        implementation_name: &str,
        operator: &AstStatement,
        parameters: Option<&AstStatement>,
        ctx: VisitorContext,
    ) {
        if let Some(PouIndexEntry::Function { generics, .. }) =
            self.index.find_pou(implementation_name)
        {
            if !generics.is_empty() {
                let generic_map = &self.derive_generic_types(generics, generics_candidates);
                //Annotate the statement with the new function call
                if let Some(StatementAnnotation::Function {
                    qualified_name,
                    return_type,
                    ..
                }) = self.annotation_map.get(operator)
                {
                    let cloned_return_type = return_type.clone(); //borrow checker will not allow to use return_type below :-(

                    //Find the generic resolver
                    let generic_name_resolver = builtins::get_builtin(qualified_name)
                        .map(|it| it.get_generic_name_resolver())
                        .unwrap_or_else(|| generic_name_resolver);
                    //Figure out the new name for the call
                    let (new_name, annotation) = self.get_generic_function_annotation(
                        generics,
                        qualified_name,
                        return_type,
                        generic_map,
                        generic_name_resolver,
                    );

                    //Create a new pou and implementation for the function
                    if let Some(pou) = self.index.find_pou(qualified_name) {
                        //only register concrete typed function if it was not indexed yet
                        if self.index.find_pou(new_name.as_str()).is_none() {
                            //register the pou-entry, implementation and member-variables for the requested (typed) implemmentation
                            // e.g. call to generic_foo(aInt)
                            self.register_generic_pou_entries(
                                pou,
                                cloned_return_type.as_str(),
                                new_name.as_str(),
                                generic_map,
                            );
                        }
                    }

                    //annotate the call-statement so it points to the new implementation
                    self.annotation_map.annotate(operator, annotation);
                }
                //Adjust annotations on the inner statement
                if let Some(s) = parameters.as_ref() {
                    self.visit_statement(&ctx, s);
                    self.update_generic_function_parameters(s, implementation_name, generic_map);
                }
            }
        }
    }

    /// douplicates the given generic_function under the `new_name` (e.g. foo__INT__REAL)using the
    /// real datatypes for the generics as given in `generics` (e.g. { T=INT, U=REAL})
    pub fn register_generic_pou_entries(
        &mut self,
        generic_function: &PouIndexEntry,
        return_type: &str,
        new_name: &str,
        generics: &HashMap<String, String>,
    ) {
        // the generic implementation
        if let Some(generic_implementation) = generic_function.find_implementation(self.index) {
            //register a copy of the generic's implemntation under the new name
            self.annotation_map.new_index.register_implementation(
                new_name,
                new_name,
                generic_implementation.get_associated_class_name(),
                generic_implementation.get_implementation_type().clone(),
                false,
            );

            //register a copy of the pou under the new name
            self.annotation_map
                .new_index
                .register_pou(PouIndexEntry::create_function_entry(
                    new_name,
                    return_type,
                    &[],
                    LinkageType::External, //it has to be external, we should have already found this in the global index if it was internal
                    generic_function.is_variadic(),
                ));

            // register the member-variables (interface) of the new function
            // copy each member-index-entry and make sure to turn the generic (e.g. T)
            // into the concrete type (e.g. INT)
            if let Some(generic_function_members) =
                self.index.get_members(generic_function.get_name())
            {
                for (_, member) in generic_function_members {
                    let new_type_name =
                        self.find_or_create_datatype(member.get_type_name(), generics);

                    //register the member under the new container (old: foo__T, new: foo__INT)
                    //with its new type-name (old: T, new: INT)
                    let entry = member.into_typed(new_name, &new_type_name);
                    self.annotation_map
                        .new_index
                        .register_member_entry(new_name, entry);
                }
            }
        }
    }

    fn find_or_create_datatype(
        &mut self,
        member_name: &str,
        generics: &HashMap<String, String>,
    ) -> String {
        match self.index.find_effective_type_info(member_name) {
            Some(DataTypeInformation::Generic { generic_symbol, .. }) => {
                // this is a generic member, so lets see what it's generic symbol is and translate it
                generics
                    .get(generic_symbol)
                    .map(String::as_str)
                    .unwrap_or_else(|| member_name)
                    .to_string()
            }
            Some(DataTypeInformation::Pointer {
                name,
                inner_type_name,
                auto_deref: true,
            }) => {
                // This is an auto deref pointer (VAR_IN_OUT or VAR_INPUT {ref}) that points to a
                // generic. We first resolve the generic type, then create a new pointer type of
                // the combination
                let inner_type_name = self.find_or_create_datatype(inner_type_name, generics);
                let name = format!("{name}__{inner_type_name}");
                let new_type_info = DataTypeInformation::Pointer {
                    name: name.clone(),
                    inner_type_name,
                    auto_deref: true,
                };

                // Registers a new pointer type to the index
                self.annotation_map.new_index.register_type(DataType {
                    information: new_type_info,
                    initial_value: None,
                    name: name.clone(),
                    nature: TypeNature::Any,
                });

                name
            }
            _ => {
                // not a generic member, just use the original type
                member_name.to_string()
            }
        }
    }

    fn update_generic_function_parameters(
        &mut self,
        s: &AstStatement,
        function_name: &str,
        generic_map: &HashMap<String, String>,
    ) {
        /// An internal struct used to hold the type and nature of a generic parameter
        struct TypeAndNature<'a> {
            datatype: &'a typesystem::DataType,
            nature: TypeNature,
        }

        // Map the input or output parameters of the function into a list of Index Entry with an optional generic type discription
        let parameters = ast::flatten_expression_list(s);
        let members: Vec<(&VariableIndexEntry, Option<TypeAndNature>)> = self
            .index
            .get_declared_parameters(function_name)
            .into_iter()
            .map(|it| {
                //if the member is generic
                if let Some(DataTypeInformation::Generic {
                    generic_symbol,
                    nature,
                    ..
                }) = self.index.find_effective_type_info(it.get_type_name())
                {
                    let real_type = generic_map
                        .get(generic_symbol)
                        .and_then(|it| self.index.find_effective_type(it))
                        .map(|datatype| TypeAndNature {
                            datatype,
                            nature: *nature,
                        });
                    (it, real_type)
                } else {
                    (it, None)
                }
            })
            .collect();

        //See if parameters have assignments, as they need to be treated differently
        if parameters.iter().any(|it| {
            matches!(
                it,
                AstStatement::Assignment { .. } | AstStatement::OutputAssignment { .. }
            )
        }) {
            for p in parameters {
                match p {
                    AstStatement::Assignment { left, right, .. }
                    | AstStatement::OutputAssignment { left, right, .. } => {
                        if let AstStatement::Reference { name, .. } = &**left {
                            //Find the member with that name
                            if let Some((_, Some(TypeAndNature { datatype, nature }))) =
                                members.iter().find(|(it, _)| it.get_name() == name)
                            {
                                self.annotation_map.add_generic_nature(p, *nature);
                                self.annotation_map.annotate(
                                    left,
                                    StatementAnnotation::value(datatype.get_name()),
                                );
                                self.update_right_hand_side_expected_type(left, right);
                            }
                        }
                    }
                    _ => { /*do nothing*/ }
                }
            }
        } else {
            //First handle the declared params
            let mut parameters = parameters.into_iter();
            for (_, dt) in members {
                if let Some(p) = parameters.next() {
                    if let Some(TypeAndNature { datatype, nature }) = dt {
                        self.annotation_map.add_generic_nature(p, nature);
                        self.annotation_map
                            .annotate_type_hint(p, StatementAnnotation::value(datatype.get_name()));
                    }
                }
            }
            //Then handle the varargs
            //Get the variadic argument if any
            if let Some(dt) = self.index.get_variadic_member(function_name).map(|it| {
                //if the member is generic
                if let Some(DataTypeInformation::Generic {
                    generic_symbol,
                    nature,
                    ..
                }) = self.index.find_effective_type_info(it.get_type_name())
                {
                    let real_type = generic_map
                        .get(generic_symbol)
                        .and_then(|it| self.index.find_effective_type(it))
                        .map(|datatype| TypeAndNature {
                            datatype,
                            nature: *nature,
                        });
                    real_type
                } else {
                    None
                }
            }) {
                for p in parameters {
                    if let Some(TypeAndNature { datatype, nature }) = dt {
                        self.annotation_map.add_generic_nature(p, nature);
                        self.annotation_map
                            .annotate_type_hint(p, StatementAnnotation::value(datatype.get_name()));
                    }
                }
            }
        }
    }
    pub fn get_generic_function_annotation(
        &self,
        generics: &[GenericBinding],
        qualified_name: &str,
        return_type: &str,
        generic_map: &HashMap<String, String>,
        generic_name_resolver: GenericNameResolver,
    ) -> (String, StatementAnnotation) {
        let call_name = generic_name_resolver(qualified_name, generics, generic_map);
        let annotation = self.index.find_pou(&call_name).map(
            |it| StatementAnnotation::from(it) ).unwrap_or_else(|| {    
                let return_type = if let DataTypeInformation::Generic { generic_symbol, .. } =
                    self.index.get_type_information_or_void(return_type)
                {
                    generic_map
                        .get(generic_symbol)
                        .map(String::as_str)
                        .unwrap_or(return_type)
                } else {
                    return_type
                }
                .to_string();
                StatementAnnotation::Function {
                    qualified_name: qualified_name.to_string(),
                    return_type,
                    call_name: Some(call_name.clone()),
                }
            });
            (
                call_name,
                annotation   
            )
    }

    /// Derives the correct type for the generic call from the list of parameters
    pub fn derive_generic_types(
        &self,
        generics: &[GenericBinding],
        generics_candidates: HashMap<String, Vec<String>>,
    ) -> HashMap<String, String> {
        let mut generic_map: HashMap<String, String> = HashMap::new();
        for GenericBinding { name, .. } in generics {
            //Get the current binding
            if let Some(candidates) = generics_candidates.get(name) {
                //Find the best suiting type
                let winner = candidates
                    .iter()
                    .fold(
                        None,
                        |previous_type: Option<&DataTypeInformation>, current| {
                            let current_type = self
                                .index
                                .find_effective_type_info(current)
                                .map(|it| self.index.find_intrinsic_type(it));
                            //Find bigger
                            if let Some((previous, current)) = previous_type.zip(current_type) {
                                Some(typesystem::get_bigger_type(previous, current, self.index))
                            } else {
                                current_type
                            }
                        },
                    )
                    .map(DataTypeInformation::get_name);
                if let Some(winner) = winner {
                    generic_map.insert(name.into(), winner.into());
                }
            }
        }
        generic_map
    }
}

type GenericNameResolver = fn(&str, &[GenericBinding], &HashMap<String, String>) -> String;

/// Builds the correct generic name from the given information
pub fn generic_name_resolver(
    qualified_name: &str,
    generics: &[GenericBinding],
    generic_map: &HashMap<String, String>,
) -> String {
    generics
        .iter()
        .map(|it| {
            generic_map
                .get(&it.name)
                .map(String::as_str)
                .unwrap_or_else(|| it.name.as_str())
        })
        .fold(qualified_name.to_string(), |accum, s| {
            format!("{accum}__{s}")
        })
}

/// This method returns the qualified name, but has the same signature as the generic resover to be used in builtins
pub fn no_generic_name_resolver(
    qualified_name: &str,
    _: &[GenericBinding],
    _: &HashMap<String, String>,
) -> String {
    generic_name_resolver(qualified_name, &[], &HashMap::new())
}
