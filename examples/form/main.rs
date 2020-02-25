#![recursion_limit = "131072"]
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate validator_derive;

use stdweb::web::event::IEvent;
use validator::Validate;
use yew::{Callback, ClickEvent, Component, ComponentLink, Html, html, InputData};

use yew_form::{Field, Form, Model, text_field};
use regex::Regex;

lazy_static! {
    static ref PROVINCE_RE: Regex = Regex::new("^[A-Z]{2}$").unwrap();
}

// TODO: Remove Clone requirement
#[derive(Validate, PartialEq, Clone)]
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

#[derive(Validate, PartialEq, Clone)]
struct Registration {
    #[validate(length(min = 1, message="First name is required"))]
    first_name: String,
    #[validate(length(min = 1, message="Last name is required"))]
    last_name: String,
    #[validate]
    address: Address,
}

// TODO: derive model
impl Model for Registration {}

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
        };

        Self {
            link,
            form: Form::new(model, vec![
                // TODO: Derive automatically
                text_field!(first_name),
                text_field!(last_name),
                text_field!(address.street),
                text_field!(address.city),
                text_field!(address.province),
                text_field!(address.country),
            ]),
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
                    // TODO: Make oninput optional
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
                        <label for="address.country">{"Country: "}</label>
                        <Field<Registration> form=&self.form field_name="address.country" oninput=self.link.callback(|_: InputData| AppMessage::Update) />
                        <div class="invalid-feedback">
                          {&self.form.field_message("address.country")}
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
                </div>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<App>();
}