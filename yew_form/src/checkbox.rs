use crate::{Form, Model};
use yew::{html, Callback, Component, ComponentLink, Html, Properties};

pub enum CheckBoxMessage {
    OnToggle,
}

#[derive(Properties, Clone)]
pub struct CheckBoxProperties<T: Model> {
    pub field_name: String,
    pub form: Form<T>,
    #[prop_or_else(Callback::noop)]
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

        self.props
            .form
            .set_field_value(field_path, &value.to_string());
    }
}

impl<T: Model + Clone> Component for CheckBox<T> {
    type Message = CheckBoxMessage;
    type Properties = CheckBoxProperties<T>;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props }
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

    fn change(&mut self, _props: Self::Properties) -> bool {
        true
    }

    fn view(&self) -> Html {
        html! {
            <input
                type="checkbox"
                value=self.props.field_name.clone()
                onclick=self.link.callback(|_e| CheckBoxMessage::OnToggle)
                checked=self.value()
                class="form-check-input form-input"
             />
        }
    }
}
