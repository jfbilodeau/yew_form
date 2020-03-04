use validator::Validate;

pub trait Model: Validate + PartialEq + Clone + 'static {
    fn fields(&self, fields: &mut Vec<String>);
    fn value(&self, field_path: &str) -> String;
    fn set_value(&mut self, field_path: &str, value: &str) -> Result<(), String>;
}

pub fn split_field_path(field_path: &str) -> (&str, &str) {
    if let Some(index) = field_path.find(".") {
        (&field_path[0..index], &field_path[index+1..])
    } else {
        (field_path, "")
    }
}

#[cfg(test)]
mod tests {
    use crate::model::split_field_path;

    #[test]
    fn test_split_field_path() {
        let path = "field";
        let (field, suffix) = split_field_path(path);

        assert_eq!(field, "field");
        assert_eq!(suffix, "");

        let path = "field.sub";
        let (field, suffix) = split_field_path(path);

        assert_eq!(field, "field");
        assert_eq!(suffix, "sub");

        let path = "field.sub.subsub";
        let (field, suffix) = split_field_path(path);

        assert_eq!(field, "field");
        assert_eq!(suffix, "sub.subsub");
    }
}