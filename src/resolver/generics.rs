use itertools::Itertools;
use plc_ast::ast::{flatten_expression_list, AstNode, AstStatement, GenericBinding, LinkageType, TypeNature};
use plc_source::source_location::SourceLocation;
use rustc_hash::FxHashMap;

use crate::{
    builtins,
    codegen::generators::expression_generator::get_implicit_call_parameter,
    index::{ArgumentType, PouIndexEntry, VariableType},
    resolver::AnnotationMap,
    typesystem::{
        self, DataType, DataTypeInformation, StringEncoding, BOOL_TYPE, CHAR_TYPE, DATE_TYPE, REAL_TYPE,
        SINT_TYPE, STRING_TYPE, TIME_TYPE, USINT_TYPE, WSTRING_TYPE,
    },
};

use super::{StatementAnnotation, TypeAnnotator, VisitorContext};

#[derive(Debug)]
pub struct GenericType {
    // this is the derived type used for the generic call
    derived_type: String,
    // this is the nature the generic was declared with
    generic_nature: TypeNature,
}

// Utility methods handling generic resolution
impl TypeAnnotator<'_> {
    /// determines a possible generic for the current statement
    /// returns a pair with the possible generics symbol and the real datatype
    /// e.g. `( "T", "INT" )`
    pub fn get_generic_candidate(&self, type_name: &str, statement: &AstNode) -> Option<(&str, &str)> {
        //find inner type if this was turned into an array or pointer (if this is `POINTER TO T` lets find out what T is)
        let effective_type = self.index.find_effective_type_info(type_name);
        let candidate = match effective_type {
            Some(DataTypeInformation::Pointer { inner_type_name, .. })
            | Some(DataTypeInformation::Array { inner_type_name, .. }) => {
                self.index.find_effective_type_info(inner_type_name)
            }
            _ => effective_type,
        };

        //If generic add a generic annotation
        if let Some(DataTypeInformation::Generic { generic_symbol, .. }) = candidate {
            let statement = match statement.get_stmt() {
                //The right side of the assignment is the source of truth
                AstStatement::Assignment(data) => &data.right,
                _ => statement,
            };
            //Find the statement's type
            self.annotation_map
                .get_type(statement, self.index)
                .map(|it| (generic_symbol.as_str(), it.get_name()))
        } else {
            None
        }
    }

    /// Updates the generic information of a function call. It collects all candidates for a generic function
    /// then chooses the best fitting function signature and reannotates the function with the found information.
    pub(crate) fn update_generic_call_statement(
        &mut self,
        generics_candidates: FxHashMap<String, Vec<String>>,
        implementation_name: &str,
        operator: &AstNode,
        parameters: &AstNode,
        ctx: VisitorContext,
    ) {
        if let Some(PouIndexEntry::Function { generics, .. }) = self.index.find_pou(implementation_name) {
            if !generics.is_empty() {
                let generic_map = &self.derive_generic_types(generics, generics_candidates);
                // Annotate the statement with the new function call
                if let Some(StatementAnnotation::Function { qualified_name, return_type, .. }) =
                    self.annotation_map.get(operator)
                {
                    // Find the generic resolver
                    let generic_name_resolver = builtins::get_builtin(qualified_name)
                        .map(|it| it.get_generic_name_resolver())
                        .unwrap_or_else(|| generic_name_resolver);
                    // get information about the generic function name and annotation
                    let (new_name, annotation) = self.get_specific_function_annotation(
                        generics,
                        qualified_name,
                        return_type,
                        generic_map,
                        generic_name_resolver,
                    );

                    // Create a new pou and implementation for the function
                    if let Some(pou) = self.index.find_pou(qualified_name) {
                        // only register concrete typed function if it was not indexed yet
                        if self.index.find_pou(new_name.as_str()).is_none() &&
                            //only register typed function if we did not register it yet
                            self.annotation_map.new_index.find_pou(new_name.as_str()).is_none()
                        {
                            if let StatementAnnotation::Function { return_type, .. } = &annotation {
                                // register the pou-entry, implementation and member-variables for the requested (typed) implementation
                                // e.g. call to generic_foo(aInt)
                                self.register_generic_pou_entries(
                                    pou,
                                    return_type.as_str(),
                                    new_name.as_str(),
                                    generic_map,
                                );
                            } else {
                                unreachable!("Annotation must be a function but was {:?}", &annotation)
                            }
                        }
                    }
                    // annotate the call-statement so it points to the new implementation
                    self.annotate(operator, annotation);
                }
                // Adjust annotations on the inner statement
                self.visit_statement(&ctx, parameters);
                self.update_generic_function_parameters(parameters, implementation_name, generic_map);
            }
        }
    }

    /// douplicates the given generic_function under the `new_name` (e.g. foo__INT__REAL)using the
    /// real datatypes for the generics as given in `generics` (e.g. { T=INT, U=REAL})
    pub fn register_generic_pou_entries(
        &mut self,
        generic_function: &PouIndexEntry,
        return_type_name: &str,
        new_name: &str,
        generics: &FxHashMap<String, GenericType>,
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
                generic_implementation.get_location().to_owned(),
            );

            let return_type = self.index.find_type(return_type_name).unwrap_or(self.index.get_void_type());
            //register a copy of the pou under the new name
            self.annotation_map.new_index.register_pou(PouIndexEntry::create_generated_function_entry(
                new_name,
                return_type_name,
                &[],
                LinkageType::External, //it has to be external, we should have already found this in the global index if it was internal
                generic_function.is_variadic(),
                generic_function.get_location().clone(),
                generic_function.is_constant(),
            ));

            // register the member-variables (interface) of the new function
            // copy each member-index-entry and make sure to turn the generic (e.g. T)
            // into the concrete type (e.g. INT)
            let old_dataype = self.index.find_pou_type(generic_function.get_name()).unwrap_or_else(|| {
                panic!("The POU {} should be in the types index", generic_function.get_name())
            });
            let information = if let DataTypeInformation::Struct { members, source, .. } =
                old_dataype.get_type_information()
            {
                let members = members
                    .iter()
                    .map(|member| {
                        let new_type_name = self.find_or_create_datatype(member.get_type_name(), generics);
                        //register the member under the new container (old: foo__T, new: foo__INT)
                        //with its new type-name (old: T, new: INT)
                        let mut entry = member.into_typed(new_name, &new_type_name);
                        if !return_type.is_aggregate_type() {
                            return entry;
                        }

                        if member.is_return() && !generic_function.is_builtin() {
                            let data_type_name =
                                crate::index::indexer::pou_indexer::register_byref_pointer_type_for(
                                    &mut self.annotation_map.new_index,
                                    entry.get_type_name(),
                                    true, // TODO(vosa): Is this correct? Why are generating a pointer type here
                                );
                            entry = member.into_typed(new_name, &data_type_name);
                            entry.location_in_parent = 0;
                            entry.argument_type = ArgumentType::ByRef(VariableType::InOut);
                        } else {
                            entry.location_in_parent += 1;
                        }

                        entry
                    })
                    .sorted_by(|a, b| a.location_in_parent.cmp(&b.location_in_parent))
                    .collect::<Vec<_>>();
                DataTypeInformation::Struct { name: new_name.to_string(), source: source.clone(), members }
            } else {
                unreachable!("The function {} type is always a struct", old_dataype.get_name())
            };

            let new_datatype = DataType {
                name: new_name.to_string(),
                information,
                initial_value: old_dataype.initial_value.to_owned(),
                location: old_dataype.location.to_owned(),
                nature: old_dataype.nature.to_owned(),
            };

            self.annotation_map.new_index.register_pou_type(new_datatype);
        }
    }

    fn find_or_create_datatype(
        &mut self,
        member_name: &str,
        generics: &FxHashMap<String, GenericType>,
    ) -> String {
        match self.index.find_effective_type_info(member_name) {
            Some(DataTypeInformation::Generic { generic_symbol, .. }) => {
                // this is a generic member, so lets see what it's generic symbol is and translate it
                generics
                    .get(generic_symbol)
                    .map(|it| it.derived_type.as_str())
                    .unwrap_or_else(|| member_name)
                    .to_string()
            }
            Some(DataTypeInformation::Pointer {
                name,
                inner_type_name,
                auto_deref: Some(kind),
                type_safe,
                is_function,
            }) => {
                // This is an auto deref pointer (VAR_IN_OUT or VAR_INPUT {ref}) that points to a
                // generic. We first resolve the generic type, then create a new pointer type of
                // the combination
                let inner_type_name = self.find_or_create_datatype(inner_type_name, generics);
                let name = format!("{name}__{inner_type_name}"); // TODO: Naming convention (see plc_util/src/convention.rs)
                let new_type_info = DataTypeInformation::Pointer {
                    name: name.clone(),
                    inner_type_name,
                    auto_deref: Some(*kind),
                    type_safe: *type_safe,
                    is_function: *is_function,
                };

                // Registers a new pointer type to the index
                self.annotation_map.new_index.register_type(DataType {
                    information: new_type_info,
                    initial_value: None,
                    name: name.clone(),
                    nature: TypeNature::Any,
                    location: SourceLocation::internal(),
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
        s: &AstNode,
        function_name: &str,
        generic_map: &FxHashMap<String, GenericType>,
    ) {
        /// An internal struct used to hold the type and nature of a generic parameter
        struct TypeAndNature<'a> {
            datatype: &'a typesystem::DataType,
            nature: TypeNature,
        }

        let declared_parameters = self.index.get_available_parameters(function_name);
        // separate variadic and non variadic parameters
        let mut passed_parameters = Vec::new();
        let mut variadic_parameters = Vec::new();
        for (i, p) in flatten_expression_list(s).iter().enumerate() {
            if let Ok((location_in_parent, passed_parameter, ..)) =
                get_implicit_call_parameter(p, &declared_parameters, i)
            {
                if let Some(declared_parameter) = declared_parameters.get(location_in_parent) {
                    passed_parameters.push((*p, passed_parameter, *declared_parameter));
                } else {
                    // variadic parameters are not included in declared_parameters
                    variadic_parameters.push(passed_parameter);
                }
            }
        }
        for (parameter_stmt, passed_parameter, declared_parameter) in passed_parameters.iter() {
            // check if declared parameter is generic
            if let Some(DataTypeInformation::Generic { generic_symbol, .. }) = self
                .index
                .find_effective_type_info(declared_parameter.get_type_name())
                .map(|t| self.index.find_elementary_pointer_type(t))
            {
                // get generic type of the declared parameter this will be our type hint as the expected type
                if let Some(generic) = generic_map.get(generic_symbol) {
                    if let Some(datatype) =
                        self.index.find_effective_type_by_name(generic.derived_type.as_str())
                    {
                        // annotate the type hint for the passed parameter
                        self.annotation_map.annotate_type_hint(
                            passed_parameter,
                            StatementAnnotation::value(datatype.get_name()),
                        );
                        // annotate the generic type nature of the passed parameter, this is the actual nature of the generic declaration
                        self.annotation_map.add_generic_nature(passed_parameter, generic.generic_nature);

                        // for assignments we need to annotate the left side aswell
                        match parameter_stmt.get_stmt() {
                            AstStatement::Assignment(data) | AstStatement::OutputAssignment(data) => {
                                self.annotate(&data.left, StatementAnnotation::value(datatype.get_name()));
                            }
                            _ => {}
                        }
                    }
                }
            }
        }

        // Then handle the varargs
        // Get the variadic argument if any
        if let Some(dt) = self.index.get_variadic_member(function_name).map(|it| {
            // if the member is generic
            if let Some(DataTypeInformation::Generic { generic_symbol, nature, .. }) =
                self.index.find_effective_type_info(it.get_type_name())
            {
                let real_type = generic_map
                    .get(generic_symbol)
                    .and_then(|it| self.index.find_effective_type_by_name(it.derived_type.as_str()))
                    .map(|datatype| TypeAndNature { datatype, nature: *nature });
                real_type
            } else {
                None
            }
        }) {
            for p in variadic_parameters {
                if let Some(TypeAndNature { datatype, nature }) = dt {
                    self.annotation_map.add_generic_nature(p, nature);
                    self.annotation_map
                        .annotate_type_hint(p, StatementAnnotation::value(datatype.get_name()));
                }
            }
        }
    }

    // takes the generic signature of a function and resolves it ot its specific types according to generic_map and the generic_name_resolver
    pub fn get_specific_function_annotation(
        &self,
        generics: &[GenericBinding],
        generic_qualified_name: &str,
        generic_return_type: &str,
        generic_map: &FxHashMap<String, GenericType>,
        generic_name_resolver: GenericNameResolver,
    ) -> (String, StatementAnnotation) {
        let call_name = generic_name_resolver(generic_qualified_name, generics, generic_map);
        let annotation = self
            .index
            .find_pou(&call_name)
            .filter(|it| !it.is_generic())
            .map(StatementAnnotation::from)
            .map(|it| it.with_generic_name(generic_qualified_name))
            .unwrap_or_else(|| {
                let return_type = if let DataTypeInformation::Generic { generic_symbol, .. } =
                    self.index.get_type_information_or_void(generic_return_type)
                {
                    generic_map
                        .get(generic_symbol)
                        .map(|it| it.derived_type.as_str())
                        .unwrap_or(generic_return_type)
                } else {
                    generic_return_type
                }
                .to_string();
                StatementAnnotation::Function {
                    qualified_name: generic_qualified_name.to_string(),
                    return_type,
                    generic_name: Some(generic_qualified_name.to_string()),
                    call_name: Some(call_name.clone()),
                }
            });
        (call_name, annotation)
    }

    /// Derives the correct type for the generic call from the list of parameters
    pub fn derive_generic_types(
        &self,
        generics: &[GenericBinding],
        generics_candidates: FxHashMap<String, Vec<String>>,
    ) -> FxHashMap<String, GenericType> {
        let mut generic_map: FxHashMap<String, GenericType> = FxHashMap::default();
        for GenericBinding { name, nature } in generics {
            let smallest_possible_type =
                self.index.find_effective_type_info(get_smallest_possible_type(nature));
            //Get the current binding
            if let Some(candidates) = generics_candidates.get(name) {
                //Find the best suiting type
                let winner = candidates
                    .iter()
                    .fold(smallest_possible_type, |previous_type: Option<&DataTypeInformation>, current| {
                        let current_type = self
                            .index
                            .find_effective_type_info(current)
                            // if type is not found, look for it in new index, because the type could have been created recently
                            .or_else(|| self.annotation_map.new_index.find_effective_type_info(current))
                            .map(|it| {
                                match it {
                                    // generic strings are a special case and need to be handled differently
                                    DataTypeInformation::String {
                                        encoding: StringEncoding::Utf8, ..
                                    } => self.index.find_effective_type_info(STRING_TYPE).unwrap_or(it),
                                    DataTypeInformation::String {
                                        encoding: StringEncoding::Utf16, ..
                                    } => self.index.find_effective_type_info(WSTRING_TYPE).unwrap_or(it),
                                    _ => self.index.get_intrinsic_type_information(it),
                                }
                            });

                        // Find bigger
                        if let Some(current) = current_type {
                            // check if the current type derives from the generic nature
                            if self
                                .index
                                .find_effective_type_by_name(current.get_name())
                                .map(|t| {
                                    t.has_nature(*nature, self.index)
                                        // INT parameter for REAL is allowed
                                            | (nature.is_real() & t.is_numerical())
                                })
                                .unwrap_or_default()
                            {
                                // if we got the right nature we can search for the bigger type
                                if let Some(previous) = previous_type {
                                    return Some(typesystem::get_bigger_type(current, previous, self.index));
                                } else {
                                    // if the previous type was None just return the current
                                    // type should be ok because of the previouse nature check
                                    return current_type;
                                }
                            }
                        }
                        // if we didn't get the right nature return the last one
                        previous_type
                    })
                    .map(DataTypeInformation::get_name);
                if let Some(winner) = winner {
                    generic_map.insert(
                        name.into(),
                        GenericType { derived_type: winner.to_string(), generic_nature: *nature },
                    );
                }
            }
        }
        generic_map
    }
}

type GenericNameResolver = fn(&str, &[GenericBinding], &FxHashMap<String, GenericType>) -> String;

/// Builds the correct generic name from the given information
pub fn generic_name_resolver(
    qualified_name: &str,
    generics: &[GenericBinding],
    generic_map: &FxHashMap<String, GenericType>,
) -> String {
    generics
        .iter()
        .map(|it| {
            generic_map.get(&it.name).map(|it| it.derived_type.as_str()).unwrap_or_else(|| it.name.as_str())
        })
        .fold(qualified_name.to_string(), |accum, s| format!("{accum}__{s}")) // TODO: Naming convention (see plc_util/src/convention.rs)
}

/// This method returns the qualified name, but has the same signature as the generic resolver to be used in builtins
pub fn no_generic_name_resolver(
    qualified_name: &str,
    _: &[GenericBinding],
    _: &FxHashMap<String, GenericType>,
) -> String {
    generic_name_resolver(qualified_name, &[], &FxHashMap::default())
}

pub fn get_smallest_possible_type(nature: &TypeNature) -> &str {
    match nature {
        TypeNature::Magnitude | TypeNature::Num | TypeNature::Int => USINT_TYPE,
        TypeNature::Real => REAL_TYPE,
        TypeNature::Unsigned => USINT_TYPE,
        TypeNature::Signed => SINT_TYPE,
        TypeNature::Duration => TIME_TYPE,
        TypeNature::Bit => BOOL_TYPE,
        TypeNature::Chars | TypeNature::Char => CHAR_TYPE,
        TypeNature::String => STRING_TYPE,
        TypeNature::Date => DATE_TYPE,
        _ => "",
    }
}
