use yew::{Component, ComponentLink, Html, html, Properties, InputData, Callback, ShouldRender};

use crate::form::{Form};
use crate::{Model, FormField};

pub enum FieldMessage {
    OnInput(InputData)
}

fn default_text() -> String {
    String::from("text")
}

#[derive(Properties, PartialEq, Clone)]
pub struct FieldProperties<T: Model> {
    #[props(default = "default_text")]
    pub input_type: String,
    #[props(required)]
    pub field_name: String,
    #[props(required)]
    pub form: Form<T>,
    #[props(default = String::new)]
    pub placeholder: String,
    #[props(default = "Callback::noop")]
    pub oninput: Callback<InputData>,
}

pub struct Field<T: Model> {
    link: ComponentLink<Self>,
    pub input_type: String,
    pub field_name: String,
    pub form: Form<T>,
    pub placeholder: String,
    pub oninput: Callback<InputData>,
}

impl<T: Model> Field<T> {
    fn field(&self) -> &FormField<T> {
        self.form.field(&self.field_name)
    }

    pub fn field_name(&self) -> &str {
        &self.field_name
    }

    pub fn class(&self) -> &str {
        let field = self.field();

        if field.dirty && field.valid {
            "form-control is-valid"
        } else if field.dirty {
            "form-control is-invalid"
        } else {
            "form-control"
        }
    }


    pub fn message(&self) -> &str {
        &self.form.field_message(&self.field_name())
    }

    pub fn valid(&self) -> bool {
        self.form.field_valid(&self.field_name())
    }

    pub fn dirty(&self) -> bool {
        self.field().dirty
    }

    pub fn set_field(&mut self, field_name: &str, value: &str) {
        self.form.set_field(field_name, value)
    }
}

impl<T: Model> Component for Field<T> {
    type Message = FieldMessage;
    type Properties = FieldProperties<T>;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut form_field = Self {
            link,
            input_type: String::from(props.input_type),
            field_name: String::from(props.field_name),
            form: props.form,
            placeholder: String::from(props.placeholder),
            oninput: props.oninput,
        };

        if form_field.input_type == "" {
            form_field.input_type = String::from("text");
        }

        form_field
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            FieldMessage::OnInput(input_data) => {
                let state = self.form.state_mut();
                state.set_field(&self.field_name, &input_data.value);
                state.validate();

                self.oninput.emit(input_data);
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <input
                class=self.class()
                id=self.field_name
                type=self.input_type
                placeholder=self.placeholder
                value=self.form.field_value(&self.field_name)
                oninput=self.link.callback(|e: InputData| FieldMessage::OnInput(e))
            />
        }
    }
}