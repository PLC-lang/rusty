use anyhow::{anyhow, Result};
use inkwell::{types::BasicType, values::PointerValue};
use plc_ast::ast::AstNode;

use crate::{
    codegen::generators::{expression_generator, expression_visitor::ExpressionVisitor},
    resolver::AnnotationMap,
    typesystem::DataTypeInformation,
};

pub fn generate_assignment(left: &AstNode, right: &AstNode, gen: &mut ExpressionVisitor) -> Result<()> {
    let left_type = gen
        .annotations
        .get_type(left, gen.index)
        .ok_or_else(|| anyhow::anyhow!("Left side of assignment has no type"))?
        .get_type_information();

    let left = gen.generate_expression(left)?.as_pointer_value()?;
    let right_type = gen.annotations.get_type_or_void(right, gen.index).get_type_information();

    // redirect aggregate types
    if left_type.is_aggregate() && right_type.is_aggregate() {
        let right = gen.generate_expression(right)?.as_pointer_value()?;
        build_memcpy(left, left_type, right, right_type, gen)?;
    } else {
        let expression = gen.generate_r_value(right)?;
        gen.llvm.builder.build_store(left, expression);
    }
    Ok(())
}

fn build_memcpy<'ink>(
    left: inkwell::values::PointerValue<'ink>,
    left_type: &DataTypeInformation,
    right: inkwell::values::PointerValue<'ink>,
    right_type: &DataTypeInformation,
    gen: &mut ExpressionVisitor<'ink, '_>,
) -> Result<PointerValue<'ink>> {
    let (size, alignment) = match (left_type, right_type) {
        (
            DataTypeInformation::String { size: lsize, .. },
            DataTypeInformation::String { size: rsize, .. },
        ) => {
            let target_size = lsize
                .as_int_value(gen.index)
                .map_err(|err| anyhow!("Cannot determine the size of target String {lsize:?}."))?;
            let value_size = rsize
                .as_int_value(gen.index)
                .map_err(|err| anyhow!("Cannot determine the size of destination String {rsize:?}."))?;
            let size = std::cmp::min(target_size - 1, value_size);
            //FIXME: use the target_layout for this operation
            let alignment = left_type.get_string_character_width(gen.index).value();
            //Multiply by the string alignment to copy enough for widestrings
            //This is done at compile time to avoid generating an extra mul
            let size = gen.llvm.context.i32_type().const_int((size * alignment as i64) as u64, true);
            (size, alignment)
        }
        (DataTypeInformation::Array { .. }, DataTypeInformation::Array { .. })
        | (DataTypeInformation::Struct { .. }, DataTypeInformation::Struct { .. }) => {
            let size =
                gen.llvm_index.get_associated_type(right_type.get_name())?.size_of().ok_or_else(|| {
                    anyhow::anyhow!("Cannot determine the size of type {}.", right_type.get_name())
                })?;
            (size, 1)
        }
        _ => unreachable!("memcpy is not used for non-aggregate types"),
    };

    gen.llvm.builder.build_memcpy(left, alignment, right, alignment, size).map_err(|e| {
        anyhow::anyhow!("Failed to build memcpy for types {left_type:?} and {right_type:?}: {e}")
    })
}
