// some helper macros to get more concise tests:
macro_rules! annotate {
    ($src:expr) => {{
        let id_provider = plc_ast::provider::IdProvider::default();
        let (mut cu, mut index) = index_with_ids($src, id_provider.clone());
        let annotations = annotate_with_ids(&cu, &mut index, id_provider);
        (std::mem::take(&mut cu.implementations[0].statements), annotations, index)
    }};
}
pub(crate) use annotate;

macro_rules! deconstruct_assignment {
    ($src:expr) => {{
        if let plc_ast::ast::AstStatement::Assignment { data, .. } = $src {
            (&data.left, &data.right)
        } else {
            unreachable!();
        }
    }};
}
pub(crate) use deconstruct_assignment;

macro_rules! deconstruct_call_statement {
    ($src:expr) => {{
        if let plc_ast::ast::AstStatement::CallStatement { data, .. } = $src {
            (
                &data.operator,
                data.parameters
                    .as_ref()
                    .as_ref()
                    .map(plc_ast::ast::flatten_expression_list)
                    .unwrap_or_default(),
            )
        } else {
            unreachable!();
        }
    }};
}
pub(crate) use deconstruct_call_statement;

macro_rules! deconstruct_binary_expression {
    ($src:expr) => {{
        if let plc_ast::ast::AstStatement::BinaryExpression { data, .. } = &$src {
            (&data.left, &data.right)
        } else {
            unreachable!();
        }
    }};
}
pub(crate) use deconstruct_binary_expression;
