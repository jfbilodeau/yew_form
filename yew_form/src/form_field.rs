pub(crate) struct FormField {
    pub field_name: String,
    pub field_value: String,
    pub message: String,
    pub dirty: bool,
    pub valid: bool,
}

impl FormField {
    pub fn new(field_name: &str, field_value: &str) -> Self {
        FormField {
            field_name: String::from(field_name),
            field_value: String::from(field_value),
            message: String::new(),
            dirty: false,
            valid: true,
        }
    }
}