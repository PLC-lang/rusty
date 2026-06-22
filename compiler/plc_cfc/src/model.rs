//! Typed mirror of the FBD subset of the IEC 61131-10 XML exchange format,
//! as defined by `assets/IEC61131/cfc/examples/IEC61131_10_Ed1_0.xsd`.
//!
//! This module is a pure deserialization layer: structs map 1:1 onto the
//! schema's complex types and carry no compiler logic. Lowering to the AST
//! happens elsewhere; XML types must not leak past this module.
//!
//! The model is deliberately lenient:
//! - everything the XSD marks `minOccurs="0"` / `use="optional"` is an
//!   `Option` (or a defaulted `Vec`/`bool`), in particular all graphical data
//!   (`RelPosition`, `Size`), which our fixtures omit entirely,
//! - unknown elements and attributes are ignored, so additive exporter
//!   changes do not break parsing,
//! - base-type content we have no use for yet (`Documentation`, per-object
//!   `AddData`) is intentionally not modeled,
//! - `globalId` (carried by every graphical object) deviates from the
//!   schema's `xsd:ID`: the IDE emits plain integers, so it is a `u64`. It
//!   later keys synthetic source locations (`CodeSpan::Block`) in lowering,
//! - non-FBD body languages (IL, ST, LD, SFC) and the SFC-oriented
//!   `ActionBlocks` common object are out of scope for CFC.

use serde::Deserialize;

/// Implements [`serde::Deserialize`] for an `xsi:type`-tagged enum.
///
/// Variant of quick-xml's `impl_deserialize_for_internally_tagged_enum!`
/// (which cannot be used directly because `xsi:type` values are QNames such
/// as `ppx:Block`): the tag value is matched on its *local part*, making
/// dispatch independent of the document's namespace prefix choice. Note that
/// quick-xml itself already strips prefixes from element/attribute *names*,
/// which is why the tag attribute is `@type`, not `@xsi:type`.
///
/// Like the quick-xml original this reuses its exported `deserialize_match!`
/// internals and expects the tag up front, but tolerates leading namespace
/// declarations (`xmlns:*`): the IDE emits `xmlns:xsi` ahead of `xsi:type` on
/// the body element, so we skip those until we reach the tag.
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
                                    // the value is a QName: match its local part
                                    break value.rsplit(':').next().unwrap_or(&value).to_string();
                                }
                                // a namespace declaration preceding the tag
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

/// Deserializes a `.cfc` document. The root element is one of the POU kinds
/// of the XSD `PouDecl` group (see [`Pou`] and `CLAUDE.md`).
pub fn from_str(xml: &str) -> Result<Pou, quick_xml::DeError> {
    quick_xml::de::from_str(xml)
}

/// XSD `PouDecl` group (§10.1): the root element of a `.cfc` file. The POU
/// kind is encoded in the *element name* (unlike TC6's `pouType` attribute),
/// so this enum dispatches on the root tag. `Class` and `Interface` are not
/// supported: they carry no `MainBody` of their own, only method bodies,
/// which are deferred until the IDE's export shape for methods is settled.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub enum Pou {
    Program(Program),
    FunctionBlock(FunctionBlock),
    Function(Function),
}

/// XSD `Program` (§10.2). The structured variable sections (`GlobalVars`,
/// `AccessVars`, `Vars`, ...) are not modeled — by convention the interface
/// is carried as ST text in the `AddData` text declaration. `Action` and
/// `Transition` children are deferred.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Program {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "AddData")]
    pub add_data: Option<AddData>,
    #[serde(rename = "MainBody")]
    pub main_body: Option<MainBody>,
}

/// XSD `FunctionBlock` (§10.3). Structured variable sections are not modeled
/// (see [`Program`]); `Method`, `Action` and `Transition` children are
/// deferred.
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

/// XSD `Function` (§10.5). Its `MainBody` is `BodyWithoutSFC` in the schema,
/// which makes no difference here since we only model FBD anyway. Parameters
/// and temp variables are carried by the text declaration (see [`Program`]).
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

/// XSD `TypeRef` (§11.5): a reference to a data type by name. The
/// `InstantlyDefinedType` alternative (inline array/reference type
/// specifications) is not modeled.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct TypeRef {
    #[serde(rename = "TypeName")]
    pub type_name: Option<String>,
}

/// XSD `AddData` (§15.2) — the vendor extension point.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct AddData {
    #[serde(rename = "Data", default)]
    pub data: Vec<Data>,
}

/// One `Data` entry inside `AddData`. The schema allows arbitrary content
/// (`xsd:any`); we only model the Bachmann text declaration we rely on.
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

/// Vendor extension (`AddData_EvaluationPriority.xsd`): explicit evaluation
/// order among the blocks of one network. Smaller = earlier, unique per
/// network, and only blocks carrying a priority are affected — everything
/// else falls back to document order.
#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub struct EvaluationPriority {
    #[serde(rename = "@priorityInNetwork")]
    pub priority_in_network: Option<u64>,
}

/// Vendor extension: whether a control object's condition wire is negated. A negated `Return`
/// triggers when its wired condition is *false* (the condition is wrapped in `NOT`).
#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub struct Negated {
    #[serde(rename = "@value")]
    pub value: bool,
}

/// Vendor extension: the POU interface as ST source text.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct TextDeclaration {
    pub content: TextContent,
}

/// Element whose character data is the payload (also used for `Comment`
/// content, where the XSD type is `SimpleText`).
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct TextContent {
    #[serde(rename = "$text", default)]
    pub text: String,
}

/// XSD `Body` (§10.14): a sequence of language-specific `BodyContent`s.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct MainBody {
    #[serde(rename = "BodyContent", default)]
    pub body_content: Vec<BodyContent>,
}

/// XSD `BehaviourRepresentationBase` dispatched via `xsi:type` (§12).
/// Only FBD is meaningful for CFC; other languages are parse errors for now.
#[derive(Debug, Clone, PartialEq)]
pub enum BodyContent {
    Fbd(FbdBody),
}

impl_deserialize_for_xsi_type_enum! {
    BodyContent, "@type",
    ("FBD" => Fbd(FbdBody)),
}

/// XSD `FBD` (§12.3): the body is a list of networks. CFC semantics require
/// exactly one network per body — enforce that during lowering (with a
/// proper diagnostic), not here.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct FbdBody {
    #[serde(rename = "Network", default)]
    pub networks: Vec<FbdNetwork>,
}

/// XSD `FbdNetwork` (§13.1): an unordered mix of `CommonObject` and
/// `FbdObject` children. Their relative interleaving is not preserved
/// (it carries no meaning — wiring is by connection point id).
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

/// XSD `CommonObjectBase` dispatched via `xsi:type` (§13.2).
/// `ActionBlocks` (SFC-only) is intentionally unsupported.
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

/// XSD `Comment` (§13.2.1).
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

/// XSD `Connector` (§13.2.2): named sink end of a cross-cutting link.
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

/// XSD `Continuation` (§13.2.3): named source end matching a `Connector`.
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

/// XSD `FbdObjectBase` dispatched via `xsi:type` (§13.3).
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

/// XSD `Block` (§13.3.1): a function, FB instance, or method call.
/// Functions have no `instanceName`; their result pin is named after the
/// function type (`parameterName == typeName`, see `CLAUDE.md`).
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

/// In-out pin of a `Block`; passes through left to right.
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

/// Input pin of a `Block` (§13.3.1). `edge` and `suppressName` only mirror
/// the graphical representation; edge behaviour is defined by the called
/// POU's declaration.
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

/// Output pin of a `Block` (§13.3.1).
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct OutputVariable {
    #[serde(rename = "@parameterName")]
    pub parameter_name: String,
    #[serde(rename = "@negated", default)]
    pub negated: bool,
    #[serde(rename = "@suppressName", default)]
    pub suppress_name: bool,
    #[serde(rename = "ConnectionPointOut")]
    pub connection_point_out: Option<ConnectionPointOut>,
}

/// XSD `DataSource` (§13.3.3): a variable or literal feeding the graph.
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

/// XSD `DataSink` (§13.3.4): assignment of a line's value to a variable.
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

/// XSD `Unconnected` (§13.3.5): an explicitly dangling pin.
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

/// XSD `Jump` (§13.3.6): jump to another network by label.
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

/// XSD `Return` (§13.3.7).
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

/// XSD `ConnectionPointIn` (§13.6.2). The XSD choice between plain and
/// feedback connections is flattened into two lists.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct ConnectionPointIn {
    #[serde(rename = "RelPosition")]
    pub rel_position: Option<XyValue>,
    #[serde(rename = "Connection", default)]
    pub connections: Vec<Connection>,
    #[serde(rename = "FeedbackConnection", default)]
    pub feedback_connections: Vec<FeedbackConnection>,
}

/// XSD `Connection` (§13.6.3): references the producing pin by id. The id is
/// unique per network; extra `RelPosition`s are routing waypoints.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Connection {
    #[serde(rename = "@refConnectionPointOutId")]
    pub ref_connection_point_out_id: u64,
    #[serde(rename = "RelPosition", default)]
    pub waypoints: Vec<XyValue>,
}

/// XSD `FeedbackConnection` (§13.6.4): a `Connection` closing a cycle, with
/// the feedback variable that breaks the evaluation loop.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct FeedbackConnection {
    #[serde(rename = "@refConnectionPointOutId")]
    pub ref_connection_point_out_id: u64,
    #[serde(rename = "@feedbackVariable")]
    pub feedback_variable: String,
    #[serde(rename = "RelPosition", default)]
    pub waypoints: Vec<XyValue>,
}

/// XSD `ConnectionPointOut` (§13.6.5): carries the network-unique pin id all
/// wiring refers to.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct ConnectionPointOut {
    #[serde(rename = "@connectionPointOutId")]
    pub id: u64,
    #[serde(rename = "RelPosition")]
    pub rel_position: Option<XyValue>,
}

/// XSD `XyDecimalValue` (§15.1).
#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub struct XyValue {
    #[serde(rename = "@x")]
    pub x: f64,
    #[serde(rename = "@y")]
    pub y: f64,
}

/// XSD `EdgeModifierType` (§15.5).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EdgeModifier {
    #[default]
    None,
    Falling,
    Rising,
}

impl Pou {
    pub fn name(&self) -> &str {
        match self {
            Pou::Program(program) => &program.name,
            Pou::FunctionBlock(function_block) => &function_block.name,
            Pou::Function(function) => &function.name,
        }
    }

    /// The POU interface as ST source text, carried by the [`TextDeclaration`]
    /// vendor extension in the POU's `AddData`.
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
    /// The evaluation priority carried by the `EvaluationPriority` vendor extension, if present.
    fn priority(&self) -> Option<u64> {
        self.data.iter().find_map(|data| data.evaluation_priority?.priority_in_network)
    }

    /// Whether the `negated` vendor extension is present and set.
    fn negated(&self) -> Option<bool> {
        self.data.iter().find_map(|data| Some(data.negated?.value))
    }
}

impl Block {
    /// The block's evaluation priority within its network, carried by the
    /// [`EvaluationPriority`] vendor extension buried in the block's `AddData`.
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
    /// This sink's evaluation priority within its network (see [`Block::get_priority`]).
    pub fn get_priority(&self) -> Option<u64> {
        self.add_data.as_ref()?.priority()
    }
}

impl Return {
    /// This return's evaluation priority within its network (see [`Block::get_priority`]).
    pub fn get_priority(&self) -> Option<u64> {
        self.add_data.as_ref()?.priority()
    }

    /// Whether the return's condition is negated (see [`Negated`]); defaults to `false`.
    pub fn is_negated(&self) -> bool {
        self.add_data.as_ref().and_then(AddData::negated).unwrap_or(false)
    }

    /// The `connectionPointOutId` of the return's condition wire, or `None` when it is
    /// unconnected — an unconditional return (see [`InputVariable::get_referenced_argument_id`]).
    pub fn get_condition_id(&self) -> Option<u64> {
        self.connection_point_in.as_ref()?.connections.first().map(|c| c.ref_connection_point_out_id)
    }
}

impl InputVariable {
    /// The `connectionPointOutId` feeding this pin, or `None` when the pin is unconnected — either
    /// because it has no `ConnectionPointIn` or because that pin carries no `Connection`. (The IDE
    /// emits an unwired pin as a present-but-empty `ConnectionPointIn`, hence `first()` over `[0]`.)
    pub fn get_referenced_argument_id(&self) -> Option<u64> {
        self.connection_point_in.as_ref()?.connections.first().map(|c| c.ref_connection_point_out_id)
    }
}

impl InOutVariable {
    /// The `connectionPointOutId` feeding this pin, or `None` when unconnected
    /// (see [`InputVariable::get_referenced_argument_id`]).
    pub fn get_referenced_argument_id(&self) -> Option<u64> {
        self.connection_point_in.as_ref()?.connections.first().map(|c| c.ref_connection_point_out_id)
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
        // Exercises the two quirks of the IDE's FBD export: a bare `<Network>`
        // (no `xsi:type`) and `xmlns:xsi` declared ahead of `xsi:type` on the
        // body element.
        let xml = include_str!("../fixtures/function_call/mainProgram.cfc");
        let pou = crate::model::from_str(xml).unwrap();

        assert_eq!(pou.name(), "mainProgram");
        // 2 data sources, 1 data sink, 1 block
        assert_eq!(pou.get_network().unwrap().objects.len(), 4);
    }
}
