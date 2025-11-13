//! The initializer lowering module is responsible for additing initialization logic
//! to PLC AST nodes. This includes generating default values for variables, handling
//! constant expressions, and ensuring that all necessary initializations are present
//! before code generation. The module traverses the AST and modifies nodes as needed
//! to include initialization code, making sure that the resulting AST is ready for
//! further compilation stages.
//! Initialization logic is as follows:
//! - Every struct(and POU) has an optional implicit initializer for constant fields.
//!     - The name for this initializer is `<StructName>_init`
//!     - The field is always private to the module
//! - Every struct(and POU) has a constructor for fields with pointer assignments or non-constant values
//!    - The name for this constructor is `<StructName>_ctor`
//!    - The constructor is always public and implicitly uses the implicit initializer
//! - Variables of the struct are initialized using the implicit initializer if their value is
//!   derived at compile time, otherwise the constructor is used.
//! - Global variables are initialized in a global constructor function called `__global_ctor`
//!   - This function is called per module inside the static initialization code
//!   - The function is private to the module
//! - Stateless POUs (functions and methods) are initialized during their call.
//!     - Variables of a stateless POU of a struct type are initialized using the constructor call.
//! - External POUs and struct constructors are marked as `extern` and have no body.
//! - External variables are not re-initialized in the global constructor, they are assumed to be
//!   initialized externally.
//! - Bulit-in types and variables are not re-initialized in the global
//! constructor.

use plc::index::{FxIndexMap, Index};
use plc_ast::{ast::AstNode, visitor::AstVisitor};

enum Body {
    Internal(Vec<AstNode>),
    External,
    None,
}

pub struct Initializer<'idx> {
    index: &'idx Index,
    implicit_initializers: FxIndexMap<String, Body>,
    constructors: FxIndexMap<String, Body>,
    global_constructor: Vec<AstNode>,
}

//TODO: might need to be a mutable ast visitor
impl AstVisitor for Initializer<'_> {
    fn visit_compilation_unit(&mut self, unit: &plc_ast::ast::CompilationUnit) {
        // Read all structs and POU structs, collect their implicit initializers if available
        unit.pous.iter().for_each(|pou| {
            // find the pou index entry
            if let Some(pie) = self.index.find_pou_type(&pou.name) {
                pie.initial_value
            }
        });
        // Add a call to the constructor to memcpy the imlicit initializer
        // For each of the call statement or reference in the pou initializer, add an assignment to
        // the constructor
    }
}

impl Initializer<'_> {
    pub fn new(index: &Index) -> Initializer<'_> {
        Initializer {
            index,
            implicit_initializers: FxIndexMap::default(),
            constructors: FxIndexMap::default(),
            global_constructor: Vec::new(),
        }
    }
}

mod tests {
    #[test]
    fn struct_gets_imlicit_initializer_and_constructor() {}

    #[test]
    fn struct_gets_constructor() {}

    #[test]
    fn nested_structs_get_initializers_and_constructors() {}

    #[test]
    fn variable_with_pointer_initializer_is_added_to_constructor() {}

    #[test]
    fn enum_default_values_in_struct() {}

    #[test]
    fn nested_struct_with_different_default_values() {}
}
