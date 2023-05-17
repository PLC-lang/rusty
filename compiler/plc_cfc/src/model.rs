use std::{collections::HashMap, str::FromStr};

use quick_xml::{events::BytesStart, Reader};

use crate::{
    model::constant::{INPUT_VARIABLES, IN_OUT_VARIABLES, OUTPUT_VARIABLES},
    parser::{extract_attributes, Error, HashMapExt, Tag, Transformer},
};

use self::constant::FBD;

pub(crate) type Attributes = HashMap<String, String>;

pub(crate) mod constant {
    pub const POU: &[u8] = b"pou";
    pub const BODY: &[u8] = b"body";
    pub const INTERFACE: &[u8] = b"interface";
    pub const ACTIONS: &[u8] = b"actions";
    pub const TRANSITIONS: &[u8] = b"transitions";
    pub const FBD: &[u8] = b"FBD";
    pub const BLOCK: &[u8] = b"block";
    pub const IN_VARIABLE: &[u8] = b"inVariable";
    pub const OUT_VARIABLE: &[u8] = b"outVariable";
    pub const IN_OUT_VARIABLE: &[u8] = b"inOutVariable";
    pub const INPUT_VARIABLES: &[u8] = b"inputVariables";
    pub const OUTPUT_VARIABLES: &[u8] = b"outputVariables";
    pub const IN_OUT_VARIABLES: &[u8] = b"inOutVariables";
}

pub(crate) trait Parseable {
    type Item;
    fn parse(transformer: &mut Transformer, tag: Option<Tag>) -> Result<Self::Item, Error>;
}
#[derive(Debug, Default)]
pub(crate) struct Pou {
    pub name: String,
    pub pou_type: PouType,
    pub global_id: Option<String>,
    pub elements: Vec<PouElement>,
}

impl Pou {
    pub(crate) fn new(attr: Attributes, elements: Vec<PouElement>) -> Result<Self, Error> {
        Ok(Pou {
            name: attr.get("name").cloned().ok_or(Error::MissingAttribute("name"))?,
            pou_type: PouType::from_str(&attr.get("pouType").ok_or(Error::MissingAttribute("pouType"))?)?,
            global_id: attr.get("globalId").cloned(),
            elements,
        })
    }
}

impl Parseable for Pou {
    type Item = Self;

    fn parse(transformer: &mut Transformer, tag: Option<Tag>) -> Result<Self::Item, Error> {
        let Some(Tag::Start(_, pou_attr)) = tag else {
            panic!()
        };
        let mut elements = vec![];
        loop {
            let tag = transformer.advance()?;
            match tag {
                Tag::Start(name, attr) => match name.as_str() {
                    BODY => elements.push(PouElement::Body(Body::parse(transformer, Some(tag))?)),
                    _ => panic!(),
                },
                Tag::Empty(name, attr) => todo!(),
                Tag::End(_) => return Pou::new(pou_attr, elements),
                Tag::Skip => continue,
            }
        }
    }
}

#[derive(Debug)]
pub(crate) enum PouType {
    Function,
    Program,
    FunctionBlock,
}

impl FromStr for PouType {
    type Err = crate::parser::Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.as_ref() {
            "function" => Ok(Self::Function),
            "program" => Ok(Self::Program),
            "functionBlock" => Ok(Self::FunctionBlock),
            _ => Err(crate::parser::Error::UnexpectedAttribute(value.to_string())),
        }
    }
}

impl Default for PouType {
    fn default() -> Self {
        Self::Function
    }
}

#[derive(Debug)]
pub(crate) enum PouElement {
    Body(Body),
    Interface(Interface),
    Action(Action),
    Transition(Transition),
}

#[derive(Debug, Default)]
pub(crate) struct Body {
    pub fbd: FunctionBlockDiagram,
}

impl Parseable for Body {
    type Item = Self;

    fn parse(transformer: &mut Transformer, tag: Option<Tag>) -> Result<Self::Item, Error> {
        let Some(Tag::Start(_, pou_attr)) = tag else {
            panic!()
        };
        loop {
            let tag = transformer.advance()?;
            match tag {
                Tag::Start(name, attr) => match name.as_bytes() {
                    FBD => return Ok(Body { fbd: FunctionBlockDiagram::parse(transformer, Some(tag))? }),
                    _ => panic!(),
                },
                Tag::Empty(name, attr) => todo!(),
                Tag::End(_) => todo!(),
                Tag::Skip => continue,
            }
        }
    }
}

#[derive(Debug)]
pub(crate) struct Interface {}

#[derive(Debug)]
pub(crate) struct Action {}

#[derive(Debug)]
pub(crate) struct Transition {}

#[derive(Debug, Default)]
pub(crate) struct FunctionBlockDiagram {
    pub blocks: Vec<Block>,
    pub variables: Vec<FbdVariable>,
    // pub jump_label: todo!(),
    // pub jump_stmt: todo!(),
    // pub return_stmt: todo!(),
}

impl Parseable for FunctionBlockDiagram {
    type Item = Self;

    fn parse(transformer: &mut Transformer, tag: Option<Tag>) -> Result<Self::Item, Error> {
        let Some(Tag::Start(_, pou_attr)) = tag else {
            panic!()
        };
        let mut blocks = vec![];
        let mut variables = vec![];
        loop {
            let tag = transformer.advance()?;
            match tag {
                Tag::Start(name, attr) => match name.as_str() {
                    "block" => blocks.push(Block::parse(transformer, Some(tag))?),
                    _ => panic!(),
                },
                Tag::Empty(name, attr) => todo!(),
                Tag::End(_) => break,
                Tag::Skip => continue,
            }
        }

        Ok(FunctionBlockDiagram { blocks, variables })
    }
}
#[derive(Debug, Default)]
pub(crate) struct Block {
    pub local_id: usize,
    pub type_name: String,
    pub instance_name: Option<String>,
    pub execution_order_id: Option<usize>,
    pub variables: Vec<BlockVariable>,
}

// TODO: remove all these `cloned()` calls
impl Block {
    pub(crate) fn new(attr: Attributes, variables: Vec<BlockVariable>) -> Result<Self, Error> {
        Ok(Block {
            local_id: attr.get_or_err("localId").map(|it| it.parse::<usize>().unwrap())?,
            type_name: attr.get_or_err("typeName")?,
            instance_name: attr.get("instanceName").cloned(),
            execution_order_id: attr.get("executionOrderId").map(|it| it.parse::<usize>().unwrap()),
            variables,
        })
    }
}

impl Parseable for Block {
    type Item = Self;
    fn parse(transformer: &mut Transformer, tag: Option<Tag>) -> Result<Self::Item, Error> {
        let Some(Tag::Start(_, block_attr)) = tag else {
            panic!()
        };
        let mut variables = vec![];

        loop {
            let tag = transformer.advance()?;
            let mut vtype: Option<Tag> = None;
            match tag {
                Tag::Start(name, attr) => match name.as_bytes() {
                    INPUT_VARIABLES | OUTPUT_VARIABLES | IN_OUT_VARIABLES => {
                        // start of a new variable block. save the tag to extract the block-type when
                        // parsing this block's variables
                        vtype = Some(tag);
                    }
                    b"variable" => variables.push(BlockVariable::parse(transformer, vtype)?),
                    _ => panic!(),
                },
                Tag::Empty(name, attr) => todo!(),
                Tag::End(name) => match name.as_bytes() {
                    b"/block" => break,
                    _ => continue,
                },
                Tag::Skip => continue,
            }
        }

        Block::new(block_attr, variables)
    }
}

impl Parseable for BlockVariable {
    type Item = Self;

    fn parse(transformer: &mut Transformer, tag: Option<Tag>) -> Result<Self::Item, Error> {
        let Some(Tag::Start(name, _)) = tag else {
            panic!()
        };
        let kind = BlockVariableKind::from(name.as_bytes());

        todo!()
    }
}

#[derive(Debug)]
pub(crate) struct BlockVariable {
    pub formal_parameter: String,
    pub negated: bool, // optional, should default to false
    pub edge: Option<Edge>,
    pub storage_modifier: Option<Storage>,
    pub hidden: bool,
    pub connection_point: Option<ConnectionPoint>,
    pub kind: BlockVariableKind,
}

impl BlockVariable {
    pub(crate) fn new(
        tag: BytesStart,
        connection_point: Option<ConnectionPoint>,
        kind: &BlockVariableKind,
    ) -> Result<Self, Error> {
        let attr = extract_attributes(tag);
        Ok(Self {
            formal_parameter: attr.get_or_err("formalParameter")?,
            negated: attr.get("negated").and_then(|it| Some(it == "true")).unwrap_or(false),
            edge: attr.get("edge").and_then(|it| Edge::try_from(it.as_str()).ok()),
            storage_modifier: attr.get("storageModifier").and_then(|it| Storage::try_from(it.as_str()).ok()),
            hidden: attr.get("hidden").and_then(|it| Some(it == "true")).unwrap_or(false),
            connection_point,
            kind: *kind,
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum BlockVariableKind {
    Input,
    InOut,
    Output,
}

impl From<&[u8]> for BlockVariableKind {
    fn from(value: &[u8]) -> Self {
        match value {
            INPUT_VARIABLES => Self::Input,
            OUTPUT_VARIABLES => Self::Output,
            IN_OUT_VARIABLES => Self::InOut,
        }
    }
}

#[derive(Debug)]
pub(crate) enum FbdVariableKind {
    Input(FbdVariable),
    InOut(FbdVariable),
    Output(FbdVariable),
}

#[derive(Debug)]
pub(crate) struct FbdVariable {
    /*
    in variable:
        sequence:
            connection point out
            expression (?)

     */
}

#[derive(Debug)]
pub(crate) enum Edge {
    Rising,
    Falling,
    None, // wtf
}

impl TryFrom<&str> for Edge {
    type Error = crate::parser::Error;
    fn try_from(value: &str) -> Result<Self, Error> {
        match value {
            "rising" => Ok(Self::Rising),
            "falling" => Ok(Self::Falling),
            _ => Err(Error::TryFrom),
        }
    }
}

#[derive(Debug)]
pub(crate) struct ConnectionPoint {
    pub global_id: Option<usize>,
    pub connection: Option<Connection>,
    pub kind: Option<ConnectionPointKind>,
}

#[derive(Debug)]
pub(crate) struct Connection {
    pub global_id: usize,
    pub ref_local_id: usize,
}

#[derive(Debug)]
pub(crate) enum ConnectionPointKind {
    In,
    Out,
}

#[derive(Debug)]
// confirm this
pub(crate) enum Storage {
    Set,
    Reset,
    None,
}

impl TryFrom<&str> for Storage {
    type Error = crate::parser::Error;
    fn try_from(value: &str) -> Result<Self, Error> {
        match value {
            "set" => Ok(Self::Set),
            "res" => Ok(Self::Reset),
            _ => Err(Error::TryFrom),
        }
    }
}
