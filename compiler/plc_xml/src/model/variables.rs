use quick_xml::events::Event;

use crate::extensions::GetOrErr;
use crate::xml_parser::Parseable;
use crate::{error::Error, extensions::TryToString, reader::PeekableReader};
use std::borrow::Cow;
use std::{collections::HashMap, str::FromStr};

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

    pub fn update_ref(&mut self, new_ref: NodeId) {
        self.ref_local_id = Some(new_ref);
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub(crate) enum VariableKind {
    Input,
    Output,
    InOut,
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
    pub fn new(hm: HashMap<String, String>, kind: VariableKind) -> Result<Self, Error> {
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
    type Item = Self;

    fn visit(reader: &mut PeekableReader) -> Result<Self::Item, Error> {
        let next = reader.peek()?;
        let kind = match &next {
            Event::Start(tag) | Event::Empty(tag) => match tag.name().as_ref() {
                b"inVariable" => VariableKind::Input,
                b"outVariable" => VariableKind::Output,
                b"inOutVariable" => VariableKind::InOut,
                _ => unreachable!(),
            },

            _ => unreachable!(),
        };

        let mut attributes = reader.attributes()?;
        loop {
            match reader.peek()? {
                Event::Start(tag) | Event::Empty(tag) if tag.name().as_ref() == b"connection" => {
                    attributes.extend(reader.attributes()?);
                }

                Event::Text(tag) => {
                    attributes.insert("expression".into(), tag.as_ref().try_to_string()?);
                    reader.consume()?;
                }

                Event::End(tag) => match tag.name().as_ref() {
                    b"inVariable" | b"outVariable" => {
                        reader.consume()?;
                        break;
                    }
                    _ => reader.consume()?,
                },

                _ => reader.consume()?,
            }
        }

        FunctionBlockVariable::new(attributes, kind)
    }
}

impl Parseable for BlockVariable {
    type Item = Vec<Self>;

    fn visit(reader: &mut PeekableReader) -> Result<Self::Item, Error> {
        let kind = match reader.next()? {
            Event::Start(tag) | Event::Empty(tag) => VariableKind::try_from(tag.name().as_ref())?,
            _ => unreachable!(),
        };

        let mut res = vec![];

        loop {
            match reader.peek()? {
                Event::Start(tag) if tag.name().as_ref() == b"variable" => {
                    let attributes = visit_variable(reader)?;
                    res.push(BlockVariable::new(attributes, kind)?);
                }

                Event::End(tag)
                    if matches!(
                        tag.name().as_ref(),
                        b"inputVariables" | b"outputVariables" | b"inOutVariables"
                    ) =>
                {
                    reader.consume()?;
                    return Ok(res);
                }

                Event::Eof => {
                    return Err(Error::UnexpectedEndOfFile(vec![
                        b"inputVariables",
                        b"outputVariables",
                        b"inOutVariables",
                    ]))
                }
                _ => reader.consume()?,
            };
        }
    }
}

fn visit_variable(reader: &mut PeekableReader) -> Result<HashMap<String, String>, Error> {
    let mut attributes = HashMap::new();
    loop {
        match reader.peek()? {
            Event::Start(tag) | Event::Empty(tag) => match tag.name().as_ref() {
                b"variable" | b"connection" => attributes.extend(reader.attributes()?),
                _ => reader.consume()?,
            },

            Event::End(tag) if tag.name().as_ref() == b"variable" => {
                reader.consume()?;
                break;
            }
            _ => reader.consume()?,
        }
    }

    Ok(attributes)
}

#[cfg(test)]
mod tests {
    use insta::assert_debug_snapshot;

    use crate::{
        model::variables::{BlockVariable, FunctionBlockVariable},
        reader::PeekableReader,
        serializer::{
            XExpression, XInOutVariables, XInVariable, XInputVariables, XOutVariable, XOutputVariables,
            XVariable,
        },
        xml_parser::Parseable,
    };

    #[test]
    fn block_input_variable() {
        let content = XInputVariables::new().with_variable(XVariable::init("", false)).serialize();

        let mut reader = PeekableReader::new(&content);
        assert_debug_snapshot!(BlockVariable::visit(&mut reader));
    }

    #[test]
    fn block_output_variable() {
        let content = XOutputVariables::new().with_variable(XVariable::init("", false)).serialize();

        let mut reader = PeekableReader::new(&content);
        assert_debug_snapshot!(BlockVariable::visit(&mut reader));
    }

    #[test]
    fn block_inout_variable() {
        let content = XInOutVariables::new().with_variable(XVariable::init("", false)).serialize();

        let mut reader = PeekableReader::new(&content);
        assert_debug_snapshot!(BlockVariable::visit(&mut reader));
    }

    #[test]
    fn fbd_in_variable() {
        let content =
            XInVariable::init("0", false).with_expression(XExpression::new().with_data("a")).serialize();

        let mut reader = PeekableReader::new(&content);
        assert_debug_snapshot!(FunctionBlockVariable::visit(&mut reader));
    }

    #[test]
    fn fbd_out_variable() {
        let content =
            XOutVariable::init("0", false).with_expression(XExpression::new().with_data("a")).serialize();

        let mut reader = PeekableReader::new(&content);
        assert_debug_snapshot!(FunctionBlockVariable::visit(&mut reader));
    }
}
