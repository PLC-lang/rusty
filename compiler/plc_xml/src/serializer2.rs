use std::collections::HashMap;

#[derive(Clone)]
pub struct Node {
    name: &'static str,
    attributes: HashMap<&'static str, &'static str>,
    children: Vec<Node>,

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
        Self { name, attributes: HashMap::new(), children: Vec::new(), closed: false, content: None }
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

    fn _content(indent: &str, name: &'static str, content: &'static str) -> String {
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
            return Node::_content(&indent, name, content);
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

        // TODO: Perhaps deref
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

newtype_impl!(YInVariable, "inVariable", true);
newtype_impl!(YOutVariable, "outVariable", true);
newtype_impl!(YInOutVariable, "inOutVariable", true);
newtype_impl!(YInterface, "interface", false);
newtype_impl!(YLocalVars, "localVars", false);
newtype_impl!(YAddData, "addData", false);
newtype_impl!(YData, "data", false);
newtype_impl!(YTextDeclaration, "textDeclaration", false);
newtype_impl!(YContent, "content", false);
newtype_impl!(YPosition, "position", false);
newtype_impl!(YConnectionPointIn, "connectionPointIn", false);
newtype_impl!(YConnectionPointOut, "connectionPointOut", false);
newtype_impl!(YRelPosition, "relPosition", false);
newtype_impl!(YConnection, "connection", false);
newtype_impl!(YBlock, "block", false);
newtype_impl!(YBody, "body", false);
newtype_impl!(YPou, "pou", false);
newtype_impl!(YInputVariables, "inputVariables", false);
newtype_impl!(YOutputVariables, "outputVariables", false);
newtype_impl!(YInOutVariables, "inOutVariables", false);
newtype_impl!(YVariable, "variable", true);
newtype_impl!(YFbd, "FBD", false);
newtype_impl!(YExpression, "expression", false);
newtype_impl!(YReturn, "return", false);
newtype_impl!(YNegate, "negated", false);
newtype_impl!(YConnector, "connector", false);
newtype_impl!(YContinuation, "continuation", false);

impl YInVariable {
    /// Adds a child node
    /// <connectPointIn>
    ///     <connection refLocalId="..."/>
    /// </connectionPointIn/>
    pub fn connect(mut self, ref_local_id: i32) -> Self {
        self = self.child(&YConnectionPointIn::new().child(&YConnection::new().with_ref_id(ref_local_id)));
        self
    }

    pub fn with_expression(self, expression: &'static str) -> Self {
        self.child(&YExpression::with_expression(expression))
    }
}

impl YOutVariable {
    /// Adds a child node
    /// <connectPointIn>
    ///     <connection refLocalId="..."/>
    /// </connectionPointIn/>
    pub fn connect(mut self, ref_local_id: i32) -> Self {
        self = self
            .child(&YConnectionPointIn::new().child(&YConnection::new().with_ref_id(ref_local_id).close()));
        self
    }

    pub fn connect_temp(mut self, ref_local_id: i32, name: &'static str) -> Self {
        self =
            self.child(&YConnectionPointIn::new().child(
                &YConnection::new().with_ref_id(ref_local_id).attribute("formalParameter", name).close(),
            ));
        self
    }

    pub fn with_expression(self, expression: &'static str) -> Self {
        self.child(&YExpression::with_expression(expression))
    }
}

impl YInOutVariable {
    pub fn with_expression(self, expression: &'static str) -> Self {
        self.child(&YExpression::with_expression(expression))
    }
}

impl YReturn {
    pub fn init(local_id: i32, execution_order: i32) -> Self {
        Self::new().with_id(local_id).with_execution_id(execution_order)
    }

    pub fn connect(self, ref_local_id: i32) -> Self {
        self.child(&YConnectionPointIn::new().child(&YConnection::new().with_ref_id(ref_local_id)))
    }

    pub fn negate(self, value: bool) -> Self {
        self.child(&YAddData::new().child(&YData::new().child(
            &YNegate::new().attribute("value", Box::leak(value.to_string().into_boxed_str())).close(),
        )))
    }
}

impl YContent {
    pub fn with_declaration(mut self, content: &'static str) -> Self {
        self.0.content = Some(content);
        self
    }
}

impl YPou {
    // TODO: kind -> enum
    pub fn init(name: &'static str, kind: &'static str, declaration: &'static str) -> Self {
        Self::new()
            .attribute("xmlns", "http://www.plcopen.org/xml/tc6_0201")
            .attribute("name", name)
            .attribute("pouType", kind)
            .child(&YInterface::new().children(vec![
                    &YLocalVars::new().close(),
                    &YAddData::new().child(
                        &YData::new()
                            .attribute("name", "www.bachmann.at/plc/plcopenxml")
                            .attribute("handleUnknown", "implementation")
                            .child(
                                &YTextDeclaration::new()
                                    .child(&YContent::new().with_declaration(declaration)),
                            ),
                    ),
                ]))
    }

    /// Implicitly wraps the fbd in a block node, i.e. <block>/* fbd */<block/>
    pub fn with_fbd(self, children: Vec<&dyn IntoNode>) -> Self {
        self.child(&YBody::new().child(&YFbd::new().children(children)))
    }

    // pub fn with_name(name: &'static str) -> Self {}
}

impl YBlock {
    pub fn init(name: &'static str, local_id: i32, execution_order_id: i32) -> Self {
        Self::new().with_name(name).with_id(local_id).with_execution_id(execution_order_id)
    }

    pub fn with_name(self, name: &'static str) -> Self {
        self.attribute("typeName", name)
    }

    pub fn with_input_variables(self, variables: Vec<&dyn IntoNode>) -> Self {
        self.child(&YInputVariables::new().children(variables))
    }

    pub fn with_output_variables(self, variables: Vec<&dyn IntoNode>) -> Self {
        self.child(&YOutputVariables::new().children(variables))
    }

    pub fn with_inout_variables(self, variables: Vec<&dyn IntoNode>) -> Self {
        self.child(&YInOutVariables::new().children(variables))
    }
}

impl YBody {
    pub fn with_fbd(self, children: Vec<&dyn IntoNode>) -> Self {
        Self::new().child(&YFbd::new().children(children))
    }
}

impl YInputVariables {
    pub fn with_variables(variables: Vec<&dyn IntoNode>) -> Self {
        Self::new().children(variables)
    }
}

impl YOutputVariables {
    pub fn with_variables(variables: Vec<&dyn IntoNode>) -> Self {
        Self::new().children(variables)
    }
}

impl YVariable {
    // TODO: Remove
    pub fn name(name: &'static str) -> Self {
        Self::new().attribute("formalParameter", name)
    }

    pub fn with_name(self, name: &'static str) -> Self {
        self.attribute("formalParameter", name)
    }

    pub fn connect(self, ref_local_id: i32) -> Self {
        self.child(&YConnection::new().with_ref_id(ref_local_id).close())
    }

    pub fn connect_in(self, ref_local_id: i32) -> Self {
        self.child(&YConnectionPointIn::new().children(vec![
            &YRelPosition::new().close(), // TODO: Positions
            &YConnection::new().with_ref_id(ref_local_id).close(),
        ]))
    }

    pub fn connect_out(self, ref_local_id: i32) -> Self {
        self.child(&YConnectionPointOut::new().children(vec![
            &YRelPosition::new().close(), // TODO: Positions
            &YConnection::new().with_ref_id(ref_local_id).close(),
        ]))
    }
}

impl YInVariable {}

impl YExpression {
    pub fn with_expression(expression: &'static str) -> Self {
        let mut node = Self::new();
        node.0.content = Some(expression);
        node
    }
}

impl YOutVariable {
    pub fn connect_in(self, ref_local_id: i32) -> Self {
        self.child(&YConnectionPointIn::new().children(vec![
            &YRelPosition::new().close(), // TODO: Positions
            &YConnection::new().with_ref_id(ref_local_id).close(),
        ]))
    }
}

impl YConnector {
    pub fn with_name(self, name: &'static str) -> Self {
        self.attribute("name", name)
    }

    // TODO: Naming?
    pub fn connect_in(self, ref_local_id: i32) -> Self {
        self.child(&YConnectionPointIn::new().children(vec![
            &YRelPosition::new().close(), // TODO: Positions
            &YConnection::new().with_ref_id(ref_local_id).close(),
        ]))
    }

    // TODO: Naming?
    pub fn connect_temp(mut self, ref_local_id: i32, name: &'static str) -> Self {
        self = self.child(&YConnectionPointIn::new().children(vec![
            &YRelPosition::new().close(), // TODO: Positions
            &YConnection::new().with_ref_id(ref_local_id).attribute("formalParameter", name).close(),
        ]));

        self
    }
}

impl YContinuation {
    pub fn with_name(self, name: &'static str) -> Self {
        self.attribute("name", name)
    }

    pub fn connect_out(self, ref_local_id: i32) -> Self {
        self.child(&YConnectionPointOut::new().children(vec![
            &YRelPosition::new().close(), // TODO: Positions
            &YConnection::new().with_ref_id(ref_local_id).close(),
        ]))
    }
}
