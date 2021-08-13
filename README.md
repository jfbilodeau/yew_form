[![License:MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT) [![License:Apache](https://img.shields.io/badge/License-Apache-yellow.svg)](https://opensource.org/licenses/Apache-2.0)
# Yew Form
Bringing MVC to Yew! A set of mildly opinionated Yew component to map and validate a model to a HTML form.

[Live demo](http://chronogears.com/yew-form/)

Supports:
- 2-way Binding to struct (with support for nested structs)
- Validation ([using Keats Validator](https://github.com/Keats/validator))
- Only `String` and `bool` fields are supported presently. More to come

## Usage
Cargo.toml:
```toml
[dependencies]
validator = "0.14"
validator_derive = "0.14"
yew = "0.18"
yew_form = "0.1"
yew_form_derive = "0.1"
```
main.rs:
```rust
#[macro_use]
extern crate validator_derive;
extern crate yew_form;
#[macro_use]
extern crate yew_form_derive;
```

## Example
Consider the following model:
```rust
#[derive(Model, Validate, PartialEq, Clone)]
struct Address {
    #[validate(length(min = 1, message="Street is required"))]
    street: String,
    #[validate(length(min = 1, message="City name is required"))]
    city: String,
    #[validate(regex(path="PROVINCE_RE", message="Enter 2 digit province code"))]
    province: String,
    postal_code: String,
    country: String,
}

#[derive(Model, Validate, PartialEq, Clone)]
struct Registration {
    #[validate(length(min = 1, message="First name is required"))]
    first_name: String,
    #[validate(length(min = 1, message="Last name is required"))]
    last_name: String,
    quantity: u32,
    price: f64,
    #[validate]
    address: Address,
    #[validate(custom = "must_be_true")]
    accept_terms: bool,
}
```

The struct can be bound to a Yew form using the following definition:

```rust
struct App {
    form: Form<Registration>,
    ...
}
```

For now, the `Form` needs to be instantiated as follows:
```rust
fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
    // Create model initial state
    let model = Registration {
        first_name: String::from("J-F"),
        last_name: String::from("Bilodeau"),
        address: Address {
            street: String::new(),
            city: String::from("Ottawa"),
            province: String::from("ONT"),
            postal_code: String::from("K2P 0A4"),
            country: String::new(),
        },
    };

    Self {
        form: Form::new(model),
        ...
    }
    ...
```

Fields can then be added to the form as follows:
```html
<Field<Registration> 
    form=&self.form 
    field_name="first_name"
    autocomplete="given-name"
    oninput=self.link.callback(|_: InputData| AppMessage::Update) />

<!-- here we use custom css classes -->
<Field<Registration> 
    form=&self.form 
    field_name="address.street"
    class="form-control"
    class_invalid="is-invalid red-border"
    class_valid="is-valid green-border"
    oninput=self.link.callback(|_: InputData| AppMessage::Update) />
...
<CheckBox<Registration> field_name="accept_terms" form=&self.form />
```
The `Field` component takes care of two way binding between `struct Registration` and the HTML `<input>`

Validation is done automatically when the user edits the form or programmatically.

```rust
if self.form.validate() {
    ...
}
```

Todo/Wish List:
- [ ] Add documentation (In progress)
- [ ] ~~Remove clone requirement from model~~
- [X] Add `derive` for model to remove need for `vec!` of fields
- [X] Make `oninput` optional
- [ ] Make Yew update the view when `Field` is updated
- [ ] Need to add additional HTML attribute to `Field`
- [X] Remove hard-coded Bootstrap styles
- [X] Add support for additional types such as `i32`
- [ ] Support `Vec<T>`
- [X] Support Rust Stable

## Change Log

### 0.1.8
- Remove hardcoded Bootstrap css classes
- Fix `examples/form`
- Add `autocomplete` attribute

### 0.1.7
- Remove `#![feature(get_mut_unchecked)]` from code (Thanks [L0g4n](https://github.com/L0g4n))

### 0.1.6
- Removed unsafe code
- Updated yew_form version in documentation

### 0.1.5
- Updated to Yew 0.17

### 0.1.4
- Added blanket implementation for FieldValue to support `i32`, `bool`, etc...

### 0.1.3
**BREAKING CHANGES**
- Added `#[derive(Model)]`
- No need to manually pass a vector of fields to `Form::new()`

### 0.1.2
- Added CheckBox

### 0.1.1
- Make `Field::oninput` optional


(Made with ❤️ with Rust)