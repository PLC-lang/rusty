use serde::Deserialize;

// mod json;
/*
interface: local vars (varList)

varList: name, constant, retrain, nonretain, persistent, nonpersistent
varListPlain...

Block {
    position (not needed)
    inputVariables {
        variable {
            connectionPointIn
            formalParameter (requried)
            negated,
            edgeModifierType (do we need this)
            storageModifierType (^)
        }
    }

    InoutVariables {
        connectionPointIn {
            globalId
        }
        connectionPointOut
    }
*/

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
enum PouType {
    Program,
    Function,
    FunctionBlock,
}

#[derive(Debug, Deserialize)]
struct Pou {
    #[serde(rename = "@name")]
    name: String,

    #[serde(rename = "@pouType")]
    pou_type: PouType, // TODO: enum

    #[serde(rename = "@returnType")]
    return_type: Option<String>,

    body: Body,
}

#[derive(Debug, Deserialize)]
struct Body {
    #[serde(rename = "$value")]
    kind: BodyKind,
}

#[derive(Debug, Deserialize)]
enum BodyKind {
    FBD(FunctionBlockDiagram),
}

#[derive(Debug, Deserialize)]
pub struct FunctionBlockDiagram {
    #[serde(rename = "$value")]
    members: Vec<FBDMembers>,
}

// FBD has two sub-objects, commonObjects and fbdObjects; commonObjects consists of mostly UI objects such as vendorElement, do we need these?
#[derive(Debug, Deserialize)]
pub enum FBDMembers {
    #[serde(rename = "block")]
    Block {
        #[serde(rename = "@instanceName")]
        instance_name: Option<String>,

        #[serde(rename = "@typeName")]
        type_name: String,

        #[serde(rename = "@localId")]
        local_id: String,

        #[serde(rename = "@executionOrderId")]
        execution_order_id: Option<String>,

        #[serde(rename = "$value")]
        variables: Vec<VariableKind>,
    },
}

// #[derive(Debug, Deserialize)]
// pub struct Variable {
//     // #[serde(rename = "@formalParameter")]
//     // formal_parameter: String,

//     // #[serde(rename = "@negated")]
//     // negated: bool,

//     // Variable {
//     //     connectionPointIn:
//     // },
//     #[serde(rename = "$value")]
//     kind: VariableKind,
// }

#[derive(Debug, Deserialize)]
#[serde(rename = "camelCase")]
pub enum VariableKind {
    InputVariables(Variable),
    OutVariable(Variable),
}

#[derive(Debug, Deserialize)]

pub struct Variable {
    #[serde(rename = "@formalParameter")]
    formal_parameter: String,

    #[serde(rename = "@negated")]
    negated: bool,

    #[serde(rename = "@connectionPointIn")]
    connection_point_in: Option<ConnectionPoint>,

    #[serde(rename = "@connectionPointOut")]
    connection_point_out: Option<ConnectionPoint>,
}

#[derive(Debug, Deserialize)]

pub struct ConnectionPoint {
    #[serde(rename = "@connection")]
    connection: Connection,
}

#[derive(Debug, Deserialize)]
pub struct Connection {
    #[serde(rename = "@refGlobalId")]
    ref_global_id: Option<String>,

    #[serde(rename = "@globalId")]
    global_id: String,

    #[serde(rename = "@formalParameter")]
    formal_parameter: String,
}

// #[derive(Debug, Deserialize)]
// struct Pou {
//     #[serde(rename = "@name")]
//     name: String,

//     #[serde(rename = "@pouType")]
//     pou_type: PouType, // TODO: enum

//     #[serde(rename = "@returnType")]
//     return_type: Option<String>,

//     body: Body,

//     // TODO: Still missing: external, access, globals
//     // globalVars: Option<Vec<Variable>>,
//     #[serde(rename = "localVariables")]
//     local_vars: Option<Vec<Variable>>,

//     #[serde(rename = "tempVariables")]
//     temp_vars: Option<Vec<Variable>>,

//     #[serde(rename = "inputVariables")]
//     input_vars: Option<Vec<Variable>>,

//     #[serde(rename = "outputVariables")]
//     output_vars: Option<Vec<Variable>>,

//     #[serde(rename = "inOutVariables")]
//     inout_vars: Option<Vec<Variable>>,
// }

// #[derive(Debug, Deserialize)]
// #[serde(rename_all = "camelCase")]
// enum PouType {
//     Program,
//     Function,
//     FunctionBlock,
// }

// #[derive(Debug, Deserialize)]
// struct Variable {
//     #[serde(rename = "@formalParameter")]
//     formal_parameter: String,

//     #[serde(rename = "@negated")]
//     negated: bool,
// }

// #[derive(Debug, Deserialize)]
// pub struct Body {
//     #[serde(rename = "FBD")]
//     fbd: FunctionBlockDiagram,
// }

// #[derive(Debug, Deserialize)]
// pub struct FunctionBlockDiagram {
//     #[serde(rename = "$value")]
//     members: Vec<FBDMembers>,
// }

// // https://stackoverflow.com/a/53366978: Duplicate field error
// // Maybe we have to write our own deserializer (https://serde.rs/impl-deserializer.html)
// #[derive(Debug, Deserialize)]
// enum FBDMembers {
//     #[serde(rename = "block")]
//     Block {
//         #[serde(rename = "@localId")]
//         local_id: String,

//         #[serde(rename = "@executionOrderId")]
//         execution_order_id: String,
//     },

//     #[serde(rename = "inVariable")]
//     InVariable {
//         #[serde(rename = "@localId")]
//         local_id: String,
//     },

//     #[serde(rename = "outVariable")]
//     OutVariable,
// }

// #[derive(Debug, Deserialize)]
// pub struct InVariable {
//     #[serde(rename = "@localId")]
//     local_id: String,
// }

// #[derive(Debug, Deserialize)]
// pub struct OutVariable {}

fn parse() -> Result<(), String> {
    let content = std::fs::read_to_string("res/demo.xml").unwrap();
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
