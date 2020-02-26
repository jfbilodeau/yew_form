use crate::{Model, Form, FormFieldType};
use yew::{Callback, ComponentLink, Component, Html, html, Properties, ClickEvent};

pub enum CheckBoxMessage {
    OnToggle,
}

#[derive(Properties, PartialEq, Clone)]
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
        let field_name = &self.props.field_name;

        if let FormFieldType::Bool(getter, _) = self.props.form.field(field_name).field_type {
            getter(&self.props.form.model())
        } else {
            panic!(format!("Field {} is not bool", field_name))
        }
    }

    fn set_value(&mut self, value: bool) {
        let field_name = &self.props.field_name;

        if let FormFieldType::Bool(_, setter) = self.props.form.field(field_name).field_type {
            setter(self.props.form.model_mut(), value);
        } else {
            panic!(format!("Field {} is not bool", field_name));
        }
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
             />
        }
    }
}

