use validator::Validate;

pub trait Model: Validate + PartialEq + Clone + 'static {
    fn fields(&self, fields: &mut Vec<String>);
    fn value(&self, field_path: &str) -> String;
    fn set_value(&mut self, field_path: &str, value: &str) -> Result<(), String>;
}

