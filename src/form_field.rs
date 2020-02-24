use validator::Validate;

pub enum FormFieldType<T> {
    String(fn(&T) -> &str, fn(&mut T, &str)),
    Bool(fn(&T) -> bool, fn(&mut T, bool)),
    I32(fn(&T) -> i32, fn(&mut T, i32)),
    F64(fn(&T) -> f64, fn(&mut T, f64)),
}

pub struct FormField<T: Validate + PartialEq> {
    pub field_name: String,
    pub field_type: FormFieldType<T>,
    pub field_value: String,
    pub message: String,
    pub dirty: bool,
    pub valid: bool,
}

impl<T: Validate + PartialEq> FormField<T> {
    pub fn new(field_name: &str, field_type: FormFieldType<T>) -> Self {
        FormField {
            field_type,
            field_name: String::from(field_name),
            field_value: String::new(),
            message: String::new(),
            dirty: false,
            valid: true,
        }
    }
}

#[macro_export]
macro_rules! text_field {
    ( $f:ident ) => (
        FormField::new(stringify!($f), FormFieldType::String(|f| &f.$f, |f, v| f.$f = v.to_string()))
    );
    ( $f1:ident . $f2:ident ) => (
        FormField::new(&format!("{}.{}", stringify!($f1), stringify!($f2)), FormFieldType::String(|f| &f.$f1.$f2, |f, v| f.$f1.$f2 = v.to_string()))
    );
}

pub use text_field;