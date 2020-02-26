#![feature(get_mut_unchecked)]

pub use checkbox::CheckBox;
pub use field::Field;
pub use form::{Form, Model};
pub use form_field::{FormField, FormFieldType};

pub mod checkbox;
pub mod form_field;
pub mod field;
pub mod form;
pub mod form_state;

#[cfg(test)]
mod tests {
    use validator::Validate;

    use experi_shared::secure_string::SecStr;

    #[macro_use]
    use crate::form::{Form, FormField, FormFieldType};
    use crate::{FormField, FormFieldType};

    #[derive(Validate, PartialEq)]
    struct Test {
        #[validate(email)]
        email: String,
        int: i32,
        float: f64,
        #[validate]
        other: Other,
    }

    #[derive(PartialEq, Validate)]
    struct Other {
        more_text: String,
        sec_str: SecStr,
        #[validate(email)]
        other_email: String,
    }

    #[test]
    fn test() {
        let model = Test {
            email: "Text".to_string(),
            int: 1,
            float: 2.3,
            other: Other {
                more_text: "more_text".to_string(),
                sec_str: SecStr::new(),
                other_email: String::from("not valid"),
            },
        };

        let mut form = Form::new(model, vec![
            FormField::new("email", FormFieldType::String(|f| &f.email, |f, v| f.email = v.to_string())),
            FormField::new("int", FormFieldType::I32(|f| f.int, |f, v| f.int = v)),
            FormField::new("float", FormFieldType::F64(|f| f.float, |f, v| f.float = v)),
            FormField::new("more_text", FormFieldType::String(|f| &f.other.more_text, |f, v| f.other.more_text = v.to_string())),
            FormField::new("sec_str", FormFieldType::String(|f| &f.other.sec_str.expose(), |f, v| f.other.sec_str = SecStr::from(v))),
            FormField::new("other.other_email", FormFieldType::String(|f| &f.other.other_email, |f, v| f.other.other_email = v.to_string())),
        ]);

        assert_eq!(form.field_value("int"), "1");
        assert_eq!(form.field_value("float"), "2.3");
        assert_eq!(form.field_value("more_text"), "more_text");
        assert_eq!(form.field_value("other.other_email"), "not valid");

        assert_eq!(form.valid(), true);

        form.validate();

        assert_eq!(form.valid(), false);
        assert_eq!(form.field_valid("email"), false);
        assert_eq!(form.field_valid("other.other_email"), false);
        assert_eq!(form.field_valid("int"), true);
    }
}