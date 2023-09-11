#[derive(Clone)]
struct Node {
    name: &'static str,
    attributes: Vec<(&'static str, &'static str)>,
    children: Vec<Node>,

    // Single line, e.g. <.../>
    closed: bool,

    /// <content>...</content> or <expression>...</expression>
    text: Option<&'static str>,
}

trait IntoNode {
    fn inner(&self) -> Node;
}

impl Node {
    pub fn new(name: &'static str) -> Self {
        Self { name, attributes: Vec::new(), children: Vec::new(), closed: false, text: None }
    }

    pub fn attribute(mut self, key: &'static str, value: &'static str) -> Self {
        self.attributes.push((key, value));
        self
    }

    pub fn child(mut self, node: &dyn IntoNode) -> Self {
        self.children.push(node.inner());
        self
    }

    pub fn children(mut self, nodes: Vec<&dyn IntoNode>) -> Self {
        self.children.extend(nodes.into_iter().map(IntoNode::inner));
        self
    }

    pub fn close(mut self) -> Self {
        self.closed = true;
        self
    }

    pub fn serialize(&self) -> () {
        if self.closed {
            println!("<{} {:?}/>", self.name, self.attributes);
            return;
        }

        println!("<{} {:?}>", self.name, self.attributes);
        for child in &self.children {
            child.serialize();
        }
        println!("</{}>", self.name);
    }

    //
    pub fn local_id(mut self, id: i32) -> Self {
        self = self.attribute("localId", Box::leak(id.to_string().into_boxed_str()));
        self
    }

    pub fn negated(mut self, negated: bool) -> Self {
        self = self.attribute("negated", Box::leak(negated.to_string().into_boxed_str()));
        self
    }
}

macro_rules! newtype_impl {
    ($name_struct:ident, $name_node:expr) => {
        struct $name_struct(Node);

        impl IntoNode for $name_struct {
            fn inner(&self) -> Node {
                self.0.clone()
            }
        }

        impl $name_struct {
            pub fn new() -> Self {
                Self(Node::new($name_node))
            }

            pub fn attribute(mut self, key: &'static str, value: &'static str) -> Self {
                self.0 = self.0.attribute(key, value);
                self
            }

            pub fn child(mut self, node: &dyn IntoNode) -> Self {
                self.0 = self.0.child(node);
                self
            }

            pub fn children(mut self, nodes: Vec<&dyn IntoNode>) -> Self {
                self.0 = self.0.children(nodes);
                self
            }

            pub fn serialize(self) -> () {
                self.0.serialize()
            }
        }
    };
}

newtype_impl!(XInVariable, "inVariable");
impl XInVariable {
    pub fn init(local_id: i32, negated: bool) -> Self {
        Self(Node::new("InVariable").local_id(local_id).negated(negated))
    }
}

newtype_impl!(XPosition, "position");
impl XPosition {
    // Dynamic
    // TODO: usize to static str
    pub fn init(x: &'static str, y: &'static str) -> Self {
        Self(Node::new("position").attribute("x", x).attribute("y", y).close())
    }
}

newtype_impl!(XConnectionPointIn, "connectionPointIn");
newtype_impl!(XRelPosition, "relPosition");
newtype_impl!(XConnection, "connection");

impl XConnectionPointIn {
    pub fn init(ref_local_id: &'static str) -> Self {
        let mut tmp = Self::new();
        // tmp.0 = tmp.children(vec![&XRelPosition::init("10", "20").inner()]);
        tmp.0 = tmp.child(&XRelPosition::init("10", "20"));
        tmp
    }
}

impl XRelPosition {
    // TODO: usize
    pub fn init(x: &'static str, y: &'static str) -> Self {
        let mut tmp = Self::new();
        tmp.0 = tmp.0.attribute("x", x).attribute("y", y);
        tmp
    }
}

impl XConnection {
    // TODO: usize
    pub fn init(ref_local_id: &'static str) -> Self {
        let mut tmp = Self::new();
        tmp.0 = tmp.0.attribute("refLocalId", ref_local_id);
        tmp
    }
}

#[test]
fn temp() {
    // XInVariable::init(1, false).children(vec![&XPosition::init("10", "20")]).0.serialize();
    // XInVariable::new(1, false).children(vec![XPosition::new().close()]);
}

// struct XBody(Node);
// impl XBody {
//     pub fn new() -> Self {
//         Self(Node::new("body"))
//     }

//     pub fn inner(self) -> Node {
//         self.0
//     }
// }

// struct XPou(Node);
// impl XPou {
//     pub fn new() -> Self {
//         Self(Node::new("pou"))
//     }

//     pub fn attribute(mut self, key: &'static str, value: &'static str) -> Self {
//         self.0 = self.0.attribute(key, value);
//         self
//     }

//     pub fn serialize(&self) -> () {
//         self.0.serialize()
//     }
// }

// impl IntoNode for XBody {
//     fn inner(&self) -> Node {
//         self.0.clone()
//     }
// }

// impl IntoNode for XPou {
//     fn inner(&self) -> Node {
//         self.0.clone()
//     }
// }

#[cfg(test)]
mod tests {
    // use super::{XBody, XPou};

    // #[test]
    // fn temp() {
    //     let pou = XPou::new();
    //     // pou.attribute("name", "foo").child(XBody::new()).child(XPou::new()).serialize();
    //     // pou.attribute("name", "foo").children(vec![&XBody::new(), &XBody::new(), &XPou::new()]).serialize();
    //     // let node = Node::new("return").attribute("negated", "false");
    //     // node.serialize();
    // }
}
