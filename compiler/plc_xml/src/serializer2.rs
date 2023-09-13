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
        self.children.iter().for_each(|child| result = format!("{result}{}", child.serialize(level + 1)));
        result = format!("{result}{indent}</{name}>\n");

        result
    }

    //
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

            fn attribute(mut self, key: &'static str, value: &'static str) -> Self {
                self.0 = self.0.attribute(key, value);
                self
            }

            fn maybe_attribute(mut self, key: &'static str, value: Option<&'static str>) -> Self {
                if let Some(value) = value {
                    self.0 = self.0.attribute(key, value);
                }

                self
            }

            fn child(mut self, node: &dyn IntoNode) -> Self {
                self.0 = self.0.child(node);
                self
            }

            fn children(mut self, nodes: Vec<&dyn IntoNode>) -> Self {
                self.0 = self.0.children(nodes);
                self
            }

            fn serialize(self) -> String {
                self.0.serialize(0)
            }

            fn local_id<T: std::fmt::Display>(mut self, id: T) -> Self {
                self = self.attribute("localId", Box::leak(id.to_string().into_boxed_str()));
                self
            }

            fn ref_local_id<T: std::fmt::Display>(mut self, id: T) -> Self {
                self = self.attribute("refLocalId", Box::leak(id.to_string().into_boxed_str()));
                self
            }

            fn execution_id<T: std::fmt::Display>(mut self, id: T) -> Self {
                self = self.attribute("executionOrderId", Box::leak(id.to_string().into_boxed_str()));
                self
            }

            fn close(mut self) -> Self {
                self.0 = self.0.close();
                self
            }
        }
    };
}

newtype_impl!(YInVariable, "inVariable");
impl YInVariable {
    /// Adds a child node
    /// <connectPointIn>
    ///     <connection refLocalId="..."/>
    /// </connectionPointIn/>
    pub fn connect(mut self, ref_local_id: i32, formal_parameter: Option<&'static str>) -> Self {
        self = self.child(
            &YConnectionPointIn::new().child(
                &YConnection::new()
                    .ref_local_id(ref_local_id)
                    .maybe_attribute("formalParameter", formal_parameter),
            ),
        );
        self
    }
}

newtype_impl!(YPou, "pou");
impl YPou {
    // TODO: Shouldn't this be merged into a `new` function alongside `with_type` since these fields are mandatory?
    pub fn with_name(name: &'static str) -> Self {
        Self::new().attribute("name", name)
    }

    pub fn with_type(self, kind: &'static str) -> Self {
        self.attribute("pouType", kind)
    }

    // pub fn new(name: &'static str, kind: &'static str, content: &'static str) -> Self {
    //     let mut node = Self::new().attribute("name", name).attribute("pouType", kind);
    // }
}

newtype_impl!(YPosition, "position");
newtype_impl!(YConnectionPointIn, "connectionPointIn");
newtype_impl!(YRelPosition, "relPosition");
newtype_impl!(YConnection, "connection");
newtype_impl!(YBlock, "block");
impl YBlock {
    pub fn with_name(self, name: &'static str) -> Self {
        self.attribute("typeName", name)
    }
}

newtype_impl!(YInputVariables, "inputVariables");
impl YInputVariables {
    pub fn with_variables(variables: Vec<&dyn IntoNode>) -> Self {
        Self::new().children(variables)
    }
}

newtype_impl!(YOutputVariables, "outputVariables");
impl YOutputVariables {
    pub fn with_variables(variables: Vec<&dyn IntoNode>) -> Self {
        Self::new().children(variables)
    }
}

newtype_impl!(YVariable, "variable");
impl YVariable {
    pub fn with_name(name: &'static str) -> Self {
        Self::new().attribute("formalParameter", name)
    }

    pub fn connect(self, ref_local_id: i32) -> Self {
        self.child(&YConnection::new().ref_local_id(ref_local_id).close())
    }
}

#[test]
fn pou() {
    #[rustfmt::skip]
    let serialized = YPou::with_name("foo");
}

#[test]
fn block() {
    // TODO: negate()
    #[rustfmt::skip]
    let serialized = YBlock::with_id(14).with_name("myAdd").execution_id(0).children(vec![
        &YInputVariables::with_variables(vec![
            &YVariable::with_name("a").connect(16),
            &YVariable::with_name("b").connect(17),
        ]),
        &YOutputVariables::with_variables(vec![&YVariable::with_name("myAdd")])
    ]).serialize();

    println!("{serialized}");
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
