use crate::{Form, Model};
<<<<<<< HEAD
use yew::{html, Callback, Component, ComponentLink, Html, Properties};
=======
use yew::{html, Callback, Component, Context, Html, Properties};
>>>>>>> fa67514a4897880b89e3e13161797e6877d3f50b

pub enum CheckBoxMessage {
    OnToggle,
}

#[derive(Properties, PartialEq, Clone)]
pub struct CheckBoxProperties<T: Model> {
    pub field_name: String,
    pub form: Form<T>,
    #[prop_or_else(Callback::noop)]
    pub ontoggle: Callback<bool>,
}

pub struct CheckBox<T: Model> {
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

<<<<<<< HEAD
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props }
=======
    fn create(ctx: &yew::Context<Self>) -> Self {
        Self {
            props: ctx.props().clone(),
        }
>>>>>>> fa67514a4897880b89e3e13161797e6877d3f50b
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            CheckBoxMessage::OnToggle => {
                let value = !self.value();
                self.set_value(value);
                ctx.props().ontoggle.emit(value);
                true
            }
        }
    }

    fn changed(&mut self, _ctx: &yew::Context<Self>) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <input
                type="checkbox"
<<<<<<< HEAD
                value=self.props.field_name.clone()
                onclick=self.link.callback(|_e| CheckBoxMessage::OnToggle)
                checked=self.value()
=======
                value={ self.props.field_name.clone() }
                onclick={ ctx.link().callback(|_| CheckBoxMessage::OnToggle)}
                checked={ self.value() }
>>>>>>> fa67514a4897880b89e3e13161797e6877d3f50b
                class="form-check-input form-input"
             />
        }
    }
}
