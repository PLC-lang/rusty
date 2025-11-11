use plc_ast::{
    ast::{
        self, AstNode, AstStatement, CompilationUnit, DataTypeDeclaration, ReferenceAccess,
        UserTypeDeclaration,
    },
    literals::AstLiteral,
};
use plc_diagnostics::diagnostics::Diagnostic;

use crate::{
    header_generator::{
        file_helper::FileHelper, header_generator_c::GeneratedHeaderForC, symbol_helper::SymbolHelper,
        template_helper::TemplateHelper, type_helper::TypeHelper,
    },
    GenerateHeaderOptions, GenerateLanguage,
};

mod file_helper;
mod header_generator_c;
mod symbol_helper;
mod template_helper;
mod type_helper;

pub trait GeneratedHeader: FileHelper + TypeHelper + TemplateHelper + SymbolHelper {
    fn is_empty(&self) -> bool;
    fn get_directory(&self) -> &str;
    fn get_path(&self) -> &str;
    fn get_contents(&self) -> &str;
    fn generate_headers(&mut self, compilation_unit: &CompilationUnit) -> Result<(), Diagnostic>;
}

pub fn get_generated_header(
    generate_header_options: &GenerateHeaderOptions,
    compilation_unit: &CompilationUnit,
) -> Result<Box<dyn GeneratedHeader>, Diagnostic> {
    let mut generated_header: Box<dyn GeneratedHeader> = match generate_header_options.language {
        GenerateLanguage::C => {
            let generated_header = GeneratedHeaderForC::new();
            Box::new(generated_header)
        }
        language => panic!("This language '{:?}' is not yet implemented!", language),
    };

    // Determine file and directory
    // If the directories could not be configured with an acceptable outcome, then we exit without performing generation for this compilation unit
    if !generated_header.determine_header_file_information(generate_header_options, compilation_unit) {
        return Ok(generated_header);
    }

    // Generate the headers
    generated_header.generate_headers(compilation_unit)?;

    Ok(generated_header)
}

pub enum GenerationSource {
    GlobalVariable,
    UserType,
    Struct,
    FunctionParameter,
}

pub struct ExtendedTypeName {
    pub type_name: String,
    pub is_variadic: bool,
}

impl Default for ExtendedTypeName {
    fn default() -> Self {
        Self::new()
    }
}

impl ExtendedTypeName {
    pub const fn new() -> Self {
        ExtendedTypeName { type_name: String::new(), is_variadic: false }
    }
}

fn coalesce_optional_strings_with_default(
    name: &Option<String>,
    field_name_override: Option<&String>,
) -> String {
    if let Some(field_name_ovr) = field_name_override {
        field_name_ovr.to_string()
    } else {
        name.clone().unwrap_or_default()
    }
}

fn extract_enum_field_name_from_statement(statement: &AstStatement) -> String {
    match statement {
        AstStatement::ReferenceExpr(reference_expression) => match &reference_expression.access {
            ReferenceAccess::Member(member_node) => {
                let member_statement = member_node.get_stmt();
                match member_statement {
                    AstStatement::Identifier(enum_field) => enum_field.to_string(),
                    _ => String::new(),
                }
            }
            _ => String::new(),
        },
        _ => String::new(),
    }
}

fn extract_enum_field_value_from_statement(statement: &AstStatement) -> String {
    match statement {
        AstStatement::Literal(literal) => literal.get_literal_value(),
        _ => String::new(),
    }
}

fn get_type_from_data_type_decleration(
    data_type_declaration: &Option<DataTypeDeclaration>,
) -> ExtendedTypeName {
    match data_type_declaration {
        Some(DataTypeDeclaration::Reference { referenced_type, .. }) => {
            ExtendedTypeName { type_name: referenced_type.clone(), is_variadic: false }
        }
        Some(DataTypeDeclaration::Definition { data_type, .. }) => {
            let type_name: String = data_type.get_name().unwrap_or("").to_string();
            let is_variadic = matches!(&**data_type, ast::DataType::VarArgs { .. });

            ExtendedTypeName { type_name, is_variadic }
        }
        _ => ExtendedTypeName::new(),
    }
}

fn get_user_generated_type_by_name<'a>(
    name: &'a str,
    user_types: &'a [UserTypeDeclaration],
) -> Option<&'a UserTypeDeclaration> {
    for user_type in user_types {
        if let Some(data_type_name) = user_type.data_type.get_name() {
            if data_type_name == name {
                return Some(user_type);
            }
        }
    }

    None
}

fn extract_string_size(size: &Option<AstNode>) -> String {
    if size.is_none() {
        return String::new();
    }

    let size = size.clone().unwrap();

    match size.stmt {
        // TODO: Verify this is necessary
        // +1 character for the string-termination-marker
        AstStatement::Literal(AstLiteral::Integer(value)) => format!("{}", value + 1),
        _ => String::new(),
    }
}

fn extract_array_size(bounds: &AstNode) -> String {
    match &bounds.stmt {
        AstStatement::RangeStatement(range_stmt) => {
            let start_value = match range_stmt.start.get_stmt() {
                AstStatement::Literal(AstLiteral::Integer(value)) => *value,
                _ => i128::default(),
            };

            let end_value = match range_stmt.end.get_stmt() {
                AstStatement::Literal(AstLiteral::Integer(value)) => *value,
                _ => i128::default(),
            };

            format!("{}", end_value - start_value + 1)
        }
        _ => String::new(),
    }
}

/*
    Refactor Notes:

    ---------------------------------------------
    -- Struct return type for header rendering --
    ---------------------------------------------

    {
        "global_variables": [
            {
                "data_type": "some-data-type", // Could include the reference
                "name": "some-field-name",
                "size": null
            },
            ...
        ],
        "user_defined_data_types": {
            "structs": [
                {
                    "name": "some-struct-name",
                    "variables": [
                        {
                            "data_type": "some-data-type",
                            "name": "some-field-name"
                        },
                        ...
                    ]
                },
                ...
            ],
            "enums": [
                {
                    "name": "some-enum-name",
                    "variables": [
                        {
                            "name": "some-enum-field-name",
                            "value": null
                        },
                        ...
                    ]
                },
                ...
            ]
        },
        "functions": [
            {
                "return_type": "some-data-type",
                "name": "some-function-name",
                "parameters": [
                    {
                        "data_type": "some-data-type",
                        "name": "some-field-name"
                    },
                    ...
                ],
                "is_variadic": false
            },
            ...
        ]
    }

    -------------
    -- EXAMPLE --
    -------------
    The following ST interface (.pli file):
    ```st
    VAR_GLOBAL
        globalCounter: INT;
    END_VAR

    TYPE RGB : (
            red,
            green,
            blue
        );
    END_TYPE

    TYPE ColourInfo:
        STRUCT
            timesPicked : INT;
            primaryColour : RGB;
        END_STRUCT
    END_TYPE

    FUNCTION PrintStatistics
    VAR_INPUT
        runCount: INT;
        colours: {sized} ColourInfo...;
    END_VAR
    END_FUNCTION

    FUNCTION_BLOCK ColourTracker
    VAR
        internalCount : INT;
    END_VAR
    VAR_OUTPUT
        printedInfo : STRING;
    END_VAR
    VAR_IN_OUT
        colour : ColourInfo;
    END_VAR
    END_FUNCTION_BLOCK
    ```

    ... will result in the following struct:
    ```json
    {
        "global_variables": [
            {
                "data_type": "int16_t",
                "name": "globalCounter",
                "is_reference": false,
                "size": null
            }
        ],
        "user_defined_data_types": {
            "structs": [
                {
                    "name": "ColourInfo",
                    "variables": [
                        {
                            "data_type": "int16_t",
                            "name": "timesPicked"
                        },
                        {
                            "data_type": "RGB",
                            "name": "primaryColour"
                        }
                    ]
                },
                {
                    "name": "ColourTracker_type",
                    "variables": [
                        {
                            "data_type": "uint64_t*",
                            "name": "__vtable"
                        },
                        {
                            "data_type": "int16_t",
                            "name": "internalCount"
                        },
                        {
                            "data_type": "char*",
                            "name": "printedInfo"
                        },
                        {
                            "data_type": "ColourInfo*",
                            "name": "colour"
                        }
                    ]
                }
            ],
            "enums": [
                {
                    "name": "eRGB",
                    "variables": [
                        {
                            "name": "red",
                            "value": "0"
                        },
                        {
                            "name": "green",
                            "value": null
                        },
                        {
                            "name": "blue",
                            "value": null
                        }
                    ]
                }
            ]
        },
        "functions": [
            {
                "return_type": "void",
                "name": "PrintStatistics",
                "parameters": [
                    {
                        "data_type": "int16_t",
                        "name": "runCount"
                    }
                ],
                "is_variadic": true
            },
            {
                "return_type": "void",
                "name": "ColourTracker",
                "parameters": [
                    {
                        "data_type": "ColourTracker_type*",
                        "name": "self"
                    }
                ],
                "is_variadic": false
            }
        ]
    }
    ```

    ... and that will result in the follow C header:
    ```c
    extern int16_t globalCounter;

    typedef enum eRGB {
        red = 0,
        green,
        blue
    } RGB;

    typedef struct {
        int16_t timesPicked;
        RGB primaryColour;
    } ColourInfo;

    typedef struct {
        uint64_t* __vtable;
        int16_t internalCount;
        char* printedInfo;
        ColourInfo* colour;
    } ColourTracker_type;

    void PrintStatistics(int16_t runCount, ...);

    void ColourTracker(ColourTracker_type* self);
    ```
*/

/*
    enum DataType {
        Struct,
        Enum,
        Array,
        Pointer(String),
        Reference(String),
    }

    struct Variable;

    struct Function;

    struct Model {
        types: HashMap<String, DataType>,
        variables: HashMap<String, Variable>,
        functions: HashMap<String, Function>,
    }

    trait Declare {
        fn declare_var(&self) -> String;
        fn declare_type(&self) -> String;
        fn declare_func(&self) -> String;
    }

*/
