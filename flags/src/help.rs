const TEMPLATE_SHORT: &'static str = include_str!("template.help");

pub fn generate_help() -> String {
    TEMPLATE_SHORT.to_string()
}
