use yew::{html, Callback, Component, Context, Html, InputEvent, Properties};

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
    #[prop_or_else(|| { "form-control".to_owned() })]
    pub class: String,
    #[prop_or_else(|| { "is-invalid".to_owned() })]
    pub class_invalid: String,
    #[prop_or_else(|| { "is-valid".to_owned() })]
    pub class_valid: String,
    #[prop_or_else(Callback::noop)]
    pub oninput: Callback<InputEvent>,
}

pub struct Field<T: Model> {
    pub autocomplete: String,
    pub input_type: String,
    pub field_name: String,
    pub form: Form<T>,
    pub placeholder: String,
    pub class: String,
    pub class_invalid: String,
    pub class_valid: String,
}

impl<T: Model> Field<T> {
    pub fn field_name(&self) -> &str {
        &self.field_name
    }

    pub fn class(&self) -> String {
        let s = self.form.state();
        let field = s.field(&self.field_name);

        if field.dirty && field.valid {
            format!("{} {}", self.class, self.class_valid)
        } else if field.dirty {
            format!("{} {}", self.class, self.class_invalid)
        } else {
            self.class.to_owned()
        }
    }

    pub fn message(&self) -> String {
        self.form.field_message(&self.field_name())
    }

    pub fn valid(&self) -> bool {
        self.form.field_valid(&self.field_name())
    }

    pub fn dirty(&self) -> bool {
        self.form.state().field(&self.field_name).dirty
    }

    pub fn set_field(&mut self, field_name: &str, value: &str) {
        self.form.set_field_value(field_name, value)
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

        if form_field.input_type == "" {
            form_field.input_type = String::from("text");
        }

        form_field
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            // TODO @Jacob -> we need to use something like gloo to access the DOM and get the
            // target of the InputEvent in order to access its value. Then we can call
            // state.set_field_value() with input_target.value
            FieldMessage::OnInput(input_data) => {
                let mut state = self.form.state_mut();
                // state.set_field_value(&self.field_name, &input_data.value);
                state.update_validation_field(&self.field_name);
                drop(state);

                ctx.props().oninput.emit(input_data);
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
                class={ self.class().to_string() }
                id={ self.field_name.clone() }
                type={ self.input_type.clone() }
                placeholder={ self.placeholder.clone() }
                autocomplete={ self.autocomplete.clone() }
                value={ self.form.field_value(&self.field_name) }
                oninput={ ctx.link().callback(|e: InputEvent| FieldMessage::OnInput(e)) }
            />
        }
    }
}
