// quick-xml's serde support strips namespace prefixes and matches on the local
// name, so the `rename`s use bare names (`Function`, `type`) rather than their
// `ppx:` / `xsi:` prefixed forms, and attributes take a leading `@`.

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum Pou {
    #[serde(rename = "Function")]
    Function(PouContent),

    #[serde(rename = "FunctionBlock")]
    FunctionBlock(PouContent),

    #[serde(rename = "Program")]
    Program(PouContent),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PouKind {
    Function,
    FunctionBlock,
    Program,
}

#[derive(Debug, Deserialize)]
pub struct PouContent {
    #[serde(rename = "@name")]
    pub name: String,

    #[serde(rename = "AddData")]
    pub add_data: AddData,

    #[serde(rename = "MainBody")]
    pub main_body: MainBody,
}

#[derive(Debug, Default, Deserialize)]
pub struct AddData {
    #[serde(rename = "Data", default)]
    pub data: Vec<Data>,
}

#[derive(Debug, Deserialize)]
pub struct Data {
    #[serde(rename = "@name")]
    pub name: String,

    #[serde(rename = "@handleUnknown")]
    pub handle_unknown: Option<String>,

    #[serde(rename = "textDeclaration")]
    pub text_declaration: Option<TextDeclaration>,

    #[serde(rename = "EvaluationPriority")]
    pub evaluation_priority: Option<EvaluationPriority>,

    #[serde(rename = "negated")]
    pub negated: Option<Negated>,
}

#[derive(Debug, Deserialize)]
pub struct TextDeclaration {
    #[serde(rename = "content")]
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct EvaluationPriority {
    #[serde(rename = "@priorityInNetwork")]
    pub priority: usize,
}

#[derive(Debug, Deserialize)]
pub struct Negated {
    #[serde(rename = "@value")]
    pub value: bool,
}

#[derive(Debug, Deserialize)]
pub struct MainBody {
    #[serde(rename = "BodyContent")]
    pub body_content: BodyContent,
}

#[derive(Debug, Deserialize)]
pub struct BodyContent {
    #[serde(rename = "@type")]
    pub kind: Option<String>,

    #[serde(rename = "Network")]
    pub network: Network,
}

#[derive(Debug, Default, Deserialize)]
pub struct Network {
    #[serde(rename = "$value", default)]
    pub nodes: Vec<NetworkElement>,
}

// The two element names interleave; only a `$value` sequence captures both.
#[derive(Debug, Deserialize)]
pub enum NetworkElement {
    FbdObject(FbdObject),
    CommonObject(FbdObject),
}

#[derive(Debug, Deserialize)]
pub struct FbdObject {
    // The `xsi:type` discriminator; its prefix survives (values aren't stripped).
    #[serde(rename = "@type")]
    pub kind: String,

    #[serde(rename = "@globalId")]
    pub global_id: usize,

    #[serde(rename = "@identifier")]
    pub identifier: Option<String>,

    #[serde(rename = "@complexIdentifier")]
    pub complex_identifier: Option<String>,

    #[serde(rename = "@label")]
    pub label: Option<String>,

    #[serde(rename = "@targetLabel")]
    pub target_label: Option<String>,

    #[serde(rename = "@typeName")]
    pub type_name: Option<String>,

    #[serde(rename = "@instanceName")]
    pub instance_name: Option<String>,

    #[serde(rename = "AddData")]
    pub add_data: Option<AddData>,

    #[serde(rename = "ConnectionPointIn")]
    pub connection_in: Option<ConnectionPointIn>,

    #[serde(rename = "ConnectionPointOut")]
    pub connection_out: Option<ConnectionPointOut>,

    #[serde(rename = "InputVariables")]
    pub input_variables: Option<PinGroup>,

    #[serde(rename = "OutputVariables")]
    pub output_variables: Option<PinGroup>,

    #[serde(rename = "InOutVariables")]
    pub inout_variables: Option<PinGroup>,
}

// Shared by the input/output/in_out groups; `$value` matches the element name.
#[derive(Debug, Default, Deserialize)]
pub struct PinGroup {
    #[serde(rename = "$value", default)]
    pub pins: Vec<Pin>,
}

#[derive(Debug, Deserialize)]
pub struct Pin {
    #[serde(rename = "@parameterName")]
    pub parameter_name: String,

    #[serde(rename = "@negated", default)]
    pub negated: bool,

    #[serde(rename = "ConnectionPointIn")]
    pub connection_in: Option<ConnectionPointIn>,

    #[serde(rename = "ConnectionPointOut")]
    pub connection_out: Option<ConnectionPointOut>,
}

#[derive(Debug, Deserialize)]
pub struct ConnectionPointIn {
    #[serde(rename = "Connection", default)]
    pub connections: Vec<Connection>,
}

#[derive(Debug, Deserialize)]
pub struct ConnectionPointOut {
    #[serde(rename = "@connectionPointOutId")]
    pub id: usize,
}

#[derive(Debug, Deserialize)]
pub struct Connection {
    #[serde(rename = "@refConnectionPointOutId")]
    pub ref_out_id: usize,
}

impl Pou {
    pub fn parse(content: &str) -> Result<Self, quick_xml::DeError> {
        quick_xml::de::from_str(content)
    }

    pub fn kind(&self) -> PouKind {
        match self {
            Pou::Function(_) => PouKind::Function,
            Pou::FunctionBlock(_) => PouKind::FunctionBlock,
            Pou::Program(_) => PouKind::Program,
        }
    }

    pub fn content(&self) -> &PouContent {
        match self {
            Pou::Function(content) | Pou::FunctionBlock(content) | Pou::Program(content) => content,
        }
    }
}

impl PouContent {
    pub fn network(&self) -> &Network {
        &self.main_body.body_content.network
    }

    pub fn declaration(&self) -> Option<&str> {
        Some(self.add_data.data.first()?.text_declaration.as_ref()?.content.as_str())
    }
}

impl Network {
    pub fn elements(&self) -> impl Iterator<Item = &FbdObject> {
        self.nodes.iter().map(|node| match node {
            NetworkElement::FbdObject(object) | NetworkElement::CommonObject(object) => object,
        })
    }
}

impl FbdObject {
    pub fn identifier(&self) -> Option<&str> {
        self.identifier.as_deref().or(self.complex_identifier.as_deref())
    }

    pub fn label(&self) -> Option<&str> {
        self.label.as_deref()
    }

    pub fn target_label(&self) -> Option<&str> {
        self.target_label.as_deref()
    }

    pub fn type_name(&self) -> Option<&str> {
        self.type_name.as_deref()
    }

    pub fn input_pins(&self) -> &[Pin] {
        self.input_variables.as_ref().map_or(&[], |group| &group.pins)
    }

    pub fn output_pins(&self) -> &[Pin] {
        self.output_variables.as_ref().map_or(&[], |group| &group.pins)
    }

    pub fn inout_pins(&self) -> &[Pin] {
        self.inout_variables.as_ref().map_or(&[], |group| &group.pins)
    }

    pub fn negated(&self) -> bool {
        self.data(|data| data.negated.as_ref()).is_some_and(|negated| negated.value)
    }

    pub fn priority(&self) -> Option<usize> {
        self.data(|data| data.evaluation_priority.as_ref()).map(|priority| priority.priority)
    }

    // `AddData` interleaves its payloads across several `Data` entries.
    fn data<'a, T>(&'a self, pick: impl Fn(&'a Data) -> Option<&'a T>) -> Option<&'a T> {
        self.add_data.as_ref()?.data.iter().find_map(pick)
    }
}

impl Pin {
    pub fn source_pin(&self) -> Option<usize> {
        self.connection_in.as_ref()?.connections.first().map(|connection| connection.ref_out_id)
    }

    pub fn output_pin(&self) -> Option<usize> {
        self.connection_out.as_ref().map(|out| out.id)
    }
}
