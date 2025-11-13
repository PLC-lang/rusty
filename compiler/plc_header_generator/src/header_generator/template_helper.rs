mod template_helper_c;

pub trait TemplateHelper {
    fn get_template(&self, template_type: TemplateType) -> Template;
}

pub struct Template {
    pub content: String,
    pub name: String,
}

pub enum TemplateType {
    Header,
}
