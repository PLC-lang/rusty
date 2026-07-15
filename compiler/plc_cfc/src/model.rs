//! Deserialization model for CFC (`.cfc`) files: a PLCopen-style XML document
//! describing a single POU whose body is an FBD/CFC network.
//!
//! `quick-xml`'s serde support strips namespace prefixes and matches on the
//! local name, so the `rename`s use bare names (`Function`, `type`) rather than
//! their `ppx:` / `xsi:` prefixed forms, and attributes take a leading `@`.

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
    /// The POU's header and variable blocks, carried verbatim as ST source.
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
    #[serde(rename = "FbdObject", default)]
    pub objects: Vec<FbdObject>,
}

#[derive(Debug, Deserialize)]
pub struct FbdObject {
    /// The `xsi:type` discriminator, e.g. `"ppx:DataSource"` (prefix retained
    /// because only names, not values, are stripped). Classified in `resolve`.
    #[serde(rename = "@type")]
    pub kind: String,

    #[serde(rename = "@globalId")]
    pub global_id: usize,

    #[serde(rename = "@identifier")]
    pub identifier: Option<String>,

    #[serde(rename = "@complexIdentifier")]
    pub complex_identifier: Option<String>,

    #[serde(rename = "AddData")]
    pub add_data: Option<AddData>,

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
        self.add_data
            .data
            .iter()
            .find_map(|data| data.text_declaration.as_ref())
            .map(|it| it.content.as_str())
    }
}

impl FbdObject {
    pub fn identifier(&self) -> Option<&str> {
        self.identifier.as_deref().or(self.complex_identifier.as_deref())
    }

    pub fn priority(&self) -> Option<usize> {
        let add_data = self.add_data.as_ref()?;
        add_data.data.iter().find_map(|data| data.evaluation_priority.as_ref()).map(|it| it.priority)
    }

    pub fn negated(&self) -> bool {
        self.add_data
            .as_ref()
            .and_then(|add_data| add_data.data.iter().find_map(|data| data.negated.as_ref()))
            .is_some_and(|negated| negated.value)
    }
}
