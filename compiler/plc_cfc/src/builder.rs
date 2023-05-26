type Attributes = Option<Vec<(&'static str, &'static str)>>;

struct Node {
    name: &'static str,
    attributes: Option<Vec<(&'static str, &'static str)>>,
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

#[test]
fn demo() {
    Node::with(
        "pou",
        Some(vec![("name", "foo")]),
        Node::with(
            "body",
            None,
            Node::with_nodes(
                "variable",
                None,
                vec![Node::with_empty("foo", None), Node::with_empty("bar", None)],
            ),
        ),
    )
    .finalize(0);
}
