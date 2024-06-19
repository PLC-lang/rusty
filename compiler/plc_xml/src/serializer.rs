#![allow(clippy::new_without_default)]

use rustc_hash::FxHashMap;

#[derive(Clone)]
pub struct Node {
    name: &'static str,
    children: Vec<Node>,

    /// XML attributes, e.g. `<position x="1">` where `x` is the attribute
    ///
    /// Design Note: We use a HashMap here to avoid duplicates but also update existing values in case of
    /// repeated function calls, e.g. `with_attribute("x", 1)` and `with_attribute("x", 2)` where the value of
    /// x has been updated from 1 to 2.
    attributes: FxHashMap<&'static str, &'static str>,

    /// Indicates if an element has a closed form, e.g. `<position x="1" y="2"/>`
    closed: bool,

    /// Indicates if an element has some text wrapped inside itself, e.g. `<expression>a + b</expression>`
    content: Option<&'static str>,
}

pub trait IntoNode {
    fn inner(&self) -> Node;
}

impl Node {
    fn new(name: &'static str) -> Self {
        Self { name, attributes: FxHashMap::default(), children: Vec::new(), closed: false, content: None }
    }

    fn attribute(mut self, key: &'static str, value: &'static str) -> Self {
        self.attributes.insert(key, value);
        self
    }

    fn child(mut self, node: &dyn IntoNode) -> Self {
        self.children.push(node.inner());
        self
    }

    fn children(mut self, nodes: Vec<&dyn IntoNode>) -> Self {
        self.children.extend(nodes.into_iter().map(IntoNode::inner));
        self
    }

    fn close(mut self) -> Self {
        self.closed = true;
        self
    }

    fn indent(level: usize) -> String {
        " ".repeat(level * 4)
    }

    fn serialize_content(indent: &str, name: &'static str, content: &'static str) -> String {
        format!("{indent}<{name}>{content}</{name}>\n")
    }

    #[allow(unused_assignments)]
    pub fn serialize(&self, level: usize) -> String {
        let (name, indent) = (self.name, Node::indent(level));
        let attributes = self.attributes.iter().map(|(key, value)| format!("{key}=\"{value}\""));
        let attributes_str = attributes.collect::<Vec<_>>().join(" ");
        let mut result = String::new();

        if self.closed {
            return format!("{indent}<{name} {attributes_str}/>\n");
        }

        if let Some(content) = self.content {
            return Node::serialize_content(&indent, name, content);
        }

        result = format!("{indent}<{name} {attributes_str}>\n");
        self.children.iter().for_each(|child| result = format!("{result}{}", child.serialize(level + 1)));
        result = format!("{result}{indent}</{name}>\n");

        result
    }
}

macro_rules! newtype_impl {
    ($name_struct:ident, $name_node:expr, $negatable:expr) => {
        pub struct $name_struct(Node);

        impl IntoNode for $name_struct {
            fn inner(&self) -> Node {
                self.0.clone()
            }
        }

        impl $name_struct {
            pub fn new() -> Self {
                match $negatable {
                    true => Self(Node::new($name_node).attribute("negated", "false")),
                    false => Self(Node::new($name_node)),
                }
            }

            pub fn id(local_id: i32) -> Self {
                let new = $name_struct::new();
                new.with_id(local_id)
            }

            fn attribute(self, key: &'static str, value: &'static str) -> Self {
                Self(self.inner().attribute(key, value))
            }

            fn maybe_attribute(self, key: &'static str, value: Option<&'static str>) -> Self {
                match value {
                    Some(value) => Self(self.inner().attribute(key, value)),
                    None => self,
                }
            }

            fn child(self, node: &dyn IntoNode) -> Self {
                Self(self.inner().child(node))
            }

            pub fn children(self, nodes: Vec<&dyn IntoNode>) -> Self {
                Self(self.inner().children(nodes))
            }

            pub fn serialize(self) -> String {
                self.inner().serialize(0)
            }

            pub fn with_id<T: std::fmt::Display>(self, id: T) -> Self {
                self.attribute("localId", Box::leak(id.to_string().into_boxed_str()))
            }

            pub fn with_ref_id<T: std::fmt::Display>(self, id: T) -> Self {
                self.attribute("refLocalId", Box::leak(id.to_string().into_boxed_str()))
            }

            pub fn with_execution_id<T: std::fmt::Display>(self, id: T) -> Self {
                self.attribute("executionOrderId", Box::leak(id.to_string().into_boxed_str()))
            }

            fn close(self) -> Self {
                Self(self.inner().close())
            }
        }
    };
}

// newtype_impl!(<struct name>, <xml name>, <is negatable>)
newtype_impl!(SInVariable, "inVariable", true);
newtype_impl!(SOutVariable, "outVariable", true);
newtype_impl!(SInOutVariable, "inOutVariable", true);
newtype_impl!(SInterface, "interface", false);
newtype_impl!(SLocalVars, "localVars", false);
newtype_impl!(SAddData, "addData", false);
newtype_impl!(SData, "data", false);
newtype_impl!(STextDeclaration, "textDeclaration", false);
newtype_impl!(SContent, "content", false);
newtype_impl!(SPosition, "position", false);
newtype_impl!(SConnectionPointIn, "connectionPointIn", false);
newtype_impl!(SConnectionPointOut, "connectionPointOut", false);
newtype_impl!(SRelPosition, "relPosition", false);
newtype_impl!(SConnection, "connection", false);
newtype_impl!(SBlock, "block", false);
newtype_impl!(SBody, "body", false);
newtype_impl!(SPou, "pou", false);
newtype_impl!(SInputVariables, "inputVariables", false);
newtype_impl!(SOutputVariables, "outputVariables", false);
newtype_impl!(SInOutVariables, "inOutVariables", false);
newtype_impl!(SVariable, "variable", true);
newtype_impl!(YFbd, "FBD", false);
newtype_impl!(SExpression, "expression", false);
newtype_impl!(SReturn, "return", false);
newtype_impl!(SNegate, "negated", false);
newtype_impl!(SConnector, "connector", false);
newtype_impl!(SContinuation, "continuation", false);
newtype_impl!(SJump, "jump", false);
newtype_impl!(SLabel, "label", false);
newtype_impl!(SAction, "action", false);
newtype_impl!(SActions, "actions", false);

impl SInVariable {
    pub fn connect(mut self, ref_local_id: i32) -> Self {
        self = self.child(&SConnectionPointIn::new().child(&SConnection::new().with_ref_id(ref_local_id)));
        self
    }

    pub fn with_expression(self, expression: &'static str) -> Self {
        self.child(&SExpression::expression(expression))
    }
}

impl SOutVariable {
    pub fn connect(mut self, ref_local_id: i32) -> Self {
        self = self
            .child(&SConnectionPointIn::new().child(&SConnection::new().with_ref_id(ref_local_id).close()));
        self
    }

    pub fn connect_name(mut self, ref_local_id: i32, name: &'static str) -> Self {
        self =
            self.child(&SConnectionPointIn::new().child(
                &SConnection::new().with_ref_id(ref_local_id).attribute("formalParameter", name).close(),
            ));
        self
    }

    pub fn with_expression(self, expression: &'static str) -> Self {
        self.child(&SExpression::expression(expression))
    }
}

impl SInOutVariable {
    pub fn with_expression(self, expression: &'static str) -> Self {
        self.child(&SExpression::expression(expression))
    }
}

impl SReturn {
    pub fn init(local_id: i32, execution_order: i32) -> Self {
        Self::new().with_id(local_id).with_execution_id(execution_order)
    }

    pub fn connect(self, ref_local_id: i32) -> Self {
        self.child(&SConnectionPointIn::new().child(&SConnection::new().with_ref_id(ref_local_id)))
    }

    pub fn negate(self, value: bool) -> Self {
        self.child(&SAddData::new().child(&SData::new().child(
            &SNegate::new().attribute("value", Box::leak(value.to_string().into_boxed_str())).close(),
        )))
    }
}

impl SContent {
    pub fn with_declaration(mut self, content: &'static str) -> Self {
        self.0.content = Some(content);
        self
    }
}

impl SPou {
    pub fn init(name: &'static str, kind: &'static str, declaration: &'static str) -> Self {
        Self::new()
            .attribute("xmlns", "http://www.plcopen.org/xml/tc6_0201")
            .attribute("name", name)
            .attribute("pouType", kind)
            .child(&SInterface::new().children(vec![
                    &SLocalVars::new().close(),
                    &SAddData::new().child(
                        &SData::new()
                            .attribute("name", "www.bachmann.at/plc/plcopenxml")
                            .attribute("handleUnknown", "implementation")
                            .child(
                                &STextDeclaration::new()
                                    .child(&SContent::new().with_declaration(declaration)),
                            ),
                    ),
                ]))
    }

    /// Implicitly wraps the fbd in a block node, i.e. <block><fbd>...<fbd/><block/>
    pub fn with_fbd(self, children: Vec<&dyn IntoNode>) -> Self {
        self.child(&SBody::new().child(&YFbd::new().children(children)))
    }

    pub fn with_actions(self, children: Vec<&dyn IntoNode>) -> Self {
        self.child(&SActions::new().children(children))
    }
}

impl SBlock {
    pub fn init(name: &'static str, local_id: i32, execution_order_id: i32) -> Self {
        Self::new().with_name(name).with_id(local_id).with_execution_id(execution_order_id)
    }

    pub fn with_name(self, name: &'static str) -> Self {
        self.attribute("typeName", name)
    }

    pub fn with_input(self, variables: Vec<&dyn IntoNode>) -> Self {
        self.child(&SInputVariables::new().children(variables))
    }

    pub fn with_output(self, variables: Vec<&dyn IntoNode>) -> Self {
        self.child(&SOutputVariables::new().children(variables))
    }

    pub fn with_inout(self, variables: Vec<&dyn IntoNode>) -> Self {
        self.child(&SInOutVariables::new().children(variables))
    }
}

impl SBody {
    pub fn with_fbd(self, children: Vec<&dyn IntoNode>) -> Self {
        Self::new().child(&YFbd::new().children(children))
    }
}

impl SInputVariables {
    pub fn with_variables(variables: Vec<&dyn IntoNode>) -> Self {
        Self::new().children(variables)
    }
}

impl SOutputVariables {
    pub fn with_variables(variables: Vec<&dyn IntoNode>) -> Self {
        Self::new().children(variables)
    }
}

impl SVariable {
    pub fn with_name(self, name: &'static str) -> Self {
        self.attribute("formalParameter", name)
    }

    pub fn connect(self, ref_local_id: i32) -> Self {
        self.child(&SConnectionPointIn::new().child(&SConnection::new().with_ref_id(ref_local_id).close()))
    }

    pub fn connect_out(self, ref_local_id: i32) -> Self {
        self.child(&SConnectionPointOut::new().child(&SConnection::new().with_ref_id(ref_local_id).close()))
    }
}

impl SExpression {
    pub fn expression(expression: &'static str) -> Self {
        let mut node = Self::new();
        node.0.content = Some(expression);
        node
    }
}

impl SConnector {
    pub fn with_name(self, name: &'static str) -> Self {
        self.attribute("name", name)
    }

    pub fn connect(self, ref_local_id: i32) -> Self {
        self.child(&SConnectionPointIn::new().child(&SConnection::new().with_ref_id(ref_local_id).close()))
    }
}

impl SContinuation {
    pub fn with_name(self, name: &'static str) -> Self {
        self.attribute("name", name)
    }

    pub fn connect_out(self, ref_local_id: i32) -> Self {
        self.child(&SConnectionPointOut::new().child(&SConnection::new().with_ref_id(ref_local_id).close()))
    }
}

impl SLabel {
    pub fn with_name(self, name: &'static str) -> Self {
        self.attribute("label", name)
    }
}

impl SJump {
    pub fn with_name(self, name: &'static str) -> Self {
        self.attribute("label", name)
    }

    pub fn connect(self, ref_local_id: i32) -> Self {
        self.child(&SConnectionPointIn::new().child(&SConnection::new().with_ref_id(ref_local_id).close()))
    }

    pub fn negate(self) -> Self {
        self.child(
            &SAddData::new().child(&SData::new().child(&SNegate::new().attribute("value", "true").close())),
        )
    }
}

impl SAction {
    pub fn name(name: &'static str) -> Self {
        Self::new().attribute("name", name)
    }

    pub fn with_fbd(self, children: Vec<&dyn IntoNode>) -> Self {
        self.child(&SBody::new().child(&YFbd::new().children(children)))
    }
}
