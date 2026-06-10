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
//! - base-type content we have no use for yet (`globalId`, `Documentation`,
//!   per-object `AddData`) is intentionally not modeled,
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
/// Like the quick-xml original, this assumes the tag is the element's first
/// attribute and reuses quick-xml's exported `deserialize_match!` internals.
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
                        let entry: Option<(String, String)> = map.next_entry()?;
                        let tag: String = match entry {
                            None => Err(A::Error::missing_field($tag)),
                            Some((attribute, value)) => {
                                if attribute == $tag {
                                    // the value is a QName: match its local part
                                    Ok(value.rsplit(':').next().unwrap_or(&value).to_string())
                                } else {
                                    Err(A::Error::unknown_field(&attribute, &[$tag]))
                                }
                            }
                        }?;

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

impl Pou {
    pub fn name(&self) -> &str {
        match self {
            Pou::Program(program) => &program.name,
            Pou::FunctionBlock(function_block) => &function_block.name,
            Pou::Function(function) => &function.name,
        }
    }

    pub fn add_data(&self) -> Option<&AddData> {
        match self {
            Pou::Program(program) => program.add_data.as_ref(),
            Pou::FunctionBlock(function_block) => function_block.add_data.as_ref(),
            Pou::Function(function) => function.add_data.as_ref(),
        }
    }

    pub fn main_body(&self) -> Option<&MainBody> {
        match self {
            Pou::Program(program) => program.main_body.as_ref(),
            Pou::FunctionBlock(function_block) => function_block.main_body.as_ref(),
            Pou::Function(function) => function.main_body.as_ref(),
        }
    }
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
    pub networks: Vec<Network>,
}

/// XSD `NetworkBase` dispatched via `xsi:type`.
#[derive(Debug, Clone, PartialEq)]
pub enum Network {
    Fbd(FbdNetwork),
}

impl_deserialize_for_xsi_type_enum! {
    Network, "@type",
    ("FbdNetwork" => Fbd(FbdNetwork)),
}

/// XSD `FbdNetwork` (§13.1): an unordered mix of `CommonObject` and
/// `FbdObject` children. Their relative interleaving is not preserved
/// (it carries no meaning — wiring is by connection point id).
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct FbdNetwork {
    #[serde(rename = "@label")]
    pub label: Option<String>,
    #[serde(rename = "@evaluationOrder")]
    pub evaluation_order: u64,
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
    #[serde(rename = "@typeName")]
    pub type_name: String,
    #[serde(rename = "@instanceName")]
    pub instance_name: Option<String>,
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
    #[serde(rename = "@identifier")]
    pub identifier: String,
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

#[cfg(test)]
mod tests {
    use super::*;

    const SIMPLE_FUNCTION_CALL: &str = include_str!("../fixtures/simple_function_call/myMain.cfc");

    #[test]
    fn deserializes_simple_function_call_fixture() {
        let Pou::Program(program) = from_str(SIMPLE_FUNCTION_CALL).unwrap() else {
            panic!("expected a Program root");
        };

        assert_eq!(program.name, "myMain");

        let data = &program.add_data.as_ref().unwrap().data[0];
        assert_eq!(data.name, "www.bachmann.at/plc/plcopenxml");
        assert_eq!(data.handle_unknown, "implementation");
        let declaration = &data.text_declaration.as_ref().unwrap().content.text;
        assert!(declaration.contains("localA: INT := 10;"));

        let body = program.main_body.as_ref().unwrap();
        let BodyContent::Fbd(fbd) = &body.body_content[0];
        assert_eq!(fbd.networks.len(), 1);
        let Network::Fbd(network) = &fbd.networks[0];
        assert_eq!(network.evaluation_order, 1);
        assert!(network.common_objects.is_empty());

        // localA -> pin 1, localB -> pin 2, myAdd(x <- 1, y <- 2) -> pin 3 -> localResult
        let [
            FbdObject::DataSource(local_a),
            FbdObject::DataSource(local_b),
            FbdObject::Block(block),
            FbdObject::DataSink(sink),
        ] = &network.objects[..]
        else {
            panic!("unexpected object layout: {:#?}", network.objects);
        };

        assert_eq!(local_a.identifier, "localA");
        assert_eq!(local_a.connection_point_out.as_ref().unwrap().id, 1);
        assert_eq!(local_b.identifier, "localB");
        assert_eq!(local_b.connection_point_out.as_ref().unwrap().id, 2);

        assert_eq!(block.type_name, "myAdd");
        assert_eq!(block.instance_name, None, "function calls have no instance name");

        let inputs = &block.input_variables.as_ref().unwrap().variables;
        let connected_to = |variable: &InputVariable| {
            variable.connection_point_in.as_ref().unwrap().connections[0].ref_connection_point_out_id
        };
        assert_eq!(inputs[0].parameter_name, "x");
        assert_eq!(connected_to(&inputs[0]), 1);
        assert!(!inputs[0].negated);
        assert_eq!(inputs[0].edge, EdgeModifier::None);
        assert_eq!(inputs[1].parameter_name, "y");
        assert_eq!(connected_to(&inputs[1]), 2);

        let outputs = &block.output_variables.as_ref().unwrap().variables;
        assert_eq!(outputs[0].parameter_name, "myAdd", "result pin is named after the function");
        assert_eq!(outputs[0].connection_point_out.as_ref().unwrap().id, 3);

        assert_eq!(sink.identifier, "localResult");
        assert_eq!(sink.connection_point_in.as_ref().unwrap().connections[0].ref_connection_point_out_id, 3);
    }

    /// The same document with the IEC namespace bound to an explicit prefix
    /// (and the vendor extension in its own namespace) must produce the same
    /// model: element names match on their local part, and `xsi:type` QName
    /// values (`ppx:Block`) are matched on their local part by our macro.
    #[test]
    fn namespace_prefixes_do_not_affect_the_model() {
        let prefixed = r#"<?xml version="1.0" encoding="UTF-8"?>
            <ppx:Program xmlns:ppx="www.iec.ch/public/TC65SC65BWG7TF10"
                         xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
                         xmlns:bmc="http://www.bachmann.at/plc/cfc"
                         name="myMain">
                <ppx:AddData>
                    <ppx:Data name="www.bachmann.at/plc/plcopenxml" handleUnknown="implementation">
                        <bmc:textDeclaration>
                            <bmc:content>PROGRAM myMain END_PROGRAM</bmc:content>
                        </bmc:textDeclaration>
                    </ppx:Data>
                </ppx:AddData>
                <ppx:MainBody>
                    <ppx:BodyContent xsi:type="ppx:FBD">
                        <ppx:Network xsi:type="ppx:FbdNetwork" evaluationOrder="1">
                            <ppx:FbdObject xsi:type="ppx:DataSource" identifier="localA">
                                <ppx:ConnectionPointOut connectionPointOutId="1"/>
                            </ppx:FbdObject>
                            <ppx:FbdObject xsi:type="ppx:Block" typeName="myAdd">
                                <ppx:InputVariables>
                                    <ppx:InputVariable parameterName="x">
                                        <ppx:ConnectionPointIn>
                                            <ppx:Connection refConnectionPointOutId="1"/>
                                        </ppx:ConnectionPointIn>
                                    </ppx:InputVariable>
                                </ppx:InputVariables>
                                <ppx:OutputVariables>
                                    <ppx:OutputVariable parameterName="myAdd">
                                        <ppx:ConnectionPointOut connectionPointOutId="2"/>
                                    </ppx:OutputVariable>
                                </ppx:OutputVariables>
                            </ppx:FbdObject>
                            <ppx:FbdObject xsi:type="ppx:DataSink" identifier="localResult">
                                <ppx:ConnectionPointIn>
                                    <ppx:Connection refConnectionPointOutId="2"/>
                                </ppx:ConnectionPointIn>
                            </ppx:FbdObject>
                        </ppx:Network>
                    </ppx:BodyContent>
                </ppx:MainBody>
            </ppx:Program>"#;

        let unprefixed = prefixed
            .replace("ppx:", "")
            .replace("bmc:", "")
            .replace("xmlns=\"http://www.bachmann.at/plc/cfc\"", "")
            .replace("xmlns:", "xmlns_ignored:"); // keep declarations harmless but distinct

        let from_prefixed = from_str(prefixed).unwrap();
        let from_unprefixed = from_str(&unprefixed).unwrap();
        assert_eq!(from_prefixed, from_unprefixed);
        assert!(matches!(from_prefixed, Pou::Program(_)));

        let BodyContent::Fbd(fbd) = &from_prefixed.main_body().unwrap().body_content[0];
        let Network::Fbd(network) = &fbd.networks[0];
        assert!(matches!(network.objects[1], FbdObject::Block(_)));
    }

    /// `CommonObject`s and `FbdObject`s may interleave freely within a
    /// network (this is what the `overlapped-lists` feature is for), and
    /// Connector/Continuation carry named cross-links.
    #[test]
    fn interleaved_common_and_fbd_objects() {
        let xml = r#"
            <Program name="prog">
                <MainBody>
                    <BodyContent xsi:type="FBD" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
                        <Network xsi:type="FbdNetwork" evaluationOrder="1">
                            <FbdObject xsi:type="DataSource" identifier="a">
                                <ConnectionPointOut connectionPointOutId="1"/>
                            </FbdObject>
                            <CommonObject xsi:type="Comment">
                                <Content xsi:type="SimpleText">to be continued</Content>
                            </CommonObject>
                            <CommonObject xsi:type="Connector" label="LINK">
                                <ConnectionPointIn>
                                    <Connection refConnectionPointOutId="1"/>
                                </ConnectionPointIn>
                            </CommonObject>
                            <CommonObject xsi:type="Continuation" label="LINK">
                                <ConnectionPointOut connectionPointOutId="2"/>
                            </CommonObject>
                            <FbdObject xsi:type="DataSink" identifier="b">
                                <ConnectionPointIn>
                                    <Connection refConnectionPointOutId="2"/>
                                </ConnectionPointIn>
                            </FbdObject>
                        </Network>
                    </BodyContent>
                </MainBody>
            </Program>"#;

        let pou = from_str(xml).unwrap();
        let BodyContent::Fbd(fbd) = &pou.main_body().unwrap().body_content[0];
        let Network::Fbd(network) = &fbd.networks[0];

        assert_eq!(network.objects.len(), 2);
        let [
            CommonObject::Comment(comment),
            CommonObject::Connector(connector),
            CommonObject::Continuation(continuation),
        ] = &network.common_objects[..]
        else {
            panic!("unexpected common objects: {:#?}", network.common_objects);
        };

        assert_eq!(comment.content.text, "to be continued");
        assert_eq!(connector.label, "LINK");
        assert_eq!(
            connector.connection_point_in.as_ref().unwrap().connections[0].ref_connection_point_out_id,
            1
        );
        assert_eq!(continuation.label, "LINK");
        assert_eq!(continuation.connection_point_out.as_ref().unwrap().id, 2);
    }

    /// Feedback loops use `FeedbackConnection` instead of `Connection`.
    #[test]
    fn feedback_connection() {
        let xml = r#"
            <Program name="prog">
                <MainBody>
                    <BodyContent xsi:type="FBD" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
                        <Network xsi:type="FbdNetwork" evaluationOrder="1">
                            <FbdObject xsi:type="Block" typeName="myAdd">
                                <InputVariables>
                                    <InputVariable parameterName="x" negated="true" edge="rising">
                                        <ConnectionPointIn>
                                            <FeedbackConnection refConnectionPointOutId="1" feedbackVariable="loop"/>
                                        </ConnectionPointIn>
                                    </InputVariable>
                                </InputVariables>
                                <OutputVariables>
                                    <OutputVariable parameterName="myAdd">
                                        <ConnectionPointOut connectionPointOutId="1"/>
                                    </OutputVariable>
                                </OutputVariables>
                            </FbdObject>
                        </Network>
                    </BodyContent>
                </MainBody>
            </Program>"#;

        let pou = from_str(xml).unwrap();
        let BodyContent::Fbd(fbd) = &pou.main_body().unwrap().body_content[0];
        let Network::Fbd(network) = &fbd.networks[0];
        let FbdObject::Block(block) = &network.objects[0] else {
            panic!("expected a block");
        };

        let input = &block.input_variables.as_ref().unwrap().variables[0];
        assert!(input.negated);
        assert_eq!(input.edge, EdgeModifier::Rising);
        let feedback = &input.connection_point_in.as_ref().unwrap().feedback_connections[0];
        assert_eq!(feedback.ref_connection_point_out_id, 1);
        assert_eq!(feedback.feedback_variable, "loop");
    }

    /// The POU kind is carried by the root element name (XSD `PouDecl`
    /// group), with kind-specific content per the standard.
    #[test]
    fn dispatches_pou_kind_on_root_element_name() {
        let Pou::Program(_) = from_str(r#"<Program name="myMain"/>"#).unwrap() else {
            panic!("expected a Program root");
        };

        let function_block = r#"
            <FunctionBlock name="myFb" abstract="true">
                <Extends><TypeName>baseFb</TypeName></Extends>
                <Implements><TypeName>interfaceA</TypeName></Implements>
                <Implements><TypeName>interfaceB</TypeName></Implements>
            </FunctionBlock>"#;
        let Pou::FunctionBlock(fb) = from_str(function_block).unwrap() else {
            panic!("expected a FunctionBlock root");
        };
        assert_eq!(fb.name, "myFb");
        assert!(fb.is_abstract);
        assert!(!fb.is_final);
        assert_eq!(fb.extends.as_ref().unwrap().type_name.as_deref(), Some("baseFb"));
        assert_eq!(fb.implements.len(), 2);
        assert_eq!(fb.implements[1].type_name.as_deref(), Some("interfaceB"));

        let function = r#"
            <Function name="myAdd">
                <ResultType><TypeName>INT</TypeName></ResultType>
            </Function>"#;
        let Pou::Function(function) = from_str(function).unwrap() else {
            panic!("expected a Function root");
        };
        assert_eq!(function.name, "myAdd");
        assert_eq!(function.result_type.as_ref().unwrap().type_name.as_deref(), Some("INT"));
    }
}
