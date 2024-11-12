use plc_ast::ast::{AstNode, AstStatement, ReferenceAccess, ReferenceExpr};

use crate::{
    index::{ImplementationIndexEntry, Index, PouIndexEntry, VariableIndexEntry},
    typesystem::DataType,
};

use super::{AnnotationMap, AnnotationMapImpl, StatementAnnotation};

#[derive(Debug)]
pub struct Signature<'i> {
    pub parameters: Vec<&'i VariableIndexEntry>,
    pub return_type: Option<&'i DataType>,
    pub pou: &'i ImplementationIndexEntry,
}

impl Signature<'_> {

    pub fn find_parameter(&self, name: &str) -> Option<&VariableIndexEntry> {
        self.parameters.iter().find(|v| v.get_name() == name).copied()
    }
}

pub fn get_signature_of_callable<'i>(call_name: &str, index: &'i Index) -> Option<Signature<'i>> {
    let implementation = index.find_implementation_by_name(call_name);
    let pou = index.find_pou(call_name)
        .map(|pou| pou.get_container());

    if let Some((pou, implementation)) = pou.zip(implementation) {
        let members = index.get_pou_members(pou);

        let parameters = members.iter().filter(|v| v.is_parameter()).collect();
        let return_type = members
            .iter()
            .find(|v| v.is_return())
            .map(|v| v.get_type_name())
            .and_then(|t| index.find_type(t));

        return Some(Signature { parameters, return_type: return_type, pou: implementation });
    }
    return None;
}


pub fn get_call_name(operator: &AstNode, annotations: &AnnotationMapImpl, index: &Index) -> Option<String> {
        annotations
            .get(operator)
            .and_then(|it| match it {
                StatementAnnotation::Function { qualified_name, call_name, .. } => {
                    call_name.as_ref().cloned().or_else(|| Some(qualified_name.clone()))
                }
                StatementAnnotation::Program { qualified_name } => Some(qualified_name.clone()),
                StatementAnnotation::Variable { resulting_type, .. } => {
                    //lets see if this is a FB
                    index
                        .find_pou(resulting_type.as_str())
                        .filter(|it| matches!(it, PouIndexEntry::FunctionBlock { .. }))
                        .map(|it| it.get_name().to_string())
                }
                // call statements on array access "arr[1]()" will return a StatementAnnotation::Value
                StatementAnnotation::Value { resulting_type } => {
                    // make sure we come from an array or function_block access
                    match operator.get_stmt() {
                        AstStatement::ReferenceExpr ( ReferenceExpr{access: ReferenceAccess::Index(_), ..},.. ) => Some(resulting_type.clone()),
                        AstStatement::ReferenceExpr ( ReferenceExpr{access: ReferenceAccess::Deref, ..}, .. ) =>
                        // AstStatement::ArrayAccess { .. } => Some(resulting_type.clone()),
                        // AstStatement::PointerAccess { .. } => {
                           index.find_pou(resulting_type.as_str()).map(|it| it.get_name().to_string()),
                        // }
                        _ => None,
                    }
                }
                _ => None,
            })
    }

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;
    use crate::test_utils::tests::index_with_ids;
    use insta::assert_debug_snapshot;
    use plc_ast::provider::IdProvider;

    #[test]
    fn signature_of_pous() {
        // GIVEN a function foo and a program prg
        let src = r#"
            FUNCTION foo : BOOL
                VAR_INPUT   foo_a: INT; END_VAR
                VAR_OUTPUT  foo_b: INT; END_VAR
            END_FUNCTION

            PROGRAM prg
                VAR         prg_a: INT;  END_VAR
                VAR_INPUT   prg_b: BOOL; END_VAR
                VAR_OUTPUT  prg_c: REAL; END_VAR
                VAR_IN_OUT  prg_d: INT;  END_VAR
            END_PROGRAM

            FUNCTION_BLOCK fb
                VAR         fb_a: INT;  END_VAR
                VAR_INPUT   fb_b: BOOL; END_VAR
                VAR_OUTPUT  fb_c: REAL; END_VAR
                VAR_IN_OUT  fb_d: INT;  END_VAR
            END_FUNCTION_BLOCK
        "#;
        let (_, index) = index_with_ids(src, IdProvider::default());
        assert_debug_snapshot!(vec![
            to_tuple(&get_signature_of_callable("foo", &index).unwrap()),
            to_tuple(&get_signature_of_callable("prg", &index).unwrap()),
            to_tuple(&get_signature_of_callable("fb", &index).unwrap()),
        ]);
    }

    #[test]
    fn signature_of_actions() {
        // TODO: can actions have parameters??????

        // GIVEN a function foo and a program prg
        let src = r#"
            PROGRAM prg
                VAR_INPUT   prg_b: BOOL; END_VAR
            END_PROGRAM
            ACTION prg.act
            END_ACTION

            FUNCTION_BLOCK fb
                VAR_INPUT   fb_b: BOOL; END_VAR
            END_FUNCTION_BLOCK
            ACTION fb.act
            END_ACTION
        "#;
        let (_, index) = index_with_ids(src, IdProvider::default());
        assert_debug_snapshot!(vec![
            to_tuple(&get_signature_of_callable("prg.act", &index).unwrap()),
            to_tuple(&get_signature_of_callable("fb.act", &index).unwrap()),
        ]);
    }

    /// turns a signature into a tuple of strings for testing purposes
    fn to_tuple(signature: &Signature<'_>) -> (String, Vec<String>, Option<String>) {
        (
            signature.pou.call_name.clone(),
            signature
                .parameters
                .iter()
                .map(|v| format!("{}: {}", v.get_name(), v.get_type_name()))
                .collect::<Vec<_>>(),
            signature.return_type.map(|t| t.get_name().to_string()),
        )
    }
}
