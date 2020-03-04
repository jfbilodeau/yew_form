use crate::{Model, Form};
use yew::{Callback, ComponentLink, Component, Html, html, Properties, ClickEvent};

pub enum CheckBoxMessage {
    OnToggle,
}

#[derive(Properties, Clone)]
pub struct CheckBoxProperties<T: Model> {
    #[props(required)]
    pub field_name: String,
    #[props(required)]
    pub form: Form<T>,
    #[props(default = "Callback::noop")]
    pub ontoggle: Callback<bool>,
}

pub struct CheckBox<T: Model> {
    link: ComponentLink<Self>,
    props: CheckBoxProperties<T>,
}

impl<T: Model> CheckBox<T> {
    fn value(&self) -> bool {
        let field_path = &self.props.field_name;

        self.props.form.field_value(field_path) == "true"
    }

    fn set_value(&mut self, value: bool) {
        let field_path = &self.props.field_name;

        self.props.form.model_mut().set_value(field_path, &value.to_string());
    }
}

impl<T: Model> Component for CheckBox<T> {
    type Message = CheckBoxMessage;
    type Properties = CheckBoxProperties<T>;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props,
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            CheckBoxMessage::OnToggle => {
                let value = !self.value();
                self.set_value(value);
                self.props.ontoggle.emit(value);
                true
            }
        }
    }

    fn view(&self) -> Html {
        html! {
            <input
                type="checkbox"
                value=self.props.field_name
                onclick=self.link.callback(|e:ClickEvent| CheckBoxMessage::OnToggle)
                checked=self.value()
                class="form-check-input form-input"
             />
        }
    }
}

