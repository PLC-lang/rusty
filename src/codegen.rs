use std::collections::HashMap;

use super::ast::*;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::module::Linkage;

use inkwell::types::BasicTypeEnum;
use inkwell::types::StringRadix;
use inkwell::types::StructType;
use inkwell::values::BasicValueEnum;
use inkwell::values::BasicValue;

use inkwell::values::PointerValue;
use inkwell::values::GlobalValue;
use inkwell::AddressSpace;
use inkwell::IntPredicate;

pub struct CodeGen<'ctx> {
    pub context: &'ctx Context,
    pub module: Module<'ctx>,
    pub builder: Builder<'ctx>,

    variables: HashMap<String, u32>,
    current_pou: String,
}

impl<'ctx> CodeGen<'ctx> {
    pub fn new(context: &'ctx Context) -> CodeGen<'ctx> {
        let module = context.create_module("main");
        let builder = context.create_builder();
        let codegen = CodeGen {
            context: context,
            module,
            builder,
            variables: HashMap::new(),
            current_pou: "".to_string(),
        };
        codegen
    }

    fn get_struct_name(pou_name: &str) -> String {
        format!("{}_interface", pou_name)
    }

    fn get_struct_instance_name(pou_name: &str) -> String {
        format!("{}_instance", pou_name)
    }

    pub fn generate(&mut self, root: &CompilationUnit) -> String {
        self.generate_compilation_unit(root);
        self.module.print_to_string().to_string()
    }

    fn generate_compilation_unit(&mut self, root: &CompilationUnit) {
        for unit in &root.units {
            self.generate_program(unit);
        }
    }

    fn generate_program(&mut self, p: &Program) {
        
        self.current_pou = p.name.clone();
        
        let return_type = self.context.void_type(); 
        //let return_type = self.context.i32_type();
        let f_type = return_type.fn_type(&[], false);
        let function = self.module.add_function(self.current_pou.as_str(), f_type, None);
        let block = self.context.append_basic_block(function, "entry");

        let mut program_members: Vec<(String, BasicTypeEnum<'ctx>)> = Vec::new();

        for var_block in &p.variable_blocks {
            let mut members = self.get_variables_information(var_block);
            program_members.append(&mut members);
        }
        //Create a struct with the value from the program
        let member_type = CodeGen::generate_instance_struct(
            self.context,
            &mut self.variables,
            program_members,
            &CodeGen::get_struct_name(p.name.as_str()),
        );

        //Create An instance variable for that struct
        //Place in global data
        self.generate_instance_variable(member_type, CodeGen::get_struct_instance_name(p.name.as_str()).as_str());
        //let mut result = None;
        for stmt in &p.statements {
            self.builder.position_at_end(&block);
            // result = 
            self.generate_statement(stmt);
        }
        //self.builder.build_return(Some(&result.unwrap()));
        self.builder.build_return(None);
    }

    fn get_variables_information(&self, v: &VariableBlock) -> Vec<(String, BasicTypeEnum<'ctx>)> {
        let mut types: Vec<(String, BasicTypeEnum<'ctx>)> = Vec::new();
        for variable in &v.variables {
            let var_type = self.get_type(&variable.data_type);
            types.push((variable.name.clone(), var_type.into()));
        }
        types
    }

    fn get_type(&self, t: &Type) -> BasicTypeEnum<'ctx> {
        if let Type::Primitive(name) = t {
            match name {
                PrimitiveType::Int => self.context.i32_type().into(),
                PrimitiveType::Bool => self.context.bool_type().into(),
            }
        } else {
            panic!("Unkown type {:?}", t)
        }
    }

    fn generate_instance_struct(
        context: &'ctx Context,
        variables: &mut HashMap<String, u32>,
        members: Vec<(String, BasicTypeEnum<'ctx>)>,
        name: &str,
    ) -> StructType<'ctx> {
        let struct_type = context.opaque_struct_type(name);
        let mut member_types: Vec<BasicTypeEnum<'ctx>> = Vec::new();

        //let member_types = members.into_iter().map(|(_, it)| it).collect::<Vec<_>>();

        for (index, (type_name, t)) in members.iter().enumerate() {
            member_types.push(*t);
            variables.insert(type_name.to_string(), index as u32);
        }

        struct_type.set_body(member_types.as_slice(), false);
        struct_type
    }

    fn generate_instance_variable(
        &self,
        variable_type: StructType<'ctx>,
        name: &str,
    ) -> GlobalValue {
        let result = self.module
            .add_global(variable_type, Some(AddressSpace::Generic), name);

        result.set_initializer(&variable_type.const_zero());
        result.set_linkage(Linkage::Common);
        result
    }

    fn generate_statement(&self, s: &Statement) -> Option<BasicValueEnum> {
        match s {
            Statement::BinaryExpression {
                operator,
                left,
                right,
            } => self.generate_binary_expression(operator, left, right),
            Statement::LiteralNumber { value } => self.generate_literal_number(value.as_str()),
            Statement::Reference { name } => self.generate_variable_reference(name),
            Statement::Assignment { left, right } => self.generate_assignment(&left, &right),
            Statement::UnaryExpression { operator, value } => self.generate_unary_expression(&operator, &value),
            _ => unimplemented!(),
        }
    }

    fn generate_unary_expression(&self, operator: &Operator, value: &Box<Statement>) -> Option<BasicValueEnum> {
        let loaded_value = self.generate_statement(value).unwrap().into_int_value();    
        let value = match operator {
            Operator::Not => self.builder.build_not(loaded_value, "tmpVar"),
            Operator::Minus => self.builder.build_int_neg(loaded_value, "tmpVar"),
            _ => unimplemented!()
        };
        Some(BasicValueEnum::IntValue(value))
    }

    fn generate_variable_lreference(&self, name: &str) -> Option<PointerValue> {
        // for now we only support locals
        let struct_type = self.module.get_type(CodeGen::get_struct_name(self.current_pou.as_str()).as_str()).unwrap().into_struct_type();
        println!("{:?} ->{}",struct_type.get_name().unwrap(), struct_type.count_fields());
        
        let ptr_struct_ype = struct_type.ptr_type(AddressSpace::Generic);
        let void_ptr_value = self
            .module
            .get_global(CodeGen::get_struct_instance_name(self.current_pou.as_str()).as_str()).unwrap().as_basic_value_enum().into_pointer_value();
        
        let ptr_value = self.builder.build_pointer_cast(void_ptr_value, ptr_struct_ype, "temp_struct");
        let index = self.variables.get(name);

        if let Some(index) = index {
            // let struct_value = self.builder.build_load(ptr_value, "temp_struct");
            // let tt = struct_value.get_type().into_struct_type();
            //println!("{:?} -> {:?}", tt, tt.get_field_types());
             let ptr_result = 
             unsafe {self.builder.build_struct_gep(ptr_value, *index, "temp_struct")};
             Some(ptr_result)
            //self.builder.build_extract_value(struct_value.into_struct_value(), *index, name)
        } else {
            None
        }
    }

    fn generate_variable_reference(&self, name: &str) -> Option<BasicValueEnum> {
        if let Some(ptr) =  self.generate_variable_lreference(name) {
            let result = self.builder.build_load(ptr, format!("load_{var_name}", var_name = name).as_str()); 
            Some(result)
        } else {
            None
        }
    }

    /**
     *  int x = 7;
        x = 7 + x;
     *  return x;
     * 
            %1 = alloca i32, align 4
            %2 = alloca i32, align 4
            store i32 0, i32* %1, align 4
            store i32 7, i32* %2, align 4
            %3 = load i32, i32* %2, align 4
            %4 = add nsw i32 7, %3
            store i32 %4, i32* %2, align 4

            https://github.com/sinato/inkwell-playground/tree/master/examples

     */

    fn generate_assignment(&self, left: &Box<Statement>, right : &Box<Statement>) -> Option<BasicValueEnum> {
        
        if let Statement::Reference { name } = &**left {
            let left_expr = self.generate_variable_lreference(&name);
            let right_res = self.generate_statement(right);
            self.builder.build_store(left_expr?, right_res?);
        }
        None


    }

    fn generate_literal_number(&self, value: &str) -> Option<BasicValueEnum> {
        let itype = self.context.i32_type();
        println!("Generating Literal {}", value);
        let value = itype.const_int_from_string(value, StringRadix::Decimal);
        Some(BasicValueEnum::IntValue(value?))
    }

    fn generate_binary_expression(
        &self,

        operator: &Operator,
        left: &Box<Statement>,
        right: &Box<Statement>,
    ) -> Option<BasicValueEnum> {
        let lval_opt = self.generate_statement(left);
        let lvalue = lval_opt.unwrap().into_int_value();

        let rval_opt = self.generate_statement(right);
        let rvalue = rval_opt.unwrap().into_int_value();

        let result = match operator {
            Operator::Plus => self.builder.build_int_add(lvalue, rvalue, "tmpVar"),
            Operator::Minus => self.builder.build_int_sub(lvalue, rvalue, "tmpVar") ,
            Operator::Multiplication => self.builder.build_int_mul(lvalue, rvalue, "tmpVar"),
            Operator::Division => self.builder.build_int_signed_div(lvalue, rvalue, "tmpVar"),
            Operator::Modulo => self.builder.build_int_signed_rem(lvalue, rvalue, "tmpVar"),
            Operator::Equal => self.builder.build_int_compare(IntPredicate::EQ, lvalue, rvalue, "tmpVar"),
            Operator::NotEqual => self.builder.build_int_compare(IntPredicate::NE, lvalue, rvalue, "tmpVar"),
            Operator::Less => self.builder.build_int_compare(IntPredicate::SLT, lvalue, rvalue, "tmpVar"),
            Operator::Greater => self.builder.build_int_compare(IntPredicate::SGT, lvalue, rvalue, "tmpVar"),
            Operator::LessOrEqual => self.builder.build_int_compare(IntPredicate::SLE, lvalue, rvalue, "tmpVar"),
            Operator::GreaterOrEqual => self.builder.build_int_compare(IntPredicate::SGE, lvalue, rvalue, "tmpVar"),
            Operator::And => self.builder.build_and(lvalue, rvalue, "tmpVar"),
            Operator::Or => self.builder.build_or(lvalue, rvalue, "tmpVar"),
            Operator::Xor => self.builder.build_xor(lvalue, rvalue, "tmpVar"),
            _ => unimplemented!(),
        };
        Some(BasicValueEnum::IntValue(result))
    }
}
    
#[cfg(test)]
mod tests {
    use pretty_assertions::{assert_eq};
    use super::super::lexer;
    use super::super::parser;
    use inkwell::context::Context;

    macro_rules! codegen {
        ($code:tt) => ({
            let lexer = lexer::lex($code);
            let ast = parser::parse(lexer).unwrap();

            let context = Context::create();
            let mut code_generator = super::CodeGen::new(&context);
            code_generator.generate(&ast)
        })
    }

    macro_rules! generate_boiler_plate {
        ($pou_name:tt, $type:tt, $body:tt)  => (
            format!(
r#"; ModuleID = 'main'
source_filename = "main"

%prg_interface = type {{{type}}}

@prg_instance = common global %prg_interface zeroinitializer

define void @{pou_name}() {{
entry:
{body}}}
"#,
            pou_name = $pou_name, type = $type, body = $body)
        )
    }


    #[test]
    fn empty_program_with_name_generates_void_function() {
        let result = codegen!("PROGRAM prg END_PROGRAM");
        let expected = generate_boiler_plate!("prg","","");

        assert_eq!(result,expected);
    }

    #[test]
    fn program_with_variables_generates_void_function_and_struct() {
        let result = codegen!(
r#"PROGRAM prg
VAR
x : INT;
y : INT;
END_VAR
END_PROGRAM
"#
        );
        let expected = generate_boiler_plate!("prg"," i32, i32 ","");

        assert_eq!(result,expected);
    }


    #[test]
    fn program_with_variables_and_references_generates_void_function_and_struct_and_body() {
        let result = codegen!(
r#"PROGRAM prg
VAR
x : INT;
y : INT;
END_VAR
x;
y;
END_PROGRAM
"#
        );
        let expected = generate_boiler_plate!("prg"," i32, i32 ",
r#"  %load_x = load i32, i32* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 0)
  %load_y = load i32, i32* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 1)
  ret void
"#
        );

        assert_eq!(result,expected);
    }

    #[test]
    fn program_with_bool_variables_and_references_generates_void_function_and_struct_and_body() {
        let result = codegen!(
r#"PROGRAM prg
VAR
x : BOOL;
y : BOOL;
END_VAR
x;
y;
END_PROGRAM
"#
        );
        let expected = generate_boiler_plate!("prg"," i1, i1 ",
r#"  %load_x = load i1, i1* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 0)
  %load_y = load i1, i1* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 1)
  ret void
"#
        );

        assert_eq!(result,expected);
    }


    #[test]
    fn program_with_variables_and_additions_generates_void_function_and_struct_and_body() {
        let result = codegen!(
r#"PROGRAM prg
VAR
x : INT;
y : INT;
END_VAR
x + y;
END_PROGRAM
"#
        );
        let expected = generate_boiler_plate!("prg"," i32, i32 ",
r#"  %load_x = load i32, i32* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 0)
  %load_y = load i32, i32* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 1)
  %tmpVar = add i32 %load_x, %load_y
  ret void
"#
        );

        assert_eq!(result,expected);
    }

    #[test]
    fn program_with_variable_and_addition_literal_generates_void_function_and_struct_and_body() {
        let result = codegen!(
r#"PROGRAM prg
VAR
x : INT;
END_VAR
x + 7;
END_PROGRAM
"#
        );
        let expected = generate_boiler_plate!("prg"," i32 ",
r#"  %load_x = load i32, i32* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 0)
  %tmpVar = add i32 %load_x, 7
  ret void
"#
        );

        assert_eq!(result,expected);
    }

    #[test]
    fn program_with_variable_assignment_generates_void_function_and_struct_and_body() {
        let result = codegen!(
r#"PROGRAM prg
VAR
y : INT;
END_VAR
y := 7;
END_PROGRAM
"#
        );
        let expected = generate_boiler_plate!("prg"," i32 ",
r#"  store i32 7, i32* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 0)
  ret void
"#
        );

        assert_eq!(result,expected);
    }



    #[test]
    fn program_with_variable_and_arithmatic_assignment_generates_void_function_and_struct_and_body() {
        let result = codegen!(
r#"PROGRAM prg
VAR
x : INT;
y : INT;
END_VAR
y := x + 1;
y := x - 2;
y := x * 3;
y := x / 4;
y := x MOD 5;
END_PROGRAM
"#
        );
        let expected = generate_boiler_plate!("prg"," i32, i32 ",
r#"  %load_x = load i32, i32* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 0)
  %tmpVar = add i32 %load_x, 1
  store i32 %tmpVar, i32* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 1)
  %load_x1 = load i32, i32* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 0)
  %tmpVar2 = sub i32 %load_x1, 2
  store i32 %tmpVar2, i32* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 1)
  %load_x3 = load i32, i32* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 0)
  %tmpVar4 = mul i32 %load_x3, 3
  store i32 %tmpVar4, i32* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 1)
  %load_x5 = load i32, i32* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 0)
  %tmpVar6 = sdiv i32 %load_x5, 4
  store i32 %tmpVar6, i32* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 1)
  %load_x7 = load i32, i32* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 0)
  %tmpVar8 = srem i32 %load_x7, 5
  store i32 %tmpVar8, i32* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 1)
  ret void
"#
        );

        assert_eq!(result,expected);
    }


    #[test]
    fn program_with_variable_and_comparison_assignment_generates_void_function_and_struct_and_body() {
        let result = codegen!(
r#"PROGRAM prg
VAR
x : INT;
y : BOOL;
END_VAR
y := x = 1;
y := x > 2;
y := x < 3;
y := x <> 4;
y := x >= 5;
y := x <= 6;
END_PROGRAM
"#
        );
        let expected = generate_boiler_plate!("prg"," i32, i1 ",
r#"  %load_x = load i32, i32* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 0)
  %tmpVar = icmp eq i32 %load_x, 1
  store i1 %tmpVar, i1* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 1)
  %load_x1 = load i32, i32* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 0)
  %tmpVar2 = icmp sgt i32 %load_x1, 2
  store i1 %tmpVar2, i1* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 1)
  %load_x3 = load i32, i32* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 0)
  %tmpVar4 = icmp slt i32 %load_x3, 3
  store i1 %tmpVar4, i1* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 1)
  %load_x5 = load i32, i32* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 0)
  %tmpVar6 = icmp ne i32 %load_x5, 4
  store i1 %tmpVar6, i1* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 1)
  %load_x7 = load i32, i32* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 0)
  %tmpVar8 = icmp sge i32 %load_x7, 5
  store i1 %tmpVar8, i1* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 1)
  %load_x9 = load i32, i32* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 0)
  %tmpVar10 = icmp sle i32 %load_x9, 6
  store i1 %tmpVar10, i1* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 1)
  ret void
"#
        );

        assert_eq!(result,expected);
    }


    #[test]
    fn program_with_variable_and_boolean_expressions_generates_void_function_and_struct_and_body() {
        let result = codegen!(
r#"PROGRAM prg
VAR
x : BOOL;
y : BOOL;
z : INT;
END_VAR
x AND y;
x OR y;
x XOR y;
END_PROGRAM
"#
        );
        let expected = generate_boiler_plate!("prg"," i1, i1, i32 ",
r#"  %load_x = load i1, i1* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 0)
  %load_y = load i1, i1* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 1)
  %tmpVar = and i1 %load_x, %load_y
  %load_x1 = load i1, i1* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 0)
  %load_y2 = load i1, i1* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 1)
  %tmpVar3 = or i1 %load_x1, %load_y2
  %load_x4 = load i1, i1* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 0)
  %load_y5 = load i1, i1* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 1)
  %tmpVar6 = xor i1 %load_x4, %load_y5
  ret void
"#
        );

        assert_eq!(result,expected);
    }


    #[test]
    fn program_with_negated_expressions_generates_void_function_and_struct_and_body() {
        let result = codegen!(
r#"PROGRAM prg
VAR
x : BOOL;
y : BOOL;
END_VAR
NOT x;
x AND NOT y;
END_PROGRAM
"#
        );
        let expected = generate_boiler_plate!("prg"," i1, i1 ",
r#"  %load_x = load i1, i1* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 0)
  %tmpVar = xor i1 %load_x, true
  %load_x1 = load i1, i1* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 0)
  %load_y = load i1, i1* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 1)
  %tmpVar2 = xor i1 %load_y, true
  %tmpVar3 = and i1 %load_x1, %tmpVar2
  ret void
"#
        );

        assert_eq!(result,expected);
    }

    #[test]
    fn program_with_negated_combined_expressions_generates_void_function_and_struct_and_body() {
        let result = codegen!(
r#"PROGRAM prg
VAR
z : INT;
y : BOOL;
END_VAR
y AND z >= 5;
NOT (z <= 6) OR y;
END_PROGRAM
"#
        );
        let expected = generate_boiler_plate!("prg"," i32, i1 ",
r#"  %load_y = load i1, i1* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 1)
  %load_z = load i32, i32* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 0)
  %tmpVar = icmp sge i32 %load_z, 5
  %tmpVar1 = and i1 %load_y, %tmpVar
  %load_z2 = load i32, i32* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 0)
  %tmpVar3 = icmp sle i32 %load_z2, 6
  %tmpVar4 = xor i1 %tmpVar3, true
  %load_y5 = load i1, i1* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 1)
  %tmpVar6 = or i1 %tmpVar4, %load_y5
  ret void
"#
        );

        assert_eq!(result,expected);
    }

    #[test]
    fn program_with_signed_combined_expressions() {
        let result = codegen!(
            r#"PROGRAM prg
            VAR
            z : INT;
            y : INT;
            END_VAR
            -1 + z;
            2 +-z;
            -y + 3;
            END_PROGRAM
            "#
        );
        let expected = generate_boiler_plate!("prg"," i32, i32 ",
r#"  %load_z = load i32, i32* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 0)
  %tmpVar = add i32 -1, %load_z
  %load_z1 = load i32, i32* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 0)
  %tmpVar2 = sub i32 0, %load_z1
  %tmpVar3 = add i32 2, %tmpVar2
  %load_y = load i32, i32* getelementptr inbounds (%prg_interface, %prg_interface* @prg_instance, i32 0, i32 1)
  %tmpVar4 = sub i32 0, %load_y
  %tmpVar5 = add i32 %tmpVar4, 3
  ret void
"#
        );

        assert_eq!(result,expected);
    }

}


