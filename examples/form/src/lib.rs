#![recursion_limit = "131072"]
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate validator_derive;
extern crate yew_form;
#[macro_use]
extern crate yew_form_derive;

use regex::Regex;
use validator::{Validate, ValidationError};
use wasm_bindgen::prelude::*;
use yew::{html, Component, Context, Html, InputEvent, MouseEvent};

use yew_form::{CheckBox, Field, Form};

lazy_static! {
    static ref PROVINCE_RE: Regex = Regex::new("^[A-Z]{2}$").unwrap();
}

fn must_be_true(value: &bool) -> Result<(), ValidationError> {
    if value == &true {
        Ok(())
    } else {
        Err(ValidationError::new("Must accept terms before continuing"))
    }
}

#[derive(Model, Validate, PartialEq, Clone)]
struct Address {
    #[validate(length(min = 1, message = "Street is required"))]
    street: String,
    #[validate(length(min = 1, message = "City name is required"))]
    city: String,
    #[validate(regex(path = "PROVINCE_RE", message = "Enter 2 digit province code"))]
    province: String,
    postal_code: String,
    country: String,
}

#[derive(Model, Validate, PartialEq, Clone)]
struct Registration {
    #[validate(length(min = 1, message = "First name is required"))]
    first_name: String,
    #[validate(length(min = 1, message = "Last name is required"))]
    last_name: String,
    quantity: u32,
    price: f64,
    #[validate]
    address: Address,
    #[validate(custom = "must_be_true")]
    accept_terms: bool,
}

impl Registration {
    pub fn total(&self) -> f64 {
        self.quantity as f64 * self.price
    }
}

enum AppMessage {
    Update,
    Submit,
}

struct App {
    form: Form<Registration>,
    submitted: bool,
}

impl Component for App {
    type Message = AppMessage;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        // Create model initial state
        let model = Registration {
            first_name: String::from("J-F"),
            last_name: String::from("Bilodeau"),
            quantity: 10,
            price: 5.99,
            address: Address {
                street: String::new(),
                city: String::from("Ottawa"),
                province: String::from("ONT"),
                postal_code: String::from("K2P 0A4"),
                country: String::new(),
            },
            accept_terms: false,
        };

        Self {
            form: Form::new(model),
            submitted: false,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AppMessage::Update => true, // Force update
            AppMessage::Submit => {
                if self.form.validate() {
                    self.submitted = true;
                }
                true
            }
        }
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let cb = ctx.link().callback(|_: InputEvent| AppMessage::Update);
        let form = &self.form;
        html! {
            <div class="container-sm">
                <h1>{"Yew Form Example"}</h1>
                <p>{format!("Hello, {} {} and welcome to Yew Form!",
                        self.form.field_value("first_name"),
                        self.form.field_value("last_name"))}</p>
                <form>
                    // TODO: support additional attributes
                    // TODO: Update form without needing oninput
                    <div class="form-group">
                        <label for="first_name">{"First Name: "}</label>
                        <Field<Registration>
                                form={ form }
                                autocomplete="given_name"
                                field_name="first_name"
                                class="form-control blue foo bar"
                                class_invalid="is-invalid very-wrong"
                                class_valid="is-valid green"
                                oninput={ cb.clone() }
                        />
                        <div class="invalid-feedback">
                            {form.field_message("first_name")}
                        </div>
                    </div>
                    <div class="form-group">
                        <label for="last_name">{"Last Name: "}</label>
                        <Field<Registration> form={ form } field_name="last_name" oninput={ cb.clone() } />
                        <div class="invalid-feedback">
                            {form.field_message("last_name")}
                        </div>
                    </div>
                    <div class="form-group">
                        <label for="last_name">{"Quantity: "}</label>
                        <Field<Registration> form={ form } field_name="quantity" oninput={ cb.clone() } />
                        <div class="invalid-feedback">
                            {form.field_message("quantity")}
                        </div>
                    </div>
                    <div class="form-group">
                        <label for="last_name">{"Price: "}</label>
                        <Field<Registration> form={ form } field_name="price" oninput={ cb.clone() } />
                        <div class="invalid-feedback">
                            {form.field_message("price")}
                        </div>
                    </div>
                    <div>
                        {"Total: "}{format!("{:.2}", form.model().total())}
                    </div>
                    <div class="form-group">
                        <label for="address.street">{"Street: "}</label>
                        <Field<Registration> form={ form } field_name="address.street" oninput={ cb.clone() } />
                        <div class="invalid-feedback">
                            {form.field_message("address.street")}
                        </div>
                    </div>
                    <div class="form-group">
                        <label for="address.city">{"City: "}</label>
                        <Field<Registration> form={ form } field_name="address.city" oninput={ cb.clone() } />
                        <div class="invalid-feedback">
                            {form.field_message("address.city")}
                        </div>
                    </div>
                    <div class="form-group">
                        <label for="address.province">{"Province: "}</label>
                        <Field<Registration> form={ form } field_name="address.province" oninput={ cb.clone() } />
                        <div class="invalid-feedback">
                            {form.field_message("address.province")}
                        </div>
                    </div>
                    <div class="form-group">
                        <label for="address.country">{"Country (optional): "}</label>
                        <Field<Registration> form={ form } field_name="address.country" oninput={ cb.clone() } />
                        <div class="invalid-feedback">
                            {form.field_message("address.country")}
                        </div>
                    </div>
                    <div class="form-group">
                        <CheckBox<Registration>
                            field_name="accept_terms"
                            form={ form }
                        />
                        <label class="form-check-label" for="accept_terms">{"Accept Terms and Conditions: "}</label>
                        <div class="invalid-feedback">
                          {form.field_message("accept_terms")}
                        </div>
                    </div>
                    <div class="form-group">
                        <button
                            type="button"
                            onclick={ ctx.link().callback(|e: MouseEvent| {e.prevent_default(); AppMessage::Submit}) }
                        >
                            {"Submit"}
                        </button>
                    </div>
                </form>
                <div hidden={ !self.submitted }>
                    <h2>{"Form data"}</h2>
                    <p>{"First Name: "}{&form.model().first_name}</p>
                    <p>{"Last Name: "}{&form.model().last_name}</p>
                    <p>{"Street: "}{&form.model().address.street}</p>
                    <p>{"City: "}{&form.model().address.city}</p>
                    <p>{"Province: "}{&form.model().address.province}</p>
                    <p>{"Country: "}{&form.model().address.country}</p>
                    <p>{"Accepted Terms: "}{form.model().accept_terms}</p>
                </div>
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::start_app::<App>();
}

#[cfg(test)]
mod tests {
    use crate::{Address, Registration};
    use yew_form::model::FormValue;

    #[test]
    fn test_address() {
        let mut address = Address {
            street: "street_i".to_string(),
            city: "city_i".to_string(),
            province: "prov_i".to_string(),
            postal_code: "po_i".to_string(),
            country: "country_i".to_string(),
        };

        let mut fields = vec![];
        address.fields("", &mut fields);

        assert_eq!(fields.len(), 5);
        assert!(fields.contains(&String::from("street")));

        assert_eq!(address.value("street"), address.street.clone());

        assert!(address.set_value("street", "street_o").is_ok());

        assert_eq!(address.value("street"), String::from("street_o"));
    }

    #[test]
    fn test_composite() {
        let mut registration = Registration {
            first_name: "first_name_i".to_string(),
            last_name: "last_name_i".to_string(),
            quantity: 10,
            price: 5.99,
            address: Address {
                street: "street_i".to_string(),
                city: "city_i".to_string(),
                province: "prov_i".to_string(),
                postal_code: "po_i".to_string(),
                country: "country_i".to_string(),
            },
            accept_terms: false,
        };

        let mut fields = vec![];
        registration.fields("", &mut fields);

        assert_eq!(fields.len(), 10);
        assert!(fields.contains(&String::from("address.street")));

        assert_eq!(&registration.value("quantity"), "10");
        assert_eq!(&registration.value("price"), "5.99");

        let result = registration.set_value("quantity", "A");
        assert!(result.is_err());

        let result = registration.set_value("quantity", "12");
        assert!(result.is_ok());
        assert_eq!(&registration.value("quantity"), "12");

        assert_eq!(
            registration.value("address.street"),
            registration.address.street.clone()
        );

        assert!(registration.set_value("address.street", "street_o").is_ok());

        assert_eq!(
            registration.address.value("street"),
            String::from("street_o")
        );
    }

    #[test]
    #[should_panic]
    fn test_invalid_field_name() {
        let address = Address {
            street: "street_i".to_string(),
            city: "city_i".to_string(),
            province: "prov_i".to_string(),
            postal_code: "po_i".to_string(),
            country: "country_i".to_string(),
        };

        address.value("not_exist");
    }
}
