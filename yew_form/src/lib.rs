#![feature(get_mut_unchecked)]

#[macro_use]
extern crate validator_derive;

pub use checkbox::CheckBox;
pub use field::Field;
pub use form::{Form};
pub use model::{Model, split_field_path};

pub mod checkbox;
pub mod form_field;
pub mod field;
pub mod form;
pub mod form_state;
pub mod model;
