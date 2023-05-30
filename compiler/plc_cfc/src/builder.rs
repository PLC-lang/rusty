type Attributes = Vec<(&'static str, &'static str)>;

/// Number of spaces to use when indenting XML
const INDENT_SPACES: usize = 2;

struct Node {
    name: &'static str,
    attributes: Attributes,
    is_empty: bool, // TODO: Rename
    nodes: Vec<Node>,
}

impl Node {
    pub fn with(name: &'static str, attributes: Attributes, nodes: Node) -> Self {
        Self { name, attributes, is_empty: false, nodes: vec![nodes] }
    }

    pub fn with_empty(name: &'static str, attributes: Attributes) -> Self {
        Self { name, attributes, is_empty: false, nodes: vec![] }
    }

    pub fn with_nodes(name: &'static str, attributes: Attributes, nodes: Vec<Node>) -> Self {
        Self { name, attributes, is_empty: false, nodes }
    }

    fn attributes(&self) -> String {
        let mut fmt = String::new();
        for attr in &self.attributes {
            fmt = format!(r#"{fmt}{key}="{value}" "#, key = attr.0, value = attr.1)
        }

        fmt
    }

    pub fn _finalize(self, level: usize) -> String {
        let mut fmt = String::new();

        if self.is_empty {
            fmt = format!(
                "{indent}<{name} {attributes} />\n",
                indent = " ".repeat(level * INDENT_SPACES),
                name = self.name,
                attributes = self.attributes()
            );
        }

        if !self.is_empty {
            fmt = format!(
                "{indent}<{name} {attributes}>\n",
                indent = " ".repeat(level * INDENT_SPACES),
                name = self.name,
                attributes = self.attributes()
            );
        }

        for node in self.nodes {
            fmt = format!("{fmt}{}", node._finalize(level + 1));
        }

        if !self.is_empty {
            fmt = format!(
                "{fmt}{indent}</{name}>\n",
                indent = " ".repeat(level * INDENT_SPACES),
                name = &self.name
            );
        }

        fmt
    }

    pub fn finalize(self) -> String {
        self._finalize(0)
    }
}

trait Inner {
    fn get_inner(self) -> Node;
    fn get_inner_ref(&self) -> &Node;
    fn get_inner_ref_mut(&mut self) -> &mut Node;
}

macro_rules! types {
    ($name:ident, $name_xml:expr) => {
        struct $name(Node);
        impl $name {
            pub fn new() -> Self {
                Self(Node { name: $name_xml, attributes: vec![], is_empty: false, nodes: vec![] })
            }

            pub fn with_attribute(mut self, key: &'static str, value: &'static str) -> Self {
                self.0.attributes.push((key, value));
                self
            }

            pub fn as_empty(mut self) -> Self {
                self.0.is_empty = true;
                self
            }
        }

        impl Inner for $name {
            fn get_inner(self) -> Node {
                self.0
            }

            fn get_inner_ref(&self) -> &Node {
                &self.0
            }

            fn get_inner_ref_mut(&mut self) -> &mut Node {
                &mut self.0
            }
        }
    };
}

macro_rules! extend {
    ($ty:ident, $arg:ident, $fn_name:ident) => {
        impl $ty {
            fn $fn_name(mut self, value: $arg) -> Self {
                self.get_inner_ref_mut().nodes.push(value.get_inner());
                self
            }
        }
    };
}

types!(Pou, "pou");
types!(Body, "body");
types!(Fbd, "FBD");
types!(Block, "block");
types!(InputVariables, "inputVariables");
types!(OutputVariables, "outputVariables");
types!(InOutVariables, "inOutVariables");
types!(Variable, "variable");
types!(RelPosition, "relPosition");
types!(Connection, "connection");
types!(ConnectionPointIn, "connectionPointIn");
types!(ConnectionPointOut, "connectionPointOut");
types!(InVariable, "inVariable");
types!(Position, "position");

extend!(Block, InOutVariables, with_inout_variables);
extend!(Block, InputVariables, with_input_variables);
extend!(Block, OutputVariables, with_output_variables);
extend!(Body, Fbd, with_fbd);
extend!(ConnectionPointIn, Connection, with_connection);
extend!(ConnectionPointIn, RelPosition, with_rel_position);
extend!(ConnectionPointOut, Connection, with_connection);
extend!(ConnectionPointOut, RelPosition, with_rel_position);
extend!(Fbd, Block, with_block);
extend!(Fbd, InVariable, with_in_variable);
extend!(InOutVariables, Variable, with_variable);
extend!(InVariable, ConnectionPointOut, with_connection_point_out);
extend!(InVariable, Position, with_position);
extend!(InputVariables, Variable, with_variable);
extend!(OutputVariables, Variable, with_variable);
extend!(Pou, Body, with_body);
extend!(Variable, ConnectionPointIn, with_connection_point_in);
extend!(Variable, ConnectionPointOut, with_connection_point_out);

impl Variable {
    fn init(name: &'static str, negated: bool) -> Self {
        Variable::new()
            .with_attribute("formalParameter", name)
            .with_attribute("negated", if negated { "true" } else { "false" })
    }
}

impl RelPosition {
    fn init() -> Self {
        RelPosition::new().with_attribute("x", "0").with_attribute("y", "0")
    }
}

impl Connection {
    fn init(name: Option<&'static str>, ref_local_id: &'static str) -> Self {
        let connection = match name {
            Some(name) => Connection::new().with_attribute("formalParameter", name),
            None => Connection::new(),
        };

        connection.with_attribute("refLocalId", ref_local_id)
    }
}

#[test]
fn demo() {
    // TODO: Create convenience methods for all variable related things to make the code below less nested
    let body = Body::new().with_fbd(
        Fbd::new()
            .with_block(
                Block::new()
                    .with_attribute("localId", "3")
                    .with_attribute("typeName", "MyAdd")
                    .with_input_variables(
                        InputVariables::new()
                            .with_variable(
                                Variable::init("a", false).with_connection_point_in(
                                    ConnectionPointIn::new()
                                        .with_rel_position(RelPosition::init().as_empty())
                                        .with_connection(Connection::init(None, "2").as_empty()),
                                ),
                            )
                            .with_variable(
                                Variable::init("a", false).with_connection_point_in(
                                    ConnectionPointIn::new()
                                        .with_rel_position(RelPosition::init().as_empty())
                                        .with_connection(Connection::init(None, "2").as_empty()),
                                ),
                            ),
                    )
                    .with_output_variables(
                        OutputVariables::new()
                            .with_variable(
                                Variable::init("a", false).with_connection_point_in(
                                    ConnectionPointIn::new()
                                        .with_rel_position(RelPosition::init().as_empty())
                                        .with_connection(Connection::init(None, "2").as_empty()),
                                ),
                            )
                            .with_variable(
                                Variable::init("a", false).with_connection_point_in(
                                    ConnectionPointIn::new()
                                        .with_rel_position(RelPosition::init().as_empty())
                                        .with_connection(Connection::init(None, "2").as_empty()),
                                ),
                            ),
                    )
                    .with_inout_variables(InOutVariables::new().as_empty()),
            )
            .with_in_variable(
                InVariable::new()
                    .with_attribute("localId", "0")
                    .with_position(Position::new().as_empty())
                    .with_connection_point_out(
                        ConnectionPointOut::new()
                            .with_rel_position(RelPosition::new().as_empty())
                            .with_connection(Connection::init(Some("a"), "0").as_empty()),
                    ),
            ),
    );

    println!("{}", Pou::new().with_body(body).get_inner().finalize());
}
