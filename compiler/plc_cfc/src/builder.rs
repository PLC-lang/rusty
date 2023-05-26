type Attributes = Vec<(&'static str, &'static str)>;

struct Node {
    name: &'static str,
    attributes: Attributes,
    nodes: Vec<Node>,
}

impl Node {
    pub fn with(name: &'static str, attributes: Attributes, nodes: Node) -> Self {
        Self { name, attributes, nodes: vec![nodes] }
    }

    pub fn with_empty(name: &'static str, attributes: Attributes) -> Self {
        Self { name, attributes, nodes: vec![] }
    }

    pub fn with_nodes(name: &'static str, attributes: Attributes, nodes: Vec<Node>) -> Self {
        Self { name, attributes, nodes }
    }

    pub fn finalize(self, level: usize) {
        println!(
            "{indent}<{name} {attributes:?}>",
            indent = " ".repeat(level * 2),
            name = self.name,
            attributes = self.attributes
        );

        for node in self.nodes {
            node.finalize(level + 1);
        }

        println!("{indent}</{name}>", indent = " ".repeat(level * 2), name = &self.name);
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
            pub fn new(attributes: Option<Attributes>) -> Self {
                Self(Node {
                    name: stringify!($name),
                    attributes: if let Some(attributes) = attributes { attributes } else { vec![] },
                    nodes: vec![],
                })
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

impl Pou {
    fn with_body(mut self, mut body: Body) -> Self {
        body.get_inner_ref_mut().nodes.push(FBD::new(None).get_inner()); // A body must have some type of kind (SF, FBD, ...)
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

#[test]
fn demo() {
    let body = Body::new(None);
    let block = Block::new(Some(vec![("localId", "3"), ("typeName", "MyAdd")]));
    Pou::new(None).with_body(body.with_block(block)).get_inner().finalize(0);
}
