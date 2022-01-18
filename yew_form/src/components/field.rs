use wasm_bindgen::JsCast;
use wasm_bindgen::UnwrapThrowExt;
use web_sys::Event;
use web_sys::HtmlInputElement;
use web_sys::InputEvent;
use yew::{classes, html, Callback, Classes, Component, Context, Html, Properties};

use crate::form::Form;
use crate::Model;

pub enum FieldMessage {
    OnInput(InputEvent),
}

fn default_text() -> String {
    String::from("text")
}

#[derive(Properties, PartialEq, Clone)]
pub struct FieldProperties<T: Model> {
    #[prop_or_else(|| { "off".to_owned() })]
    pub autocomplete: String,
    #[prop_or_else(default_text)]
    pub input_type: String,
    pub field_name: String,
    pub form: Form<T>,
    #[prop_or_else(String::new)]
    pub placeholder: String,
    pub disabled: Option<bool>,
    #[prop_or_else(|| { classes!("form-control") })]
    pub class: Classes,
    #[prop_or_else(|| { classes!("is-invalid") })]
    pub class_invalid: Classes,
    #[prop_or_else(|| { classes!("is-valid") })]
    pub class_valid: Classes,
    #[prop_or_else(Callback::noop)]
    pub oninput: Callback<InputEvent>,
}

pub struct Field<T: Model> {
    pub autocomplete: String,
    pub input_type: String,
    pub field_name: String,
    pub form: Form<T>,
    pub placeholder: String,
    pub class: Classes,
    pub class_invalid: Classes,
    pub class_valid: Classes,
}

impl<T: Model> Field<T> {
    pub fn field_name(&self) -> &str {
        &self.field_name
    }

    pub fn class(&self) -> Classes {
        let s = self.form.state();
        let field = s.field(&self.field_name);

        if field.dirty && field.valid {
            classes!(self.class.clone(), self.class_valid.clone())
        } else if field.dirty {
            classes!(self.class.clone(), self.class_invalid.clone())
        } else {
            self.class.to_owned()
        }
    }

    pub fn message(&self) -> String {
        self.form.field_message(self.field_name())
    }

    pub fn valid(&self) -> bool {
        self.form.field_valid(self.field_name())
    }

    pub fn dirty(&self) -> bool {
        self.form.state().field(&self.field_name).dirty
    }

    pub fn set_field(&mut self, field_name: &str, value: &str) {
        self.form.set_field_value(field_name, value)
    }
    pub fn get_input_value(&self, e: InputEvent) -> String {
        let event: Event = e.dyn_into().unwrap_throw();
        let event_target = event.target().unwrap_throw();
        let target: HtmlInputElement = event_target.dyn_into().unwrap_throw();
        target.value()
    }
}

impl<T: Model> Component for Field<T> {
    type Message = FieldMessage;
    type Properties = FieldProperties<T>;

    fn create(ctx: &Context<Self>) -> Self {
        let mut form_field = Self {
            autocomplete: String::from(&ctx.props().autocomplete),
            input_type: String::from(&ctx.props().input_type),
            field_name: String::from(&ctx.props().field_name),
            form: ctx.props().form.clone(),
            placeholder: String::from(&ctx.props().placeholder),
            class: ctx.props().class.clone(),
            class_invalid: ctx.props().class_invalid.clone(),
            class_valid: ctx.props().class_valid.clone(),
        };

        if form_field.input_type.is_empty() {
            form_field.input_type = String::from("text");
        }

        form_field
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            FieldMessage::OnInput(event) => {
                let value = self.get_input_value(event.clone());
                let mut state = self.form.state_mut();
                state.set_field_value(&self.field_name, &value);
                state.update_validation_field(&self.field_name);
                drop(state);
                ctx.props().oninput.emit(event);
                true
            }
        }
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <input
                class={self.class().to_string()}
                id={self.field_name.clone()}
                type={self.input_type.clone()}
                autocomplete={self.autocomplete.clone()}
                placeholder={self.placeholder.clone()}
                value={self.form.field_value(&self.field_name)}
                oninput={ctx.link().callback(FieldMessage::OnInput )}
                disabled={ctx.props().disabled.unwrap_or_default()}
            />
        }
    }
}
