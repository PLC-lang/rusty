#[derive(Clone)]
struct Node {
    name: &'static str,
    attributes: Vec<Attribute>,
    children: Vec<Node>,

    // Single line, e.g. <.../>
    closed: bool,

    /// <content>...</content> or <expression>...</expression>
    text: Option<&'static str>,
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
    pub fn new(name: &'static str) -> Self {
        Self { name, attributes: Vec::new(), children: Vec::new(), closed: false, text: None }
    }

    pub fn attribute(mut self, key: &'static str, value: &'static str) -> Self {
        self.attributes.push(Attribute { key, value });
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

    #[allow(unused_assignments)]
    pub fn serialize(&self, level: usize) -> String {
        let (name, indent) = (self.name, " ".repeat(level * 4));
        let attributes = self.attributes.iter().map(Attribute::to_string).collect::<Vec<_>>().join(" ");
        let mut result = String::new();

        if self.closed {
            return format!("{indent}<{name} {attributes}/>\n");
        }

        result = format!("{indent}<{name} {attributes}>\n");
        self.children.iter().for_each(|it| result = format!("{result}{}", it.serialize(level + 1)));
        result = format!("{result}{indent}</{name}>\n");

        result
    }

    //
    pub fn local_id(mut self, id: i32) -> Self {
        self = self.attribute("localId", Box::leak(id.to_string().into_boxed_str()));
        self
    }

    pub fn ref_local_id(mut self, id: i32) -> Self {
        self = self.attribute("refLocalId", Box::leak(id.to_string().into_boxed_str()));
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

        // TODO: Perhaps deref
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

            // Pou::new().children(vec![
            //     relPOsition::new().local_id()
            //    Variable::new().local_id(10).negated().connect_to(5)
            // ])
            pub fn children(mut self, nodes: Vec<&dyn IntoNode>) -> Self {
                self.0 = self.0.children(nodes);
                self
            }

            pub fn serialize(self) -> String {
                self.0.serialize(0)
            }
        }
    };
}

newtype_impl!(XInVariable, "inVariable");
impl XInVariable {
    /// <inVariable localId=... negated=...>
    ///     <position x=... y=.../>
    ///     <connection refLocalId=.../>
    /// </inVariable>
    pub fn init(local_id: i32, negated: bool, ref_local_id: Option<i32>) -> Self {
        let mut node = Node::new("inVariable").local_id(local_id).negated(negated);
        node = node.child(&XPosition::init("1", "2"));

        if let Some(ref_local_id) = ref_local_id {
            node = node.child(&XConnectionPointIn::init(ref_local_id));
        }

        Self(node)
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
    pub fn init(ref_local_id: i32) -> Self {
        let mut tmp = Self::new();
        tmp.0 = tmp.0.child(&XRelPosition::init("10", "20"));
        tmp.0 = tmp.0.child(&XConnection::init(ref_local_id));
        tmp
    }
}

impl XRelPosition {
    // TODO: usize
    pub fn init(x: &'static str, y: &'static str) -> Self {
        let mut tmp = Self::new();
        tmp.0 = tmp.0.attribute("x", x).attribute("y", y).close();
        tmp.0 = tmp.0.close();
        tmp
    }
}

impl XConnection {
    // TODO: usize
    pub fn init(ref_local_id: i32) -> Self {
        let mut tmp = Self::new();
        tmp.0 = tmp.0.ref_local_id(ref_local_id);
        tmp.0 = tmp.0.close();
        tmp
    }
}

#[test]
fn temp() {
    println!("{}", XInVariable::init(1, false, None).serialize());
    println!("{}", XInVariable::init(1, false, Some(2)).serialize());
}

// #[test]
// fn temp() {
//     // // serialize(), to_ast()
//     // fn parse(content: &str) -> (CompilationUnit, Vec<Diagnostic>) {
//     //     let source_code = SourceCode::new(content, "test.cfc");
//     //     xml_parser::parse(&source_code, LinkageType::Internal, IdProvider::default())
//     // }

//     let pou = format!(
//         r#"
//     <?xml version="1.0" encoding="UTF-8"?>
//     <pou xmlns="http://www.plcopen.org/xml/tc6_0201" name="conditional_return" pouType="functionBlock">
//         <interface>
//             <localVars/>
//             <addData>
//                 <data name="www.bachmann.at/plc/plcopenxml" handleUnknown="implementation">
//                     <textDeclaration>
//                         <content>
//                         FUNCTION_BLOCK conditional_return
//                         VAR_INPUT
//                             val : DINT;
//                         END_VAR</content>
//                     </textDeclaration>
//                 </data>
//             </addData>
//         </interface>
//         <body>
//             <FBD>
//                 {content}
//             </FBD>
//         </body>
//     </pou>
//     "#,
//         content = XInVariable::init(1, false, None).serialize()
//     );

//     //     let pou = todo!();
//     //     pou.add_block()

//     //     XPou::new(...).with_children(vec![
//     //         // XInVariable::init(1, false, Some(2)).serialize();
//     //         XInVariable::with_id(1).with_expression("...").with_exec_id(...).connected(5).negated(),
//     //         XInVariable::with_id(1).with_expression("...").with_exec_id(...).connected(5),
//     //         XInVariable::with_id(1).with_expression("...").with_exec_id(...).connected(5),
//     //         XInVariable::with_id(1).with_expression("...").with_exec_id(...).connected(5),
//     //     ])

//     //     XInVariable::with_id(...).
//     //     XInVariable::init(1, false, Some(2)).serialize();
//     //     println!("{repeat}", repeat = "=".repeat(100));
//     //     XInVariable::init(1, false, None).serialize();

//     //     //
//     //     XInVariable::init(1, false, Some(2));
//     }
