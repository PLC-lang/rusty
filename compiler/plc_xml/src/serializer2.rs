#[derive(Clone)]
struct Node {
    name: &'static str,
    attributes: Vec<Attribute>, // TODO: HashMap this
    children: Vec<Node>,

    // Single line, e.g. <.../>
    closed: bool,

    /// <content>...</content> or <expression>...</expression>
    content: Option<&'static str>,
}

#[derive(Clone)]
struct Attribute {
    key: &'static str,
    value: &'static str,
}

impl std::fmt::Display for Attribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, r#"{key}="{value}""#, key = self.key, value = self.value)
    }
}

trait IntoNode {
    fn inner(&self) -> Node;
}

impl Node {
    fn new(name: &'static str) -> Self {
        Self { name, attributes: Vec::new(), children: Vec::new(), closed: false, content: None }
    }

    fn attribute(mut self, key: &'static str, value: &'static str) -> Self {
        self.attributes.push(Attribute { key, value });
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
        format!("{indent}<{name}>{content}<{name}/>\n")
    }

    #[allow(unused_assignments)]
    fn serialize(&self, level: usize) -> String {
        let (name, indent) = (self.name, Node::indent(level));
        let attributes = self.attributes.iter().map(Attribute::to_string).collect::<Vec<_>>().join(" ");
        let mut result = String::new();

        if self.closed {
            return format!("{indent}<{name} {attributes}/>\n");
        }

        if let Some(content) = self.content {
            return Node::_content(&indent, name, content);
        }

        result = format!("{indent}<{name} {attributes}>\n");
        self.children.iter().for_each(|child| result = format!("{result}{}", child.serialize(level + 1)));
        result = format!("{result}{indent}</{name}>\n");

        result
    }
}

macro_rules! newtype_impl {
    ($name_struct:ident, $name_node:expr) => {
        struct $name_struct(Node);

        // TODO: Perhaps deref
        impl IntoNode for $name_struct {
            fn inner(&self) -> Node {
                self.0.clone()
            }
        }

        impl $name_struct {
            fn new() -> Self {
                Self(Node::new($name_node))
            }

            // TODO: Restrict this to only nodes that actually can have an id
            fn with_id(id: i32) -> Self {
                Self(Node::new($name_node)).local_id(id)
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

            fn children(self, nodes: Vec<&dyn IntoNode>) -> Self {
                Self(self.inner().children(nodes))
            }

            fn serialize(self) -> String {
                self.inner().serialize(0)
            }

            fn local_id<T: std::fmt::Display>(self, id: T) -> Self {
                self.attribute("localId", Box::leak(id.to_string().into_boxed_str()))
            }

            fn ref_local_id<T: std::fmt::Display>(self, id: T) -> Self {
                self.attribute("refLocalId", Box::leak(id.to_string().into_boxed_str()))
            }

            fn execution_id<T: std::fmt::Display>(self, id: T) -> Self {
                self.attribute("executionOrderId", Box::leak(id.to_string().into_boxed_str()))
            }

            fn close(self) -> Self {
                Self(self.inner().close())
            }
        }
    };
}

impl YInVariable {
    /// Adds a child node
    /// <connectPointIn>
    ///     <connection refLocalId="..."/>
    /// </connectionPointIn/>
    pub fn connect(mut self, ref_local_id: i32) -> Self {
        self = self.child(&YConnectionPointIn::new().child(&YConnection::new().ref_local_id(ref_local_id)));
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
        self = self.child(&YConnectionPointIn::new().child(&YConnection::new().ref_local_id(ref_local_id)));
        self
    }

    pub fn with_execution_id(self, id: i32) -> Self {
        self.execution_id(id)
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

newtype_impl!(YInVariable, "inVariable");
newtype_impl!(YOutVariable, "inVariable");
newtype_impl!(YInOutVariable, "inVariable");
newtype_impl!(YInterface, "interface");
newtype_impl!(YLocalVars, "localVars");
newtype_impl!(YAddData, "addData");
newtype_impl!(YData, "data");
newtype_impl!(YTextDeclaration, "textDeclaration");
newtype_impl!(YContent, "content");
newtype_impl!(YPosition, "position");
newtype_impl!(YConnectionPointIn, "connectionPointIn");
newtype_impl!(YRelPosition, "relPosition");
newtype_impl!(YConnection, "connection");
newtype_impl!(YBlock, "block");
newtype_impl!(YPou, "pou");
newtype_impl!(YInputVariables, "inputVariables");
newtype_impl!(YOutputVariables, "outputVariables");
newtype_impl!(YVariable, "variable");
newtype_impl!(YFbd, "FBD");
newtype_impl!(YExpression, "expression");

impl YContent {
    pub fn with_content(self, content: &'static str) -> Self {
        self.inner().content = Some(content);
        self
    }
}

trait FbdElements: IntoNode {}
impl FbdElements for YInVariable {}
impl FbdElements for YOutVariable {}
impl FbdElements for YInOutVariable {}

impl YPou {
    // TODO: kind -> enum
    #[rustfmt::skip]
    pub fn init(name: &'static str, kind: &'static str, content: &'static str) -> Self {
        Self::new().attribute("name", name).attribute("pouType", kind).child(
            &YInterface::new().children(vec![
                &YLocalVars::new().close(),
                &YAddData::new().child(
                    &YData::new().attribute("name", "...").child(
                        &YTextDeclaration::new().child(
                            &YContent::new().with_content(content)
                        )
                    ),
                ),
            ]),
        )
    }

    // /// Implicitly wraps the fbd in a block node, i.e. <block>/* fbd */<block/>
    // pub fn with_fbd(self, children: Vec<&dyn FbdElements>) -> Self {
    //     self.child(
    //         &YBlock::new()
    //             .child(&YFbd::new().children(children.into_iter().map(|it| it as &dyn IntoNode).collect())),
    //     )
    // }
}

impl YBlock {
    pub fn with_name(self, name: &'static str) -> Self {
        self.attribute("typeName", name)
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
    pub fn with_name(name: &'static str) -> Self {
        Self::new().attribute("formalParameter", name)
    }

    pub fn connect(self, ref_local_id: i32) -> Self {
        self.child(&YConnection::new().ref_local_id(ref_local_id).close())
    }
}

impl YExpression {
    pub fn with_expression(expression: &'static str) -> Self {
        let mut node = Self::new();
        node.0.content = Some(expression);
        node
    }
}

#[test]
fn pou() {
    #[rustfmt::skip]
    let serialized = YPou::init("a", "b", "VERY LONG STRING AAAAAAAAAAAAAAAAAAAAAAAAHHHHHHHHHHH").serialize();
    println!("{serialized}");
}

#[test]
fn block() {
    #[rustfmt::skip]
    let serialized = YPou::init("conditional_return", "functionBlock", "...").child(
        &YFbd::new().children(vec![
            &YInVariable::with_id(1).with_expression("val = 5"),
            &YInVariable::with_id(3).with_expression("10"),
            &YOutVariable::with_id(4).with_expression("val").with_execution_id(1).connect(3),
            &YInOutVariable::with_id(5).with_expression("a"),
        ])
    ).serialize();

    // let serialized = YPou::init("conditional_return", "functionBlock", "...")
    //     .with_fbd(vec![
    //         &YInVariable::with_id(1).with_expression("val = 5"),
    //         &YInVariable::with_id(3).with_expression("10"),
    //         &YOutVariable::with_id(4).with_expression("val").with_execution_id(1).connect(3),
    //         &YInOutVariable::with_id(5).with_expression("a"),
    //     ])
    //     .serialize();

    // println!("{serialized}");
}
