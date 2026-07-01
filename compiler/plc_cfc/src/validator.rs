use plc_diagnostics::diagnostics::Diagnostic;
use plc_source::source_location::SourceLocationFactory;

use crate::model::{FbdNetwork, FbdObject, Pou};
use crate::resolver::Resolver;

pub fn validate(pou: &Pou, _: &Resolver, factory: &SourceLocationFactory) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    validate_text_declaration(pou, factory, &mut diagnostics);

    if let Some(network) = pou.get_network() {
        validate_unconnected_sink(network, factory, &mut diagnostics);
    }

    diagnostics
}

fn validate_text_declaration(pou: &Pou, factory: &SourceLocationFactory, diagnostics: &mut Vec<Diagnostic>) {
    if pou.text_declaration().is_none_or(|declaration| declaration.trim().is_empty()) {
        diagnostics.push(
            Diagnostic::new("CFC POU is missing its text declaration")
                .with_error_code("E142")
                .with_location(factory.create_file_only_location()),
        );
    }
}

fn validate_unconnected_sink(
    network: &FbdNetwork,
    factory: &SourceLocationFactory,
    diagnostics: &mut Vec<Diagnostic>,
) {
    for object in &network.objects {
        let FbdObject::DataSink(sink) = object else {
            continue;
        };

        if sink.get_referenced_argument_id().is_some() {
            continue;
        }

        let location = factory.create_block_location(
            sink.global_id.unwrap_or_default() as usize,
            sink.get_priority().map(|priority| priority as usize),
        );

        diagnostics.push(
            Diagnostic::new(format!("Data sink '{}' is not connected to a source", sink.identifier))
                .with_error_code("E084")
                .with_location(location),
        );
    }
}

#[cfg(test)]
mod tests {
    use plc_source::source_location::SourceLocationFactory;

    use crate::model;
    use crate::resolver::Resolver;
    use crate::validator::validate;

    fn diagnose(xml: &str) -> Vec<&'static str> {
        let pou = model::from_str(xml).unwrap();
        let resolver = Resolver::resolve(&pou);
        let factory = SourceLocationFactory::internal(xml);
        validate(&pou, &resolver, &factory).iter().map(|diagnostic| diagnostic.get_error_code()).collect()
    }

    #[test]
    fn missing_text_declaration() {
        let xml = r#"
            <ppx:Program xmlns:ppx="www.iec.ch/public/TC65SC65BWG7TF10" name="mainProgram">
                <ppx:MainBody>
                    <ppx:BodyContent xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:type="ppx:FBD">
                        <ppx:Network/>
                    </ppx:BodyContent>
                </ppx:MainBody>
            </ppx:Program>
        "#;

        assert_eq!(diagnose(xml), ["E142"]);
    }

    #[test]
    fn unconnected_sink() {
        let xml = r#"
            <ppx:Program xmlns:ppx="www.iec.ch/public/TC65SC65BWG7TF10" name="mainProgram">
                <ppx:AddData>
                    <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
                        <textDeclaration>
                            <content>PROGRAM mainProgram
VAR
    result : DINT;
END_VAR</content>
                        </textDeclaration>
                    </ppx:Data>
                </ppx:AddData>
                <ppx:MainBody>
                    <ppx:BodyContent xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:type="ppx:FBD">
                        <ppx:Network>
                            <ppx:FbdObject xsi:type="ppx:DataSink" identifier="result" globalId="3">
                                <ppx:ConnectionPointIn>
                                    <ppx:RelPosition x="0" y="10"/>
                                </ppx:ConnectionPointIn>
                            </ppx:FbdObject>
                        </ppx:Network>
                    </ppx:BodyContent>
                </ppx:MainBody>
            </ppx:Program>
        "#;

        assert_eq!(diagnose(xml), ["E084"]);
    }

    #[test]
    fn connected_sink_is_valid() {
        let xml = include_str!("../fixtures/expression_source/mainProgram.cfc");
        assert!(diagnose(xml).is_empty());
    }
}
