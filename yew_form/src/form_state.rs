use validator::{ValidationErrors, ValidationErrorsKind};
use crate::{Model};
use crate::form_field::FormField;

pub struct FormState<T: Model> {
    pub(crate) model: T,
    fields: Vec<FormField>,
}

impl<T: Model> FormState<T> {
    pub fn new(model: T) -> FormState<T> {
        let mut fields = vec![];

        model.fields(&mut fields);

        let form = FormState {
            model,
            fields: fields.iter().map(|f| FormField::new(f)).collect()
        };

        // for field in &mut form.fields {
        //     match field.field_type {
        //         FormFieldType::String(f, _) => {
        //             field.field_value = f(&form.model).to_string();
        //         }
        //         FormFieldType::Bool(f, _) => {
        //             field.field_value = String::from(if f(&form.model) { "checked" } else { "" });
        //         }
        //         FormFieldType::I32(f, _) => {
        //             field.field_value = f(&form.model).to_string();
        //         }
        //         FormFieldType::F64(f, _) => {
        //             field.field_value = f(&form.model).to_string();
        //         }
        //     }
        // }
        form
    }

    pub fn model(&self) -> &T {
        &self.model
    }

    pub fn model_mut(&mut self) -> &mut T {
        &mut self.model
    }

    pub(crate) fn field(&self, name: &str) -> &FormField {
        self.fields.iter().find(|&f| f.field_name == name).expect(&format!("Field {} does not exist", name))
    }

    pub(crate) fn field_mut(&mut self, name: &str) -> &mut FormField {
        self.fields.iter_mut().find(|f| f.field_name == name).expect(&format!("Field {} does not exist", name))
    }

    pub fn field_value(&self, field_name: &str) -> &str {
        let field = self.field(field_name);

        &field.field_value
    }

    pub fn set_field_value(&mut self, field_path: &str, field_value: &str) {
        let result = self.model.set_value(field_path, field_value);

        let field = self.field_mut(field_path);
        field.field_value = String::from(field_value);

        if let Err(e) = result {
            field.message = String::from(e);
        }

        // let mut field = self.field_mut(field_path);
        //
        // field.field_value = field_value.to_string();
        // field.dirty = true;
        //
        // match field.field_type {
        //     FormFieldType::String(_, f) => {
        //         let value = field.field_value.clone();
        //         f(&mut self.model, &value);
        //     }
        //     FormFieldType::Bool(_, f) => {
        //         let value = field.field_value != "";
        //         f(&mut self.model, value);
        //     }
        //     FormFieldType::I32(_, f) => {
        //         if let Ok(value) = field.field_value.parse::<i32>() {
        //             f(&mut self.model, value);
        //         } else {
        //             field.valid = false;
        //             field.message = "Invalid number".to_string();
        //         }
        //     }
        //     FormFieldType::F64(_, f) => {
        //         if let Ok(value) = field.field_value.parse::<f64>() {
        //             f(&mut self.model, value);
        //         } else {
        //             field.valid = false;
        //             field.message = "Invalid number".to_string();
        //         }
        //     }
        // }
    }

    pub fn valid(&self) -> bool {
        self.fields.iter().all(|f| f.valid)
    }

    pub fn field_valid(&self, field_path: &str) -> bool {
        self.field(field_path).valid
    }

    pub fn field_message(&self, field_path: &str) -> &str {
        &self.field(field_path).message
    }

    /// Perform validation on the model
    /// Returns `true` if the model passes validation
    pub fn validate(&mut self) -> bool {
        self.fields.iter_mut().for_each(|f| {
            f.valid = true;
            f.dirty = true;
        });

        let result = self.model.validate();

        if let Err(errors) = result {
            self.add_errors("", &errors);
        }

        self.valid()
    }

    fn generate_field_name(&self, prefix: &str, field_name: &str) -> String {
        // NOTE: Though &self is not require, it avoids having to call the function as follows:
        // FormState<T>::generate_field_name()
        if prefix == "" {
            String::from(field_name)
        } else {
            format!("{}.{}", prefix, field_name)
        }
    }

    fn add_errors(&mut self, prefix: &str, errors: &ValidationErrors) {
        for (field_name, error) in errors.errors() {
            match error {
                ValidationErrorsKind::Struct(errors) => {
                    self.add_errors(&self.generate_field_name(prefix, field_name), errors)
                }
                ValidationErrorsKind::List(_) => { /* Ignore? */ }
                ValidationErrorsKind::Field(errors) => {
                    let field = self.field_mut(&self.generate_field_name(prefix, field_name));

                    field.valid = false;

                    field.message = if let Some(message) = errors[0].message.as_ref() {
                        message.to_string()
                    } else {
                        "Error".to_string()
                    }
                }
            };
        }
    }
}
