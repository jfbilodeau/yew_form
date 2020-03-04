#![recursion_limit = "131072"]
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate validator_derive;
extern crate yew_form;
#[macro_use]
extern crate yew_form_derive;

use regex::Regex;
use stdweb::web::event::IEvent;
use validator::{Validate, ValidationError};
use yew::{ClickEvent, Component, ComponentLink, Html, html, InputData};

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

// TODO: Remove Clone requirement
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
    #[validate]
    address: Address,
    #[validate(custom = "must_be_true")]
    accept_terms: bool,
}

// impl Model for Registration {
//     fn fields(&self, fields: &mut Vec<String>) {
//         fields.push(String::from("first_name"));
//         fields.push(String::from("last_name"));
//         fields.push(String::from("address.street"));
//         fields.push(String::from("accept_terms"));
//     }
//
//     fn set_value(&mut self, field_path: &str, value: &str) {
//         unimplemented!()
//     }
//
//     fn value(&self, field_path: &str) -> String {
//         match field_path {
//             "first_name" => self.first_name.to_string(),
//             "last_name" => self.last_name.to_string(),
//             "address.street" => self.address.get_value("street"),
//             "accept_terms" => self.accept_terms.to_string(),
//             _ => panic!(format!("Field {} does not exist", field_path))
//         }
//     }
// }

enum AppMessage {
    Update,
    Submit,
}

struct App {
    link: ComponentLink<Self>,
    form: Form<Registration>,
    submitted: bool,
}

impl Component for App {
    type Message = AppMessage;
    type Properties = ();

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
            accept_terms: false,
        };

        Self {
            link,
            form: Form::new(model),
            submitted: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            AppMessage::Update => true,
            AppMessage::Submit => {
                if self.form.validate() {
                    self.submitted = true;
                }
                true
            }
        }
    }

    fn view(&self) -> Html {
        html! {
            <div class="container-sm">
                <h1>{"Yew Form Example"}</h1>
                <p>{format!("Hello, {} {} and welcome to Yew Form!", self.form.model().first_name, self.form.model().last_name)}</p>
                <form>
                    // TODO: support additional attributes
                    // TODO: Remove hard-coded Bootstrap classes
                    // TODO: Update form without needing oninput
                    <div class="form-group">
                        <label for="first_name">{"First Name: "}</label>
                        <Field<Registration> form=&self.form field_name="first_name" oninput=self.link.callback(|_: InputData| AppMessage::Update) />
                        <div class="invalid-feedback">
                            {&self.form.field_message("first_name")}
                        </div>
                    </div>
                    <div class="form-group">
                        <label for="last_name">{"Last Name: "}</label>
                        <Field<Registration> form=&self.form field_name="last_name" oninput=self.link.callback(|_: InputData| AppMessage::Update) />
                        <div class="invalid-feedback">
                            {&self.form.field_message("last_name")}
                        </div>
                    </div>
                    <div class="form-group">
                        <label for="address.street">{"Street: "}</label>
                        <Field<Registration> form=&self.form field_name="address.street" oninput=self.link.callback(|_: InputData| AppMessage::Update) />
                        <div class="invalid-feedback">
                            {&self.form.field_message("address.street")}
                        </div>
                    </div>
                    <div class="form-group">
                        <label for="address.city">{"City: "}</label>
                        <Field<Registration> form=&self.form field_name="address.city" oninput=self.link.callback(|_: InputData| AppMessage::Update) />
                        <div class="invalid-feedback">
                            {&self.form.field_message("address.city")}
                        </div>
                    </div>
                    <div class="form-group">
                        <label for="address.province">{"Province: "}</label>
                        <Field<Registration> form=&self.form field_name="address.province" oninput=self.link.callback(|_: InputData| AppMessage::Update) />
                        <div class="invalid-feedback">
                            {&self.form.field_message("address.province")}
                        </div>
                    </div>
                    <div class="form-group">
                        <label for="address.country">{"Country (optional): "}</label>
                        <Field<Registration> form=&self.form field_name="address.country" oninput=self.link.callback(|_: InputData| AppMessage::Update) />
                        <div class="invalid-feedback">
                            {&self.form.field_message("address.country")}
                        </div>
                    </div>
                    <div class="form-group">
                        <CheckBox<Registration>
                            field_name="accept_terms"
                            form=&self.form
                        />
                        <label class="form-check-label" for="accept_terms">{"Accept Terms and Conditions: "}</label>
                        <div class="invalid-feedback">
                          {&self.form.field_message("accept_terms")}
                        </div>
                    </div>
                    <div class="form-group">
                        <button type="button" onclick=self.link.callback(|e: ClickEvent| {e.prevent_default(); AppMessage::Submit})>{"Submit"}</button>
                    </div>
                </form>
                <div hidden=!self.submitted>
                    <h2>{"Form data"}</h2>
                    <p>{"First Name: "}{&self.form.model().first_name}</p>
                    <p>{"Last Name: "}{&self.form.model().last_name}</p>
                    <p>{"Street: "}{&self.form.model().address.street}</p>
                    <p>{"City: "}{&self.form.model().address.city}</p>
                    <p>{"Province: "}{&self.form.model().address.province}</p>
                    <p>{"Country: "}{&self.form.model().address.country}</p>
                    <p>{"Accepted Terms: "}{self.form.model().accept_terms}</p>
                </div>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<App>();
}


#[cfg(test)]
mod tests {
    use crate::yew_form::Model;
    use crate::{Address, Registration};

    #[test]
    fn test_address() {
        let mut address = Address {
            street: "street_i".to_string(),
            city: "city_i".to_string(),
            province: "prov_i".to_string(),
            postal_code: "po_i".to_string(),
            country: "country_i".to_string()
        };

        let mut fields = vec![];
        address.fields(&mut fields);

        assert_eq!(fields.len(), 5);
        assert!(fields.contains(&String::from("street")));

        assert_eq!(address.value("street"), String::from(address.street.clone()));

        assert!(address.set_value("street", "street_o").is_ok());

        assert_eq!(address.value("street"), String::from("street_o"));
    }

    #[test]
    fn test_composite() {
        let mut registration = Registration {
            first_name: "first_name_i".to_string(),
            last_name: "last_name_i".to_string(),
            address: Address {
                street: "street_i".to_string(),
                city: "city_i".to_string(),
                province: "prov_i".to_string(),
                postal_code: "po_i".to_string(),
                country: "country_i".to_string()
            },
            accept_terms: false
        };

        let mut fields = vec![];
        registration.fields(&mut fields);

        assert_eq!(fields.len(), 8);
        assert!(fields.contains(&String::from("address.street")));

        assert_eq!(registration.value("address.street"), String::from(registration.address.street.clone()));

        registration.set_value("address.street", "street_o");

        assert_eq!(registration.address.value("street"), String::from("street_o"));
    }

    #[test]
    #[should_panic]
    fn test_invalid_field_name() {
        let address = Address {
            street: "street_i".to_string(),
            city: "city_i".to_string(),
            province: "prov_i".to_string(),
            postal_code: "po_i".to_string(),
            country: "country_i".to_string()
        };

        address.value("not_exist");
    }
}