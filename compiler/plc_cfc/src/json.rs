use serde::Deserialize;
use xmltojson::to_json;

#[derive(Debug, Deserialize)]
struct pou {
    #[serde(rename = "name")]
    name: String,

    #[serde(rename = "@returnType")]
    return_type: Option<String>,
}

fn parse() -> Result<(), String> {
    let content = std::fs::read_to_string("res/demo.xml").unwrap();
    let value = to_json(&content).unwrap();
    println!("{value}");
    // we need to parse twice :/
    let json: pou = serde_json::from_value(value).unwrap();
    println!("{json:#?}");

    Ok(())
}

// TODO: Move into folder
mod parse {
    #[test]
    fn xml() {
        let content = crate::json::parse().unwrap();
    }
}
