use crate::header_generator::{
    header_generator_c::GeneratedHeaderForC,
    template_helper::{Template, TemplateHelper, TemplateType},
};

impl TemplateHelper for GeneratedHeaderForC {
    fn get_template(&self, template_type: TemplateType) -> Template {
        match template_type {
            TemplateType::Function => Template {
                content: include_str!("templates/function_template.h").to_string(),
                name: String::from("function_template.h"),
            },
            TemplateType::Header => Template {
                content: include_str!("templates/header_template.h").to_string(),
                name: String::from("header_template.h"),
            },
            TemplateType::ParamArray => Template {
                content: include_str!("templates/param_array_template.h").to_string(),
                name: String::from("param_array_template.h"),
            },
            TemplateType::ParamEnum => Template {
                content: include_str!("templates/param_enum_template.h").to_string(),
                name: String::from("param_enum_template.h"),
            },
            TemplateType::ParamStruct => Template {
                content: include_str!("templates/param_struct_template.h").to_string(),
                name: String::from("param_struct_template.h"),
            },
            TemplateType::UserTypeArray => Template {
                content: include_str!("templates/user_type_array_template.h").to_string(),
                name: String::from("user_type_string_template.h"),
            },
            TemplateType::UserTypeEnum => Template {
                content: include_str!("templates/user_type_enum_template.h").to_string(),
                name: String::from("user_type_enum_template.h"),
            },
            TemplateType::UserTypeStruct => Template {
                content: include_str!("templates/user_type_struct_template.h").to_string(),
                name: String::from("user_type_struct_template.h"),
            },
            TemplateType::Variable => Template {
                content: include_str!("templates/variable_template.h").to_string(),
                name: String::from("variable_template.h"),
            },
        }
    }
}
