use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

use crate::form_state::FormState;
use crate::Model;

#[derive(Clone)]
pub struct Form<T: Model> {
    state: Rc<RefCell<FormState<T>>>,
}

impl<T: Model> Form<T> {
    pub fn new(model: T) -> Form<T> {
        Form {
            state: Rc::new(RefCell::new(FormState::new(model))),
        }
    }

    pub(crate) fn state(&self) -> Ref<FormState<T>> {
        self.state.borrow()
    }

    pub(crate) fn state_mut(&mut self) -> RefMut<FormState<T>> {
        self.state.borrow_mut()
    }

    pub fn validate(&mut self) -> bool {
        self.state_mut().validate()
    }

    pub fn valid(&self) -> bool {
        self.state.borrow().valid()
    }

    pub fn field_value(&self, field_name: &str) -> String {
        self.state.borrow().field(field_name).field_value.to_owned()
    }

    pub fn field_valid(&self, field_name: &str) -> bool {
        self.state.borrow().field_valid(field_name)
    }

    pub fn field_message(&self, field_name: &str) -> String {
        self.state.borrow().field(field_name).message.to_owned()
    }

    pub fn set_field_value(&mut self, field_name: &str, field_value: &str) {
        self.state_mut().set_field_value(field_name, field_value);
    }

    // pub fn model(&self) -> &T {
    //     &self.state().model()
    // }

    // pub fn model_mut(&mut self) -> &mut T {
    //     self.state_mut().model_mut()
    // }
}

impl<T: Model> PartialEq for Form<T> {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.state, &other.state)
            || self.state.borrow().model == other.state.borrow().model
    }

    fn ne(&self, other: &Self) -> bool {
        self.state.borrow().model != other.state.borrow().model
    }
}

// impl<T: Model> Component for Form<T> {
//     type Message = ();
//     type Properties = FormProperties;
//
//     fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
//         Self {
//             children: props.children,
//             state: T::new(),
//         }
//     }
//
//     fn update(&mut self, msg: Self::Message) -> bool {
//         unimplemented!()
//     }
//
//     fn view(&self) -> Html {
//         unimplemented!()
//     }
// }
