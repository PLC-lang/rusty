type Attributes = Vec<(&'static str, &'static str)>;

/// Number of spaces to use when indenting XML
const INDENT_SPACES: usize = 4;

struct Node {
    name: &'static str,
    attributes: Attributes,
    closed: bool,
    nodes: Vec<Node>,
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
    fn finalize(self, level: usize) -> String {
        let mut fmt = String::new();

        if self.closed {
            fmt = format!(
                "{indent}<{name} {attributes} />\n",
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

        for node in self.nodes {
            fmt = format!("{fmt}{}", node.finalize(level + 1));
        }

        if !self.closed {
            fmt = format!(
                "{fmt}{indent}</{name}>\n",
                indent = " ".repeat(level * INDENT_SPACES),
                name = &self.name
            );
        }

        #[cfg(feature = "debug")]
        println!("{fmt}");

        fmt
    }
}

// TODO: Merge `types!` and `extend!`
macro_rules! types {
    ($(($name:ident, $name_xml:expr)),+) => {
        $(pub(crate) struct $name(Node);
        impl $name {
            pub fn new() -> Self {
                Self(Node { name: $name_xml, attributes: vec![], closed: false, nodes: vec![] })
            }

            pub fn with_attribute(mut self, key: &'static str, value: &'static str) -> Self {
                self.0.attributes.push((key, value));
                self
            }

            pub fn close(mut self) -> Self {
                self.0.closed = true;
                self
            }

            pub fn finalize(self) -> String {
                self.0.finalize(0)
            }

            fn get_inner(self) -> Node {
                self.0
            }

            fn get_inner_ref(&self) -> &Node {
                &self.0
            }

            fn get_inner_ref_mut(&mut self) -> &mut Node {
                &mut self.0
            }
        })*
    };
}

// For `extend! {Pou, (Body, with_body), ... }` the macro will expand to
// impl Pou {
//     fn with_body(arg: Body) -> Self {
//         self.get_inner_ref_mut().nodes.push(arg.get_inner());
//         self
//     }
//
//     // ...
// }
macro_rules! extend {
    ($($type:ty, $(($arg:ty, $fn_name:ident)),+); +) => {
         $(impl $type {
            $( pub(crate) fn $fn_name(mut self, value: $arg) -> Self {
                self.get_inner_ref_mut().nodes.push(value.get_inner());
                self
            })*
        })*
    }
}

extend! {
    Pou,
    (Body, with_body);

    Block,
    (InOutVariables, with_inout_variables),
    (InputVariables, with_input_variables),
    (OutputVariables, with_output_variables);

    Body,
    (Fbd, with_fbd);

    ConnectionPointIn,
    (Connection, with_connection),
    (RelPosition, with_rel_position);

    ConnectionPointOut,
    (Connection, with_connection),
    (RelPosition, with_rel_position);

    Fbd,
    (Block, with_block),
    (InVariable, with_in_variable);

    Variable,
    (ConnectionPointIn, with_connection_in),
    (ConnectionPointOut, with_connection_point_out);

    InVariable,
    (ConnectionPointOut, with_connection_point_out),
    (Position, with_position);

    InOutVariables,
    (Variable, with_variable);

    InputVariables,
    (Variable, with_variable);

    OutputVariables,
    (Variable, with_variable)
}

types! {
    (Pou, "pou"),
    (Body, "body"),
    (Fbd, "FBD"),
    (Block, "block"),
    (InputVariables, "inputVariables"),
    (OutputVariables, "outputVariables"),
    (InOutVariables, "inOutVariables"),
    (Variable, "variable"),
    (RelPosition, "relPosition"),
    (Connection, "connection"),
    (ConnectionPointIn, "connectionPointIn"),
    (ConnectionPointOut, "connectionPointOut"),
    (InVariable, "inVariable"),
    (Position, "position")
}

impl Variable {
    pub(crate) fn init(name: &'static str, negated: bool) -> Self {
        Variable::new()
            .with_attribute("formalParameter", name)
            .with_attribute("negated", if negated { "true" } else { "false" })
    }
}

impl RelPosition {
    pub(crate) fn init() -> Self {
        RelPosition::new().with_attribute("x", "0").with_attribute("y", "0")
    }
}

impl ConnectionPointIn {
    pub(crate) fn with_ref(ref_local_id: &'static str) -> Self {
        ConnectionPointIn::new()
            .with_rel_position(RelPosition::init().close())
            .with_connection(Connection::new().with_attribute("refLocalId", ref_local_id).close())
    }
}

impl ConnectionPointOut {
    pub(crate) fn with_rel_pos() -> Self {
        ConnectionPointOut::new().with_rel_position(RelPosition::init().close())
    }
}

impl InVariable {
    pub(crate) fn init(local_id: &'static str, negated: bool) -> Self {
        InVariable::new()
            .with_attribute("localId", local_id)
            .with_attribute("negated", if negated { "true" } else { "false" })
    }
}

impl Block {
    pub(crate) fn init(local_id: &'static str, type_name: &'static str) -> Self {
        Block::new().with_attribute("localId", local_id).with_attribute("typeName", type_name)
    }
}

#[test]
fn demo() {
    // TODO: Create convenience methods for all variable related things to make the code below less nested
    let body = Body::new().with_fbd(
        Fbd::new()
            .with_block(
                Block::init("5", "MyAdd")
                    .with_attribute("instanceName", "local_add")
                    .with_input_variables(
                        InputVariables::new()
                            .with_variable(
                                Variable::init("a", false)
                                    .with_connection_in(ConnectionPointIn::with_ref("6")),
                            )
                            .with_variable(
                                Variable::init("b", false)
                                    .with_connection_in(ConnectionPointIn::with_ref("7")),
                            ),
                    )
                    .with_inout_variables(InOutVariables::new().close())
                    .with_output_variables(
                        OutputVariables::new().with_variable(
                            Variable::init("c", false)
                                .with_connection_point_out(ConnectionPointOut::with_rel_pos()),
                        ),
                    ),
            )
            .with_in_variable(
                InVariable::new().with_connection_point_out(ConnectionPointOut::with_rel_pos()),
            ),
    );

    println!("{}", Pou::new().with_body(body).finalize())
}
