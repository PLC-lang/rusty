use serde::Deserialize;

macro_rules! impl_deserialize_for_xsi_type_enum {
    ($enum:ty, $tag:literal, $($cases:tt)*) => {
        impl<'de> serde::de::Deserialize<'de> for $enum {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::de::Deserializer<'de>,
            {
                use serde::de::{Error, MapAccess, Visitor};

                struct TheVisitor;
                impl<'de> Visitor<'de> for TheVisitor {
                    type Value = $enum;

                    fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                        f.write_str("expecting map with tag in ")?;
                        f.write_str($tag)
                    }

                    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
                    where
                        A: MapAccess<'de>,
                    {
                        let tag: String = loop {
                            match map.next_entry::<String, String>()? {
                                Some((attribute, value)) if attribute == $tag => {
                                    break value.rsplit(':').next().unwrap_or(&value).to_string();
                                }
                                Some((attribute, _)) if attribute.starts_with("@xmlns") => continue,
                                Some((attribute, _)) => {
                                    return Err(A::Error::unknown_field(&attribute, &[$tag]))
                                }
                                None => return Err(A::Error::missing_field($tag)),
                            }
                        };

                        let de = serde::de::value::MapAccessDeserializer::new(map);
                        quick_xml::deserialize_match!(tag, de, $enum, $($cases)*)
                    }
                }
                deserializer.deserialize_map(TheVisitor)
            }
        }
    }
}

pub fn from_str(xml: &str) -> Result<Pou, quick_xml::DeError> {
    quick_xml::de::from_str(xml)
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub enum Pou {
    Program(Program),
    FunctionBlock(FunctionBlock),
    Function(Function),
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Program {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "AddData")]
    pub add_data: Option<AddData>,
    #[serde(rename = "MainBody")]
    pub main_body: Option<MainBody>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct FunctionBlock {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@abstract", default)]
    pub is_abstract: bool,
    #[serde(rename = "@final", default)]
    pub is_final: bool,
    #[serde(rename = "Extends")]
    pub extends: Option<TypeRef>,
    #[serde(rename = "Implements", default)]
    pub implements: Vec<TypeRef>,
    #[serde(rename = "AddData")]
    pub add_data: Option<AddData>,
    #[serde(rename = "MainBody")]
    pub main_body: Option<MainBody>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Function {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "ResultType")]
    pub result_type: Option<TypeRef>,
    #[serde(rename = "AddData")]
    pub add_data: Option<AddData>,
    #[serde(rename = "MainBody")]
    pub main_body: Option<MainBody>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct TypeRef {
    #[serde(rename = "TypeName")]
    pub type_name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct AddData {
    #[serde(rename = "Data", default)]
    pub data: Vec<Data>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Data {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@handleUnknown")]
    pub handle_unknown: String,
    #[serde(rename = "textDeclaration")]
    pub text_declaration: Option<TextDeclaration>,
    #[serde(rename = "EvaluationPriority")]
    pub evaluation_priority: Option<EvaluationPriority>,
    #[serde(rename = "negated")]
    pub negated: Option<Negated>,
}

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub struct EvaluationPriority {
    #[serde(rename = "@priorityInNetwork")]
    pub priority_in_network: Option<u64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub struct Negated {
    #[serde(rename = "@value")]
    pub value: bool,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct TextDeclaration {
    pub content: TextContent,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct TextContent {
    #[serde(rename = "$text", default)]
    pub text: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct MainBody {
    #[serde(rename = "BodyContent", default)]
    pub body_content: Vec<BodyContent>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BodyContent {
    Fbd(FbdBody),
}

impl_deserialize_for_xsi_type_enum! {
    BodyContent, "@type",
    ("FBD" => Fbd(FbdBody)),
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct FbdBody {
    #[serde(rename = "Network", default)]
    pub networks: Vec<FbdNetwork>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct FbdNetwork {
    #[serde(rename = "@label")]
    pub label: Option<String>,
    #[serde(rename = "RelPosition")]
    pub rel_position: Option<XyValue>,
    #[serde(rename = "Size")]
    pub size: Option<XyValue>,
    #[serde(rename = "CommonObject", default)]
    pub common_objects: Vec<CommonObject>,
    #[serde(rename = "FbdObject", default)]
    pub objects: Vec<FbdObject>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CommonObject {
    Comment(Comment),
    Connector(Connector),
    Continuation(Continuation),
}

impl_deserialize_for_xsi_type_enum! {
    CommonObject, "@type",
    ("Comment" => Comment(Comment)),
    ("Connector" => Connector(Connector)),
    ("Continuation" => Continuation(Continuation)),
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Comment {
    #[serde(rename = "@globalId")]
    pub global_id: Option<u64>,
    #[serde(rename = "RelPosition")]
    pub rel_position: Option<XyValue>,
    #[serde(rename = "Size")]
    pub size: Option<XyValue>,
    #[serde(rename = "Content")]
    pub content: TextContent,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Connector {
    #[serde(rename = "@globalId")]
    pub global_id: Option<u64>,
    #[serde(rename = "@label")]
    pub label: String,
    #[serde(rename = "RelPosition")]
    pub rel_position: Option<XyValue>,
    #[serde(rename = "Size")]
    pub size: Option<XyValue>,
    #[serde(rename = "ConnectionPointIn")]
    pub connection_point_in: Option<ConnectionPointIn>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Continuation {
    #[serde(rename = "@globalId")]
    pub global_id: Option<u64>,
    #[serde(rename = "@label")]
    pub label: String,
    #[serde(rename = "RelPosition")]
    pub rel_position: Option<XyValue>,
    #[serde(rename = "Size")]
    pub size: Option<XyValue>,
    #[serde(rename = "ConnectionPointOut")]
    pub connection_point_out: Option<ConnectionPointOut>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FbdObject {
    Block(Block),
    DataSource(DataSource),
    DataSink(DataSink),
    Unconnected(Unconnected),
    Jump(Jump),
    Return(Return),
}

impl_deserialize_for_xsi_type_enum! {
    FbdObject, "@type",
    ("Block" => Block(Block)),
    ("DataSource" => DataSource(DataSource)),
    ("DataSink" => DataSink(DataSink)),
    ("Unconnected" => Unconnected(Unconnected)),
    ("Jump" => Jump(Jump)),
    ("Return" => Return(Return)),
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Block {
    #[serde(rename = "@globalId")]
    pub global_id: Option<u64>,
    #[serde(rename = "@typeName")]
    pub type_name: String,
    #[serde(rename = "@instanceName")]
    pub instance_name: Option<String>,
    #[serde(rename = "AddData")]
    pub add_data: Option<AddData>,
    #[serde(rename = "RelPosition")]
    pub rel_position: Option<XyValue>,
    #[serde(rename = "Size")]
    pub size: Option<XyValue>,
    #[serde(rename = "InOutVariables")]
    pub in_out_variables: Option<InOutVariables>,
    #[serde(rename = "InputVariables")]
    pub input_variables: Option<InputVariables>,
    #[serde(rename = "OutputVariables")]
    pub output_variables: Option<OutputVariables>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct InOutVariables {
    #[serde(rename = "InOutVariable", default)]
    pub variables: Vec<InOutVariable>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct InputVariables {
    #[serde(rename = "InputVariable", default)]
    pub variables: Vec<InputVariable>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct OutputVariables {
    #[serde(rename = "OutputVariable", default)]
    pub variables: Vec<OutputVariable>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct InOutVariable {
    #[serde(rename = "@parameterName")]
    pub parameter_name: String,
    #[serde(rename = "@negated", default)]
    pub negated: bool,
    #[serde(rename = "ConnectionPointIn")]
    pub connection_point_in: Option<ConnectionPointIn>,
    #[serde(rename = "ConnectionPointOut")]
    pub connection_point_out: Option<ConnectionPointOut>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct InputVariable {
    #[serde(rename = "@parameterName")]
    pub parameter_name: String,
    #[serde(rename = "@negated", default)]
    pub negated: bool,
    #[serde(rename = "@edge", default)]
    pub edge: EdgeModifier,
    #[serde(rename = "@suppressName", default)]
    pub suppress_name: bool,
    #[serde(rename = "ConnectionPointIn")]
    pub connection_point_in: Option<ConnectionPointIn>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct OutputVariable {
    #[serde(rename = "@parameterName")]
    pub parameter_name: String,
    #[serde(rename = "@negated", default)]
    pub negated: bool,
    #[serde(rename = "@suppressName", default)]
    pub suppress_name: bool,
    #[serde(rename = "ConnectionPointOut")]
    pub connection_point_out: ConnectionPointOut,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct DataSource {
    #[serde(rename = "@globalId")]
    pub global_id: Option<u64>,
    #[serde(rename = "@identifier")]
    pub identifier: String,
    #[serde(rename = "RelPosition")]
    pub rel_position: Option<XyValue>,
    #[serde(rename = "Size")]
    pub size: Option<XyValue>,
    #[serde(rename = "ConnectionPointOut")]
    pub connection_point_out: Option<ConnectionPointOut>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct DataSink {
    #[serde(rename = "@globalId")]
    pub global_id: Option<u64>,
    #[serde(rename = "@identifier")]
    pub identifier: String,
    #[serde(rename = "AddData")]
    pub add_data: Option<AddData>,
    #[serde(rename = "RelPosition")]
    pub rel_position: Option<XyValue>,
    #[serde(rename = "Size")]
    pub size: Option<XyValue>,
    #[serde(rename = "ConnectionPointIn")]
    pub connection_point_in: Option<ConnectionPointIn>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Unconnected {
    #[serde(rename = "@globalId")]
    pub global_id: Option<u64>,
    #[serde(rename = "@complexIdentifier")]
    pub complex_identifier: String,
    #[serde(rename = "RelPosition")]
    pub rel_position: Option<XyValue>,
    #[serde(rename = "Size")]
    pub size: Option<XyValue>,
    #[serde(rename = "ConnectionPointIn")]
    pub connection_point_in: Option<ConnectionPointIn>,
    #[serde(rename = "ConnectionPointOut")]
    pub connection_point_out: Option<ConnectionPointOut>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Jump {
    #[serde(rename = "@globalId")]
    pub global_id: Option<u64>,
    #[serde(rename = "@targetNetworkLabel")]
    pub target_network_label: String,
    #[serde(rename = "RelPosition")]
    pub rel_position: Option<XyValue>,
    #[serde(rename = "Size")]
    pub size: Option<XyValue>,
    #[serde(rename = "ConnectionPointIn")]
    pub connection_point_in: Option<ConnectionPointIn>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Return {
    #[serde(rename = "@globalId")]
    pub global_id: Option<u64>,
    #[serde(rename = "AddData")]
    pub add_data: Option<AddData>,
    #[serde(rename = "RelPosition")]
    pub rel_position: Option<XyValue>,
    #[serde(rename = "Size")]
    pub size: Option<XyValue>,
    #[serde(rename = "ConnectionPointIn")]
    pub connection_point_in: Option<ConnectionPointIn>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct ConnectionPointIn {
    #[serde(rename = "RelPosition")]
    pub rel_position: Option<XyValue>,
    #[serde(rename = "Connection", default)]
    pub connections: Vec<Connection>,
    #[serde(rename = "FeedbackConnection", default)]
    pub feedback_connections: Vec<FeedbackConnection>,
}

impl ConnectionPointIn {
    /// The id of the value wired into this pin, taken from its first incoming connection (if any).
    pub fn referenced_id(&self) -> Option<u64> {
        Some(self.connections.first()?.ref_connection_point_out_id)
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Connection {
    #[serde(rename = "@refConnectionPointOutId")]
    pub ref_connection_point_out_id: u64,
    #[serde(rename = "RelPosition", default)]
    pub waypoints: Vec<XyValue>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct FeedbackConnection {
    #[serde(rename = "@refConnectionPointOutId")]
    pub ref_connection_point_out_id: u64,
    #[serde(rename = "@feedbackVariable")]
    pub feedback_variable: String,
    #[serde(rename = "RelPosition", default)]
    pub waypoints: Vec<XyValue>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct ConnectionPointOut {
    #[serde(rename = "@connectionPointOutId")]
    pub id: u64,
    #[serde(rename = "RelPosition")]
    pub rel_position: Option<XyValue>,
}

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub struct XyValue {
    #[serde(rename = "@x")]
    pub x: f64,
    #[serde(rename = "@y")]
    pub y: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EdgeModifier {
    #[default]
    None,
    Falling,
    Rising,
}

impl Pou {
    #[allow(dead_code)]
    pub fn name(&self) -> &str {
        match self {
            Pou::Program(program) => &program.name,
            Pou::FunctionBlock(function_block) => &function_block.name,
            Pou::Function(function) => &function.name,
        }
    }

    pub fn text_declaration(&self) -> Option<&str> {
        let add_data = match self {
            Pou::Program(program) => &program.add_data,
            Pou::FunctionBlock(function_block) => &function_block.add_data,
            Pou::Function(function) => &function.add_data,
        };

        add_data
            .as_ref()?
            .data
            .iter()
            .find_map(|data| Some(data.text_declaration.as_ref()?.content.text.as_str()))
    }

    pub fn get_network(&self) -> Option<&FbdNetwork> {
        let main_body = match self {
            Pou::Program(program) => &program.main_body,
            Pou::FunctionBlock(function_block) => &function_block.main_body,
            Pou::Function(function) => &function.main_body,
        };

        let BodyContent::Fbd(fbd) = main_body.as_ref()?.body_content.first()?;
        fbd.networks.first()
    }
}

impl AddData {
    fn priority(&self) -> Option<u64> {
        self.data.iter().find_map(|data| data.evaluation_priority?.priority_in_network)
    }

    fn negated(&self) -> Option<bool> {
        self.data.iter().find_map(|data| Some(data.negated?.value))
    }
}

impl Block {
    pub fn get_priority(&self) -> Option<u64> {
        self.add_data.as_ref()?.priority()
    }

    pub fn get_input_variables(&self) -> Vec<&InputVariable> {
        let Some(block) = &self.input_variables else {
            return Vec::new();
        };

        block.variables.iter().collect()
    }

    pub fn get_inout_variables(&self) -> Vec<&InOutVariable> {
        let Some(block) = &self.in_out_variables else {
            return Vec::new();
        };

        block.variables.iter().collect()
    }

    pub fn get_output_variables(&self) -> Vec<&OutputVariable> {
        let Some(block) = &self.output_variables else {
            return Vec::new();
        };

        block.variables.iter().collect()
    }
}

impl DataSink {
    pub fn get_priority(&self) -> Option<u64> {
        self.add_data.as_ref()?.priority()
    }

    pub fn get_referenced_argument_id(&self) -> Option<u64> {
        self.connection_point_in.as_ref()?.referenced_id()
    }
}

impl Return {
    pub fn get_priority(&self) -> Option<u64> {
        self.add_data.as_ref()?.priority()
    }

    pub fn is_negated(&self) -> bool {
        self.add_data.as_ref().and_then(AddData::negated).unwrap_or(false)
    }

    pub fn get_condition_id(&self) -> Option<u64> {
        self.connection_point_in.as_ref()?.referenced_id()
    }
}

impl Connector {
    pub fn get_referenced_argument_id(&self) -> Option<u64> {
        self.connection_point_in.as_ref()?.referenced_id()
    }
}

impl Continuation {
    pub fn get_connection_point_out_id(&self) -> Option<u64> {
        Some(self.connection_point_out.as_ref()?.id)
    }
}

impl InputVariable {
    pub fn get_referenced_argument_id(&self) -> Option<u64> {
        self.connection_point_in.as_ref()?.referenced_id()
    }
}

impl InOutVariable {
    pub fn get_referenced_argument_id(&self) -> Option<u64> {
        self.connection_point_in.as_ref()?.referenced_id()
    }
}

impl DataSource {
    pub fn get_connection_point_out_id(&self) -> Option<u64> {
        Some(self.connection_point_out.as_ref()?.id)
    }
}

impl OutputVariable {
    pub fn get_connection_point_out_id(&self) -> u64 {
        self.connection_point_out.id
    }
}

#[cfg(test)]
mod tests {
    use crate::model::Block;

    #[test]
    fn block_priority() {
        let xml = r#"
            <FbdObject xsi:type="Block" globalId="3" typeName="myAdd">
                <AddData>
                    <Data name="www.iec.ch/61131-10/EvaluationPriority" handleUnknown="discard">
                        <EvaluationPriority priorityInNetwork="2"/>
                    </Data>
                </AddData>
            </FbdObject>
        "#;
        let block: Block = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(block.get_priority(), Some(2));

        let xml = r#"<FbdObject xsi:type="Block" globalId="3" typeName="myAdd"/>"#;
        let block: Block = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(block.get_priority(), None);
    }

    #[test]
    fn deserializes_ide_export() {
        let xml = include_str!("../fixtures/function_call/mainProgram.cfc");
        let pou = crate::model::from_str(xml).unwrap();

        assert_eq!(pou.name(), "mainProgram");
        assert_eq!(pou.get_network().unwrap().objects.len(), 4);
    }
}
