// use validator::Validate;

// pub enum FormFieldType<T> {
//     String(fn(&T) -> &str, fn(&mut T, &str)),
//     Bool(fn(&T) -> bool, fn(&mut T, bool)),
//     I32(fn(&T) -> i32, fn(&mut T, i32)),
//     F64(fn(&T) -> f64, fn(&mut T, f64)),
// }

pub(crate) struct FormField {
    pub field_name: String,
    // pub field_type: FormFieldType<T>,
    pub field_value: String,
    pub message: String,
    pub dirty: bool,
    pub valid: bool,
}

impl FormField {
    pub fn new(field_name: &str) -> Self {
        FormField {
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
        yew_form::FormField::new(stringify!($f), yew_form::FormFieldType::String(|f| &f.$f, |f, v| f.$f = v.to_string()))
    );
    ( $f1:ident . $f2:ident ) => (
        yew_form::FormField::new(&format!("{}.{}", stringify!($f1), stringify!($f2)), yew_form::FormFieldType::String(|f| &f.$f1.$f2, |f, v| f.$f1.$f2 = v.to_string()))
    );
}

#[macro_export]
macro_rules! bool_field {
    ( $f:ident ) => (
        yew_form::FormField::new(stringify!($f), yew_form::FormFieldType::Bool(|f| f.$f, |f, v| f.$f = v))
    );
    ( $f1:ident . $f2:ident ) => (
        yew_form::FormField::new(&format!("{}.{}", stringify!($f1), stringify!($f2)), yew_form::FormFieldType::Bool(|f| f.$f1.$f2, |f, v| f.$f1.$f2 = v))
    );
}

pub use text_field;