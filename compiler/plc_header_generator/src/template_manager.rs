use crate::GenerateLanguage;

pub struct TemplateManager {
    pub language: GenerateLanguage,
}

impl TemplateManager {
    pub const fn new() -> Self {
        TemplateManager { language: GenerateLanguage::C }
    }

    pub fn get_template(&self, template_type: TemplateType) -> Option<Template> {
        match &self.language {
            GenerateLanguage::C => get_template_c(template_type),
            _ => None,
        }
    }
}

impl Default for TemplateManager {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Template {
    pub content: String,
    pub name: String,
}

fn get_template_c(template_type: TemplateType) -> Option<Template> {
    match template_type {
        TemplateType::Function => Some(Template {
            content: include_str!("templates/c/function_template.h").to_string(),
            name: String::from("function_template.h"),
        }),
        TemplateType::Header => Some(Template {
            content: include_str!("templates/c/header_template.h").to_string(),
            name: String::from("header_template.h"),
        }),
        TemplateType::ParamArray => Some(Template {
            content: include_str!("templates/c/param_array_template.h").to_string(),
            name: String::from("param_array_template.h"),
        }),
        TemplateType::ParamEnum => Some(Template {
            content: include_str!("templates/c/param_enum_template.h").to_string(),
            name: String::from("param_enum_template.h"),
        }),
        TemplateType::ParamStruct => Some(Template {
            content: include_str!("templates/c/param_struct_template.h").to_string(),
            name: String::from("param_struct_template.h"),
        }),
        TemplateType::UserTypeArray => Some(Template {
            content: include_str!("templates/c/user_type_array_template.h").to_string(),
            name: String::from("user_type_string_template.h"),
        }),
        TemplateType::UserTypeEnum => Some(Template {
            content: include_str!("templates/c/user_type_enum_template.h").to_string(),
            name: String::from("user_type_enum_template.h"),
        }),
        TemplateType::UserTypeStruct => Some(Template {
            content: include_str!("templates/c/user_type_struct_template.h").to_string(),
            name: String::from("user_type_struct_template.h"),
        }),
        TemplateType::Variable => Some(Template {
            content: include_str!("templates/c/variable_template.h").to_string(),
            name: String::from("variable_template.h"),
        }),
    }
}

pub enum TemplateType {
    Header,
    Function,
    ParamArray,
    ParamEnum,
    ParamStruct,
    UserTypeArray,
    UserTypeEnum,
    UserTypeStruct,
    Variable,
}
