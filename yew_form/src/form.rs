use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;
<<<<<<< HEAD
=======
use yew::html::ImplicitClone;
>>>>>>> fa67514a4897880b89e3e13161797e6877d3f50b

use crate::form_state::FormState;
use crate::Model;

#[derive(Clone)]
pub struct Form<T: Model> {
    state: Rc<RefCell<FormState<T>>>,
}

impl<T: Model> ImplicitClone for Form<T> {}

impl<T: Model> Form<T> {
    pub fn new(model: T) -> Form<T> {
        Form {
            state: Rc::new(RefCell::new(FormState::new(model))),
        }
    }

    pub(crate) fn state(&self) -> Ref<FormState<T>> {
        self.state.as_ref().borrow()
    }

    pub(crate) fn state_mut(&mut self) -> RefMut<FormState<T>> {
        self.state.borrow_mut()
    }

    pub fn validate(&mut self) -> bool {
        self.state_mut().validate()
    }

    pub fn valid(&self) -> bool {
        self.state().valid()
    }

    pub fn field_value(&self, field_name: &str) -> String {
        self.state().field(field_name).field_value.to_owned()
    }

    pub fn field_valid(&self, field_name: &str) -> bool {
        self.state().field_valid(field_name)
    }

    pub fn field_message(&self, field_name: &str) -> String {
        self.state().field(field_name).message.to_owned()
    }

    pub fn set_field_value(&mut self, field_name: &str, field_value: &str) {
        self.state_mut().set_field_value(field_name, field_value);
    }

    /// returns a clone of the inner model
    pub fn model(&self) -> T {
        self.state().model().clone()
    }
}

impl<T: Model> PartialEq for Form<T> {
    fn eq(&self, other: &Self) -> bool {
<<<<<<< HEAD
        Rc::ptr_eq(&self.state, &other.state)
            || self.state.borrow().model == other.state.borrow().model
=======
        Rc::ptr_eq(&self.state, &other.state) || self.state().model == other.state().model
>>>>>>> fa67514a4897880b89e3e13161797e6877d3f50b
    }

    fn ne(&self, other: &Self) -> bool {
        self.state().model != other.state().model
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
