use crate::header_generator::{
    header_generator_c::GeneratedHeaderForC,
    template_helper::{
        Function, Template, TemplateData, TemplateHelper, TemplateType, UserDefinedTypes, Variable,
    },
};

impl TemplateHelper for GeneratedHeaderForC {
    fn get_template_data(&self) -> &TemplateData {
        &self.template_data
    }

    fn set_template_data(&mut self, template_data: TemplateData) {
        self.template_data = template_data;
    }

    fn get_mutable_template_data_user_defined_types(&mut self) -> &mut UserDefinedTypes {
        &mut self.template_data.user_defined_types
    }

    fn get_mutable_template_data_global_variables(&mut self) -> &mut Vec<Variable> {
        &mut self.template_data.global_variables
    }

    fn get_mutable_template_data_functions(&mut self) -> &mut Vec<Function> {
        &mut self.template_data.functions
    }

    fn get_template(&self, template_type: TemplateType) -> Template {
        match template_type {
            TemplateType::Header => Template {
                content: include_str!("templates/c/header_template.h").to_string(),
                name: String::from("header_template.h"),
            },
        }
    }
}
