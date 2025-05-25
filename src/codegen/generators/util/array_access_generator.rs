
use anyhow::{anyhow, bail, Result};
use inkwell::values::{BasicValue, IntValue, PointerValue};
use plc_ast::ast::AstNode;

use crate::{
    codegen::{
        generators::expression_visitor::ExpressionVisitor,
        llvm_typesystem::cast_if_needed,
    },
    resolver::AnnotationMap,
    typesystem::{DataTypeInformation, Dimension, DINT_TYPE},
};

/// generates a gep statement for a array-reference with an optional qualifier
///
/// - `qualifier` an optional qualifier for a reference (e.g. myStruct.x[2] where myStruct is the qualifier for x)
/// - `reference` the reference-statement pointing to the array
/// - `access` the accessor expression (the expression between the brackets: reference[access])
pub fn generate_element_pointer_for_array<'ink>(
    reference: &AstNode,
    access: &AstNode,
    gen: &mut ExpressionVisitor<'ink, '_>,
) -> Result<PointerValue<'ink>> {
    //Load the reference
    let lvalue = gen.generate_expression(reference)?.as_pointer_value()?;
    if let DataTypeInformation::Array { dimensions, .. } =
        gen.annotations.get_type_or_void(reference, gen.index).get_type_information()
    {
        // make sure dimensions match statement list
        let statements = access.get_as_list();
        if statements.is_empty() || statements.len() != dimensions.len() {
            bail!("Invalid array access");
        }

        // e.g. an array like `ARRAY[0..3, 0..2, 0..1] OF ...` has the lengths [ 4 , 3 , 2 ]
        let lengths = dimensions
            .iter()
            .map(|d| d.get_length(gen.index))
            .collect::<Result<Vec<_>, _>>()
            .map_err(|msg| anyhow!("Cannot determine length of array {dimensions:?}: {msg}"))?;

        // the portion indicates how many elements are represented by the corresponding dimension
        // the first dimensions corresponds to the number of elements of the rest of the dimensions
        //          - portion = 6 (size to the right = 3 x 2 = 6)
        //         /        - portion = 2 (because the size to the right of this dimension = 2)
        //        /        /         - the last dimension is directly translated into array-coordinates (skip-size = 1)
        //                /         /
        // [    4   ,    3    ,    2  ]
        let dimension_portions = (0..lengths.len())
            .map(|index| {
                if index == lengths.len() - 1 {
                    1
                } else {
                    lengths[index + 1..lengths.len()].iter().product()
                }
            })
            .collect::<Vec<u32>>();

        let accessors_and_portions = statements
            .iter()
            .zip(dimensions)
            .map(|(statement, dimension)|
                            // generate array-accessors
                            generate_access_for_dimension(dimension, statement, gen))
            .zip(dimension_portions)
            .collect::<Vec<_>>(); //collect here so the borrow of gen is released

        // accessing [ 1, 2, 2] means to access [ 1*6 + 2*2 + 2*1 ] = 12
        let (index_access, _) = accessors_and_portions.into_iter().fold(
            (Ok(gen.llvm.i32_type().const_zero()), 1),
            |(accumulated_value, _), (current_v, current_portion)| {
                let result = accumulated_value.and_then(|last_v| {
                    current_v.map(|v| {
                        let current_portion_value =
                            gen.llvm.i32_type().const_int(current_portion as u64, false);
                        // multiply the accessor with the dimension's portion
                        let m_v = gen.llvm.builder.build_int_mul(current_portion_value, v, "");
                        // let m_v = self.create_llvm_int_binary_expression(
                        //     &Operator::Multiplication,
                        //     current_portion_value,
                        //     v,
                        // );

                        // take the sum of the mulitlication and the previous accumulated_value
                        // this now becomes the new accumulated value
                        gen.llvm.builder.build_int_add(m_v, last_v, "")
                        // self.create_llvm_int_binary_expression(&Operator::Plus, m_v, last_v)
                    })
                });
                (result, 0 /* the 0 will be ignored */)
            },
        );

        let accessor_sequence = if lvalue.get_type().get_element_type().is_array_type() {
            // e.g.: [81 x i32]*
            // the first index (0) will point to the array -> [81 x i32]
            // the second index (index_access) will point to the element in the array
            vec![gen.llvm.i32_type().const_zero(), index_access?]
        } else {
            // lvalue is a pointer to type -> e.g.: i32*
            // only one index (index_access) is needed to access the element

            // IGNORE the additional first index (0)
            // it would point to -> i32
            // we can't access any element of i32
            vec![index_access?]
        };

        // load the access from that array
        let pointer = gen.llvm.load_array_element(lvalue, &accessor_sequence, "tmpVar")?;

        return Ok(pointer);
    }
    bail!("Invalid array access: {access:?}")
}

fn generate_access_for_dimension<'ink>(
    dimension: &Dimension,
    access_expression: &AstNode,
    gen: &mut ExpressionVisitor<'ink, '_>,
) -> Result<IntValue<'ink>> {
    let start_offset = dimension.start_offset.as_int_value(gen.index).map_err(|e| anyhow!(e))?;
    // .map_err(|it| Diagnostic::codegen_error(it, access_expression))?;

    // let access_value = {
    //     let expr = gen.generate_expression(access_expression)?;
    //     gen.as_r_value_with_name(expr, Some("tmpVar"))
    // };
    let access_value = gen.generate_r_value(access_expression)?;
    
    //If start offset is not 0, adjust the current statement with an add operation
    let result = if start_offset != 0 {
        let access_int_value = access_value.into_int_value();
        let access_int_type = access_int_value.get_type();
        gen.llvm.builder.build_int_sub(
            access_int_value,
            access_int_type.const_int(start_offset as u64, true), //TODO error handling for cast
            "",
        )
    } else {
        access_value.into_int_value()
    };
    //turn it into i32 immediately
    Ok(cast_if_needed!(
        gen.llvm,
        gen.index,
        gen.llvm_index,
        gen.index.get_type(DINT_TYPE)?,
        gen.annotations.get_hint_or_void(access_expression, gen.index),
        result.as_basic_value_enum(),
        None
    )
    .into_int_value())
}
