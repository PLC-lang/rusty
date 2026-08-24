
#[cfg(test)]
mod xml_gen_tests {
    use std::path::PathBuf;

    use plc_ast::ast::CompilationUnit;

    use crate::xml_gen::*;
    use crate::serializer::*;

    use plc_ast::{
        literals::AstLiteral,
        ast::{
        AstFactory, AstNode, DataType,
        DataTypeDeclaration, Implementation, LinkageType, Pou, PouType,
        UserTypeDeclaration, Variable, VariableBlock, VariableBlockType,
    }};
    use plc_source::source_location::{CodeSpan, FileMarker, SourceLocation, TextLocation};
    use std::collections::HashSet;

    /// Helper: Create a SourceLocation with a non-None CodeSpan::Range so variables are not skipped.
    fn make_source_location() -> SourceLocation {
        SourceLocation {
            span: CodeSpan::Range(TextLocation::new(0, 0, 0)..TextLocation::new(0, 10, 10)),
            file: FileMarker::Internal("<test>"),
        }
    }

    /// Helper: Create a simple Variable with a given name and type.
    fn make_variable(name: &str, type_name: &str) -> Variable {
        Variable {
            name: String::from(name),
            data_type_declaration: DataTypeDeclaration::Reference {
                referenced_type: String::from(type_name),
                location: SourceLocation::internal(),
            },
            initializer: None,
            address: None,
            location: make_source_location(),
        }
    }

    /// Helper: Create a CompilationUnit from the given file name.
    fn make_unit(file_name: &'static str) -> CompilationUnit {
        CompilationUnit::new(file_name)
    }

    /// Helper: Create an enum Assignment AST node: `VariantName := IntegerValue`
    fn make_enum_assignment(variant_name: &str, value: i128) -> AstNode {
        let loc = SourceLocation::internal();
        let left = AstFactory::create_member_reference(
            AstFactory::create_identifier(variant_name, loc.clone(), 0),
            None,
            0,
        );
        let right = AstFactory::create_literal(
            AstLiteral::Integer(value),
            loc.clone(),
            0,
        );
        AstFactory::create_assignment(left, right, 0)
    }

    #[test]
    fn test_generation_parameters_default() {
        let params = GenerationParameters::new();
        assert_eq!(params.output_xml_omron, false);
    }

    #[test]
    fn test_omron_template_has_correct_root() {
        let template = get_omron_template();
        assert_eq!(template.name, "Project");
    }

    #[test]
    fn test_omron_template_has_required_attributes() {
        let template = get_omron_template();
        let attr_map: std::collections::HashMap<&str, &str> = template
            .attributes
            .iter()
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect();

        assert_eq!(
            attr_map.get("xmlns:xsi"),
            Some(&"http://www.w3.org/2001/XMLSchema-instance")
        );
        assert_eq!(
            attr_map.get("xmlns:smcext"),
            Some(&"https://www.ia.omron.com/Smc")
        );
        assert_eq!(attr_map.get("xsi:schemaLocation"), Some(&OMRON_SCHEMA));
        assert_eq!(attr_map.get("schemaVersion"), Some(&"1"));
        assert_eq!(
            attr_map.get("xmlns"),
            Some(&"www.iec.ch/public/TC65SC65BWG7TF10")
        );
    }

    #[test]
    fn test_omron_template_has_four_children() {
        let template = get_omron_template();
        // FileHeader, ContentHeader, Types, Instances
        assert_eq!(template.children.len(), 4);
    }

    #[test]
    fn test_omron_template_children_names() {
        let template = get_omron_template();
        let child_names: Vec<&str> = template.children.iter().map(|c| c.name.as_str()).collect();
        assert!(child_names.contains(&FILE_HEADER));
        assert!(child_names.contains(&CONTENT_HEADER));
        assert!(child_names.contains(&TYPES));
        assert!(child_names.contains(&INSTANCES));
    }

    #[test]
    fn test_omron_template_types_has_global_namespace() {
        let template = get_omron_template();
        let types_node = template
            .children
            .iter()
            .find(|c| c.name == TYPES)
            .expect("Types node should exist");
        assert_eq!(types_node.children.len(), 1);
        assert_eq!(types_node.children[0].name, GLOBAL_NAMESPACE);
    }

    #[test]
    fn test_omron_template_file_header_attributes() {
        let template = get_omron_template();
        let file_header = template
            .children
            .iter()
            .find(|c| c.name == FILE_HEADER)
            .expect("FileHeader node should exist");

        let attr_map: std::collections::HashMap<&str, &str> = file_header
            .attributes
            .iter()
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect();

        assert_eq!(attr_map.get("companyName"), Some(&"OMRON Corporation"));
        assert_eq!(attr_map.get("productName"), Some(&"Sysmac Studio"));
        assert_eq!(attr_map.get("productVersion"), Some(&"1.30.0.0"));
    }

    #[test]
    fn test_omron_template_content_header_has_name() {
        let template = get_omron_template();
        let content_header = template
            .children
            .iter()
            .find(|c| c.name == CONTENT_HEADER)
            .expect("ContentHeader node should exist");

        let attr_map: std::collections::HashMap<&str, &str> = content_header
            .attributes
            .iter()
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect();

        assert_eq!(attr_map.get("name"), Some(&"Sample"));
        assert!(attr_map.contains_key("creationDateTime"));
    }

    #[test]
    fn test_omron_schema_constant() {
        assert_eq!(
            OMRON_SCHEMA,
            "https://www.ia.omron.com/Smc IEC61131_10_Ed1_0_SmcExt1_0_Spc1_0.xsd"
        );
    }

    #[test]
    fn test_parse_project_into_nodetree() {
        let temp_dir = std::env::temp_dir();
        let output_path = temp_dir.join("test_parse_nodetree.xml");
        let template = get_omron_template();
        let params = GenerationParameters::new();

        // Create a unit with a global variable
        let mut unit = make_unit("myfile.st");
        unit.global_vars.push(
            VariableBlock::global()
                .with_variables(vec![make_variable("gCounter", "INT")]),
        );

        // Create a struct user type
        unit.user_types.push(UserTypeDeclaration {
            data_type: DataType::StructType {
                name: Some(String::from("MyStruct")),
                variables: vec![make_variable("field1", "DINT")],
            },
            initializer: None,
            location: make_source_location(),
            scope: None,
            linkage: LinkageType::Internal,
        });

        let units: Vec<&CompilationUnit> = vec![&unit];
        let result = parse_project_into_nodetree(&params, &units, OMRON_SCHEMA, &output_path, template);
        assert!(result.is_ok());
        assert!(output_path.exists());

        let contents = std::fs::read_to_string(&output_path).unwrap();
        // Globals should produce a Configuration/Resource with the variable
        assert!(contents.contains(CONFIGURATION));
        assert!(contents.contains(RESOURCE));
        assert!(contents.contains("gCounter"));
        // Custom types should produce DataTypeDecl
        assert!(contents.contains("MyStruct"));
        assert!(contents.contains("field1"));

        let _ = std::fs::remove_file(&output_path);
    }

    #[test]
    fn test_generate_globals() {
        let params = GenerationParameters::new();
        let mut template = get_omron_template();
        let mut order: HashSet<(String, usize)> = HashSet::new();

        // Unit with one normal global, one constant global, one retain global
        let mut unit = make_unit("globals.st");

        // Normal global block
        unit.global_vars.push(
            VariableBlock::global()
                .with_variables(vec![make_variable("normalVar", "INT")]),
        );

        // Constant global block
        let mut const_block = VariableBlock::global()
            .with_variables(vec![make_variable("constVar", "REAL")]);
        const_block.constant = true;
        unit.global_vars.push(const_block);

        // Retain global block
        let mut retain_block = VariableBlock::global()
            .with_variables(vec![make_variable("retainVar", "BOOL")]);
        retain_block.retain = true;
        unit.global_vars.push(retain_block);

        let result = generate_globals(&params, &unit, "globals.st", OMRON_SCHEMA, &mut order, &mut template);
        assert!(result.is_ok());

        // Write the tree to verify the structure
        let temp_dir = std::env::temp_dir();
        let output_path = temp_dir.join("test_generate_globals_output.xml");
        write_xml_file(&output_path, template).unwrap();

        let contents = std::fs::read_to_string(&output_path).unwrap();
        assert!(contents.contains(CONFIGURATION));
        assert!(contents.contains(RESOURCE));
        assert!(contents.contains("GlobalVars"));
        assert!(contents.contains("normalVar"));
        assert!(contents.contains("constVar"));
        assert!(contents.contains("retainVar"));
        // Check Configuration name follows the pattern "unitname_Configuration"
        assert!(contents.contains("globals.st_Configuration"));
        assert!(contents.contains("globals.st_Resource"));

        let _ = std::fs::remove_file(&output_path);
    }

    #[test]
    fn test_generate_custom_types() {
        let params = GenerationParameters::new();
        let mut template = get_omron_template();

        let mut unit = make_unit("types.st");

        // Add a StructType
        unit.user_types.push(UserTypeDeclaration {
            data_type: DataType::StructType {
                name: Some(String::from("Motor")),
                variables: vec![
                    make_variable("speed", "INT"),
                    make_variable("running", "BOOL"),
                ],
            },
            initializer: None,
            location: make_source_location(),
            scope: None,
            linkage: LinkageType::Internal,
        });

        // Add an EnumType with an ExpressionList of Assignments
        let enum_elements = AstFactory::create_expression_list(
            vec![
                make_enum_assignment("RED", 0),
                make_enum_assignment("GREEN", 1),
                make_enum_assignment("BLUE", 2),
            ],
            SourceLocation::internal(),
            0,
        );

        unit.user_types.push(UserTypeDeclaration {
            data_type: DataType::EnumType {
                name: Some(String::from("Color")),
                numeric_type: String::from("INT"),
                elements: enum_elements,
            },
            initializer: None,
            location: make_source_location(),
            scope: None,
            linkage: LinkageType::Internal,
        });

        let result = generate_custom_types(&params, &unit, &mut template);
        assert!(result.is_ok());

        // Write and verify
        let temp_dir = std::env::temp_dir();
        let output_path = temp_dir.join("test_generate_custom_types_output.xml");
        write_xml_file(&output_path, template).unwrap();

        let contents = std::fs::read_to_string(&output_path).unwrap();
        // Struct
        assert!(contents.contains("Motor"));
        assert!(contents.contains("speed"));
        assert!(contents.contains("running"));
        assert!(contents.contains("StructTypeSpec"));

        // Enum
        assert!(contents.contains("Color"));
        assert!(contents.contains("RED"));
        assert!(contents.contains("GREEN"));
        assert!(contents.contains("BLUE"));
        assert!(contents.contains("EnumTypeWithNamedValueSpec"));
        assert!(contents.contains("BaseType"));

        let _ = std::fs::remove_file(&output_path);
    }

    #[test]
    fn test_generate_pous() {
        let params = GenerationParameters::new();
        let mut template = get_omron_template();
        let mut order: HashSet<(String, usize)> = HashSet::new();

        // Create a temp .st file for grab_file_statement_from_span to read
        let temp_dir = std::env::temp_dir();
        let st_path = temp_dir.join("test_pou_source.st");
        let program_body = "x := 1 + 2;";
        std::fs::write(&st_path, program_body).unwrap();

        // We need a &'static str for FileMarker::File, so we leak the path string.
        // This is acceptable in tests.
        let st_path_str: &'static str = Box::leak(st_path.to_string_lossy().into_owned().into_boxed_str());

        let body_len = program_body.len();
        let impl_location = SourceLocation {
            span: CodeSpan::Range(TextLocation::new(0, 0, 0)..TextLocation::new(0, body_len, body_len)),
            file: FileMarker::File(st_path_str),
        };

        let mut unit = make_unit("test_pou_source.st");

        // Add a Program POU metadata
        unit.pous.push(Pou {
            id: 1,
            name: String::from("MainProg"),
            kind: PouType::Program,
            variable_blocks: vec![
                VariableBlock::default()
                    .with_block_type(VariableBlockType::Local)
                    .with_variables(vec![make_variable("localX", "INT")]),
            ],
            return_type: None,
            location: SourceLocation::internal(),
            name_location: SourceLocation::internal(),
            poly_mode: None,
            generics: vec![],
            linkage: LinkageType::Internal,
            super_class: None,
            is_const: false,
            interfaces: vec![],
            properties: vec![],
        });

        // Matching implementation
        unit.implementations.push(Implementation {
            name: String::from("MainProg"),
            type_name: String::from("MainProg"),
            linkage: LinkageType::Internal,
            pou_type: PouType::Program,
            statements: vec![],
            location: impl_location,
            name_location: SourceLocation::internal(),
            end_location: SourceLocation::internal(),
            overriding: false,
            generic: false,
            access: None,
        });

        let result = generate_pous(&params, &unit, OMRON_SCHEMA, &mut order, &mut template);
        assert!(result.is_ok());

        // Write and verify
        let output_path = temp_dir.join("test_generate_pous_output.xml");
        write_xml_file(&output_path, template).unwrap();

        let contents = std::fs::read_to_string(&output_path).unwrap();
        assert!(contents.contains("Program"));
        assert!(contents.contains("MainProg"));
        assert!(contents.contains("localX"));
        assert!(contents.contains("x := 1 + 2;"));
        assert!(contents.contains("MainBody"));

        let _ = std::fs::remove_file(&output_path);
        let _ = std::fs::remove_file(&st_path);
    }

    #[test]
    fn test_write_xml_file_creates_file() {
        let temp_dir = std::env::temp_dir();
        let output_path = temp_dir.join("test_write_xml_output.xml");

        let node = Node::new_str("TestRoot")
            .attribute_str("version", "1.0");

        let result = write_xml_file(&output_path, node);
        assert!(result.is_ok());
        assert!(output_path.exists());

        // Clean up
        let _ = std::fs::remove_file(&output_path);
    }

    #[test]
    fn test_write_xml_file_with_children() {
        let temp_dir = std::env::temp_dir();
        let output_path = temp_dir.join("test_write_xml_children.xml");

        let child1 = STypeName::new().content(String::from("INT"));
        let child2 = STypeName::new().content(String::from("BOOL"));

        let node = Node::new_str("Root")
            .child(&child1)
            .child(&child2);

        let result = write_xml_file(&output_path, node);
        assert!(result.is_ok());

        let contents = std::fs::read_to_string(&output_path).unwrap();
        assert!(contents.contains("Root"));
        assert!(contents.contains("INT"));
        assert!(contents.contains("BOOL"));

        let _ = std::fs::remove_file(&output_path);
    }

    #[test]
    fn test_write_xml_file_with_attributes() {
        let temp_dir = std::env::temp_dir();
        let output_path = temp_dir.join("test_write_xml_attrs.xml");

        let node = Node::new_str("Element")
            .attribute_str("key1", "value1")
            .attribute_str("key2", "value2");

        let result = write_xml_file(&output_path, node);
        assert!(result.is_ok());

        let contents = std::fs::read_to_string(&output_path).unwrap();
        assert!(contents.contains("key1=\"value1\""));
        assert!(contents.contains("key2=\"value2\""));

        let _ = std::fs::remove_file(&output_path);
    }

    #[test]
    fn test_write_xml_file_full_template() {
        let temp_dir = std::env::temp_dir();
        let output_path = temp_dir.join("test_write_xml_full_template.xml");

        let template = get_omron_template();
        let result = write_xml_file(&output_path, template);
        assert!(result.is_ok());

        let contents = std::fs::read_to_string(&output_path).unwrap();
        assert!(contents.contains("Project"));
        assert!(contents.contains(FILE_HEADER));
        assert!(contents.contains(CONTENT_HEADER));
        assert!(contents.contains(TYPES));
        assert!(contents.contains(INSTANCES));
        assert!(contents.contains(GLOBAL_NAMESPACE));

        let _ = std::fs::remove_file(&output_path);
    }

    #[test]
    fn test_write_xml_file_invalid_path() {
        let output_path = PathBuf::from("/nonexistent/directory/file.xml");
        let node = Node::new_str("Root");
        let result = write_xml_file(&output_path, node);
        assert!(result.is_err());
    }

    #[test]
    fn test_copy_xmlfile_to_output_empty_paths() {
        let temp_dir = std::env::temp_dir();
        let output_path = temp_dir.join("test_copy_empty.xml");
        let result = copy_xmlfile_to_output(vec![], output_path.clone());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), output_path);
    }

    #[test]
    fn test_copy_xmlfile_to_output_with_xml_file() {
        let temp_dir = std::env::temp_dir();

        // Create a source XML file
        let source_path = temp_dir.join("test_source_copy.XML");
        std::fs::write(&source_path, "<Root/>").unwrap();

        let output_path = temp_dir.join("test_dest_copy.xml");

        let result = copy_xmlfile_to_output(vec![source_path.as_path()], output_path.clone());
        assert!(result.is_ok());
        assert!(output_path.exists());

        let contents = std::fs::read_to_string(&output_path).unwrap();
        assert_eq!(contents, "<Root/>");

        let _ = std::fs::remove_file(&source_path);
        let _ = std::fs::remove_file(&output_path);
    }

    #[test]
    fn test_format_enum_initials_no_conflicts() {
        let variants = vec![
            NameAndInitialValue {
                name: String::from("A"),
                initial_value: String::from("0"),
            },
            NameAndInitialValue {
                name: String::from("B"),
                initial_value: String::from("1"),
            },
            NameAndInitialValue {
                name: String::from("C"),
                initial_value: String::from("2"),
            },
        ];

        let result = format_enum_initials(variants);
        assert_eq!(result.len(), 3);
    }

    #[test]
    fn test_format_enum_initials_with_conflicts() {
        let variants = vec![
            NameAndInitialValue {
                name: String::from("A"),
                initial_value: String::from("0"),
            },
            NameAndInitialValue {
                name: String::from("B"),
                initial_value: String::from("0"), // conflict with A
            },
            NameAndInitialValue {
                name: String::from("C"),
                initial_value: String::from("1"), // conflict with auto-incremented B
            },
        ];

        let result = format_enum_initials(variants);
        assert_eq!(result.len(), 3);
        // After resolution: A=0, B=1 (incremented), C=2 (incremented since 1 is taken)
    }

    #[test]
    fn test_format_enum_initials_empty() {
        let variants: Vec<NameAndInitialValue> = vec![];
        let result = format_enum_initials(variants);
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_format_enum_initials_single_element() {
        let variants = vec![NameAndInitialValue {
            name: String::from("ONLY"),
            initial_value: String::from("42"),
        }];

        let result = format_enum_initials(variants);
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn test_format_enum_initials_all_same_value() {
        let variants = vec![
            NameAndInitialValue {
                name: String::from("X"),
                initial_value: String::from("5"),
            },
            NameAndInitialValue {
                name: String::from("Y"),
                initial_value: String::from("5"),
            },
            NameAndInitialValue {
                name: String::from("Z"),
                initial_value: String::from("5"),
            },
        ];

        let result = format_enum_initials(variants);
        assert_eq!(result.len(), 3);
        // Should auto-increment: X=5, Y=6, Z=7
    }

    #[test]
    fn test_parse_project_empty_units() {
        let temp_dir = std::env::temp_dir();
        let output_path = temp_dir.join("test_parse_empty_units.xml");
        let template = get_omron_template();
        let params = GenerationParameters::new();
        let units: Vec<&CompilationUnit> = vec![];

        let result =
            parse_project_into_nodetree(&params, &units, OMRON_SCHEMA, &output_path, template);
        assert!(result.is_ok());
        assert!(output_path.exists());

        let _ = std::fs::remove_file(&output_path);
    }

    #[test]
    fn test_write_xml_nested_structure() {
        let temp_dir = std::env::temp_dir();
        let output_path = temp_dir.join("test_nested_structure.xml");

        let inner_child = STypeName::new().content(String::from("DINT"));
        let type_node = SType::new().child(&inner_child);
        let member = SMember::new()
            .attribute_str("name", "field1")
            .child(&type_node);
        let root = Node::new_str("DataType").child(&member);

        let result = write_xml_file(&output_path, root);
        assert!(result.is_ok());

        let contents = std::fs::read_to_string(&output_path).unwrap();
        assert!(contents.contains("DataType"));
        assert!(contents.contains("Member"));
        assert!(contents.contains("field1"));
        assert!(contents.contains("DINT"));

        let _ = std::fs::remove_file(&output_path);
    }

    #[test]
    fn test_generation_parameters_omron_flag() {
        let mut params = GenerationParameters::new();
        assert!(!params.output_xml_omron);

        params.output_xml_omron = true;
        assert!(params.output_xml_omron);
    }

    #[test]
    fn test_write_xml_file_produces_valid_xml_header() {
        let temp_dir = std::env::temp_dir();
        let output_path = temp_dir.join("test_xml_header.xml");

        let node = Node::new_str("Root");
        write_xml_file(&output_path, node).unwrap();

        let contents = std::fs::read_to_string(&output_path).unwrap();
        assert!(contents.starts_with("<?xml version=\"1.0\" encoding=\"UTF-8\"?>"));

        let _ = std::fs::remove_file(&output_path);
    }

    #[test]
    fn test_write_xml_empty_node() {
        let temp_dir = std::env::temp_dir();
        let output_path = temp_dir.join("test_empty_node.xml");

        let node = Node::new_str("Empty");
        write_xml_file(&output_path, node).unwrap();

        let contents = std::fs::read_to_string(&output_path).unwrap();
        assert!(contents.contains("Empty"));

        let _ = std::fs::remove_file(&output_path);
    }

    #[test]
    fn test_write_xml_cdata_content() {
        let temp_dir = std::env::temp_dir();
        let output_path = temp_dir.join("test_cdata_content.xml");

        let node = Node::new_str("Code");
        // Node with content should produce CDATA
        let mut node_with_content = node;
        node_with_content.content = Some(String::from("x := 1 + 2;"));

        write_xml_file(&output_path, node_with_content).unwrap();

        let contents = std::fs::read_to_string(&output_path).unwrap();
        assert!(contents.contains("x := 1 + 2;"));

        let _ = std::fs::remove_file(&output_path);
    }

    #[test]
    fn test_format_enum_initials_negative_values() {
        let variants = vec![
            NameAndInitialValue {
                name: String::from("NEG"),
                initial_value: String::from("-1"),
            },
            NameAndInitialValue {
                name: String::from("NEG2"),
                initial_value: String::from("-1"), // conflict
            },
        ];

        let result = format_enum_initials(variants);
        assert_eq!(result.len(), 2);
        // NEG=-1, NEG2=0 (incremented from -1)
    }

    #[test]
    fn test_format_enum_initials_consecutive_conflicts() {
        let variants = vec![
            NameAndInitialValue {
                name: String::from("A"),
                initial_value: String::from("0"),
            },
            NameAndInitialValue {
                name: String::from("B"),
                initial_value: String::from("1"),
            },
            NameAndInitialValue {
                name: String::from("C"),
                initial_value: String::from("0"), // conflicts with A, tries 1 (taken by B), settles on 2
            },
        ];

        let result = format_enum_initials(variants);
        assert_eq!(result.len(), 3);
    }
}