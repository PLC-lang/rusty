type Attributes = Vec<(&'static str, &'static str)>;

struct Node {
    name: &'static str,
    attributes: Attributes,
    is_empty: bool,
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

    pub fn _finalize(self, level: usize) {
        println!(
            "{indent}<{name} {attributes:?}>",
            indent = " ".repeat(level * 2),
            name = self.name,
            attributes = self.attributes
        );

        for node in self.nodes {
            node._finalize(level + 1);
        }

        println!("{indent}</{name}>", indent = " ".repeat(level * 2), name = &self.name);
    }

    pub fn _finaliz(self, level: usize) -> String {
        // if self.is_empty {
        //     return format!(
        //         "{indent}<{name} {attributes:?} />",
        //         indent = " ".repeat(level * 4),
        //         name = self.name,
        //         attributes = self.attributes
        //     );
        // }

        todo!()
    }

    pub fn finalize(self) {
        self._finalize(0);
    }
}

trait Inner {
    fn get_inner(self) -> Node;
    fn get_inner_ref(&self) -> &Node;
    fn get_inner_ref_mut(&mut self) -> &mut Node;
}

macro_rules! types {
    ($name:ident) => {
        struct $name(Node);
        impl $name {
            pub fn new() -> Self {
                Self(Node { name: stringify!($name), attributes: vec![], is_empty: false, nodes: vec![] })
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

types!(Pou);
types!(Body);
types!(FBD);
types!(Block);
types!(InputVariables);
types!(OutputVariables);
types!(InOutVariables);
types!(Variable);
types!(RelPosition);
types!(Connection);
types!(ConnectionPointIn);
types!(ConnectionPointOut);

impl Pou {
    fn with_body(mut self, mut body: Body) -> Self {
        body.get_inner_ref_mut().nodes.push(FBD::new().get_inner()); // A body must have some type of kind (SF, FBD, ...)
        self.get_inner_ref_mut().nodes.push(body.get_inner());
        self
    }
}

impl Body {
    fn with_block(mut self, block: Block) -> Self {
        self.get_inner_ref_mut().nodes.push(block.get_inner());
        self
    }
}

impl Block {
    fn with_input_variables(mut self, variables: InputVariables) -> Self {
        self.get_inner_ref_mut().nodes.push(variables.get_inner());
        self
    }

    fn with_output_variables(mut self, variables: OutputVariables) -> Self {
        self.get_inner_ref_mut().nodes.push(variables.get_inner());
        self
    }

    fn with_inout_variables(mut self, variables: InOutVariables) -> Self {
        self.get_inner_ref_mut().nodes.push(variables.get_inner());
        self
    }
}

impl InputVariables {
    fn with_variable(mut self, variable: Variable) -> Self {
        self.get_inner_ref_mut().nodes.push(variable.get_inner());
        self
    }
}

impl OutputVariables {
    fn with_variable(mut self, variable: Variable) -> Self {
        self.get_inner_ref_mut().nodes.push(variable.get_inner());
        self
    }
}
impl InOutVariables {
    fn with_variable(mut self, variable: Variable) -> Self {
        self.get_inner_ref_mut().nodes.push(variable.get_inner());
        self
    }
}

impl Variable {
    fn init(name: &'static str, negated: &'static str) -> Self {
        let mut variable = Variable::new();
        variable.get_inner_ref_mut().attributes.push(("formalParameter", name));
        variable.get_inner_ref_mut().attributes.push(("negated", negated));

        variable
    }

    fn with_connection_point(mut self, is_point_in: bool, ref_id: Option<&'static str>) -> Self {
        let mut con = match is_point_in {
            true => ConnectionPointIn::new().get_inner(),
            false => ConnectionPointOut::new().get_inner(),
        };

        let pos = RelPosition::new().with_attribute("x", "0").with_attribute("y", "0").as_empty();
        con.nodes.push(pos.get_inner());

        if let Some(ref_id) = ref_id {
            let conn = Connection::new().with_attribute("refLocalId", ref_id).as_empty();
            con.nodes.push(conn.get_inner());
        }

        self.get_inner_ref_mut().nodes.push(con);
        self
    }
}

#[test]
fn demo() {
    let body = Body::new().with_block(
        Block::new()
            .with_attribute("localId", "3")
            .with_attribute("typeName", "MyAdd")
            .with_input_variables(
                InputVariables::new()
                    .with_variable(Variable::init("a", "false").with_connection_point(true, Some("1")))
                    .with_variable(Variable::init("b", "false").with_connection_point(true, Some("2"))),
            )
            .with_output_variables(
                OutputVariables::new()
                    .with_variable(Variable::init("a", "false").with_connection_point(false, Some("1")))
                    .with_variable(Variable::init("b", "false").with_connection_point(false, Some("2"))),
            )
            .with_inout_variables(InOutVariables::new().as_empty()),
    );

    Pou::new().with_body(body).get_inner().finalize();
}
