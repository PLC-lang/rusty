//! This crate provides functionality for encoding and decoding the type
//! information of Strutured Text functions and variables. The goal is to store
//! that information in the resulting binary, in the section name containing the
//! function/variable. This enables a Structured Text loader to access this
//! type information. In order to stay simple and light, this crate should
//! not rely on too many dependencies other than the standard library.
//!
//! The main type provided by this struct is [`SectionMangler`], a simple
//! builder for creating mangling contexts for functions and variables. These
//! contexts can then be mangled down to a string, as well as be recreated from a
//! mangled string.
//!
//! You will notice the use of `unreachable!()` in a lot of places, since this crate
//! relies on fully typechecked global variables and functions. If that is not
//! the case, and you try adding a return type to a global variable, then it is
//! a programming error and the compiler should crash.
//!
//! ## Mangling Scheme
//!
//! There are two main mangling schemes currently implemented: one for
//! functions, and one for global variables. They are distinct by the prefix
//! they use at the beginning of the mangled string: `fn` for functions, and `var`
//! for variables.
//!
//! ### Mangling global variables
//!
//! ```text
//! var-<name>:<type>
//! ```
//!
//! Is is necessary to know the type of a global variable in order to encode it.
//! That type is appened to the name of the variable, after a colon.
//!
//! ### Mangling functions
//!
//! ```text
//! fn-<name>:<return_type>[<arg1>][<arg2>][<arg3>]
//! ```
//!
//! Just like global variables, the function's name is added right after the
//! function prefix. Then, a colon, and the return type of the function. Each of the
//! function's parameters' type is then added, surrounded by brackets (`[` and `]`).
//! This eases the parsing of the type.

use std::fmt;

mod parser;

/// The main builder type of this crate. Use it to create mangling contexts, in
/// order to encode and decode binary type information.
// TODO: Add example code for using this builder
#[derive(Debug, PartialEq, Clone)]
pub enum SectionMangler {
    Function(FunctionMangler),
    Variable(VariableMangler),
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionMangler {
    name: String,
    parameters: Vec<FunctionArgument>,
    return_type: Option<Type>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct VariableMangler {
    name: String,
    ty: Type,
}

pub const RUSTY_PREFIX: &str = "$RUSTY$";

// TODO: How to encode variadics?
fn mangle_function(FunctionMangler { name, parameters, return_type }: FunctionMangler) -> String {
    /* FIXME: Is that correct? */
    let return_type = return_type.unwrap_or(Type::Void);

    let mangled = match parameters.as_slice() {
        [] => format!("{return_type}[]"),
        parameters => {
            parameters.iter().fold(return_type.to_string(), |mangled, arg| format!("{mangled}[{arg}]"))
        }
    };

    format!("{name}:{mangled}")
}

fn mangle_variable(VariableMangler { name, ty }: VariableMangler) -> String {
    format!("{name}:{ty}")
}

impl SectionMangler {
    pub fn function<S: Into<String>>(name: S) -> SectionMangler {
        SectionMangler::Function(FunctionMangler { name: name.into(), parameters: vec![], return_type: None })
    }

    pub fn variable<S: Into<String>>(name: S, ty: Type) -> SectionMangler {
        SectionMangler::Variable(VariableMangler { name: name.into(), ty })
    }

    pub fn name(&self) -> &str {
        match self {
            SectionMangler::Function(FunctionMangler { name, .. })
            | SectionMangler::Variable(VariableMangler { name, .. }) => name,
        }
    }

    pub fn with_parameter(self, param: FunctionArgument) -> SectionMangler {
        match self {
            SectionMangler::Function(f) => {
                let mut parameters = f.parameters;
                parameters.push(param);

                Self::Function(FunctionMangler { parameters, ..f })
            }
            SectionMangler::Variable(_) => unreachable!("global variables do not accept parameters."),
        }
    }

    pub fn with_return_type(self, return_type: Type) -> SectionMangler {
        match self {
            SectionMangler::Function(f) => {
                SectionMangler::Function(FunctionMangler { return_type: Some(return_type), ..f })
            }
            SectionMangler::Variable(_) => unreachable!("global variables do not have a return type."),
        }
    }

    pub fn mangle(self) -> String {
        let (prefix, content) = match self {
            SectionMangler::Function(f) => ("fn", mangle_function(f)),
            SectionMangler::Variable(v) => ("var", mangle_variable(v)),
        };

        format!("{RUSTY_PREFIX}{prefix}-{content}")
    }
}

// TODO: We have to encode if the initial value changes or not
// pub initial_value: Option<ConstId>,

// TODO: Handle ArgumentType, which looks like this: `enum ArgumentType { ByVal(VariableType), ByRef(VariableType) }`
// and `enum VariableType { Local, Temp, Input, Output, InOut, Global, Return }`
// so this is not really about the type in itself, but about how the parameter is passed into the function?
// pub argument_type: ArgumentType,
// NOTE: This is called `variable_linkage` in the `MemberInfo` struct.

/// We have to encode this because if it changes, the function needs to be reloaded - this is an ABI breakage
#[derive(Debug, PartialEq, Clone)]
pub enum FunctionArgument {
    ByValue(Type),
    ByRef(Type),
}

impl fmt::Display for FunctionArgument {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // FIXME: Mangle by-value and by-ref args differently
        match self {
            FunctionArgument::ByValue(ty) => write!(f, "{ty}"),
            FunctionArgument::ByRef(ty) => write!(f, "{ty}"),
        }
    }
}

// TODO: Do we have to encode this? Does that affect ABI? Probably
#[derive(Debug, PartialEq, Clone)]
pub enum StringEncoding {
    // TODO: Should we encode this differently? this could cause problems compared to encoding unsigned types
    /// Encoded as `8u`
    Utf8,
    /// Encoded as `16u`
    Utf16,
}

impl fmt::Display for StringEncoding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StringEncoding::Utf8 => write!(f, "8u"),
            StringEncoding::Utf16 => write!(f, "16u"),
        }
    }
}

// This maps directly to the [`DataTypeInformation`] enum in RuSTy - we simply remove some fields and add the ability to encode/decode serialize/deserialize
// TODO: Do we have to handle Generic?
#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    /// Encoded as `v`
    Void,
    /// Encoded as `i<size>` or `u<size>`
    // TODO: Handle semantic size
    Integer {
        signed: bool,
        size: u32,
        // TODO: Can the semantic size change without the size changing? Does that need a reload?
        semantic_size: Option<u32>,
    },
    /// Encoded as `f<size>`
    Float {
        size: u32,
    },
    /// Encoded as `s<encoding><size>`
    String {
        size: usize, // FIXME: Is that okay? will all the constant expressions be folded at that point? Can we have TypeSize::Undetermined still?
        encoding: StringEncoding,
    },
    /// Encoded as `p<inner>`. For example, a 32bit int pointer will become `pi32`
    Pointer {
        inner: Box<Type>,
        // TODO: Is changing the `auto_deref` mode an ABI break?
        // auto_deref: bool,
    },
    Struct {
        members: Vec<Type>,
    },
    Enum {
        referenced_type: Box<Type>,
        elements: usize,
    },
    Array {
        inner: Box<Type>,
        // FIXME: Handle dimensions here
        // dimensions: Vec<Dimension>,
    },
    SubRange {
        // name: TypeId,
        // referenced_type: TypeId,
        // sub_range: Range<AstNode>,
    },
    Alias {
        // name: TypeId,
        // referenced_type: TypeId,
    },
    Generic {
        // name: TypeId,
        // generic_symbol: String,
        // nature: TypeNature,
    },
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Void => write!(f, "v"),
            Type::Integer { signed: true, size, semantic_size: _ } => {
                // FIXME: Handle semantic_size
                write!(f, "i{size}")
            }
            Type::Integer { signed: false, size, semantic_size: _ } => {
                // FIXME: Handle semantic_size
                write!(f, "u{size}")
            }
            Type::Float { size } => write!(f, "f{size}"),
            Type::String { size, encoding } => write!(f, "s{encoding}{size}",),
            Type::Pointer { inner } => write!(f, "p{}", inner),
            Type::Struct { members } => {
                write!(
                    f,
                    "r{}{}",
                    members.len(),
                    members.iter().fold(String::new(), |acc, m| format!("{acc}{m}"))
                )
            }
            Type::Enum { referenced_type, elements } => write!(f, "e{elements}{referenced_type}"),
            Type::Array { inner } => write!(f, "a{inner}"),
            // -- Unimplemented
            Type::SubRange {} => todo!(),
            Type::Alias {} => todo!(),
            Type::Generic {} => todo!(),
        }
    }
}
