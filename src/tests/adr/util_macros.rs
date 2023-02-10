// some helper macros to get more concise tests:
macro_rules! annotate {
    ($src:expr) => {{
        let id_provider = IdProvider::default();
        let (mut cu, mut index) = index_with_ids($src, id_provider.clone());
        let annotations = annotate_with_ids(&cu, &mut index, id_provider);
        (std::mem::take(&mut cu.implementations[0].statements), annotations, index)
    }};
}
pub(crate) use annotate;


macro_rules! deconstruct_assignment {
    ($src:expr) => {{
        if let AstStatement::Assignment{left, right, ..} = $src {
            (left, right)
        } else {
            unreachable!();
        }
    }};
}
pub(crate) use deconstruct_assignment;

macro_rules! deconstruct_call_statement {
    ($src:expr) => {{
        if let AstStatement::CallStatement{operator, parameters, ..} = $src {
            (operator, 
                parameters.as_ref().as_ref().map(crate::ast::flatten_expression_list).unwrap_or_default())
        } else {
            unreachable!();
        }
    }};
}
pub(crate) use deconstruct_call_statement;

macro_rules! deconstruct_qualified_reference {
    ($src:expr) => {{
        if let AstStatement::QualifiedReference { elements, .. } = &$src {
            elements
        }else{
            unreachable!();
        }
    }};
}
pub(crate) use deconstruct_qualified_reference;


macro_rules! deconstruct_binary_expression {
    ($src:expr) => {{
        if let AstStatement::BinaryExpression { left, right , .. } = &$src {
            (left, right)
        }else{
            unreachable!();
        }
    }};
}
pub(crate) use deconstruct_binary_expression;