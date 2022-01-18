use wasm_bindgen::JsCast;
use wasm_bindgen::UnwrapThrowExt;
use web_sys::Event;
use web_sys::HtmlInputElement;
use web_sys::InputEvent;
use yew::{classes, html, Callback, Classes, Component, Context, Html, Properties};

use crate::form::Form;
use crate::Model;

pub enum FileMessage {
    OnInput(InputEvent),
}

#[derive(Properties, PartialEq, Clone)]
pub struct FilePropeties<T: Model> {
    pub form: Form<T>,
    pub field_name: String,
    #[prop_or_else(|| false )]
    pub disabled: bool,
    #[prop_or_else(|| false )]
    pub multiple: bool,
    #[prop_or_default]
    pub accept: String,
    #[prop_or_default]
    pub capture: String,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub class_valid: Classes,
    #[prop_or_default]
    pub class_invalid: Classes,
    #[prop_or_else(Callback::noop)]
    pub oninput: Callback<InputEvent>,
}

pub struct File<T: Model> {
    pub form: Form<T>,
    pub field_name: String,
    pub class: Classes,
    pub class_valid: Classes,
    pub class_invalid: Classes,
}

impl<T: Model> File<T> {
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
        self.form.state().field(self.field_name()).dirty
    }

    pub fn set_field(&mut self, field_name: &str, value: &str) {
        self.form.set_field_value(field_name, value)
    }

    pub fn get_select_value(&self, e: InputEvent) -> String {
        let event: Event = e.dyn_into().unwrap_throw();
        let event_target = event.target().unwrap_throw();
        let target: HtmlInputElement = event_target.dyn_into().unwrap_throw();
        target.value()
    }
}

impl<T: Model> Component for File<T> {
    type Message = FileMessage;
    type Properties = FilePropeties<T>;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            form: ctx.props().form.clone(),
            field_name: String::from(&ctx.props().field_name),
            class: ctx.props().class.clone(),
            class_valid: ctx.props().class_valid.clone(),
            class_invalid: ctx.props().class_invalid.clone(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            FileMessage::OnInput(event) => {
                let value = self.get_select_value(event.clone());
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
        let props = ctx.props();

        html! {
            <input
                id={self.field_name.clone()}
                type="file"
                name={self.field_name.clone()}
                disabled={props.disabled}
                accept={props.accept.clone()}
                multiple={props.multiple}
                class={self.class().to_string()}
                oninput={ctx.link().callback(FileMessage::OnInput)}
            />
        }
    }
}
