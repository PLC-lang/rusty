use plc_ast::ast::AstId;
use plc_ast::provider::IdProvider;
use plc_ast::visitor::{AstVisitor, Walker};
use plc_source::SourceCode;
use std::collections::HashSet;
use std::ops::Range;
use tabled::derive::display;
use tabled::settings::object::Columns;
use tabled::settings::{Alignment, Style};
use tabled::Tabled;

use crate::resolver::{AnnotationMap, AnnotationMapImpl, StatementAnnotation};
use crate::{resolver::TypeAnnotator, test_utils::tests::index_with_ids};

/// Extracts marked ranges from the source code, returning the cleaned source and marker ranges.
/// Markers are denoted by `{...}` in the source.
fn extract_markers(src: &str) -> Result<(String, Vec<Range<usize>>), String> {
    let mut clean_src = String::new();
    let mut marker_ranges = Vec::new();
    let mut open_marker_positions = Vec::new();

    for (i, c) in src.chars().enumerate() {
        match c {
            '{' => {
                open_marker_positions.push(clean_src.len());
            }
            '}' => {
                if let Some(start) = open_marker_positions.pop() {
                    let end = clean_src.len();
                    marker_ranges.push(start..end);
                } else {
                    return Err(format!("Unmatched closing marker '}}' at offset {i}."));
                }
            }
            _ => {
                clean_src.push(c);
            }
        }
    }

    if !open_marker_positions.is_empty() {
        return Err(format!("Unmatched opening marker '{{' at offset {open_marker_positions:?}."));
    }

    Ok((clean_src, marker_ranges))
}

/// Collects AST nodes whose spans match the provided marker ranges.
struct ExpressionCollector {
    // the location of the marked expression and its id
    expressions: Vec<(Range<usize>, AstId)>,
    // the location of un-processed markers
    // once an expression was visisted for a marker, it is removed from this set
    ordered_markers: HashSet<Range<usize>>,
}

impl ExpressionCollector {
    /// Creates a new ExpressionCollector with the given marker ranges.
    ///
    /// see also `fn extract_markers(...)`
    pub fn new(markers: &[Range<usize>]) -> Self {
        Self { expressions: Vec::new(), ordered_markers: markers.iter().cloned().collect() }
    }
}

impl AstVisitor for ExpressionCollector {
    fn visit(&mut self, node: &plc_ast::ast::AstNode) {
        let current_range = node.get_location().to_range().unwrap_or_else(|| 0..0);

        // lets see if we have a marker for this range
        if self.ordered_markers.remove(&current_range) {
            // record the id and the range
            self.expressions.push((current_range, node.get_id()))
        }

        node.walk(self);
    }
}

/// Represents the result of resolving an expression, including its type and hint.
#[derive(Tabled)]
struct ResolveResult {
    #[tabled(rename = "EXPR")]
    expression: String,
    #[tabled(rename = "TYPE", display("display::option", "-"))]
    ty: Option<String>,
    #[tabled(rename = "HINT", display("display::option", "-"))]
    hint: Option<String>,
}

/// Helper function to display the type of a StatementAnnotation.
fn display_annotation(
    annotation: &StatementAnnotation,
    annotation_map: &AnnotationMapImpl,
) -> Option<String> {
    annotation_map.get_type_name_for_annotation(annotation).map(|it| it.to_string())
}

/// Runs type resolution on the provided source code and returns a formatted table of results.
/// Expressions to be resolved should be surrounded by `{}` in the source.
pub fn test_resolve<T: Into<SourceCode>>(src: T) -> Result<String, String> {
    let id_provider = IdProvider::default();

    let (clean_src, marker_ranges) = extract_markers(src.into().source.as_str())?;

    let (unit, index) = index_with_ids(clean_src.as_str(), id_provider.clone());
    let (annotation_map, ..) = TypeAnnotator::visit_unit(&index, &unit, id_provider);

    let mut collector = ExpressionCollector::new(&marker_ranges);
    collector.visit_compilation_unit(&unit);

    let results = collector
        .expressions
        .iter()
        .map(|(marker, ast_id)| {
            let ty = annotation_map
                .get_with_id(*ast_id)
                .and_then(|annotation| display_annotation(annotation, &annotation_map));
            let hint = annotation_map
                .get_hint_with_id(*ast_id)
                .and_then(|annotation| display_annotation(annotation, &annotation_map));

            ResolveResult { expression: clean_src[marker.start..marker.end].to_string(), ty, hint }
        })
        .collect::<Vec<_>>();

    let table = tabled::Table::new(results)
        .with(Style::psql())
        .modify(Columns::first(), Alignment::right())
        .to_string();

    if collector.ordered_markers.is_empty() {
        Ok(table)
    } else {
        Err(format!(
            "Cannot find Expression for markers at {:?}.",
            collector.ordered_markers.iter().collect::<Vec<_>>()
        ))
    }
}

#[cfg(test)]
mod tests {
    use insta::assert_snapshot;

    use super::*;

    #[test]
    fn print_marked_expressions() {
        let src = "
        FUNCTION foo
            VAR
                x : INT;
                y : DINT;
            END_VAR
           {{x} + {y}}  // surrounded expressions to be evaluated
        END_FUNCTION
        ";

        let resolves = test_resolve(src).unwrap();
        assert_snapshot!(resolves, @r"
          EXPR | TYPE | HINT 
        -------+------+------
         x + y | DINT | -    
             x | INT  | DINT 
             y | DINT | -
        ");
    }

    #[test]
    fn test_unclosed_marker() {
        let src_unmatched_open = "
        FUNCTION foo
            VAR
                x : INT;
                y : DINT;
            END_VAR
           {x + {y  // Missing closing markers
        END_FUNCTION
        ";

        assert_eq!(
            test_resolve(src_unmatched_open),
            Err("Unmatched opening marker '{' at offset [120, 124].".to_string())
        );
    }

    #[test]
    fn test_invalid_closing_marker() {
        let src_unmatched_close = "
        FUNCTION foo
            VAR
                x : INT;
                y : DINT;
            END_VAR
           {x + y}}  // Extra closing marker
        END_FUNCTION
        ";

        assert_eq!(
            test_resolve(src_unmatched_close),
            Err("Unmatched closing marker '}' at offset 127.".to_string())
        );
    }

    #[test]
    fn test_no_markers() {
        let src_no_markers = "
        FUNCTION foo
            VAR
                x : INT;
                y : DINT;
            END_VAR
           x + y  // No markers present   
        END_FUNCTION
        ";

        let resolves = test_resolve(src_no_markers).unwrap();
        assert_snapshot!(resolves, @r"
         EXPR | TYPE | HINT 
        ------+------+------
        ");
    }

    #[test]
    fn test_invalid_marker_position() {
        let src_invalid_marker = "
        FUNCTION foo
            VAR
                x : INT;
                y : DINT;
            END_VAR
           {x +} y  // marker does not match any expression
        END_FUNCTION
        ";

        assert_eq!(
            test_resolve(src_invalid_marker),
            Err("Cannot find Expression for markers at [120..123].".to_string())
        );
    }
}
