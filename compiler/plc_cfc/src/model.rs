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
    #[serde(rename = "$value", default)]
    pub nodes: Vec<NetworkElement>,
}

/// Graphical elements share a structure but arrive under two element names,
/// interleaved in document order; a `$value` sequence captures both.
#[derive(Debug, Deserialize)]
pub enum NetworkElement {
    FbdObject(FbdObject),
    CommonObject(FbdObject),
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

    /// The pairing name of a connector or continuation.
    #[serde(rename = "@label")]
    pub label: Option<String>,

    /// The called POU's name; present on a `ppx:Block`.
    #[serde(rename = "@typeName")]
    pub type_name: Option<String>,

    /// The caller-declared instance a function-block call runs on; absent for a
    /// program call, which is a singleton reached by its bare type name.
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

/// A block's parameter pins of one kind (input, output, or in_out). All three
/// groups share a shape; only the child element name differs, and quick-xml
/// matches `$value` on that local name.
#[derive(Debug, Default, Deserialize)]
pub struct PinGroup {
    #[serde(rename = "$value", default)]
    pub pins: Vec<Pin>,
}

/// One block parameter pin. An input/in_out pin carries an incoming wire; an
/// output pin exposes a producer pin. `negated` is the pin's inversion bubble.
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
        self.add_data
            .data
            .iter()
            .find_map(|data| data.text_declaration.as_ref())
            .map(|it| it.content.as_str())
    }
}

impl Network {
    /// Every graphical element, regardless of its `FbdObject` / `CommonObject`
    /// XML representation.
    pub fn elements(&self) -> impl Iterator<Item = &FbdObject> {
        self.nodes.iter().map(|node| match node {
            NetworkElement::FbdObject(object) | NetworkElement::CommonObject(object) => object,
        })
    }
}

impl FbdObject {
    /// A plain variable arrives in `identifier`; an indexed or qualified one
    /// (`arr[i]`, `foo.bar`) arrives in `complexIdentifier` instead.
    pub fn identifier(&self) -> Option<&str> {
        self.identifier.as_deref().or(self.complex_identifier.as_deref())
    }

    pub fn label(&self) -> Option<&str> {
        self.label.as_deref()
    }

    pub fn type_name(&self) -> Option<&str> {
        self.type_name.as_deref()
    }

    /// The reference a block's outputs are *read* through: its declared instance
    /// (function block), falling back to the owner POU (a program singleton, or
    /// an action's owner — outputs live on the instance, not the action).
    pub fn instance(&self) -> Option<&str> {
        self.instance_name.as_deref().or_else(|| self.owner())
    }

    /// The output pin carrying a function's return value: the pin named after the
    /// callee. Only a non-void function exposes one.
    // TODO: A void function has no return pin and is currently indistinguishable
    //       from a program here; telling them apart needs the callee's kind,
    //       which the untyped model doesn't carry.
    pub fn return_pin(&self) -> Option<&Pin> {
        let type_name = self.type_name()?;
        self.output_pins().iter().find(|pin| pin.parameter_name == type_name)
    }

    /// A stateless function call: no caller instance and a return pin present. Its
    /// outputs don't persist, so they are read through generated temporaries
    /// rather than `instance.member`.
    pub fn is_function(&self) -> bool {
        self.instance_name.is_none() && self.return_pin().is_some()
    }

    /// Whether `pin` is this block's return pin (named after the callee).
    pub fn is_return_pin(&self, pin: &Pin) -> bool {
        self.type_name() == Some(pin.parameter_name.as_str())
    }

    /// The reference a block is *called* through: the instance, suffixed with the
    /// action when `typeName` is qualified (`inst.act`); otherwise the instance.
    pub fn call_target(&self) -> Option<String> {
        let instance = self.instance()?;
        Some(match self.action() {
            Some(action) => format!("{instance}.{action}"),
            None => instance.to_string(),
        })
    }

    /// The owner portion of `typeName`: everything before an action suffix
    /// (`MyFb.act` → `MyFb`), or the whole name when unqualified.
    fn owner(&self) -> Option<&str> {
        self.type_name.as_deref().map(|name| name.rsplit_once('.').map_or(name, |(owner, _)| owner))
    }

    /// The action a qualified `typeName` (`owner.action`) dispatches to, if any.
    fn action(&self) -> Option<&str> {
        self.type_name.as_deref().and_then(|name| name.rsplit_once('.').map(|(_, action)| action))
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

impl Pin {
    /// The producer pin feeding an input/in_out pin's incoming wire.
    pub fn source_pin(&self) -> Option<usize> {
        self.connection_in.as_ref()?.connections.first().map(|connection| connection.ref_out_id)
    }

    /// The producer pin an output pin exposes to downstream consumers.
    pub fn output_pin(&self) -> Option<usize> {
        self.connection_out.as_ref().map(|out| out.id)
    }
}
