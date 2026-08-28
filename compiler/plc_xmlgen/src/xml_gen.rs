use std::{
    borrow::Cow,
    collections::HashSet,
    fs::{File, copy},
    io::{Error, Read, Seek, SeekFrom},
    ops::Range,
    path::{Path, PathBuf},
};

use super::serializer::*;

use plc_ast::ast::*;

use chrono::Local;
use plc_source::source_location::{CodeSpan, TextLocation};
use xml::{
    EmitterConfig, EventWriter, attribute::Attribute, common::XmlVersion, name::Name, namespace::Namespace,
    writer::XmlEvent,
};

#[derive(Debug, Default)]
pub struct GenerationParameters {
    pub output_xml_omron: bool,
}

impl GenerationParameters {
    pub fn new() -> Self {
        GenerationParameters { output_xml_omron: false }
    }
}

/// <?xml version=\"1.0\"?>
/// <Project xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\" xmlns:smcext=\"https://www.ia.omron.com/Smc\" xsi:schemaLocation=\"https://www.ia.omron.com/Smc IEC61131_10_Ed1_0_SmcExt1_0_Spc1_0.xsd\" schemaVersion=\"1\" xmlns=\"www.iec.ch/public/TC65SC65BWG7TF10\">
///     <FileHeader companyName=\"OMRON Corporation\" productName=\"Sysmac Studio\" productVersion=\"1.30.0.0\" />
///     <ContentHeader name=\"Sample\" creationDateTime="">
///     </ContentHeader>
///     <Types>
///         <GlobalNamespace>
///         </GlobalNamespace>
///     </Types>
///     <Instances>
///     </Instances>
/// </Project>
pub fn get_omron_template() -> Node {
    Node::new_str("Project")
        .attribute_str("xmlns:xsi", "http://www.w3.org/2001/XMLSchema-instance")
        .attribute_str("xmlns:smcext", "https://www.ia.omron.com/Smc")
        .attribute_str("xsi:schemaLocation", OMRON_SCHEMA)
        .attribute_str("schemaVersion", "1")
        .attribute_str("xmlns", "www.iec.ch/public/TC65SC65BWG7TF10")
        .child(
            &SFileHeader::new()
                .attribute_str("companyName", "OMRON Corporation")
                .attribute_str("productName", "Sysmac Studio")
                .attribute_str("productVersion", "1.30.0.0"),
        )
        .child(
            &SContentHeader::new()
                .attribute_str("name", "Sample")
                .attribute(String::from("creationDateTime"), Local::now().to_rfc3339()),
        )
        .child(&STypes::new().child(&SGlobalNamespace::new()))
        .child(&SInstances::new())
}

pub const OMRON_SCHEMA: &str = "https://www.ia.omron.com/Smc IEC61131_10_Ed1_0_SmcExt1_0_Spc1_0.xsd";

pub const STRUCT_TYPE_SPEC: &str = "StructTypeSpec";
pub const ENUM_TYPE_SPEC: &str = "EnumTypeWithNamedValueSpec";

pub const POINTER_UNSUPPORTED_BY_OMRON: &str =
    "Sysmac Studio has no pointer type, so POINTER TO cannot be exported with --xml-omron";

pub const WSTRING_UNSUPPORTED_BY_OMRON: &str =
    "Sysmac Studio has no wide string type, so WSTRING cannot be exported with --xml-omron";

pub fn find_unsupported_omron_type(
    units: &[&CompilationUnit],
) -> Option<(String, plc_source::source_location::SourceLocation)> {
    for &current_unit in units {
        let unit_name = current_unit.file.get_name().unwrap_or("");

        if !unit_name.to_lowercase().ends_with(".st") {
            continue;
        }

        for current_usertype in &current_unit.user_types {
            if current_usertype.linkage == LinkageType::External {
                continue;
            }

            if current_usertype.location.span == CodeSpan::None {
                continue;
            }

            let (reason, name) = match &current_usertype.data_type {
                DataType::PointerType { name, .. } => (POINTER_UNSUPPORTED_BY_OMRON, name),
                DataType::StringType { name, is_wide: true, .. } => (WSTRING_UNSUPPORTED_BY_OMRON, name),
                _ => {
                    continue;
                }
            };

            let reported = match name {
                Some(a) => a.clone(),
                None => String::from("<anonymous>"),
            };

            return Some((format!("{reason}: `{reported}`"), current_usertype.location.clone()));
        }

        for current_variable in collect_declared_variables(current_unit) {
            if current_variable.location.span == CodeSpan::None {
                continue;
            }

            let Some(reason) = unsupported_reason(&current_variable.data_type_declaration) else {
                continue;
            };

            return Some((
                format!("{reason}: `{}`", current_variable.name),
                current_variable.location.clone(),
            ));
        }
    }

    None
}

fn collect_declared_variables(current_unit: &CompilationUnit) -> Vec<&Variable> {
    let mut collected: Vec<&Variable> = Vec::new();

    for current_block in &current_unit.global_vars {
        if current_block.linkage == LinkageType::External {
            continue;
        }

        collected.extend(current_block.variables.iter());
    }

    for current_pou in &current_unit.pous {
        if current_pou.linkage == LinkageType::External {
            continue;
        }

        for current_block in &current_pou.variable_blocks {
            collected.extend(current_block.variables.iter());
        }
    }

    for current_usertype in &current_unit.user_types {
        if let DataType::StructType { variables, .. } = &current_usertype.data_type {
            collected.extend(variables.iter());
        }
    }

    collected
}

fn unsupported_reason(declaration: &DataTypeDeclaration) -> Option<&'static str> {
    match declaration {
        DataTypeDeclaration::Definition { data_type, .. } => match data_type.as_ref() {
            DataType::PointerType { .. } => Some(POINTER_UNSUPPORTED_BY_OMRON),
            DataType::StringType { is_wide: true, .. } => Some(WSTRING_UNSUPPORTED_BY_OMRON),
            _ => None,
        },
        _ => {
            let declared = declaration.get_name()?;

            declared.eq_ignore_ascii_case("WSTRING").then_some(WSTRING_UNSUPPORTED_BY_OMRON)
        }
    }
}

pub type TypeNameMap = std::collections::HashMap<String, String>;

type TypeSources<'a> = std::collections::HashMap<&'a str, &'a DataType>;

pub fn build_type_name_map(units: &[&CompilationUnit]) -> TypeNameMap {
    let mut sources = TypeSources::default();

    for &current_unit in units {
        for current_usertype in &current_unit.user_types {
            let Some(type_name) = current_usertype.data_type.get_name() else {
                continue;
            };

            sources.insert(type_name, &current_usertype.data_type);
        }
    }

    let mut resolved = TypeNameMap::default();

    for (type_name, data_type) in &sources {
        if let Some(rendered) = render_data_type(data_type, &sources, 0) {
            resolved.insert(String::from(*type_name), rendered);
        }
    }

    resolved
}

fn render_data_type(data_type: &DataType, sources: &TypeSources, depth: usize) -> Option<String> {
    if depth > MAX_ALIAS_HOPS {
        return None;
    }

    match data_type {
        DataType::StringType { is_wide, size, .. } => Some(format_string_type(*is_wide, size.as_ref())),
        DataType::SubRangeType { referenced_type, bounds: None, .. } => {
            Some(render_named_type(referenced_type, sources, depth + 1))
        }
        DataType::ArrayType { bounds, referenced_type, .. } => {
            let dimensions = extract_dimensions(bounds);

            if dimensions.is_empty() {
                return None;
            }

            let ranges: Vec<String> =
                dimensions.iter().map(|(lower, upper)| format!("{lower}..{upper}")).collect();

            let base = render_declared_type(referenced_type, sources, depth + 1)?;

            Some(format!("ARRAY[{}] OF {}", ranges.join(","), base))
        }
        _ => None,
    }
}

fn render_named_type(type_name: &str, sources: &TypeSources, depth: usize) -> String {
    match sources.get(type_name) {
        Some(inner) => render_data_type(inner, sources, depth).unwrap_or_else(|| String::from(type_name)),
        None => String::from(type_name),
    }
}

fn render_declared_type(
    declaration: &DataTypeDeclaration,
    sources: &TypeSources,
    depth: usize,
) -> Option<String> {
    match declaration {
        DataTypeDeclaration::Definition { data_type, .. } => {
            render_data_type(data_type, sources, depth).or_else(|| data_type.get_name().map(String::from))
        }
        _ => declaration.get_name().map(|a| render_named_type(a, sources, depth)),
    }
}

const MAX_ALIAS_HOPS: usize = 16;

pub const DEFAULT_OMRON_STRING_LEN: u32 = 80;

fn format_string_type(is_wide: bool, size: Option<&AstNode>) -> String {
    let base = if is_wide { "WString" } else { "String" };

    match size.and_then(extract_literal) {
        Some(length) => format!("{base}[{length}]"),
        None => format!("{base}[{DEFAULT_OMRON_STRING_LEN}]"),
    }
}

pub fn parse_project_into_nodetree(
    generation_parameters: &GenerationParameters,
    units: &[&CompilationUnit],
    schema_path: &'static str,
    output_path: &PathBuf,
    mut output_root: Node,
) -> Result<(), Error> {
    if generation_parameters.output_xml_omron
        && let Some((message, _)) = find_unsupported_omron_type(units)
    {
        return Err(Error::other(message));
    }

    let mut param_order: HashSet<(String, usize)> = HashSet::new(); //the unique combination of (ParameterName, orderWithinParamSet) for the entire generation.
    let borrowed_order = &mut param_order;
    let type_names = build_type_name_map(units);

    for &current_unit in units {
        let unit_name = current_unit.file.get_name().unwrap_or("");

        if !unit_name.to_lowercase().ends_with(".st") {
            continue; //skip this unit since it is an internally generated file, not the users source code
        }
        let borrowed_root = &mut output_root;

        let _ = generate_globals(
            generation_parameters,
            current_unit,
            unit_name,
            schema_path,
            &type_names,
            borrowed_order,
            borrowed_root,
        );
        let _ = generate_custom_types(generation_parameters, current_unit, &type_names, borrowed_root);
        let _ = generate_pous(
            generation_parameters,
            current_unit,
            schema_path,
            &type_names,
            borrowed_order,
            borrowed_root,
        );
    }
    write_xml_file(output_path, output_root)?;
    Ok(())
}

pub(crate) fn generate_globals(
    generation_parameters: &GenerationParameters,
    current_unit: &CompilationUnit,
    unit_name: &str,
    schema_path: &'static str,
    type_names: &TypeNameMap,
    preused_order: &mut HashSet<(String, usize)>,
    output_root: &mut Node,
) -> Result<(), ()> {
    let maybe_globals_root: Option<&mut Node> = output_root.children.iter_mut().find(|a| a.name == INSTANCES);
    let globals_root = maybe_globals_root.ok_or(())?;

    //create the 4 destinations for <GlobalVars>
    let mut constant_retain_globals =
        SGlobalVars::new().attribute_str("constant", "true").attribute_str("retain", "true");

    let mut constant_globals = SGlobalVars::new().attribute_str("constant", "true");

    let mut retain_globals = SGlobalVars::new().attribute_str("retain", "true");

    let mut normal_globals = SGlobalVars::new();

    //parse the unit into nodes
    for a in 0..current_unit.global_vars.len() {
        let current_global = &current_unit.global_vars[a];
        let mut parsed_variables: Vec<Box<dyn IntoNode>> = Vec::with_capacity(current_global.variables.len());

        if current_global.linkage == LinkageType::External {
            //Don't include globals that are external
            continue;
        }

        for b in 0..current_global.variables.len() {
            let current_variable = &current_global.variables[b];

            if current_variable.location.span == CodeSpan::None {
                continue; //discard compiler interally generated variables
            }

            let network_publish = match current_global.kind {
                VariableBlockType::Global => String::from("DoNotPublish"),
                _ => {
                    continue; //skip non global variables
                }
            };

            let cloned_unitname = String::from(unit_name);

            let maybe_newvar = generate_variable_element(
                current_variable,
                generation_parameters,
                &cloned_unitname,
                schema_path,
                type_names,
                network_publish,
                preused_order,
                b,
                false,
            );

            let new_var = match maybe_newvar {
                Some(a) => a,
                None => {
                    continue;
                } //no variable element created so skip it
            };
            parsed_variables.push(Box::new(new_var));
        }

        //add globals to the correct element
        if current_global.constant && current_global.retain {
            constant_retain_globals = constant_retain_globals.children(parsed_variables);
        } else if current_global.constant {
            constant_globals = constant_globals.children(parsed_variables);
        } else if current_global.retain {
            retain_globals = retain_globals.children(parsed_variables);
        } else {
            normal_globals = normal_globals.children(parsed_variables);
        }
    }

    //relinquish copies of the nodes into the tree
    let name_label = String::from("name");
    let resources_name = format!("{}_{}", unit_name, RESOURCE);

    let resource_node = SResource::new()
        .attribute(name_label.clone(), resources_name)
        .attribute_str("resourceTypeName", "")
        .child(&constant_retain_globals)
        .child(&constant_globals)
        .child(&retain_globals)
        .child(&normal_globals);

    let config_name = format!("{}_{}", unit_name, CONFIGURATION);

    let configuration_node = SConfiguration::new().attribute(name_label, config_name).child(&resource_node);

    globals_root.child_borrowed(&configuration_node); //need to borrow a mut Node so I don't break the root nodes reference to the globals node
    Ok(())
}

pub(crate) fn generate_custom_types(
    _generation_parameters: &GenerationParameters,
    current_unit: &CompilationUnit,
    type_names: &TypeNameMap,
    output_root: &mut Node,
) -> Result<(), ()> {
    let maybe_types_root: Option<&mut Node> = output_root.children.iter_mut().find(|a| a.name == TYPES);
    let types_root: &mut Node = maybe_types_root.ok_or(())?;
    let maybe_global_root: Option<&mut Node> =
        types_root.children.iter_mut().find(|a| a.name == GLOBAL_NAMESPACE);
    let global_root: &mut Node = maybe_global_root.ok_or(())?;

    for a in 0..current_unit.user_types.len() {
        let current_usertype = &current_unit.user_types[a];

        if current_usertype.location.span == CodeSpan::None {
            continue; //discard internally generated types
        }

        if current_usertype.linkage == LinkageType::External {
            continue; //discard externally defined types; same as externally defined functions
        }

        let customtype_maybe: Option<SDataTypeDecl> = match &current_usertype.data_type {
            DataType::StructType { name, variables } => {
                //STRUCT
                let unwrapped_name = match name {
                    Some(a) => a.clone(),
                    None => {
                        continue;
                    } //every structure must have a name
                };

                let mut spec_node = SUserDefinedTypeSpec::new().attribute_str("xsi:type", STRUCT_TYPE_SPEC);

                for current_variable in variables {
                    let typename =
                        match resolve_type_name(&current_variable.data_type_declaration, type_names) {
                            Some(a) => a,
                            None => {
                                continue;
                            } //every variable must have a type
                        };

                    let typename_node = STypeName::new().content(typename);

                    let type_node = SType::new().child(&typename_node);

                    let member_node = SMember::new()
                        .attribute(String::from("name"), current_variable.name.clone())
                        .child(&type_node);

                    spec_node = spec_node.child(&member_node);
                }

                if spec_node.inner().children.is_empty() {
                    //structs must have <Member> elements, otherwise delete it
                    None
                } else {
                    let decl_node1 = SDataTypeDecl::new()
                        .attribute(String::from("name"), unwrapped_name)
                        .child(&spec_node);

                    Some(decl_node1)
                }
            }
            DataType::EnumType { name, numeric_type, elements } => {
                //ENUM
                let unwrapped_enum_type = match name {
                    Some(a) => a.clone(),
                    None => {
                        continue;
                    } //every structure must have a name
                };

                let enumerators = match &elements.stmt {
                    AstStatement::ExpressionList(ast_nodes) => ast_nodes
                        .iter()
                        .map(|a| match &a.stmt {
                            AstStatement::Assignment(assignment) => parse_enum_expression(assignment),
                            other => panic!("Expected Assignment. Instead got: {:?}", other),
                        })
                        .collect(),

                    AstStatement::Assignment(assignment) => vec![parse_enum_expression(assignment)],
                    other => panic!("Expected ExpressionList or Assignment. Instead got: {:?}", other),
                };

                let base_node = SBaseType::new().content(numeric_type.clone());

                let formatted = format_enum_initials(enumerators);

                let spec_node = SUserDefinedTypeSpec::new()
                    .attribute_str("xsi:type", ENUM_TYPE_SPEC)
                    .children(formatted)
                    .child(&base_node); //<BaseType> element must be declared below all the <Member> elements, apparently

                let decl_node2 = SDataTypeDecl::new()
                    .attribute(String::from("name"), unwrapped_enum_type)
                    .child(&spec_node);

                Some(decl_node2)
            }
            _ => None,
        };

        if let Some(unwrapped_ready) = customtype_maybe {
            global_root.child_borrowed(&unwrapped_ready);
        }
    }
    Ok(())
}

fn resolve_type_name(declaration: &DataTypeDeclaration, type_names: &TypeNameMap) -> Option<String> {
    if let DataTypeDeclaration::Definition { data_type, .. } = declaration
        && let DataType::StringType { is_wide, size, .. } = data_type.as_ref()
    {
        return Some(format_string_type(*is_wide, size.as_ref()));
    }

    let declared = declaration.get_name()?;

    if let Some(replacement) = type_names.get(declared) {
        return Some(replacement.clone());
    }

    if declared.eq_ignore_ascii_case("STRING") {
        return Some(format_string_type(false, None));
    }

    if declared.eq_ignore_ascii_case("WSTRING") {
        return Some(format_string_type(true, None));
    }

    Some(String::from(declared))
}

fn extract_literal(input: &AstNode) -> Option<String> {
    match &input.stmt {
        AstStatement::Literal(literal) => Some(literal.to_string()),
        _ => None,
    }
}

fn extract_bounds(input: &AstNode) -> Option<(String, String)> {
    let AstStatement::RangeStatement(range) = &input.stmt else {
        return None;
    };

    Some((extract_literal(&range.start)?, extract_literal(&range.end)?))
}

fn extract_dimensions(input: &AstNode) -> Vec<(String, String)> {
    match &input.stmt {
        AstStatement::ExpressionList(nodes) => nodes.iter().filter_map(extract_bounds).collect(),
        _ => extract_bounds(input).into_iter().collect(),
    }
}

fn parse_enum_expression(input: &Assignment) -> NameAndInitialValue {
    let enum_variant_name = match &input.left.stmt {
        AstStatement::ReferenceExpr(reference_exp) => match &reference_exp.access {
            ReferenceAccess::Member(member_exp) => match &member_exp.stmt {
                AstStatement::Identifier(name) => name.clone(),
                other => panic!("Expected identifier. Instead got: {:?}", other),
            },
            other => panic!("Expected Member. Instead got: {:?}", other),
        },
        other => panic!("Expected ReferenceExpr. Instead got: {:?}", other),
    };

    let enum_variant_initialiser = match &input.right.stmt {
        AstStatement::Literal(literal) => literal.to_string(),
        AstStatement::BinaryExpression(binary_exp) => match &binary_exp.right.stmt {
            AstStatement::Literal(literal) => literal.to_string(),
            other => panic!("Expected Literal. Instead got: {:?}", other),
        },
        other => panic!("Expected LiteralInteger or BinaryExpression. Instead got: {:?}", other),
    };

    NameAndInitialValue { name: enum_variant_name, initial_value: enum_variant_initialiser }
}

pub struct NameAndInitialValue {
    pub name: String,
    pub initial_value: String,
}

pub fn format_enum_initials(mut enum_variants: Vec<NameAndInitialValue>) -> Vec<Box<dyn IntoNode>> {
    let mut viewed_values: HashSet<String> = HashSet::new(); // Own strings for ownership

    for variant in enum_variants.iter_mut() {
        let current_initial = &mut variant.initial_value;

        if !viewed_values.contains(current_initial) {
            viewed_values.insert(current_initial.clone());
            continue;
        }

        // Conflict: auto-increment
        let parsed_value = current_initial.parse::<i32>().expect("signed integer");
        let mut increment = 1;
        loop {
            let new_value = parsed_value.checked_add(increment).expect("no overflow");
            let new_str = new_value.to_string();

            if !viewed_values.contains(&new_str) {
                *current_initial = new_str;
                viewed_values.insert(current_initial.clone());
                break;
            }
            increment += 1;
        }
    }

    enum_variants
        .into_iter()
        .map(|a| {
            Box::new(
                SEnumerator::new()
                    .attribute(String::from("name"), a.name)
                    .attribute(String::from("value"), a.initial_value),
            ) as Box<dyn IntoNode>
        })
        .collect()
}

pub(crate) fn generate_pous(
    generation_parameters: &GenerationParameters,
    current_unit: &CompilationUnit,
    schema_path: &'static str,
    type_names: &TypeNameMap,
    param_order: &mut HashSet<(String, usize)>,
    output_root: &mut Node,
) -> Result<(), ()> {
    let maybe_types_root: Option<&mut Node> = output_root.children.iter_mut().find(|a| a.name == TYPES);
    let types_root: &mut Node = maybe_types_root.ok_or(())?;
    let maybe_global_root: Option<&mut Node> =
        types_root.children.iter_mut().find(|a| a.name == GLOBAL_NAMESPACE);
    let global_root: &mut Node = maybe_global_root.ok_or(())?;

    for a in 0..current_unit.implementations.len() {
        let current_impl = &current_unit.implementations[a];

        let omron_supported = current_impl.pou_type == PouType::Program
            || current_impl.pou_type == PouType::Function
            || current_impl.pou_type == PouType::FunctionBlock;

        let extended_supported =
            matches!(current_impl.pou_type, PouType::Class | PouType::Method { .. } | PouType::Action);

        if !omron_supported && !(extended_supported && !generation_parameters.output_xml_omron) {
            continue;
        }

        if current_impl.linkage == LinkageType::External {
            //discard externally linked POUs since the receiving platform will have those implemented already
            continue;
        }

        let matching_metadata = current_unit.pous.iter().find(|b| b.name == current_impl.name);

        if matching_metadata.is_none() && current_impl.pou_type != PouType::Action {
            continue;
        }

        let owning_name: &str = match matching_metadata {
            Some(metadata) => &metadata.name,
            None => &current_impl.type_name,
        };

        let procedure_text = match &current_impl.location.span {
            CodeSpan::Range(inner_range) => {
                match current_impl.location.file {
                    plc_source::source_location::FileMarker::File(file_path) => {
                        match grab_file_statement_from_span(file_path, inner_range) {
                            Some(pou_procedure_text) => pou_procedure_text,
                            None => {
                                continue;
                            }
                        }
                    }
                    _ => {
                        continue; //don't parse FileMarkers that didn't come from ST files
                    }
                }
            }
            _ => {
                continue; //dont parse CodeSpans that aren't Ranges
            }
        };

        let info_node = SPouInfo::new()
            .attribute_str("version", "0.0.0")
            .attribute(String::from("creationDateTime"), Local::now().to_rfc3339());

        let data_node = SOmronData::new() //<Data>
            .attribute_str("name", schema_path)
            .attribute_str("handleUnknown", "discard")
            .child(&info_node);

        let adddata_node = SOmronAddData::new() //<AddData>
            .child(&data_node);

        let mut resulttype_node = SResultType::new(); //<ResultType>

        let mut typename_node = STypeName::new();

        if (current_impl.pou_type == PouType::Function
            || current_impl.pou_type == PouType::FunctionBlock
            || matches!(current_impl.pou_type, PouType::Method { .. }))
            && let Some(metadata) = matching_metadata
            && let Some(result_type) = &metadata.return_type
            && let Some(type_name) = result_type.get_name()
        {
            typename_node = typename_node.content(String::from(type_name));
        } else {
            typename_node = typename_node.content(String::from("BOOL")); //default to boolean output
        }

        resulttype_node = resulttype_node.child(&typename_node);

        //<Parameters>
        let mut input_vars = SInputVars::new();
        let mut inout_vars = SInoutVars::new();
        let mut output_vars = SOutputVars::new();
        let mut parameters_node = SParameters::new();

        //<ExternalVars>
        let mut externals = SExternalVars::new();

        let mut constant_externals = SExternalVars::new().attribute_str("constant", "true");

        //<Vars>
        let mut vars = SVars::new().attribute_str("accessSpecifier", "private");

        let mut constant_vars =
            SVars::new().attribute_str("accessSpecifier", "private").attribute_str("constant", "true");

        let mut retain_vars =
            SVars::new().attribute_str("accessSpecifier", "private").attribute_str("retain", "true");

        let mut constant_retain_vars = SVars::new()
            .attribute_str("accessSpecifier", "private")
            .attribute_str("constant", "true")
            .attribute_str("retain", "true");

        //<TempVars>
        let mut temp_vars = STempVars::new();

        let mut constant_temp_vars = STempVars::new().attribute_str("constant", "true");

        //put all the variables in the right containers
        let variable_blocks: &[VariableBlock] = match matching_metadata {
            Some(metadata) => &metadata.variable_blocks,
            None => &[],
        };

        for current_block in variable_blocks {
            for c in 0..current_block.variables.len() {
                let current_variable = &current_block.variables[c];
                let use_order_attr = current_block.kind != VariableBlockType::Local
                    && current_block.kind != VariableBlockType::External;

                if current_variable.location.span == CodeSpan::None {
                    continue; //discard compiler interally generated variables
                }

                let network_publish = String::from("DoNotPublish");

                let maybe_variablenode = generate_variable_element(
                    current_variable,
                    generation_parameters,
                    owning_name,
                    schema_path,
                    type_names,
                    network_publish,
                    param_order,
                    c,
                    use_order_attr,
                );

                let variable_node = match maybe_variablenode {
                    Some(a) => a,
                    None => {
                        continue;
                    }
                };

                match current_block.kind {
                    VariableBlockType::Local => {
                        if current_block.constant && current_block.retain {
                            constant_retain_vars = constant_retain_vars.child(&variable_node);
                        } else if current_block.constant {
                            constant_vars = constant_vars.child(&variable_node);
                        } else if current_block.retain {
                            retain_vars = retain_vars.child(&variable_node);
                        } else {
                            vars = vars.child(&variable_node);
                        }
                    }
                    VariableBlockType::Temp => {
                        if current_block.constant {
                            constant_temp_vars = constant_temp_vars.child(&variable_node);
                        } else {
                            temp_vars = temp_vars.child(&variable_node);
                        }
                    }
                    VariableBlockType::Input(_) => {
                        input_vars = input_vars.child(&variable_node);
                    }
                    VariableBlockType::Output => {
                        output_vars = output_vars.child(&variable_node);
                    }
                    VariableBlockType::InOut => {
                        inout_vars = inout_vars.child(&variable_node);
                    }
                    VariableBlockType::External => {
                        if current_block.constant {
                            constant_externals = constant_externals.child(&variable_node);
                        } else {
                            externals = externals.child(&variable_node);
                        }
                    }
                    _ => (),
                }
            }
        }

        parameters_node = parameters_node.child(&input_vars).child(&inout_vars).child(&output_vars);

        //implementation statements
        let mut st_element = SST::new(); //<ST>

        if !procedure_text.is_empty() {
            st_element = st_element.content(procedure_text);
        }

        let body_content = SBodyContent::new().attribute_str("xsi:type", "ST").child(&st_element);

        let main_body = SMainBody::new().child(&body_content);

        let name_key = String::from("name");
        let name_value = current_impl.name.clone();

        let chosen_element: &dyn IntoNode = match current_impl.pou_type {
            PouType::Program => &SProgram::new()
                .attribute(name_key, name_value)
                .child(&adddata_node)
                .child(&externals)
                .child(&constant_externals)
                .child(&vars)
                .child(&constant_vars)
                .child(&retain_vars)
                .child(&constant_retain_vars)
                .child(&main_body),
            PouType::Function => &SFunction::new()
                .attribute(name_key, name_value)
                .child(&adddata_node)
                .child(&resulttype_node)
                .child(&parameters_node)
                .child(&externals)
                .child(&constant_externals)
                .child(&temp_vars)
                .child(&constant_temp_vars)
                .child(&main_body),
            PouType::FunctionBlock => &SFunctionBlock::new()
                .attribute(name_key, name_value)
                .child(&adddata_node)
                .child(&parameters_node)
                .child(&externals)
                .child(&constant_externals)
                .child(&vars)
                .child(&main_body),
            PouType::Class => &SClass::new()
                .attribute(name_key, name_value)
                .child(&adddata_node)
                .child(&externals)
                .child(&constant_externals)
                .child(&vars)
                .child(&constant_vars)
                .child(&retain_vars)
                .child(&constant_retain_vars)
                .child(&main_body),
            PouType::Method { .. } => &SMethod::new()
                .attribute(name_key, name_value)
                .child(&adddata_node)
                .child(&resulttype_node)
                .child(&parameters_node)
                .child(&temp_vars)
                .child(&constant_temp_vars)
                .child(&main_body),
            PouType::Action => {
                &SOmronAction::new().attribute(name_key, name_value).child(&adddata_node).child(&main_body)
            }
            _ => continue,
        };

        global_root.child_borrowed(chosen_element);
    }
    Ok(())
}

///returns the generated element.
/// add_order - whether to add the "orderWithinParamSet" attribute.
#[allow(clippy::too_many_arguments)]
fn generate_variable_element(
    current_variable: &Variable,
    _generation_parameters: &GenerationParameters,
    pou_name: &str,
    schema_path: &'static str,
    type_names: &TypeNameMap,
    network_publish: String,
    preused_order: &mut HashSet<(String, usize)>,
    order: usize,
    add_order: bool,
) -> Option<SGenVariable> {
    let mut variable_node =
        SGenVariable::new().attribute(String::from("name"), current_variable.name.clone());

    //<AddData>
    let additional_property_node = SOmronGlobalVariableAdditionalProperties::new()
        .attribute(String::from("networkPublish"), network_publish);

    let data_node = SOmronData::new() //<Data>
        .attribute_str("name", schema_path)
        .attribute_str("handleUnknown", "discard")
        .child(&additional_property_node);

    let adddata_node = SOmronAddData::new() //<AddData>
        .child(&data_node);

    variable_node = variable_node.child(&adddata_node);

    //<Type>
    let typename = match resolve_type_name(&current_variable.data_type_declaration, type_names) {
        Some(a) => a,
        None => {
            return None;
        } //every variable must have a typename
    };

    let typename_node = STypeName::new() //<TypeName>
        .content(typename);

    let typenode = SType::new() //<Type>
        .child(&typename_node);

    variable_node = variable_node.child(&typenode);

    if add_order {
        let mut iteration_order: usize = order;
        let mut increment: usize = 0;

        loop {
            iteration_order += increment;
            increment += 1;
            let key = (pou_name.to_string(), iteration_order);

            if !preused_order.contains(&key) {
                //an unused order number was found. There cannot be duplicate order numbers for any POU variable
                preused_order.insert(key);
                break;
            }
        }
        variable_node =
            variable_node.attribute(String::from("orderWithinParamSet"), iteration_order.to_string());
    }

    //<InitialValue>
    if let Some(variable_ast) = &current_variable.initializer
        && let AstStatement::Literal(literal_value) = &variable_ast.stmt
    {
        let simple_node =
            SSimpleValue::new().attribute(String::from("value"), literal_value.to_string()).close();

        let initial_node = SInitialValue::new().child(&simple_node);

        variable_node = variable_node.child(&initial_node);
    }

    //<Address>
    if let Some(address) = &current_variable.address {
        //not every variable has an address
        if let AstStatement::Literal(ast_literal) = &address.stmt {
            let address_node = SAddress::new().attribute(String::from("address"), ast_literal.to_string());

            variable_node = variable_node.child(&address_node);
        }
    }
    Some(variable_node)
}

fn grab_file_statement_from_span(file_path: &'static str, range: &Range<TextLocation>) -> Option<String> {
    let mut file = File::open(file_path).unwrap_or_else(|_| panic!("source file exists: {}", file_path));
    let unsigned_start = TryInto::<u64>::try_into(range.start.offset).expect("u64");
    file.seek(SeekFrom::Start(unsigned_start)).expect("seeks to starting offset");
    let maybe_size = range.end.offset.checked_sub(range.start.offset);

    let size = match maybe_size {
        Some(a) => a,
        None => {
            return None;
        } //don't parse statement if it has a negative size
    };
    let mut buffer = vec![0u8; size];
    file.read_exact(buffer.as_mut_slice()).expect("reads successfully");
    let formatted = String::from_utf8(buffer).expect("valid utf8 string");
    Some(formatted)
}

pub fn write_xml_file(output_path: &PathBuf, treenode: Node) -> Result<(), Error> {
    let file = File::create(output_path)?;

    let mut writer = EmitterConfig::new().perform_indent(true).create_writer(file);

    let top =
        XmlEvent::StartDocument { encoding: Some("UTF-8"), version: XmlVersion::Version10, standalone: None };

    let _ = writer.write(top).map_err(Error::other);

    recurse_write_xml(&mut writer, treenode)
}

fn recurse_write_xml(writer: &mut EventWriter<File>, mut treenode: Node) -> Result<(), Error> {
    //open the element
    let start = XmlEvent::StartElement {
        name: Name::from(treenode.name.as_str()),
        attributes: treenode
            .attributes
            .iter()
            .map(|a| Attribute { name: Name::from(a.0.as_str()), value: a.1.as_str() })
            .collect(),
        namespace: Cow::Owned(Namespace::empty()),
    };

    let _ = writer.write(start).map_err(Error::other);

    if let Some(content) = &treenode.content
        && treenode.children.is_empty()
    {
        let content_event = XmlEvent::CData(content);

        let _ = writer.write(content_event).map_err(Error::other);
    }

    //recurse through children
    for item in treenode.children.drain(0..) {
        recurse_write_xml(writer, item)?;
    }

    //close the element
    let end = XmlEvent::end_element();

    let _ = writer.write(end).map_err(Error::other);
    Ok(())
}

pub fn copy_xmlfile_to_output(temp_paths: Vec<&Path>, output_path: PathBuf) -> Result<PathBuf, Error> {
    if temp_paths.is_empty() {
        return Ok(output_path);
    }
    let xml_file = temp_paths
        .iter()
        .find(|a| {
            //grab the file which has the right name, although both xml duplicates have the same content
            if let Some(ext) = a.extension()
                && ext.eq_ignore_ascii_case("xml")
            {
                return true;
            }
            false
        })
        .unwrap();

    copy(xml_file, &output_path)?;
    Ok(output_path)
}
