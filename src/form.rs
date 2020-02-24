use std::rc::Rc;

use validator::Validate;

use crate::form_state::FormState;
use crate::FormField;

pub trait Model: Validate + PartialEq + Clone + 'static {}

//pub type Form<T> = Rc<FormState<T>>;
#[derive(Clone)]
pub struct Form<T: Validate + PartialEq> {
    pub state: Rc<FormState<T>>
}

impl<T: Validate + PartialEq> Form<T> {
    pub fn new(model: T, fields: Vec<FormField<T>>) -> Form<T> {
        Form {
            state: Rc::new(FormState::new(model, fields)),
        }
    }

    fn state(&self) -> &FormState<T> {
        &self.state
    }

    pub(crate) fn state_mut(&mut self) -> &mut FormState<T> {
        unsafe {
            Rc::get_mut_unchecked(&mut self.state)
        }
    }

    pub fn validate(&mut self) -> bool {
        self.state_mut().validate()
    }

    pub fn valid(&self) -> bool {
        self.state.valid()
    }

    pub fn field_value(&self, field_name: &str) -> &str {
        &self.field(field_name).field_value
    }

    pub fn field_valid(&self, field_name: &str) -> bool {
        self.state.field_valid(field_name)
    }

    pub fn field(&self, field_name: &str) -> &FormField<T> {
        &self.state.field(field_name)
    }

    pub fn field_message(&self, field_name: &str) -> &str {
        &self.field(field_name).message
    }

    pub fn set_field(&mut self, field_name: &str, field_value: &str) {
        self.state_mut().set_field(field_name, field_value);
    }

    pub fn model(&self) -> &T {
        &self.state().model()
    }

    pub fn model_mut(&mut self) -> &mut T {
        self.state_mut().model_mut()
    }
}

impl<T: Validate + PartialEq> PartialEq for Form<T> {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.state, &other.state)
            ||
        self.state.model == other.state.model
    }

    fn ne(&self, other: &Self) -> bool {
        self.state.model != other.state.model
    }
}
