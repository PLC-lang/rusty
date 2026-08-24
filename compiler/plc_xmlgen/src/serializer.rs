#![allow(clippy::new_without_default)]

use rustc_hash::FxHashMap;

#[derive(Clone)]
pub struct Node {
    pub name: String,
    pub children: Vec<Node>,

    /// XML attributes, e.g. `<position x="1">` where `x` is the attribute
    ///
    /// Design Note: We use a HashMap here to avoid duplicates but also update existing values in case of
    /// repeated function calls, e.g. `with_attribute("x", 1)` and `with_attribute("x", 2)` where the value of
    /// x has been updated from 1 to 2.
    pub attributes: FxHashMap<String, String>,

    /// Indicates if an element has a closed form, e.g. `<position x="1" y="2"/>`
    pub closed: bool,

    /// Indicates if an element has some text wrapped inside itself, e.g. `<expression>a + b</expression>`
    pub content: Option<String>,
}

pub trait IntoNode {
    fn inner(&self) -> Node;
}

impl Node {
    pub fn new(name: String) -> Self {
        Self { name, attributes: FxHashMap::default(), children: Vec::new(), closed: false, content: None }
    }

    pub fn new_str(name: &'static str) -> Self {
        Self::new(name.to_string())
    }

    pub fn content_borrowed(mut self, input: String) -> Self {
        self.content = Some(input);
        self
    }

    pub fn attribute(mut self, key: String, value: String) -> Self {
        self.attributes.insert(key, value);
        self
    }

    pub fn attribute_str(self, key: &'static str, value: &'static str) -> Self {
        Self::attribute(self, key.to_string(), value.to_string())
    }    

    pub fn child(mut self, node: &dyn IntoNode) -> Self {
        self.children.push(node.inner());
        self
    }

    pub fn child_borrowed(&mut self, node: &dyn IntoNode) -> &Self {
        self.children.push(node.inner());
        self
    }

    pub fn children(mut self, nodes: Vec<Box<dyn IntoNode>>) -> Self {
        let mapped: Vec<Node> = nodes.iter().map(|a| a.inner()).collect();
        self.children.extend(mapped);
        self
    }

    pub fn close(mut self) -> Self {
        self.closed = true;
        self
    }

    pub fn indent(level: usize) -> String {
        " ".repeat(level * 4)
    }

    fn serialize_content(indent: String, name: String, content: String) -> String {
        format!("{indent}<{name}>{content}</{name}>\n")
    }

    #[allow(unused_assignments)]
    pub fn serialize(&self, level: usize) -> String {
        let (name, indent) = (self.name.clone(), Node::indent(level));
        let attributes = self.attributes.iter().map(|(key, value)| format!("{key}=\"{value}\""));
        let attributes_str = attributes.collect::<Vec<_>>().join(" ");
        let mut result = String::new();

        if self.closed {
            return format!("{indent}<{name} {attributes_str}/>\n");
        }

        if let Some(content) = self.content.clone() {
            return Node::serialize_content(indent.to_string(), name, content);
        }

        result = format!("{indent}<{name} {attributes_str}>\n");
        self.children.iter().for_each(|child| result = format!("{result}{}", child.serialize(level + 1)));
        result = format!("{result}{indent}</{name}>\n");

        result
    }
}

macro_rules! newtype_impl {
    ($name_struct:ident, $name_node:expr, $negatable:expr) => {
        pub struct $name_struct(Node);

        impl IntoNode for $name_struct {
            fn inner(&self) -> Node {
                self.0.clone()
            }
        }

        impl $name_struct {
            pub fn new() -> Self {
                match $negatable {
                    true => Self(Node::new_str($name_node).attribute_str("negated", "false")),
                    false => Self(Node::new_str($name_node)),
                }
            }

            pub fn content(self, input: String) -> Self {
                let mut inner = self.inner();
                inner.content = Some(input);
                Self(inner)
            }

            pub fn id(local_id: i32) -> Self {
                let new = $name_struct::new();
                new.with_id(local_id)
            }

            pub fn attribute(self, key: String, value: String) -> Self {
                Self(self.inner().attribute(key, value))
            }

            pub fn attribute_str(self, key: &'static str, value: &'static str) -> Self {
                Self(self.inner().attribute_str(key, value))
            }            

            pub fn maybe_attribute(self, key: String, value: Option<String>) -> Self {
                match value {
                    Some(value) => Self(self.inner().attribute(key, value)),
                    None => self,
                }
            }

            pub fn child(self, node: &dyn IntoNode) -> Self {
                Self(self.inner().child(node))
            }

            pub fn children(self, nodes: Vec<Box<dyn IntoNode>>) -> Self {
                Self(self.inner().children(nodes))
            }

            pub fn serialize(self) -> String {
                self.inner().serialize(0)
            }

            pub fn with_id<T: std::fmt::Display>(self, id: T) -> Self {
                self.attribute_str("localId", Box::leak(id.to_string().into_boxed_str()))
            }

            pub fn with_ref_id<T: std::fmt::Display>(self, id: T) -> Self {
                self.attribute_str("refLocalId", Box::leak(id.to_string().into_boxed_str()))
            }

            pub fn with_execution_id<T: std::fmt::Display>(self, id: T) -> Self {
                self.attribute_str("executionOrderId", Box::leak(id.to_string().into_boxed_str()))
            }

            pub fn close(self) -> Self {
                Self(self.inner().close())
            }
        }
    };
}

// newtype_impl!(<struct name>, <xml name>, <is negatable>)
newtype_impl!(SInVariable, "inVariable", true);
newtype_impl!(SOutVariable, "outVariable", true);
newtype_impl!(SInOutVariable, "inOutVariable", true);
newtype_impl!(SInterface, "interface", false);
newtype_impl!(SLocalVars, "localVars", false);
newtype_impl!(SAddData, "addData", false);
newtype_impl!(SData, "data", false);
newtype_impl!(STextDeclaration, "textDeclaration", false);
newtype_impl!(SContent, "content", false);
newtype_impl!(SPosition, "position", false);
newtype_impl!(SConnectionPointIn, "connectionPointIn", false);
newtype_impl!(SConnectionPointOut, "connectionPointOut", false);
newtype_impl!(SRelPosition, "relPosition", false);
newtype_impl!(SConnection, "connection", false);
newtype_impl!(SBlock, "block", false);
newtype_impl!(SBody, "body", false);
newtype_impl!(SPou, "pou", false);
newtype_impl!(SInputVariables, "inputVariables", false);
newtype_impl!(SOutputVariables, "outputVariables", false);
newtype_impl!(SInOutVariables, "inOutVariables", false);
newtype_impl!(SVariable, "variable", true);
newtype_impl!(YFbd, "FBD", false);
newtype_impl!(SExpression, "expression", false);
newtype_impl!(SReturn, "return", false);
newtype_impl!(SNegate, "negated", false);
newtype_impl!(SConnector, "connector", false);
newtype_impl!(SContinuation, "continuation", false);
newtype_impl!(SJump, "jump", false);
newtype_impl!(SLabel, "label", false);
newtype_impl!(SAction, "action", false);
newtype_impl!(SActions, "actions", false);
newtype_impl!(SFileHeader, FILE_HEADER, false);
newtype_impl!(SContentHeader, CONTENT_HEADER, false);
newtype_impl!(STypes, TYPES, false);

pub const FILE_HEADER: &'static str = "FileHeader";
pub const CONTENT_HEADER: &'static str = "ContentHeader";

pub trait SizedVariable: IntoNode + Sized {

}

impl SInVariable {
    pub fn connect(mut self, ref_local_id: i32) -> Self {
        self = self.child(&SConnectionPointIn::new().child(&SConnection::new().with_ref_id(ref_local_id)));
        self
    }

    pub fn with_expression(self, expression: String) -> Self {
        self.child(&SExpression::expression(expression))
    }

    pub fn with_expression_str(self, expression: &'static str) -> Self {
        self.child(&SExpression::expression_str(expression))
    }
}

impl SOutVariable {
    pub fn connect(mut self, ref_local_id: i32) -> Self {
        self = self
            .child(&SConnectionPointIn::new().child(&SConnection::new().with_ref_id(ref_local_id).close()));
        self
    }

    pub fn connect_name(mut self, ref_local_id: i32, name: String) -> Self {
        self =
            self.child(&SConnectionPointIn::new().child(
                &SConnection::new().with_ref_id(ref_local_id).attribute("formalParameter".to_string(), name).close(),
            ));
        self
    }

    pub fn connect_name_str(self, ref_local_id: i32, name: &'static str) -> Self {
        self.connect_name(ref_local_id, name.to_string())
    }    

    pub fn with_expression(self, expression: String) -> Self {
        self.child(&SExpression::expression(expression))
    }

    pub fn with_expression_str(self, expression: &'static str) -> Self {
        self.child(&SExpression::expression_str(expression))
    }
}

impl SInOutVariable {
    pub fn with_expression(self, expression: String) -> Self {
        self.child(&SExpression::expression(expression))
    }

    pub fn with_expression_str(self, expression: &'static str) -> Self {
        self.with_expression(expression.to_string())
    }    
}

impl SReturn {
    pub fn init(local_id: i32, execution_order: i32) -> Self {
        Self::new().with_id(local_id).with_execution_id(execution_order)
    }

    pub fn connect(self, ref_local_id: i32) -> Self {
        self.child(&SConnectionPointIn::new().child(&SConnection::new().with_ref_id(ref_local_id)))
    }

    pub fn negate(self, value: bool) -> Self {
        self.child(&SAddData::new().child(&SData::new().child(
            &SNegate::new().attribute(String::from("value"), value.to_string()).close(),
        )))
    }
}

impl SContent {
    pub fn with_declaration(mut self, content: String) -> Self {
        self.0.content = Some(content);
        self
    }
}

impl SPou {
    pub fn init(name: String, kind: String, declaration: String) -> Self {
        Self::new()
            .attribute_str("xmlns", "http://www.plcopen.org/xml/tc6_0201")
            .attribute("name".to_string(), name)
            .attribute("pouType".to_string(), kind)
            .child(&SInterface::new().children(vec![
                    Box::new(SLocalVars::new().close()),
                    Box::new(SAddData::new().child(
                        &SData::new()
                            .attribute_str("name", "www.bachmann.at/plc/plcopenxml")
                            .attribute_str("handleUnknown", "implementation")
                            .child(
                                &STextDeclaration::new()
                                    .child(&SContent::new().with_declaration(declaration)),
                            ),
                    )),
                ]))
    }

    pub fn init_str(name: &'static str, kind: &'static str, declaration: &'static str) -> Self {
        Self::init(name.to_string(), kind.to_string(), declaration.to_string())
    }

    /// Implicitly wraps the fbd in a block node, i.e. <block><fbd>...<fbd/><block/>
    pub fn with_fbd(self, children: Vec<Box<dyn IntoNode>>) -> Self {
        self.child(&SBody::new().child(&YFbd::new().children(children)))
    }

    pub fn with_actions(self, children: Vec<Box<dyn IntoNode>>) -> Self {
        self.child(&SActions::new().children(children))
    }
}

impl SBlock {
    pub fn init(name: String, local_id: i32, execution_order_id: i32) -> Self {
        Self::new().with_name(name).with_id(local_id).with_execution_id(execution_order_id)
    }

    pub fn init_str(name: &'static str, local_id: i32, execution_order_id: i32) -> Self {
        Self::init(name.to_string(), local_id, execution_order_id)
    }    

    pub fn with_name(self, name: String) -> Self {
        self.attribute("typeName".to_string(), name)
    }

    pub fn with_name_str(self, name: &'static str) -> Self {
        self.attribute("typeName".to_string(), name.to_string())
    }    

    pub fn with_input(self, variables: Vec<Box<dyn IntoNode>>) -> Self {
        self.child(&SInputVariables::new().children(variables))
    }

    pub fn with_output(self, variables: Vec<Box<dyn IntoNode>>) -> Self {
        self.child(&SOutputVariables::new().children(variables))
    }

    pub fn with_inout(self, variables: Vec<Box<dyn IntoNode>>) -> Self {
        self.child(&SInOutVariables::new().children(variables))
    }
}

impl SBody {
    pub fn with_fbd(self, children: Vec<Box<dyn IntoNode>>) -> Self {
        Self::new().child(&YFbd::new().children(children))
    }
}

impl SInputVariables {
    pub fn with_variables(variables: Vec<Box<dyn IntoNode>>) -> Self {
        Self::new().children(variables)
    }
}

impl SOutputVariables {
    pub fn with_variables(variables: Vec<Box<dyn IntoNode>>) -> Self {
        Self::new().children(variables)
    }
}

impl SVariable {
    pub fn with_name(self, name: String) -> Self {
        self.attribute("formalParameter".to_string(), name)
    }

    pub fn with_name_str(self, name: &'static str) -> Self {
        self.with_name(name.to_string())
    }

    pub fn connect(self, ref_local_id: i32) -> Self {
        self.child(&SConnectionPointIn::new().child(&SConnection::new().with_ref_id(ref_local_id).close()))
    }

    pub fn connect_out(self, ref_local_id: i32) -> Self {
        self.child(&SConnectionPointOut::new().child(&SConnection::new().with_ref_id(ref_local_id).close()))
    }
}

impl SExpression {
    pub fn expression(input: String) -> Self {
        let mut node = Self::new();
        node.0.content = Some(input);
        node
    }

    pub fn expression_str(input: &'static str) -> Self {
        Self::expression(input.to_string())
    }
}

impl SConnector {
    pub fn with_name(self, name: String) -> Self {
        self.attribute("name".to_string(), name)
    }

    pub fn with_name_str(self, name: &'static str) -> Self {
        self.with_name(name.to_string())
    }    

    pub fn connect(self, ref_local_id: i32) -> Self {
        self.child(&SConnectionPointIn::new().child(&SConnection::new().with_ref_id(ref_local_id).close()))
    }
}

impl SContinuation {
    pub fn with_name(self, name: String) -> Self {
        self.attribute("name".to_string(), name)
    }

    pub fn with_name_str(self, name: &'static str) -> Self {
        self.with_name(name.to_string())
    }    

    pub fn connect_out(self, ref_local_id: i32) -> Self {
        self.child(&SConnectionPointOut::new().child(&SConnection::new().with_ref_id(ref_local_id).close()))
    }
}

impl SLabel {
    pub fn with_name(self, name: String) -> Self {
        self.attribute("label".to_string(), name)
    }

    pub fn with_name_str(self, name: &'static str) -> Self {
        self.with_name(name.to_string())
    }
}

impl SJump {
    pub fn with_name(self, name: String) -> Self {
        self.attribute("label".to_string(), name)
    }

    pub fn with_name_str(self, name: &'static str) -> Self {
        self.with_name(name.to_string())
    }    

    pub fn connect(self, ref_local_id: i32) -> Self {
        self.child(&SConnectionPointIn::new().child(&SConnection::new().with_ref_id(ref_local_id).close()))
    }

    pub fn negate(self) -> Self {
        self.child(
            &SAddData::new().child(&SData::new().child(&SNegate::new().attribute(String::from("value"), String::from("true")).close())),
        )
    }
}

impl SAction {
    pub fn name(name: String) -> Self {
        Self::new().attribute("name".to_string(), name)
    }

    pub fn name_str(name: &'static str) -> Self {
        Self::name(name.to_string())
    }    

    pub fn with_fbd(self, children: Vec<Box<dyn IntoNode>>) -> Self {
        self.child(&SBody::new().child(&YFbd::new().children(children)))
    }
}

impl SGenVariable {
    pub fn with_name(self, name: String) -> Self {
        self.attribute("name".to_string(), name)
    }
}

//Omron specific xml
newtype_impl!(SGlobalNamespace, GLOBAL_NAMESPACE, false);
newtype_impl!(SInstances, INSTANCES, false);
newtype_impl!(SConfiguration, CONFIGURATION, false);
newtype_impl!(SResource, RESOURCE, false);
newtype_impl!(SGlobalVars, "GlobalVars", false);
newtype_impl!(SType, "Type", false);
newtype_impl!(STypeName, "TypeName", false);
newtype_impl!(SInitialValue, "InitialValue", false);
newtype_impl!(SSimpleValue, "SimpleValue", false);
newtype_impl!(SGenVariable, "Variable", false);
newtype_impl!(SOmronAddData, "AddData", false);
newtype_impl!(SOmronData, "Data", false);
newtype_impl!(SOmronGlobalVariableAdditionalProperties, "GlobalVariableAdditionalProperties", false);
newtype_impl!(SDataTypeDecl, "DataTypeDecl", false);
newtype_impl!(SDocumentation, "Documentation", false);
newtype_impl!(SUserDefinedTypeSpec, "UserDefinedTypeSpec", false);
newtype_impl!(SMember, "Member", false);
newtype_impl!(SEnumerator, "Enumerator", false);
newtype_impl!(SBaseType, "BaseType", false);
newtype_impl!(SST, "ST", false);
newtype_impl!(SBodyContent, "BodyContent", false);
newtype_impl!(SMainBody, "MainBody", false);
newtype_impl!(SProgram, "Program", false);
newtype_impl!(SFunction, "Function", false);
newtype_impl!(SFunctionBlock, "FunctionBlock", false);
newtype_impl!(SVars, "Vars", false);
newtype_impl!(STempVars, "TempVars", false);
newtype_impl!(SPouInfo, "smcext:PouInfo", false);
newtype_impl!(SResultType, "ResultType", false);
newtype_impl!(SParameters, "Parameters", false);
newtype_impl!(SExternalVars, "ExternalVars", false);
newtype_impl!(SInputVars, "InputVars", false);
newtype_impl!(SInoutVars, "InoutVars", false);
newtype_impl!(SOutputVars, "OutputVars", false);
newtype_impl!(SAddress, "Address", false);

pub const GLOBAL_NAMESPACE: &'static str = "GlobalNamespace";
pub const INSTANCES: &'static str = "Instances";
pub const CONFIGURATION: &'static str = "Configuration";
pub const RESOURCE: &'static str = "Resource";
pub const TYPES: &'static str = "Types";
