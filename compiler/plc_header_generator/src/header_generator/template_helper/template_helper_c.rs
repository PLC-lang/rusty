use crate::header_generator::{
    header_generator_c::GeneratedHeaderForC,
    template_helper::{Template, TemplateHelper, TemplateType},
};

impl TemplateHelper for GeneratedHeaderForC {
    fn get_template(&self, template_type: TemplateType) -> Template {
        match template_type {
            TemplateType::Header => Template {
                content: include_str!("templates/header_template.h").to_string(),
                name: String::from("header_template.h"),
            },
        }
    }
}
