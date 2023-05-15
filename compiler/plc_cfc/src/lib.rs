use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Pou {
    #[serde(rename = "@name")]
    name: String,

    #[serde(rename = "@pouType")]
    pou_type: PouType, // TODO: enum

    #[serde(rename = "@returnType")]
    return_type: Option<String>,

    body: Body,

    // TODO: Still missing: external, access, globals
    // globalVars: Option<Vec<Variable>>,
    #[serde(rename = "localVariables")]
    local_vars: Option<Vec<Variable>>,

    #[serde(rename = "tempVariables")]
    temp_vars: Option<Vec<Variable>>,

    #[serde(rename = "inputVariables")]
    input_vars: Option<Vec<Variable>>,

    #[serde(rename = "outputVariables")]
    output_vars: Option<Vec<Variable>>,

    #[serde(rename = "inOutVariables")]
    inout_vars: Option<Vec<Variable>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
enum PouType {
    Program,
    Function,
    FunctionBlock,
}

#[derive(Debug, Deserialize)]
struct Variable {
    #[serde(rename = "@formalParameter")]
    formal_parameter: String,

    #[serde(rename = "@negated")]
    negated: bool,
}

#[derive(Debug, Deserialize)]
pub struct Body {
    #[serde(rename = "FBD")]
    fbd: FunctionBlockDiagram,
}

#[derive(Debug, Deserialize)]
pub struct FunctionBlockDiagram {
    #[serde(rename = "$value")]
    members: Vec<FBDMembers>,
}

// https://stackoverflow.com/a/53366978: Duplicate field error
// Maybe we have to write our own deserializer (https://serde.rs/impl-deserializer.html)
#[derive(Debug, Deserialize)]
enum FBDMembers {
    #[serde(rename = "block")]
    Block {
        #[serde(rename = "@localId")]
        local_id: String,

        #[serde(rename = "@executionOrderId")]
        execution_order_id: String,
    },

    #[serde(rename = "inVariable")]
    InVariable {
        #[serde(rename = "@localId")]
        local_id: String,
    },

    #[serde(rename = "outVariable")]
    OutVariable,
}

#[derive(Debug, Deserialize)]
pub struct InVariable {
    #[serde(rename = "@localId")]
    local_id: String,
}

#[derive(Debug, Deserialize)]
pub struct OutVariable {}

fn parse() -> Result<(), String> {
    let content = std::fs::read_to_string("demo.xml").unwrap();
    let root: Pou = quick_xml::de::from_str(&content).unwrap();
    println!("{root:#?}");

    Ok(())
}

// TODO: Move into folder
mod parse {
    #[test]
    fn xml() {
        let content = crate::parse().unwrap();
        println!("{content:#?}");
    }
}
