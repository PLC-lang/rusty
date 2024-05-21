use quick_xml::events::{BytesStart, Event};

use crate::error::Error;
use crate::extensions::GetOrErr;
use crate::reader::Reader;
use crate::xml_parser::{get_attributes, Parseable};
use rustc_hash::FxHashMap;
use std::borrow::Cow;
use std::str::FromStr;

use super::fbd::NodeId;

#[derive(Debug, PartialEq, Eq, Hash)]
pub(crate) struct BlockVariable {
    pub kind: VariableKind,
    pub formal_parameter: String,
    pub negated: bool,
    pub ref_local_id: Option<usize>,
    pub edge: Option<Edge>,
    pub storage: Option<Storage>,
    pub enable: Option<bool>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub(crate) enum Edge {
    Falling,
    Rising,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub(crate) enum Storage {
    Set,
    Reset,
}

impl BlockVariable {
    pub fn new(hm: FxHashMap<String, String>, kind: VariableKind) -> Result<Self, Error> {
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

    pub fn update_ref(&mut self, new_ref: NodeId) {
        self.ref_local_id = Some(new_ref);
    }

    pub fn with_kind(mut self, kind: VariableKind) -> Self {
        self.kind = kind;
        self
    }
}

#[derive(Default, Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub(crate) enum VariableKind {
    Input,
    Output,
    InOut,
    #[default]
    Temp,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub(crate) struct FunctionBlockVariable<'xml> {
    pub kind: VariableKind,
    pub local_id: usize,
    pub negated: bool,
    pub expression: Cow<'xml, str>,
    pub execution_order_id: Option<usize>,
    pub ref_local_id: Option<usize>,
}

impl<'xml> FunctionBlockVariable<'xml> {
    pub fn new(hm: FxHashMap<String, String>, kind: VariableKind) -> Result<Self, Error> {
        Ok(Self {
            kind,
            local_id: hm.get_or_err("localId").map(|it| it.parse())??,
            negated: hm.get_or_err("negated").map(|it| it == "true")?,
            expression: Cow::from(hm.get_or_err("expression")?),
            execution_order_id: hm.get("executionOrderId").map(|it| it.parse()).transpose()?,
            ref_local_id: hm.get("refLocalId").map(|it| it.parse()).transpose()?,
        })
    }

    pub fn update_ref(&mut self, new_ref: NodeId) {
        self.ref_local_id = Some(new_ref);
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

impl<'xml> Parseable for FunctionBlockVariable<'xml> {
    fn visit(reader: &mut Reader, tag: Option<quick_xml::events::BytesStart>) -> Result<Self, Error> {
        let Some(tag) = tag else { unreachable!() };
        let kind = tag.name().as_ref().try_into()?;
        let mut attributes = get_attributes(tag.attributes())?;

        loop {
            match reader.read_event().map_err(Error::ReadEvent)? {
                Event::Start(tag) | Event::Empty(tag) if tag.name().as_ref() == b"connection" => {
                    attributes.extend(get_attributes(tag.attributes())?);
                }

                Event::Text(tag) => {
                    attributes.insert("expression".into(), tag.unescape()?.to_string());
                }

                Event::End(tag) => match tag.name().as_ref() {
                    b"inVariable" | b"outVariable" => {
                        break;
                    }
                    _ => {}
                },

                _ => {}
            }
        }

        FunctionBlockVariable::new(attributes, kind)
    }
}

impl Parseable for Vec<BlockVariable> {
    fn visit(reader: &mut Reader, tag: Option<BytesStart>) -> Result<Self, Error> {
        let Some(tag) = tag else { unreachable!() };

        let mut variables = vec![];
        let kind = VariableKind::try_from(tag.name().as_ref())?;
        loop {
            match reader.read_event().map_err(Error::ReadEvent)? {
                Event::Start(tag) if tag.name().as_ref() == b"variable" => {
                    variables.push(BlockVariable::visit(reader, Some(tag))?.with_kind(kind))
                }
                Event::End(tag)
                    if matches!(
                        tag.name().as_ref(),
                        b"inputVariables" | b"outputVariables" | b"inOutVariables"
                    ) =>
                {
                    break
                }

                Event::Eof => {
                    return Err(Error::UnexpectedEndOfFile(vec![
                        b"inputVariables",
                        b"outputVariables",
                        b"inOutVariables",
                    ]))
                }
                _ => {}
            };
        }
        Ok(variables)
    }
}

impl Parseable for BlockVariable {
    fn visit(reader: &mut Reader, tag: Option<quick_xml::events::BytesStart>) -> Result<Self, Error> {
        let Some(tag) = tag else { unreachable!() };

        let mut attributes = get_attributes(tag.attributes())?;
        loop {
            match reader.read_event().map_err(Error::ReadEvent)? {
                Event::Start(tag) | Event::Empty(tag) if tag.name().as_ref() == b"connection" => {
                    attributes.extend(get_attributes(tag.attributes())?);
                }

                Event::End(tag) if tag.name().as_ref() == b"variable" => {
                    break;
                }

                _ => {}
            }
        }

        BlockVariable::new(attributes, VariableKind::default())
    }
}

#[cfg(test)]
mod tests {
    use insta::assert_debug_snapshot;

    use crate::serializer::{
        SInOutVariables, SInVariable, SInputVariables, SOutVariable, SOutputVariables, SVariable,
    };
    use crate::{
        model::variables::{BlockVariable, FunctionBlockVariable},
        reader::{get_start_tag, Reader},
        xml_parser::Parseable,
    };

    #[test]
    fn block_input_variable() {
        let content = SInputVariables::new().children(vec![&SVariable::new().with_name("")]).serialize();

        let mut reader = Reader::new(&content);
        let tag = get_start_tag(reader.read_event().unwrap());
        let variables: Result<Vec<BlockVariable>, _> = Parseable::visit(&mut reader, tag);
        assert_debug_snapshot!(variables);
    }

    #[test]
    fn block_output_variable() {
        let content = SOutputVariables::new().children(vec![&SVariable::new().with_name("")]).serialize();

        let mut reader = Reader::new(&content);
        let tag = get_start_tag(reader.read_event().unwrap());
        let variables: Result<Vec<BlockVariable>, _> = Parseable::visit(&mut reader, tag);
        assert_debug_snapshot!(variables);
    }

    #[test]
    fn block_inout_variable() {
        let content = SInOutVariables::new().children(vec![&SVariable::new().with_name("")]).serialize();

        let mut reader = Reader::new(&content);
        let tag = get_start_tag(reader.read_event().unwrap());
        let variables: Result<Vec<BlockVariable>, _> = Parseable::visit(&mut reader, tag);
        assert_debug_snapshot!(variables);
    }

    #[test]
    fn fbd_in_variable() {
        let content = SInVariable::id(0).with_expression("a").serialize();

        let mut reader = Reader::new(&content);
        let tag = get_start_tag(reader.read_event().unwrap());
        assert_debug_snapshot!(FunctionBlockVariable::visit(&mut reader, tag));
    }

    #[test]
    fn fbd_out_variable() {
        let content = SOutVariable::id(0).with_expression("a").serialize();

        let mut reader = Reader::new(&content);
        let tag = get_start_tag(reader.read_event().unwrap());
        assert_debug_snapshot!(FunctionBlockVariable::visit(&mut reader, tag));
    }
}
