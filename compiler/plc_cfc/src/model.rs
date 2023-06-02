use std::{collections::HashMap, str::FromStr};

use crate::error::Error;

trait GetOrErr {
    fn get_or_err(&self, key: &str) -> Result<String, Error>;
}

impl GetOrErr for HashMap<String, String> {
    // TODO: Use `remove`
    fn get_or_err(&self, key: &str) -> Result<String, Error> {
        self.get(key).map(|it| it.to_owned()).ok_or(Error::MissingAttribute(key.to_string()))
    }
}

#[derive(Debug)]
pub(crate) struct FunctionBlockDiagram {
    pub blocks: Vec<Block>,
    pub variables: Vec<FunctionBlockVariable>,
    pub controls: Vec<Control>,
    pub connectors: Vec<Connector>,
}

#[derive(Debug)]
pub(crate) struct Connector {
    pub kind: ConnectorKind,
    pub name: String,
    pub local_id: usize,
    pub ref_local_id: Option<usize>,
    pub global_id: Option<usize>,
    pub formal_parameter: Option<String>,
}

#[derive(Debug)]
pub(crate) struct Control {
    pub kind: ControlKind,
    pub name: Option<String>,
    pub local_id: usize,
    pub global_id: Option<usize>,
    pub ref_local_id: Option<usize>,
    pub execution_order_id: Option<usize>,
    pub negated: bool,
}

impl Connector {
    pub fn new(mut hm: HashMap<String, String>, kind: ConnectorKind) -> Result<Self, Error> {
        Ok(Self {
            kind,
            name: hm.get_or_err("name")?,
            local_id: hm.get_or_err("localId").map(|it| it.parse())??,
            ref_local_id: hm.get("refLocalId").map(|it| it.parse()).transpose()?,
            global_id: hm.get("globalId").map(|it| it.parse()).transpose()?,
            formal_parameter: hm.remove("formalParameter"),
        })
    }
}

// impl Continuation {
//     pub fn new(mut hm: HashMap<String, String>, kind: ControlKind) -> Result<Self, Error> {
//         Ok(Self {})
//     }
// }

impl Control {
    pub fn new(mut hm: HashMap<String, String>, kind: ControlKind) -> Result<Self, Error> {
        Ok(Self {
            kind,
            name: hm.remove("label"),
            local_id: hm.get_or_err("localId").map(|it| it.parse())??,
            global_id: hm.get("globalId").map(|it| it.parse()).transpose()?,
            ref_local_id: hm.get("refLocalId").map(|it| it.parse()).transpose()?,
            execution_order_id: hm.get("executionOrderId").map(|it| it.parse()).transpose()?,
            negated: hm.get("negated").map(|it| it == "true").unwrap_or(false),
        })
    }
}

#[derive(Debug)]
pub(crate) enum ControlKind {
    Jump,
    Label,
    Return,
}

#[derive(Debug)]
pub(crate) enum ConnectorKind {
    Source,
    Sink,
}

#[derive(Debug)]
pub(crate) struct Block {
    pub local_id: usize,
    pub global_id: Option<usize>,
    pub type_name: String,
    pub instance_name: Option<String>,
    pub execution_order_id: Option<usize>,
    pub variables: Vec<BlockVariable>,
}

impl Block {
    pub fn new(mut hm: HashMap<String, String>, variables: Vec<BlockVariable>) -> Result<Self, Error> {
        Ok(Self {
            local_id: hm.get_or_err("localId").map(|it| it.parse())??,
            global_id: hm.get("globalId").map(|it| it.parse()).transpose()?,
            type_name: hm.get_or_err("typeName")?,
            instance_name: hm.remove("instanceName"),
            execution_order_id: hm.get("executionOrderId").map(|it| it.parse()).transpose()?,
            variables,
        })
    }
}

#[derive(Debug)]
pub(crate) struct BlockVariable {
    pub kind: VariableKind,
    pub formal_parameter: String,
    pub negated: bool,
    pub ref_local_id: Option<usize>,
    pub edge: Option<Edge>,
    pub storage: Option<Storage>,
    pub enable: Option<bool>,
}

#[derive(Debug)]
pub(crate) enum Edge {
    Falling,
    Rising,
}

#[derive(Debug)]
pub(crate) enum Storage {
    Set,
    Reset,
}

impl BlockVariable {
    pub fn new(hm: HashMap<String, String>, kind: VariableKind) -> Result<Self, Error> {
        Ok(Self {
            kind,
            formal_parameter: hm.get_or_err("formalParameter")?,
            negated: hm.get_or_err("negated").map(|it| it == "true")?,
            ref_local_id: hm.get("refLocalId").map(|it| it.parse()).transpose()?,
            edge: hm.get("edge").map(|it| it.parse()).transpose()?,
            storage: hm.get("storage").map(|it| it.parse()).transpose()?,
            enable: hm.get("enable").map(|it| it == "true"),
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum VariableKind {
    Input,
    Output,
    InOut,
}

#[derive(Debug)]
pub(crate) struct FunctionBlockVariable {
    pub kind: VariableKind,
    pub local_id: usize,
    pub negated: bool,
    pub expression: String,
    pub execution_order_id: Option<usize>,
    pub ref_local_id: Option<usize>,
}

impl FunctionBlockVariable {
    pub fn new(hm: HashMap<String, String>, kind: VariableKind) -> Result<Self, Error> {
        Ok(Self {
            kind,
            local_id: hm.get_or_err("localId").map(|it| it.parse())??,
            negated: hm.get_or_err("negated").map(|it| it == "true")?,
            expression: hm.get_or_err("expression")?,
            execution_order_id: hm.get("executionOrderId").map(|it| it.parse()).transpose()?,
            ref_local_id: hm.get("refLocalId").map(|it| it.parse()).transpose()?,
        })
    }
}

#[derive(Debug)]
pub(crate) struct Body {
    pub function_block_diagram: FunctionBlockDiagram,
    pub global_id: Option<usize>,
}

impl Body {
    pub fn new(hm: HashMap<String, String>, fbd: FunctionBlockDiagram) -> Result<Self, Error> {
        Ok(Self {
            function_block_diagram: fbd,
            global_id: hm.get("globalId").map(|it| it.parse()).transpose()?,
        })
    }
}

#[derive(Debug)]
pub(crate) struct Pou {
    // TODO: interface
    pub name: String,
    pub pou_type: PouType,
    pub body: Body,
}

impl Pou {
    pub fn new(hm: HashMap<String, String>, body: Body) -> Result<Self, Error> {
        Ok(Self {
            name: hm.get_or_err("name")?,
            pou_type: hm.get_or_err("pouType").map(|it| it.parse())??,
            body,
        })
    }
}

#[derive(Debug)]
pub(crate) enum PouType {
    Program,
    Function,
    FunctionBlock,
}

impl TryFrom<&str> for PouType {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "program" => Ok(PouType::Program),
            "function" => Ok(PouType::Function),
            "functionBlock" => Ok(PouType::FunctionBlock),
            _ => Err(Error::UnexpectedElement(value.to_string())),
        }
    }
}

impl TryFrom<&[u8]> for VariableKind {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        match value {
            b"inputVariables" | b"inVariable" => Ok(VariableKind::Input),
            b"outputVariables" | b"outVariable" => Ok(VariableKind::Output),
            b"inOutVariables" | b"inOutVariable" => Ok(VariableKind::InOut),
            _ => {
                let value = std::str::from_utf8(value).map_err(Error::Encoding)?;
                Err(Error::UnexpectedElement(value.to_string()))
            }
        }
    }
}

// TODO: these impls should probably return a parse error instead of UnexpectedElement?

impl FromStr for Edge {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "falling" => Ok(Edge::Falling),
            "rising" => Ok(Edge::Rising),
            _ => Err(Error::UnexpectedElement(s.to_string())),
        }
    }
}

impl FromStr for Storage {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "set" => Ok(Storage::Set),
            "reset" => Ok(Storage::Reset),
            _ => Err(Error::UnexpectedElement(s.to_string())),
        }
    }
}

impl FromStr for PouType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "program" => Ok(PouType::Program),
            "function" => Ok(PouType::Function),
            "functionBlock" => Ok(PouType::FunctionBlock),
            _ => Err(Error::UnexpectedElement(s.to_string())),
        }
    }
}

impl FromStr for ControlKind {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "jump" => Ok(ControlKind::Jump),
            "label" => Ok(ControlKind::Label),
            "return" => Ok(ControlKind::Return),
            _ => Err(Error::UnexpectedElement(s.to_string())),
        }
    }
}
