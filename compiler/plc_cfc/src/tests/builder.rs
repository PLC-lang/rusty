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
    fn serialize(self, level: usize) -> String {
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
            fmt = format!("{fmt}{}", node.serialize(level + 1));
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
            pub(crate) struct $name(Node);
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
                    self.get_inner_ref_mut().nodes.push(value.get_inner());
                    self
                })*
        })*
    }
}

declare_type_and_extend_if_needed! {
    (
        Pou, "pou",
        (Body, with_body)
    ),
    (
        Block, "block",
        (InOutVariables, with_inout_variables),
        (InputVariables, with_input_variables),
        (OutputVariables, with_output_variables)
    ),
    (

        Body, "body",
        (Fbd, with_fbd)
    ),
    (
        ConnectionPointIn, "connectionPointIn",
        (Connection, with_connection),
        (RelPosition, with_rel_position)
    ),
    (
        ConnectionPointOut, "connectionPointOut",
        (Connection, with_connection),
        (RelPosition, with_rel_position)
    ),
    (
        Fbd, "FBD",
        (Block, with_block),
        (InVariable, with_in_variable)
    ),
    (
        Variable, "variable",
        (ConnectionPointIn, with_connection_in),
        (ConnectionPointOut, with_connection_point_out)
    ),
    (
        InVariable, "inVariable",
        (ConnectionPointOut, with_connection_point_out),
        (Position, with_position)
    ),
    (
        InOutVariables, "inOutVariables",
        (Variable, with_variable)
    ),
    (
        InputVariables, "inputVariables",
        (Variable, with_variable)
    ),
    (
        OutputVariables, "outputVariables",
        (Variable, with_variable)
    ),

    // these are not being extended:
    (Position, "position",),
    (RelPosition, "relPosition",),
    (Connection, "connection",),
}

// convenience methods to reduce amount of boiler-plate-code
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

    println!("{}", Pou::new().with_body(body).serialize())
}
