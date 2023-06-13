use crate::deserializer::Parseable;

type Attributes = Vec<(&'static str, &'static str)>;

/// Number of spaces to use when indenting XML
const INDENT_SPACES: usize = 4;

#[derive(Debug)]
enum Content {
    Node(Vec<Node>),
    Data(&'static str),
    Empty,
}

impl Default for Content {
    fn default() -> Self {
        Content::Empty
    }
}

impl<'b> Content {
    fn push(&mut self, node: Node) {
        let mut nodes = match self.take() {
            Content::Node(nodes) => nodes,
            Content::Empty => vec![],
            _ => unreachable!("cannot push onto data field"),
        };

        nodes.push(node);

        self.replace(Content::Node(nodes));
    }

    fn replace(&'b mut self, other: Content) -> Content {
        std::mem::replace(self, other)
    }

    fn take(&'b mut self) -> Content {
        std::mem::take(self)
    }

    fn iter(self) -> impl Iterator<Item = Node> {
        match self {
            Content::Node(nodes) => nodes,
            _ => vec![],
        }
        .into_iter()
    }
}

// impl<'b> IntoIterator for &'b Content {
//     type Item = &'b Node;
//     type IntoIter = std::slice::Iter<'b, Node>;

//     fn into_iter(self) -> Self::IntoIter {
//         match self {
//             Content::Node(nodes) => nodes.as_slice(),
//             _ => &[],
//         }.into_iter()
//     }
// }

// impl IntoIterator for Content {
//     type Item = Node;
//     type IntoIter = std::vec::IntoIter<Node>;

//     fn into_iter(mut self) -> Self::IntoIter {
//         match self.take() {
//             Content::Node(nodes) => nodes,
//             _ => vec![]
//         }.into_iter()
//     }

// }

#[derive(Debug)]
struct Node {
    name: &'static str,
    attributes: Attributes,
    closed: bool,
    // TODO: Data can only exist if nodes is empty and vice-versa, update such that this behavior is reflected
    // (e.g. with an enum)
    content: Content,
}

impl Node {
    fn attributes(&self) -> String {
        let mut fmt = String::new();
        for attr in &self.attributes {
            fmt = format!(r#"{fmt}{key}="{value}" "#, key = attr.0, value = attr.1)
        }

        fmt
    }

    // TODO: ✨ Beautify ✨ this
    fn serialize(self, level: usize) -> String {
        let (indent, name, attributes) = (" ".repeat(level * INDENT_SPACES), self.name, self.attributes());
        let mut fmt = String::new();
        match self.content {
            Content::Data(data) => fmt = format!("{indent}<{name} {attributes}>{data}</{name}>\n",),
            _ => {
                if self.closed {
                    fmt = format!(
                        "{indent}<{name} {attributes}/>\n",
                        indent = " ".repeat(level * INDENT_SPACES),
                        name = self.name,
                        attributes = self.attributes()
                    );
                }

                if !self.closed {
                    fmt = format!(
                        "{indent}<{name} {attributes}>\n",
                        indent = " ".repeat(level * INDENT_SPACES),
                        name = self.name,
                        attributes = self.attributes()
                    );
                }

                for node in self.content.iter() {
                    fmt = format!("{fmt}{}", node.serialize(level + 1));
                }

                if !self.closed {
                    fmt = format!(
                        "{fmt}{indent}</{name}>\n",
                        indent = " ".repeat(level * INDENT_SPACES),
                        name = &self.name
                    );
                }
            }
        };

        #[cfg(feature = "debug")]
        println!("{fmt}");

        fmt
    }
}

// For `declare_type_and_extend_if_needed! { (Pou, "pou", (Body, with_body)) }` the macro will expand to
//
// pub(crate) struct Pou(Node);
//
// impl Pou {
//     pub fn new() -> Self {
//         Self(Node { name: "pou", attributes: vec![], closed: false, nodes: vec![] })
//     }
//
//     ... the remaining non-optional functions in the impl block ...
//
//     ... the optional extend method
//     pub(crate) fn with_body(arg: Body) -> Self {
//         self.get_inner_ref_mut().nodes.push(arg.get_inner());
//         self
//     }
// }
// TODO: revisit visiblity of functions
macro_rules! declare_type_and_extend_if_needed {
    ($(($name:ident, $name_xml:expr, $(($arg:ty, $fn_name:ident)),*),) +) => {
        $(
            // will be implemented for every $name
            #[derive(Debug)]
            pub(crate) struct $name(Node);
            impl $name {
                pub fn new() -> Self {
                    Self(Node { name: $name_xml, attributes: vec![], closed: false, content: Content::Empty })
                }

                pub fn with_attribute(mut self, key: &'static str, value: &'static str) -> Self {
                    self.0.attributes.push((key, value));
                    self
                }

                pub fn with_data(mut self, data: &'static str) -> Self {
                    self.0.content = Content::Data(data);
                    self
                }

                pub fn close(mut self) -> Self {
                    self.0.closed = true;
                    self
                }

                pub fn serialize(self) -> String {
                    self.0.serialize(0)
                }

                fn get_inner(self) -> Node {
                    self.0
                }

                fn get_inner_ref_mut(&mut self) -> &mut Node {
                    &mut self.0
                }

                // this part is optional.
                $(
                    pub(crate) fn $fn_name(mut self, value: $arg) -> Self {
                    self.get_inner_ref_mut().content.push(value.get_inner());
                    self
                })*
        })*
    }
}

declare_type_and_extend_if_needed! {
    (
        XPou, "pou",
        (XBody, with_body)
    ),
    (
        XBlock, "block",
        (XInOutVariables, with_inout_variables),
        (XInputVariables, with_input_variables),
        (XOutputVariables, with_output_variables)
    ),
    (

        XBody, "body",
        (XFbd, with_fbd)
    ),
    (
        XConnectionPointIn, "connectionPointIn",
        (XConnection, with_connection),
        (XRelPosition, with_rel_position)
    ),
    (
        XConnectionPointOut, "connectionPointOut",
        (XConnection, with_connection),
        (XRelPosition, with_rel_position)
    ),
    (
        XFbd, "FBD",
        (XBlock, with_block),
        (XInVariable, with_in_variable),
        (XContinuation, with_continuation),
        (XConnector, with_connector)
    ),
    (
        XVariable, "variable",
        (XConnectionPointIn, with_connection_in),
        (XConnectionPointOut, with_connection_point_out)
    ),
    (
        XInVariable, "inVariable",
        (XConnectionPointOut, with_connection_point_out),
        (XPosition, with_position)
    ),
    (
        XOutVariable, "outVariable",
        (XPosition, with_position),
        (XConnectionPointIn, with_connection_point_in),
        (XExpression, with_expression)
    ),
    (
        XInOutVariables, "inOutVariables",
        (XVariable, with_variable)
    ),
    (
        XInputVariables, "inputVariables",
        (XVariable, with_variable)
    ),
    (
        XOutputVariables, "outputVariables",
        (XVariable, with_variable)
    ),

    // these are not being extended:
    (XPosition, "position",),
    (XRelPosition, "relPosition",),
    (XConnection, "connection",),
    (XExpression, "expression",),

    (
        XContinuation, "continuation",
        (XPosition, with_position),
        (XConnectionPointOut, with_connection_point_out)
    ),
    (
        XConnector, "connector",
        (XPosition, with_position),
        (XConnectionPointIn, with_connection_point_in)
    ),
}

#[test]
fn demoo() {
    let xml = XContinuation::new()
        .with_attribute("name", "abc")
        .with_attribute("localId", "1")
        .with_position(XPosition::new().close())
        .with_connection_point_out(XConnectionPointOut::with_rel_pos().close())
        .serialize();

    println!("{xml}");
    let mut reader = crate::reader::PeekableReader::new(&xml);
    dbg!(crate::model::connector::Connector::visit(&mut reader).unwrap());

    let xml = XConnector::new()
        .with_attribute("name", "abc")
        .with_attribute("localId", "1")
        .with_position(XPosition::new().close())
        .with_connection_point_in(
            XConnectionPointIn::new().with_rel_position(XRelPosition::init()).with_connection(
                XConnection::new()
                    .with_attribute("refLocalId", "11")
                    .with_attribute("formalParameter", "function_0")
                    .close(),
            ),
        )
        .serialize();

    let mut reader = crate::reader::PeekableReader::new(&xml);
    dbg!(crate::model::connector::Connector::visit(&mut reader).unwrap());
}

// convenience methods to reduce amount of boiler-plate-code
impl XVariable {
    pub(crate) fn init(name: &'static str, negated: bool) -> Self {
        XVariable::new()
            .with_attribute("formalParameter", name)
            .with_attribute("negated", if negated { "true" } else { "false" })
    }
}

impl XRelPosition {
    pub(crate) fn init() -> Self {
        XRelPosition::new().with_attribute("x", "0").with_attribute("y", "0")
    }
}

impl XConnectionPointIn {
    pub(crate) fn with_ref(ref_local_id: &'static str) -> Self {
        XConnectionPointIn::new()
            .with_rel_position(XRelPosition::init().close())
            .with_connection(XConnection::new().with_attribute("refLocalId", ref_local_id).close())
    }
}

impl XConnectionPointOut {
    pub(crate) fn with_rel_pos() -> Self {
        XConnectionPointOut::new().with_rel_position(XRelPosition::init().close())
    }
}

impl XInVariable {
    pub(crate) fn init(local_id: &'static str, negated: bool) -> Self {
        XInVariable::new()
            .with_attribute("localId", local_id)
            .with_attribute("negated", if negated { "true" } else { "false" })
    }
}

impl XBlock {
    pub(crate) fn init(local_id: &'static str, type_name: &'static str) -> Self {
        XBlock::new().with_attribute("localId", local_id).with_attribute("typeName", type_name)
    }
}
