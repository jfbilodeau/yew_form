use validator::Validate;

pub trait FormValue {
    fn fields(&self, prefix: &str, fields: &mut Vec<String>) {
        // By default, announce the value to be a scalar
        fields.push(String::from(prefix));
    }
    fn value(&self, field_path: &str) -> String;
    fn set_value(&mut self, field_path: &str, value: &str) -> Result<(), String>;

}

pub trait Model: FormValue + Validate + PartialEq + Clone + 'static {
}

pub fn split_field_path(field_path: &str) -> (&str, &str) {
    if let Some(index) = field_path.find(".") {
        (&field_path[0..index], &field_path[index+1..])
    } else {
        (field_path, "")
    }
}

impl FormValue for String {
    fn value(&self, field_path: &str) -> String {
        debug_assert!(field_path == "");

        self.clone()
    }

    fn set_value(&mut self, field_path: &str, value: &str) -> Result<(), String> {
        debug_assert!(field_path == "");

        self.clone_from(&String::from(value));

        Ok(())
    }
}

impl FormValue for bool {
    fn value(&self, field_path: &str) -> String {
        debug_assert!(field_path == "");

        self.to_string()
    }

    fn set_value(&mut self, field_path: &str, value: &str) -> Result<(), String> {
        debug_assert!(field_path == "");

        if let Ok(v) = value.parse::<bool>() {
            *self = v;
            Ok(())
        } else {
            Err(String::from("Could not convert to bool"))
        }
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