#![feature(prelude_import)]
#![recursion_limit = "131072"]

#[prelude_import]
use std::prelude::v1::*;

#[macro_use]
extern crate std;
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

#[allow(missing_copy_implementations)]
#[allow(non_camel_case_types)]
#[allow(dead_code)]
struct PROVINCE_RE {
    __private_field: (),
}

#[doc(hidden)]
static PROVINCE_RE: PROVINCE_RE = PROVINCE_RE {
    __private_field: (),
};

impl ::lazy_static::__Deref for PROVINCE_RE {
    type Target = Regex;
    fn deref(&self) -> &Regex {
        #[inline(always)]
        fn __static_ref_initialize() -> Regex {
            Regex::new("^[A-Z]{2}$").unwrap()
        }
        #[inline(always)]
        fn __stability() -> &'static Regex {
            static LAZY: ::lazy_static::lazy::Lazy<Regex> = ::lazy_static::lazy::Lazy::INIT;
            LAZY.get(__static_ref_initialize)
        }
        __stability()
    }
}

impl ::lazy_static::LazyStatic for PROVINCE_RE {
    fn initialize(lazy: &Self) {
        let _ = &**lazy;
    }
}

fn must_be_true(value: &bool) -> Result<(), ValidationError> {
    if value == &true {
        Ok(())
    } else {
        Err(ValidationError::new("Must accept terms before continuing"))
    }
}

struct Address {
    #[validate(length(min = 1, message = "Street is required"))]
    street: String,
    #[validate(length(min = 1, message = "City name is required"))]
    city: String,
    #[validate(regex(path = "PROVINCE_RE", message = "Enter 2 digit province code"))]
    province: String,
    postal_code: String,
    country: String,
}

impl ::yew_form::model::Model for Address {}

impl ::yew_form::model::FormValue for Address {
    fn fields(&self, prefix: &str, fields: &mut Vec<String>) {
        let field_prefix = if prefix == "" {
            String::new()
        } else {
            {
                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                    &["", "."],
                    &match (&prefix, ) {
                        (arg0, ) => [::core::fmt::ArgumentV1::new(
                            arg0,
                            ::core::fmt::Display::fmt,
                        )],
                    },
                ));
                res
            }
        };
        let field_path = {
            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                &["", ""],
                &match (&field_prefix, &"street") {
                    (arg0, arg1) => [
                        ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                        ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                    ],
                },
            ));
            res
        };
        self.street.fields(&field_path, fields);
        let field_path = {
            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                &["", ""],
                &match (&field_prefix, &"city") {
                    (arg0, arg1) => [
                        ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                        ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                    ],
                },
            ));
            res
        };
        self.city.fields(&field_path, fields);
        let field_path = {
            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                &["", ""],
                &match (&field_prefix, &"province") {
                    (arg0, arg1) => [
                        ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                        ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                    ],
                },
            ));
            res
        };
        self.province.fields(&field_path, fields);
        let field_path = {
            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                &["", ""],
                &match (&field_prefix, &"postal_code") {
                    (arg0, arg1) => [
                        ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                        ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                    ],
                },
            ));
            res
        };
        self.postal_code.fields(&field_path, fields);
        let field_path = {
            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                &["", ""],
                &match (&field_prefix, &"country") {
                    (arg0, arg1) => [
                        ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                        ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                    ],
                },
            ));
            res
        };
        self.country.fields(&field_path, fields);
    }
    fn value(&self, field_path: &str) -> String {
        let (field_name, suffix) = ::yew_form::split_field_path(field_path);
        match field_name {
            "street" => self.street.value(suffix),
            "city" => self.city.value(suffix),
            "province" => self.province.value(suffix),
            "postal_code" => self.postal_code.value(suffix),
            "country" => self.country.value(suffix),
            _ => ::std::rt::begin_panic({
                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                    &["Field ", " does not exist in "],
                    &match (&field_path, &"Address") {
                        (arg0, arg1) => [
                            ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                            ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                        ],
                    },
                ));
                res
            }),
        }
    }
    fn set_value(&mut self, field_path: &str, value: &str) -> Result<(), String> {
        let (field_name, suffix) = ::yew_form::split_field_path(field_path);
        match field_name {
            "street" => self.street.set_value(suffix, value),
            "city" => self.city.set_value(suffix, value),
            "province" => self.province.set_value(suffix, value),
            "postal_code" => self.postal_code.set_value(suffix, value),
            "country" => self.country.set_value(suffix, value),
            _ => ::std::rt::begin_panic({
                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                    &["Field ", " does not exist in "],
                    &match (&field_path, &"Address") {
                        (arg0, arg1) => [
                            ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                            ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                        ],
                    },
                ));
                res
            }),
        }
    }
}

impl Validate for Address {
    #[allow(unused_mut)]
    fn validate(&self) -> ::std::result::Result<(), ::validator::ValidationErrors> {
        let mut errors = ::validator::ValidationErrors::new();
        if !::validator::validate_length(
            ::validator::Validator::Length {
                min: ::std::option::Option::Some(1u64),
                max: ::std::option::Option::None,
                equal: ::std::option::Option::None,
            },
            &self.street,
        ) {
            let mut err = ::validator::ValidationError::new("length");
            err.message = Some(::std::borrow::Cow::from("Street is required"));
            err.add_param(::std::borrow::Cow::from("min"), &1u64);
            err.add_param(::std::borrow::Cow::from("value"), &&self.street);
            errors.add("street", err);
        }
        if !::validator::validate_length(
            ::validator::Validator::Length {
                min: ::std::option::Option::Some(1u64),
                max: ::std::option::Option::None,
                equal: ::std::option::Option::None,
            },
            &self.city,
        ) {
            let mut err = ::validator::ValidationError::new("length");
            err.message = Some(::std::borrow::Cow::from("City name is required"));
            err.add_param(::std::borrow::Cow::from("min"), &1u64);
            err.add_param(::std::borrow::Cow::from("value"), &&self.city);
            errors.add("city", err);
        }
        if !PROVINCE_RE.is_match(&self.province) {
            let mut err = ::validator::ValidationError::new("regex");
            err.message = Some(::std::borrow::Cow::from("Enter 2 digit province code"));
            err.add_param(::std::borrow::Cow::from("value"), &&self.province);
            errors.add("province", err);
        }
        let mut result = if errors.is_empty() {
            ::std::result::Result::Ok(())
        } else {
            ::std::result::Result::Err(errors)
        };
        result
    }
}

impl ::core::marker::StructuralPartialEq for Address {}

#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::cmp::PartialEq for Address {
    #[inline]
    fn eq(&self, other: &Address) -> bool {
        match *other {
            Address {
                street: ref __self_1_0,
                city: ref __self_1_1,
                province: ref __self_1_2,
                postal_code: ref __self_1_3,
                country: ref __self_1_4,
            } => match *self {
                Address {
                    street: ref __self_0_0,
                    city: ref __self_0_1,
                    province: ref __self_0_2,
                    postal_code: ref __self_0_3,
                    country: ref __self_0_4,
                } => {
                    (*__self_0_0) == (*__self_1_0)
                        && (*__self_0_1) == (*__self_1_1)
                        && (*__self_0_2) == (*__self_1_2)
                        && (*__self_0_3) == (*__self_1_3)
                        && (*__self_0_4) == (*__self_1_4)
                }
            },
        }
    }
    #[inline]
    fn ne(&self, other: &Address) -> bool {
        match *other {
            Address {
                street: ref __self_1_0,
                city: ref __self_1_1,
                province: ref __self_1_2,
                postal_code: ref __self_1_3,
                country: ref __self_1_4,
            } => match *self {
                Address {
                    street: ref __self_0_0,
                    city: ref __self_0_1,
                    province: ref __self_0_2,
                    postal_code: ref __self_0_3,
                    country: ref __self_0_4,
                } => {
                    (*__self_0_0) != (*__self_1_0)
                        || (*__self_0_1) != (*__self_1_1)
                        || (*__self_0_2) != (*__self_1_2)
                        || (*__self_0_3) != (*__self_1_3)
                        || (*__self_0_4) != (*__self_1_4)
                }
            },
        }
    }
}

#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::clone::Clone for Address {
    #[inline]
    fn clone(&self) -> Address {
        match *self {
            Address {
                street: ref __self_0_0,
                city: ref __self_0_1,
                province: ref __self_0_2,
                postal_code: ref __self_0_3,
                country: ref __self_0_4,
            } => Address {
                street: ::core::clone::Clone::clone(&(*__self_0_0)),
                city: ::core::clone::Clone::clone(&(*__self_0_1)),
                province: ::core::clone::Clone::clone(&(*__self_0_2)),
                postal_code: ::core::clone::Clone::clone(&(*__self_0_3)),
                country: ::core::clone::Clone::clone(&(*__self_0_4)),
            },
        }
    }
}

struct Registration {
    #[validate(length(min = 1, message = "First name is required"))]
    first_name: String,
    #[validate(length(min = 1, message = "Last name is required"))]
    last_name: String,
    quantity: u32,
    price: f64,
    #[validate]
    address: Address,
    #[validate(custom = "must_be_true")]
    accept_terms: bool,
}

impl ::yew_form::model::Model for Registration {}

impl ::yew_form::model::FormValue for Registration {
    fn fields(&self, prefix: &str, fields: &mut Vec<String>) {
        let field_prefix = if prefix == "" {
            String::new()
        } else {
            {
                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                    &["", "."],
                    &match (&prefix, ) {
                        (arg0, ) => [::core::fmt::ArgumentV1::new(
                            arg0,
                            ::core::fmt::Display::fmt,
                        )],
                    },
                ));
                res
            }
        };
        let field_path = {
            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                &["", ""],
                &match (&field_prefix, &"first_name") {
                    (arg0, arg1) => [
                        ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                        ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                    ],
                },
            ));
            res
        };
        self.first_name.fields(&field_path, fields);
        let field_path = {
            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                &["", ""],
                &match (&field_prefix, &"last_name") {
                    (arg0, arg1) => [
                        ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                        ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                    ],
                },
            ));
            res
        };
        self.last_name.fields(&field_path, fields);
        let field_path = {
            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                &["", ""],
                &match (&field_prefix, &"quantity") {
                    (arg0, arg1) => [
                        ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                        ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                    ],
                },
            ));
            res
        };
        self.quantity.fields(&field_path, fields);
        let field_path = {
            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                &["", ""],
                &match (&field_prefix, &"price") {
                    (arg0, arg1) => [
                        ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                        ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                    ],
                },
            ));
            res
        };
        self.price.fields(&field_path, fields);
        let field_path = {
            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                &["", ""],
                &match (&field_prefix, &"address") {
                    (arg0, arg1) => [
                        ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                        ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                    ],
                },
            ));
            res
        };
        self.address.fields(&field_path, fields);
        let field_path = {
            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                &["", ""],
                &match (&field_prefix, &"accept_terms") {
                    (arg0, arg1) => [
                        ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                        ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                    ],
                },
            ));
            res
        };
        self.accept_terms.fields(&field_path, fields);
    }
    fn value(&self, field_path: &str) -> String {
        let (field_name, suffix) = ::yew_form::split_field_path(field_path);
        match field_name {
            "first_name" => self.first_name.value(suffix),
            "last_name" => self.last_name.value(suffix),
            "quantity" => self.quantity.value(suffix),
            "price" => self.price.value(suffix),
            "address" => self.address.value(suffix),
            "accept_terms" => self.accept_terms.value(suffix),
            _ => ::std::rt::begin_panic({
                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                    &["Field ", " does not exist in "],
                    &match (&field_path, &"Registration") {
                        (arg0, arg1) => [
                            ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                            ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                        ],
                    },
                ));
                res
            }),
        }
    }
    fn set_value(&mut self, field_path: &str, value: &str) -> Result<(), String> {
        let (field_name, suffix) = ::yew_form::split_field_path(field_path);
        match field_name {
            "first_name" => self.first_name.set_value(suffix, value),
            "last_name" => self.last_name.set_value(suffix, value),
            "quantity" => self.quantity.set_value(suffix, value),
            "price" => self.price.set_value(suffix, value),
            "address" => self.address.set_value(suffix, value),
            "accept_terms" => self.accept_terms.set_value(suffix, value),
            _ => ::std::rt::begin_panic({
                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                    &["Field ", " does not exist in "],
                    &match (&field_path, &"Registration") {
                        (arg0, arg1) => [
                            ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                            ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                        ],
                    },
                ));
                res
            }),
        }
    }
}

impl Validate for Registration {
    #[allow(unused_mut)]
    fn validate(&self) -> ::std::result::Result<(), ::validator::ValidationErrors> {
        let mut errors = ::validator::ValidationErrors::new();
        if !::validator::validate_length(
            ::validator::Validator::Length {
                min: ::std::option::Option::Some(1u64),
                max: ::std::option::Option::None,
                equal: ::std::option::Option::None,
            },
            &self.first_name,
        ) {
            let mut err = ::validator::ValidationError::new("length");
            err.message = Some(::std::borrow::Cow::from("First name is required"));
            err.add_param(::std::borrow::Cow::from("min"), &1u64);
            err.add_param(::std::borrow::Cow::from("value"), &&self.first_name);
            errors.add("first_name", err);
        }
        if !::validator::validate_length(
            ::validator::Validator::Length {
                min: ::std::option::Option::Some(1u64),
                max: ::std::option::Option::None,
                equal: ::std::option::Option::None,
            },
            &self.last_name,
        ) {
            let mut err = ::validator::ValidationError::new("length");
            err.message = Some(::std::borrow::Cow::from("Last name is required"));
            err.add_param(::std::borrow::Cow::from("min"), &1u64);
            err.add_param(::std::borrow::Cow::from("value"), &&self.last_name);
            errors.add("last_name", err);
        }
        match must_be_true(&self.accept_terms) {
            ::std::result::Result::Ok(()) => (),
            ::std::result::Result::Err(mut err) => {
                err.add_param(::std::borrow::Cow::from("value"), &&self.accept_terms);
                errors.add("accept_terms", err);
            }
        };
        let mut result = if errors.is_empty() {
            ::std::result::Result::Ok(())
        } else {
            ::std::result::Result::Err(errors)
        };
        result = ::validator::ValidationErrors::merge(result, "address", self.address.validate());
        result
    }
}

impl ::core::marker::StructuralPartialEq for Registration {}

#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::cmp::PartialEq for Registration {
    #[inline]
    fn eq(&self, other: &Registration) -> bool {
        match *other {
            Registration {
                first_name: ref __self_1_0,
                last_name: ref __self_1_1,
                quantity: ref __self_1_2,
                price: ref __self_1_3,
                address: ref __self_1_4,
                accept_terms: ref __self_1_5,
            } => match *self {
                Registration {
                    first_name: ref __self_0_0,
                    last_name: ref __self_0_1,
                    quantity: ref __self_0_2,
                    price: ref __self_0_3,
                    address: ref __self_0_4,
                    accept_terms: ref __self_0_5,
                } => {
                    (*__self_0_0) == (*__self_1_0)
                        && (*__self_0_1) == (*__self_1_1)
                        && (*__self_0_2) == (*__self_1_2)
                        && (*__self_0_3) == (*__self_1_3)
                        && (*__self_0_4) == (*__self_1_4)
                        && (*__self_0_5) == (*__self_1_5)
                }
            },
        }
    }
    #[inline]
    fn ne(&self, other: &Registration) -> bool {
        match *other {
            Registration {
                first_name: ref __self_1_0,
                last_name: ref __self_1_1,
                quantity: ref __self_1_2,
                price: ref __self_1_3,
                address: ref __self_1_4,
                accept_terms: ref __self_1_5,
            } => match *self {
                Registration {
                    first_name: ref __self_0_0,
                    last_name: ref __self_0_1,
                    quantity: ref __self_0_2,
                    price: ref __self_0_3,
                    address: ref __self_0_4,
                    accept_terms: ref __self_0_5,
                } => {
                    (*__self_0_0) != (*__self_1_0)
                        || (*__self_0_1) != (*__self_1_1)
                        || (*__self_0_2) != (*__self_1_2)
                        || (*__self_0_3) != (*__self_1_3)
                        || (*__self_0_4) != (*__self_1_4)
                        || (*__self_0_5) != (*__self_1_5)
                }
            },
        }
    }
}

#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::clone::Clone for Registration {
    #[inline]
    fn clone(&self) -> Registration {
        match *self {
            Registration {
                first_name: ref __self_0_0,
                last_name: ref __self_0_1,
                quantity: ref __self_0_2,
                price: ref __self_0_3,
                address: ref __self_0_4,
                accept_terms: ref __self_0_5,
            } => Registration {
                first_name: ::core::clone::Clone::clone(&(*__self_0_0)),
                last_name: ::core::clone::Clone::clone(&(*__self_0_1)),
                quantity: ::core::clone::Clone::clone(&(*__self_0_2)),
                price: ::core::clone::Clone::clone(&(*__self_0_3)),
                address: ::core::clone::Clone::clone(&(*__self_0_4)),
                accept_terms: ::core::clone::Clone::clone(&(*__self_0_5)),
            },
        }
    }
}

impl Registration {
    pub fn total(&self) -> f64 {
        self.quantity as f64 * self.price
    }
}

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
        let model = Registration {
            first_name: String::from("J-F"),
            last_name: String::from("Bilodeau"),
            quantity: 10,
            price: 5.99,
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
        {
            enum ProcMacroHack {
                Nested = ("< div class = \"container-sm\" > < h1 > { \"Yew Form Example\" } < / h1 > < p >\n{\n    format !\n    (\"Hello, {} {} and welcome to Yew Form!\", self . form . model () .\n     first_name, self . form . model () . last_name)\n} < / p > < form > < div class = \"form-group\" > < label for = \"first_name\" >\n{ \"First Name: \" } < / label > < Field < Registration > form = & self . form\nfield_name = \"first_name\" oninput = self . link . callback\n(| _ : InputData | AppMessage :: Update) / > < div class = \"invalid-feedback\"\n> { & self . form . field_message (\"first_name\") } < / div > < / div > < div\nclass = \"form-group\" > < label for = \"last_name\" > { \"Last Name: \" } < / label\n> < Field < Registration > form = & self . form field_name = \"last_name\"\noninput = self . link . callback (| _ : InputData | AppMessage :: Update) / >\n< div class = \"invalid-feedback\" >\n{ & self . form . field_message (\"last_name\") } < / div > < / div > < div\nclass = \"form-group\" > < label for = \"last_name\" > { \"Quantity: \" } < / label\n> < Field < Registration > form = & self . form field_name = \"quantity\"\noninput = self . link . callback (| _ : InputData | AppMessage :: Update) / >\n< div class = \"invalid-feedback\" >\n{ & self . form . field_message (\"quantity\") } < / div > < / div > < div class\n= \"form-group\" > < label for = \"last_name\" > { \"Price: \" } < / label > < Field\n< Registration > form = & self . form field_name = \"price\" oninput = self .\nlink . callback (| _ : InputData | AppMessage :: Update) / > < div class =\n\"invalid-feedback\" > { & self . form . field_message (\"price\") } < / div > < /\ndiv > < div > { \"Total: \" }\n{ format ! (\"{:.2}\", self . form . model () . total ()) } < / div > < div\nclass = \"form-group\" > < label for = \"address.street\" > { \"Street: \" } < /\nlabel > < Field < Registration > form = & self . form field_name =\n\"address.street\" oninput = self . link . callback\n(| _ : InputData | AppMessage :: Update) / > < div class = \"invalid-feedback\"\n> { & self . form . field_message (\"address.street\") } < / div > < / div > <\ndiv class = \"form-group\" > < label for = \"address.city\" > { \"City: \" } < /\nlabel > < Field < Registration > form = & self . form field_name =\n\"address.city\" oninput = self . link . callback\n(| _ : InputData | AppMessage :: Update) / > < div class = \"invalid-feedback\"\n> { & self . form . field_message (\"address.city\") } < / div > < / div > < div\nclass = \"form-group\" > < label for = \"address.province\" > { \"Province: \" } < /\nlabel > < Field < Registration > form = & self . form field_name =\n\"address.province\" oninput = self . link . callback\n(| _ : InputData | AppMessage :: Update) / > < div class = \"invalid-feedback\"\n> { & self . form . field_message (\"address.province\") } < / div > < / div > <\ndiv class = \"form-group\" > < label for = \"address.country\" >\n{ \"Country (optional): \" } < / label > < Field < Registration > form = & self\n. form field_name = \"address.country\" oninput = self . link . callback\n(| _ : InputData | AppMessage :: Update) / > < div class = \"invalid-feedback\"\n> { & self . form . field_message (\"address.country\") } < / div > < / div > <\ndiv class = \"form-group\" > < CheckBox < Registration > field_name =\n\"accept_terms\" form = & self . form / > < label class = \"form-check-label\" for\n= \"accept_terms\" > { \"Accept Terms and Conditions: \" } < / label > < div class\n= \"invalid-feedback\" > { & self . form . field_message (\"accept_terms\") } < /\ndiv > < / div > < div class = \"form-group\" > < button type = \"button\" onclick\n= self . link . callback\n(| e : ClickEvent | { e . prevent_default () ; AppMessage :: Submit }) >\n{ \"Submit\" } < / button > < / div > < / form > < div hidden = ! self .\nsubmitted > < h2 > { \"Form data\" } < / h2 > < p > { \"First Name: \" }\n{ & self . form . model () . first_name } < / p > < p > { \"Last Name: \" }\n{ & self . form . model () . last_name } < / p > < p > { \"Street: \" }\n{ & self . form . model () . address . street } < / p > < p > { \"City: \" }\n{ & self . form . model () . address . city } < / p > < p > { \"Province: \" }\n{ & self . form . model () . address . province } < / p > < p >\n{ \"Country: \" } { & self . form . model () . address . country } < / p > < p >\n{ \"Accepted Terms: \" } { self . form . model () . accept_terms } < / p > < /\ndiv > < / div >", 0).1,
            }
            macro_rules! proc_macro_call_3 { ( ) => { :: yew :: virtual_dom :: VNode :: from ( { let mut __yew_vtag = :: yew :: virtual_dom :: vtag :: VTag :: new ( "div" ) ; __yew_vtag . set_classes ( "container-sm" ) ; __yew_vtag . add_attributes ( vec ! [ ] ) ; __yew_vtag . add_listeners ( vec ! [ ] ) ; __yew_vtag . add_children ( vec ! [ :: yew :: virtual_dom :: VNode :: from ( { let mut __yew_vtag = :: yew :: virtual_dom :: vtag :: VTag :: new ( "h1" ) ; __yew_vtag . add_attributes ( vec ! [ ] ) ; __yew_vtag . add_listeners ( vec ! [ ] ) ; __yew_vtag . add_children ( vec ! [ :: yew :: virtual_dom :: VNode :: from ( "Yew Form Example" ) ] ) ; :: yew :: virtual_dom :: VNode :: from ( __yew_vtag ) } ) , :: yew :: virtual_dom :: VNode :: from ( { let mut __yew_vtag = :: yew :: virtual_dom :: vtag :: VTag :: new ( "p" ) ; __yew_vtag . add_attributes ( vec ! [ ] ) ; __yew_vtag . add_listeners ( vec ! [ ] ) ; __yew_vtag . add_children ( vec ! [ :: yew :: virtual_dom :: VNode :: from ( { format ! ( "Hello, {} {} and welcome to Yew Form!" , self . form . model ( ) . first_name , self . form . model ( ) . last_name ) } ) ] ) ; :: yew :: virtual_dom :: VNode :: from ( __yew_vtag ) } ) , :: yew :: virtual_dom :: VNode :: from ( { let mut __yew_vtag = :: yew :: virtual_dom :: vtag :: VTag :: new ( "form" ) ; __yew_vtag . add_attributes ( vec ! [ ] ) ; __yew_vtag . add_listeners ( vec ! [ ] ) ; __yew_vtag . add_children ( vec ! [ :: yew :: virtual_dom :: VNode :: from ( { let mut __yew_vtag = :: yew :: virtual_dom :: vtag :: VTag :: new ( "div" ) ; __yew_vtag . set_classes ( "form-group" ) ; __yew_vtag . add_attributes ( vec ! [ ] ) ; __yew_vtag . add_listeners ( vec ! [ ] ) ; __yew_vtag . add_children ( vec ! [ :: yew :: virtual_dom :: VNode :: from ( { let mut __yew_vtag = :: yew :: virtual_dom :: vtag :: VTag :: new ( "label" ) ; __yew_vtag . add_attributes ( vec ! [ ( "for" . to_owned ( ) , ( "first_name" ) . to_string ( ) ) ] ) ; __yew_vtag . add_listeners ( vec ! [ ] ) ; __yew_vtag . add_children ( vec ! [ :: yew :: virtual_dom :: VNode :: from ( "First Name: " ) ] ) ; :: yew :: virtual_dom :: VNode :: from ( __yew_vtag ) } ) , :: yew :: virtual_dom :: VNode :: from ( { if false { trait __yew_validate_comp : :: yew :: html :: Component { } impl __yew_validate_comp for Field < Registration > { } let _ = | props : < Field < Registration > as :: yew :: html :: Component > :: Properties | { props . field_name ; props . form ; props . oninput ; } ; } :: yew :: virtual_dom :: VChild :: < Field < Registration > > :: new ( << Field < Registration > as :: yew :: html :: Component > :: Properties as :: yew :: html :: Properties > :: builder ( ) . field_name ( < :: yew :: virtual_dom :: vcomp :: VComp as :: yew :: virtual_dom :: Transformer < _ , _ >> :: transform ( "first_name" ) ) . form ( < :: yew :: virtual_dom :: vcomp :: VComp as :: yew :: virtual_dom :: Transformer < _ , _ >> :: transform ( & self . form ) ) . oninput ( < :: yew :: virtual_dom :: vcomp :: VComp as :: yew :: virtual_dom :: Transformer < _ , _ >> :: transform ( self . link . callback ( | _ : InputData | AppMessage :: Update ) ) ) . build ( ) , :: yew :: html :: NodeRef :: default ( ) ) } ) , :: yew :: virtual_dom :: VNode :: from ( { let mut __yew_vtag = :: yew :: virtual_dom :: vtag :: VTag :: new ( "div" ) ; __yew_vtag . set_classes ( "invalid-feedback" ) ; __yew_vtag . add_attributes ( vec ! [ ] ) ; __yew_vtag . add_listeners ( vec ! [ ] ) ; __yew_vtag . add_children ( vec ! [ :: yew :: virtual_dom :: VNode :: from ( { & self . form . field_message ( "first_name" ) } ) ] ) ; :: yew :: virtual_dom :: VNode :: from ( __yew_vtag ) } ) ] ) ; :: yew :: virtual_dom :: VNode :: from ( __yew_vtag ) } ) , :: yew :: virtual_dom :: VNode :: from ( { let mut __yew_vtag = :: yew :: virtual_dom :: vtag :: VTag :: new ( "div" ) ; __yew_vtag . set_classes ( "form-group" ) ; __yew_vtag . add_attributes ( vec ! [ ] ) ; __yew_vtag . add_listeners ( vec ! [ ] ) ; __yew_vtag . add_children ( vec ! [ :: yew :: virtual_dom :: VNode :: from ( { let mut __yew_vtag = :: yew :: virtual_dom :: vtag :: VTag :: new ( "label" ) ; __yew_vtag . add_attributes ( vec ! [ ( "for" . to_owned ( ) , ( "last_name" ) . to_string ( ) ) ] ) ; __yew_vtag . add_listeners ( vec ! [ ] ) ; __yew_vtag . add_children ( vec ! [ :: yew :: virtual_dom :: VNode :: from ( "Last Name: " ) ] ) ; :: yew :: virtual_dom :: VNode :: from ( __yew_vtag ) } ) , :: yew :: virtual_dom :: VNode :: from ( { if false { trait __yew_validate_comp : :: yew :: html :: Component { } impl __yew_validate_comp for Field < Registration > { } let _ = | props : < Field < Registration > as :: yew :: html :: Component > :: Properties | { props . field_name ; props . form ; props . oninput ; } ; } :: yew :: virtual_dom :: VChild :: < Field < Registration > > :: new ( << Field < Registration > as :: yew :: html :: Component > :: Properties as :: yew :: html :: Properties > :: builder ( ) . field_name ( < :: yew :: virtual_dom :: vcomp :: VComp as :: yew :: virtual_dom :: Transformer < _ , _ >> :: transform ( "last_name" ) ) . form ( < :: yew :: virtual_dom :: vcomp :: VComp as :: yew :: virtual_dom :: Transformer < _ , _ >> :: transform ( & self . form ) ) . oninput ( < :: yew :: virtual_dom :: vcomp :: VComp as :: yew :: virtual_dom :: Transformer < _ , _ >> :: transform ( self . link . callback ( | _ : InputData | AppMessage :: Update ) ) ) . build ( ) , :: yew :: html :: NodeRef :: default ( ) ) } ) , :: yew :: virtual_dom :: VNode :: from ( { let mut __yew_vtag = :: yew :: virtual_dom :: vtag :: VTag :: new ( "div" ) ; __yew_vtag . set_classes ( "invalid-feedback" ) ; __yew_vtag . add_attributes ( vec ! [ ] ) ; __yew_vtag . add_listeners ( vec ! [ ] ) ; __yew_vtag . add_children ( vec ! [ :: yew :: virtual_dom :: VNode :: from ( { & self . form . field_message ( "last_name" ) } ) ] ) ; :: yew :: virtual_dom :: VNode :: from ( __yew_vtag ) } ) ] ) ; :: yew :: virtual_dom :: VNode :: from ( __yew_vtag ) } ) , :: yew :: virtual_dom :: VNode :: from ( { let mut __yew_vtag = :: yew :: virtual_dom :: vtag :: VTag :: new ( "div" ) ; __yew_vtag . set_classes ( "form-group" ) ; __yew_vtag . add_attributes ( vec ! [ ] ) ; __yew_vtag . add_listeners ( vec ! [ ] ) ; __yew_vtag . add_children ( vec ! [ :: yew :: virtual_dom :: VNode :: from ( { let mut __yew_vtag = :: yew :: virtual_dom :: vtag :: VTag :: new ( "label" ) ; __yew_vtag . add_attributes ( vec ! [ ( "for" . to_owned ( ) , ( "last_name" ) . to_string ( ) ) ] ) ; __yew_vtag . add_listeners ( vec ! [ ] ) ; __yew_vtag . add_children ( vec ! [ :: yew :: virtual_dom :: VNode :: from ( "Quantity: " ) ] ) ; :: yew :: virtual_dom :: VNode :: from ( __yew_vtag ) } ) , :: yew :: virtual_dom :: VNode :: from ( { if false { trait __yew_validate_comp : :: yew :: html :: Component { } impl __yew_validate_comp for Field < Registration > { } let _ = | props : < Field < Registration > as :: yew :: html :: Component > :: Properties | { props . field_name ; props . form ; props . oninput ; } ; } :: yew :: virtual_dom :: VChild :: < Field < Registration > > :: new ( << Field < Registration > as :: yew :: html :: Component > :: Properties as :: yew :: html :: Properties > :: builder ( ) . field_name ( < :: yew :: virtual_dom :: vcomp :: VComp as :: yew :: virtual_dom :: Transformer < _ , _ >> :: transform ( "quantity" ) ) . form ( < :: yew :: virtual_dom :: vcomp :: VComp as :: yew :: virtual_dom :: Transformer < _ , _ >> :: transform ( & self . form ) ) . oninput ( < :: yew :: virtual_dom :: vcomp :: VComp as :: yew :: virtual_dom :: Transformer < _ , _ >> :: transform ( self . link . callback ( | _ : InputData | AppMessage :: Update ) ) ) . build ( ) , :: yew :: html :: NodeRef :: default ( ) ) } ) , :: yew :: virtual_dom :: VNode :: from ( { let mut __yew_vtag = :: yew :: virtual_dom :: vtag :: VTag :: new ( "div" ) ; __yew_vtag . set_classes ( "invalid-feedback" ) ; __yew_vtag . add_attributes ( vec ! [ ] ) ; __yew_vtag . add_listeners ( vec ! [ ] ) ; __yew_vtag . add_children ( vec ! [ :: yew :: virtual_dom :: VNode :: from ( { & self . form . field_message ( "quantity" ) } ) ] ) ; :: yew :: virtual_dom :: VNode :: from ( __yew_vtag ) } ) ] ) ; :: yew :: virtual_dom :: VNode :: from ( __yew_vtag ) } ) , :: yew :: virtual_dom :: VNode :: from ( { let mut __yew_vtag = :: yew :: virtual_dom :: vtag :: VTag :: new ( "div" ) ; __yew_vtag . set_classes ( "form-group" ) ; __yew_vtag . add_attributes ( vec ! [ ] ) ; __yew_vtag . add_listeners ( vec ! [ ] ) ; __yew_vtag . add_children ( vec ! [ :: yew :: virtual_dom :: VNode :: from ( { let mut __yew_vtag = :: yew :: virtual_dom :: vtag :: VTag :: new ( "label" ) ; __yew_vtag . add_attributes ( vec ! [ ( "for" . to_owned ( ) , ( "last_name" ) . to_string ( ) ) ] ) ; __yew_vtag . add_listeners ( vec ! [ ] ) ; __yew_vtag . add_children ( vec ! [ :: yew :: virtual_dom :: VNode :: from ( "Price: " ) ] ) ; :: yew :: virtual_dom :: VNode :: from ( __yew_vtag ) } ) , :: yew :: virtual_dom :: VNode :: from ( { if false { trait __yew_validate_comp : :: yew :: html :: Component { } impl __yew_validate_comp for Field < Registration > { } let _ = | props : < Field < Registration > as :: yew :: html :: Component > :: Properties | { props . field_name ; props . form ; props . oninput ; } ; } :: yew :: virtual_dom :: VChild :: < Field < Registration > > :: new ( << Field < Registration > as :: yew :: html :: Component > :: Properties as :: yew :: html :: Properties > :: builder ( ) . field_name ( < :: yew :: virtual_dom :: vcomp :: VComp as :: yew :: virtual_dom :: Transformer < _ , _ >> :: transform ( "price" ) ) . form ( < :: yew :: virtual_dom :: vcomp :: VComp as :: yew :: virtual_dom :: Transformer < _ , _ >> :: transform ( & self . form ) ) . oninput ( < :: yew :: virtual_dom :: vcomp :: VComp as :: yew :: virtual_dom :: Transformer < _ , _ >> :: transform ( self . link . callback ( | _ : InputData | AppMessage :: Update ) ) ) . build ( ) , :: yew :: html :: NodeRef :: default ( ) ) } ) , :: yew :: virtual_dom :: VNode :: from ( { let mut __yew_vtag = :: yew :: virtual_dom :: vtag :: VTag :: new ( "div" ) ; __yew_vtag . set_classes ( "invalid-feedback" ) ; __yew_vtag . add_attributes ( vec ! [ ] ) ; __yew_vtag . add_listeners ( vec ! [ ] ) ; __yew_vtag . add_children ( vec ! [ :: yew :: virtual_dom :: VNode :: from ( { & self . form . field_message ( "price" ) } ) ] ) ; :: yew :: virtual_dom :: VNode :: from ( __yew_vtag ) } ) ] ) ; :: yew :: virtual_dom :: VNode :: from ( __yew_vtag ) } ) , :: yew :: virtual_dom :: VNode :: from ( { let mut __yew_vtag = :: yew :: virtual_dom :: vtag :: VTag :: new ( "div" ) ; __yew_vtag . add_attributes ( vec ! [ ] ) ; __yew_vtag . add_listeners ( vec ! [ ] ) ; __yew_vtag . add_children ( vec ! [ :: yew :: virtual_dom :: VNode :: from ( "Total: " ) , :: yew :: virtual_dom :: VNode :: from ( { format ! ( "{:.2}" , self . form . model ( ) . total ( ) ) } ) ] ) ; :: yew :: virtual_dom :: VNode :: from ( __yew_vtag ) } ) , :: yew :: virtual_dom :: VNode :: from ( { let mut __yew_vtag = :: yew :: virtual_dom :: vtag :: VTag :: new ( "div" ) ; __yew_vtag . set_classes ( "form-group" ) ; __yew_vtag . add_attributes ( vec ! [ ] ) ; __yew_vtag . add_listeners ( vec ! [ ] ) ; __yew_vtag . add_children ( vec ! [ :: yew :: virtual_dom :: VNode :: from ( { let mut __yew_vtag = :: yew :: virtual_dom :: vtag :: VTag :: new ( "label" ) ; __yew_vtag . add_attributes ( vec ! [ ( "for" . to_owned ( ) , ( "address.street" ) . to_string ( ) ) ] ) ; __yew_vtag . add_listeners ( vec ! [ ] ) ; __yew_vtag . add_children ( vec ! [ :: yew :: virtual_dom :: VNode :: from ( "Street: " ) ] ) ; :: yew :: virtual_dom :: VNode :: from ( __yew_vtag ) } ) , :: yew :: virtual_dom :: VNode :: from ( { if false { trait __yew_validate_comp : :: yew :: html :: Component { } impl __yew_validate_comp for Field < Registration > { } let _ = | props : < Field < Registration > as :: yew :: html :: Component > :: Properties | { props . field_name ; props . form ; props . oninput ; } ; } :: yew :: virtual_dom :: VChild :: < Field < Registration > > :: new ( << Field < Registration > as :: yew :: html :: Component > :: Properties as :: yew :: html :: Properties > :: builder ( ) . field_name ( < :: yew :: virtual_dom :: vcomp :: VComp as :: yew :: virtual_dom :: Transformer < _ , _ >> :: transform ( "address.street" ) ) . form ( < :: yew :: virtual_dom :: vcomp :: VComp as :: yew :: virtual_dom :: Transformer < _ , _ >> :: transform ( & self . form ) ) . oninput ( < :: yew :: virtual_dom :: vcomp :: VComp as :: yew :: virtual_dom :: Transformer < _ , _ >> :: transform ( self . link . callback ( | _ : InputData | AppMessage :: Update ) ) ) . build ( ) , :: yew :: html :: NodeRef :: default ( ) ) } ) , :: yew :: virtual_dom :: VNode :: from ( { let mut __yew_vtag = :: yew :: virtual_dom :: vtag :: VTag :: new ( "div" ) ; __yew_vtag . set_classes ( "invalid-feedback" ) ; __yew_vtag . add_attributes ( vec ! [ ] ) ; __yew_vtag . add_listeners ( vec ! [ ] ) ; __yew_vtag . add_children ( vec ! [ :: yew :: virtual_dom :: VNode :: from ( { & self . form . field_message ( "address.street" ) } ) ] ) ; :: yew :: virtual_dom :: VNode :: from ( __yew_vtag ) } ) ] ) ; :: yew :: virtual_dom :: VNode :: from ( __yew_vtag ) } ) , :: yew :: virtual_dom :: VNode :: from ( { let mut __yew_vtag = :: yew :: virtual_dom :: vtag :: VTag :: new ( "div" ) ; __yew_vtag . set_classes ( "form-group" ) ; __yew_vtag . add_attributes ( vec ! [ ] ) ; __yew_vtag . add_listeners ( vec ! [ ] ) ; __yew_vtag . add_children ( vec ! [ :: yew :: virtual_dom :: VNode :: from ( { let mut __yew_vtag = :: yew :: virtual_dom :: vtag :: VTag :: new ( "label" ) ; __yew_vtag . add_attributes ( vec ! [ ( "for" . to_owned ( ) , ( "address.city" ) . to_string ( ) ) ] ) ; __yew_vtag . add_listeners ( vec ! [ ] ) ; __yew_vtag . add_children ( vec ! [ :: yew :: virtual_dom :: VNode :: from ( "City: " ) ] ) ; :: yew :: virtual_dom :: VNode :: from ( __yew_vtag ) } ) , :: yew :: virtual_dom :: VNode :: from ( { if false { trait __yew_validate_comp : :: yew :: html :: Component { } impl __yew_validate_comp for Field < Registration > { } let _ = | props : < Field < Registration > as :: yew :: html :: Component > :: Properties | { props . field_name ; props . form ; props . oninput ; } ; } :: yew :: virtual_dom :: VChild :: < Field < Registration > > :: new ( << Field < Registration > as :: yew :: html :: Component > :: Properties as :: yew :: html :: Properties > :: builder ( ) . field_name ( < :: yew :: virtual_dom :: vcomp :: VComp as :: yew :: virtual_dom :: Transformer < _ , _ >> :: transform ( "address.city" ) ) . form ( < :: yew :: virtual_dom :: vcomp :: VComp as :: yew :: virtual_dom :: Transformer < _ , _ >> :: transform ( & self . form ) ) . oninput ( < :: yew :: virtual_dom :: vcomp :: VComp as :: yew :: virtual_dom :: Transformer < _ , _ >> :: transform ( self . link . callback ( | _ : InputData | AppMessage :: Update ) ) ) . build ( ) , :: yew :: html :: NodeRef :: default ( ) ) } ) , :: yew :: virtual_dom :: VNode :: from ( { let mut __yew_vtag = :: yew :: virtual_dom :: vtag :: VTag :: new ( "div" ) ; __yew_vtag . set_classes ( "invalid-feedback" ) ; __yew_vtag . add_attributes ( vec ! [ ] ) ; __yew_vtag . add_listeners ( vec ! [ ] ) ; __yew_vtag . add_children ( vec ! [ :: yew :: virtual_dom :: VNode :: from ( { & self . form . field_message ( "address.city" ) } ) ] ) ; :: yew :: virtual_dom :: VNode :: from ( __yew_vtag ) } ) ] ) ; :: yew :: virtual_dom :: VNode :: from ( __yew_vtag ) } ) , :: yew :: virtual_dom :: VNode :: from ( { let mut __yew_vtag = :: yew :: virtual_dom :: vtag :: VTag :: new ( "div" ) ; __yew_vtag . set_classes ( "form-group" ) ; __yew_vtag . add_attributes ( vec ! [ ] ) ; __yew_vtag . add_listeners ( vec ! [ ] ) ; __yew_vtag . add_children ( vec ! [ :: yew :: virtual_dom :: VNode :: from ( { let mut __yew_vtag = :: yew :: virtual_dom :: vtag :: VTag :: new ( "label" ) ; __yew_vtag . add_attributes ( vec ! [ ( "for" . to_owned ( ) , ( "address.province" ) . to_string ( ) ) ] ) ; __yew_vtag . add_listeners ( vec ! [ ] ) ; __yew_vtag . add_children ( vec ! [ :: yew :: virtual_dom :: VNode :: from ( "Province: " ) ] ) ; :: yew :: virtual_dom :: VNode :: from ( __yew_vtag ) } ) , :: yew :: virtual_dom :: VNode :: from ( { if false { trait __yew_validate_comp : :: yew :: html :: Component { } impl __yew_validate_comp for Field < Registration > { } let _ = | props : < Field < Registration > as :: yew :: html :: Component > :: Properties | { props . field_name ; props . form ; props . oninput ; } ; } :: yew :: virtual_dom :: VChild :: < Field < Registration > > :: new ( << Field < Registration > as :: yew :: html :: Component > :: Properties as :: yew :: html :: Properties > :: builder ( ) . field_name ( < :: yew :: virtual_dom :: vcomp :: VComp as :: yew :: virtual_dom :: Transformer < _ , _ >> :: transform ( "address.province" ) ) . form ( < :: yew :: virtual_dom :: vcomp :: VComp as :: yew :: virtual_dom :: Transformer < _ , _ >> :: transform ( & self . form ) ) . oninput ( < :: yew :: virtual_dom :: vcomp :: VComp as :: yew :: virtual_dom :: Transformer < _ , _ >> :: transform ( self . link . callback ( | _ : InputData | AppMessage :: Update ) ) ) . build ( ) , :: yew :: html :: NodeRef :: default ( ) ) } ) , :: yew :: virtual_dom :: VNode :: from ( { let mut __yew_vtag = :: yew :: virtual_dom :: vtag :: VTag :: new ( "div" ) ; __yew_vtag . set_classes ( "invalid-feedback" ) ; __yew_vtag . add_attributes ( vec ! [ ] ) ; __yew_vtag . add_listeners ( vec ! [ ] ) ; __yew_vtag . add_children ( vec ! [ :: yew :: virtual_dom :: VNode :: from ( { & self . form . field_message ( "address.province" ) } ) ] ) ; :: yew :: virtual_dom :: VNode :: from ( __yew_vtag ) } ) ] ) ; :: yew :: virtual_dom :: VNode :: from ( __yew_vtag ) } ) , :: yew :: virtual_dom :: VNode :: from ( { let mut __yew_vtag = :: yew :: virtual_dom :: vtag :: VTag :: new ( "div" ) ; __yew_vtag . set_classes ( "form-group" ) ; __yew_vtag . add_attributes ( vec ! [ ] ) ; __yew_vtag . add_listeners ( vec ! [ ] ) ; __yew_vtag . add_children ( vec ! [ :: yew :: virtual_dom :: VNode :: from ( { let mut __yew_vtag = :: yew :: virtual_dom :: vtag :: VTag :: new ( "label" ) ; __yew_vtag . add_attributes ( vec ! [ ( "for" . to_owned ( ) , ( "address.country" ) . to_string ( ) ) ] ) ; __yew_vtag . add_listeners ( vec ! [ ] ) ; __yew_vtag . add_children ( vec ! [ :: yew :: virtual_dom :: VNode :: from ( "Country (optional): " ) ] ) ; :: yew :: virtual_dom :: VNode :: from ( __yew_vtag ) } ) , :: yew :: virtual_dom :: VNode :: from ( { if false { trait __yew_validate_comp : :: yew :: html :: Component { } impl __yew_validate_comp for Field < Registration > { } let _ = | props : < Field < Registration > as :: yew :: html :: Component > :: Properties | { props . field_name ; props . form ; props . oninput ; } ; } :: yew :: virtual_dom :: VChild :: < Field < Registration > > :: new ( << Field < Registration > as :: yew :: html :: Component > :: Properties as :: yew :: html :: Properties > :: builder ( ) . field_name ( < :: yew :: virtual_dom :: vcomp :: VComp as :: yew :: virtual_dom :: Transformer < _ , _ >> :: transform ( "address.country" ) ) . form ( < :: yew :: virtual_dom :: vcomp :: VComp as :: yew :: virtual_dom :: Transformer < _ , _ >> :: transform ( & self . form ) ) . oninput ( < :: yew :: virtual_dom :: vcomp :: VComp as :: yew :: virtual_dom :: Transformer < _ , _ >> :: transform ( self . link . callback ( | _ : InputData | AppMessage :: Update ) ) ) . build ( ) , :: yew :: html :: NodeRef :: default ( ) ) } ) , :: yew :: virtual_dom :: VNode :: from ( { let mut __yew_vtag = :: yew :: virtual_dom :: vtag :: VTag :: new ( "div" ) ; __yew_vtag . set_classes ( "invalid-feedback" ) ; __yew_vtag . add_attributes ( vec ! [ ] ) ; __yew_vtag . add_listeners ( vec ! [ ] ) ; __yew_vtag . add_children ( vec ! [ :: yew :: virtual_dom :: VNode :: from ( { & self . form . field_message ( "address.country" ) } ) ] ) ; :: yew :: virtual_dom :: VNode :: from ( __yew_vtag ) } ) ] ) ; :: yew :: virtual_dom :: VNode :: from ( __yew_vtag ) } ) , :: yew :: virtual_dom :: VNode :: from ( { let mut __yew_vtag = :: yew :: virtual_dom :: vtag :: VTag :: new ( "div" ) ; __yew_vtag . set_classes ( "form-group" ) ; __yew_vtag . add_attributes ( vec ! [ ] ) ; __yew_vtag . add_listeners ( vec ! [ ] ) ; __yew_vtag . add_children ( vec ! [ :: yew :: virtual_dom :: VNode :: from ( { if false { trait __yew_validate_comp : :: yew :: html :: Component { } impl __yew_validate_comp for CheckBox < Registration > { } let _ = | props : < CheckBox < Registration > as :: yew :: html :: Component > :: Properties | { props . field_name ; props . form ; } ; } :: yew :: virtual_dom :: VChild :: < CheckBox < Registration > > :: new ( << CheckBox < Registration > as :: yew :: html :: Component > :: Properties as :: yew :: html :: Properties > :: builder ( ) . field_name ( < :: yew :: virtual_dom :: vcomp :: VComp as :: yew :: virtual_dom :: Transformer < _ , _ >> :: transform ( "accept_terms" ) ) . form ( < :: yew :: virtual_dom :: vcomp :: VComp as :: yew :: virtual_dom :: Transformer < _ , _ >> :: transform ( & self . form ) ) . build ( ) , :: yew :: html :: NodeRef :: default ( ) ) } ) , :: yew :: virtual_dom :: VNode :: from ( { let mut __yew_vtag = :: yew :: virtual_dom :: vtag :: VTag :: new ( "label" ) ; __yew_vtag . set_classes ( "form-check-label" ) ; __yew_vtag . add_attributes ( vec ! [ ( "for" . to_owned ( ) , ( "accept_terms" ) . to_string ( ) ) ] ) ; __yew_vtag . add_listeners ( vec ! [ ] ) ; __yew_vtag . add_children ( vec ! [ :: yew :: virtual_dom :: VNode :: from ( "Accept Terms and Conditions: " ) ] ) ; :: yew :: virtual_dom :: VNode :: from ( __yew_vtag ) } ) , :: yew :: virtual_dom :: VNode :: from ( { let mut __yew_vtag = :: yew :: virtual_dom :: vtag :: VTag :: new ( "div" ) ; __yew_vtag . set_classes ( "invalid-feedback" ) ; __yew_vtag . add_attributes ( vec ! [ ] ) ; __yew_vtag . add_listeners ( vec ! [ ] ) ; __yew_vtag . add_children ( vec ! [ :: yew :: virtual_dom :: VNode :: from ( { & self . form . field_message ( "accept_terms" ) } ) ] ) ; :: yew :: virtual_dom :: VNode :: from ( __yew_vtag ) } ) ] ) ; :: yew :: virtual_dom :: VNode :: from ( __yew_vtag ) } ) , :: yew :: virtual_dom :: VNode :: from ( { let mut __yew_vtag = :: yew :: virtual_dom :: vtag :: VTag :: new ( "div" ) ; __yew_vtag . set_classes ( "form-group" ) ; __yew_vtag . add_attributes ( vec ! [ ] ) ; __yew_vtag . add_listeners ( vec ! [ ] ) ; __yew_vtag . add_children ( vec ! [ :: yew :: virtual_dom :: VNode :: from ( { let mut __yew_vtag = :: yew :: virtual_dom :: vtag :: VTag :: new ( "button" ) ; __yew_vtag . set_kind ( & ( "button" ) ) ; __yew_vtag . add_attributes ( vec ! [ ] ) ; __yew_vtag . add_listeners ( vec ! [ :: std :: rc :: Rc :: new ( { :: yew :: html :: onclick :: Wrapper :: new ( < :: yew :: virtual_dom :: vtag :: VTag as :: yew :: virtual_dom :: Transformer < _ , _ >> :: transform ( self . link . callback ( | e : ClickEvent | { e . prevent_default ( ) ; AppMessage :: Submit } ) ) ) } ) ] ) ; __yew_vtag . add_children ( vec ! [ :: yew :: virtual_dom :: VNode :: from ( "Submit" ) ] ) ; :: yew :: virtual_dom :: VNode :: from ( __yew_vtag ) } ) ] ) ; :: yew :: virtual_dom :: VNode :: from ( __yew_vtag ) } ) ] ) ; :: yew :: virtual_dom :: VNode :: from ( __yew_vtag ) } ) , :: yew :: virtual_dom :: VNode :: from ( { let mut __yew_vtag = :: yew :: virtual_dom :: vtag :: VTag :: new ( "div" ) ; if ! self . submitted { __yew_vtag . add_attribute ( & "hidden" , & "hidden" ) ; } __yew_vtag . add_attributes ( vec ! [ ] ) ; __yew_vtag . add_listeners ( vec ! [ ] ) ; __yew_vtag . add_children ( vec ! [ :: yew :: virtual_dom :: VNode :: from ( { let mut __yew_vtag = :: yew :: virtual_dom :: vtag :: VTag :: new ( "h2" ) ; __yew_vtag . add_attributes ( vec ! [ ] ) ; __yew_vtag . add_listeners ( vec ! [ ] ) ; __yew_vtag . add_children ( vec ! [ :: yew :: virtual_dom :: VNode :: from ( "Form data" ) ] ) ; :: yew :: virtual_dom :: VNode :: from ( __yew_vtag ) } ) , :: yew :: virtual_dom :: VNode :: from ( { let mut __yew_vtag = :: yew :: virtual_dom :: vtag :: VTag :: new ( "p" ) ; __yew_vtag . add_attributes ( vec ! [ ] ) ; __yew_vtag . add_listeners ( vec ! [ ] ) ; __yew_vtag . add_children ( vec ! [ :: yew :: virtual_dom :: VNode :: from ( "First Name: " ) , :: yew :: virtual_dom :: VNode :: from ( { & self . form . model ( ) . first_name } ) ] ) ; :: yew :: virtual_dom :: VNode :: from ( __yew_vtag ) } ) , :: yew :: virtual_dom :: VNode :: from ( { let mut __yew_vtag = :: yew :: virtual_dom :: vtag :: VTag :: new ( "p" ) ; __yew_vtag . add_attributes ( vec ! [ ] ) ; __yew_vtag . add_listeners ( vec ! [ ] ) ; __yew_vtag . add_children ( vec ! [ :: yew :: virtual_dom :: VNode :: from ( "Last Name: " ) , :: yew :: virtual_dom :: VNode :: from ( { & self . form . model ( ) . last_name } ) ] ) ; :: yew :: virtual_dom :: VNode :: from ( __yew_vtag ) } ) , :: yew :: virtual_dom :: VNode :: from ( { let mut __yew_vtag = :: yew :: virtual_dom :: vtag :: VTag :: new ( "p" ) ; __yew_vtag . add_attributes ( vec ! [ ] ) ; __yew_vtag . add_listeners ( vec ! [ ] ) ; __yew_vtag . add_children ( vec ! [ :: yew :: virtual_dom :: VNode :: from ( "Street: " ) , :: yew :: virtual_dom :: VNode :: from ( { & self . form . model ( ) . address . street } ) ] ) ; :: yew :: virtual_dom :: VNode :: from ( __yew_vtag ) } ) , :: yew :: virtual_dom :: VNode :: from ( { let mut __yew_vtag = :: yew :: virtual_dom :: vtag :: VTag :: new ( "p" ) ; __yew_vtag . add_attributes ( vec ! [ ] ) ; __yew_vtag . add_listeners ( vec ! [ ] ) ; __yew_vtag . add_children ( vec ! [ :: yew :: virtual_dom :: VNode :: from ( "City: " ) , :: yew :: virtual_dom :: VNode :: from ( { & self . form . model ( ) . address . city } ) ] ) ; :: yew :: virtual_dom :: VNode :: from ( __yew_vtag ) } ) , :: yew :: virtual_dom :: VNode :: from ( { let mut __yew_vtag = :: yew :: virtual_dom :: vtag :: VTag :: new ( "p" ) ; __yew_vtag . add_attributes ( vec ! [ ] ) ; __yew_vtag . add_listeners ( vec ! [ ] ) ; __yew_vtag . add_children ( vec ! [ :: yew :: virtual_dom :: VNode :: from ( "Province: " ) , :: yew :: virtual_dom :: VNode :: from ( { & self . form . model ( ) . address . province } ) ] ) ; :: yew :: virtual_dom :: VNode :: from ( __yew_vtag ) } ) , :: yew :: virtual_dom :: VNode :: from ( { let mut __yew_vtag = :: yew :: virtual_dom :: vtag :: VTag :: new ( "p" ) ; __yew_vtag . add_attributes ( vec ! [ ] ) ; __yew_vtag . add_listeners ( vec ! [ ] ) ; __yew_vtag . add_children ( vec ! [ :: yew :: virtual_dom :: VNode :: from ( "Country: " ) , :: yew :: virtual_dom :: VNode :: from ( { & self . form . model ( ) . address . country } ) ] ) ; :: yew :: virtual_dom :: VNode :: from ( __yew_vtag ) } ) , :: yew :: virtual_dom :: VNode :: from ( { let mut __yew_vtag = :: yew :: virtual_dom :: vtag :: VTag :: new ( "p" ) ; __yew_vtag . add_attributes ( vec ! [ ] ) ; __yew_vtag . add_listeners ( vec ! [ ] ) ; __yew_vtag . add_children ( vec ! [ :: yew :: virtual_dom :: VNode :: from ( "Accepted Terms: " ) , :: yew :: virtual_dom :: VNode :: from ( { self . form . model ( ) . accept_terms } ) ] ) ; :: yew :: virtual_dom :: VNode :: from ( __yew_vtag ) } ) ] ) ; :: yew :: virtual_dom :: VNode :: from ( __yew_vtag ) } ) ] ) ; :: yew :: virtual_dom :: VNode :: from ( __yew_vtag ) } ) } }
            ::yew::virtual_dom::VNode::from({
                let mut __yew_vtag = ::yew::virtual_dom::vtag::VTag::new("div");
                __yew_vtag.set_classes("container-sm");
                __yew_vtag.add_attributes(<[_]>::into_vec(box []));
                __yew_vtag.add_listeners(<[_]>::into_vec(box []));
                __yew_vtag.add_children(<[_]>::into_vec(box [::yew::virtual_dom::VNode::from({
                    let mut __yew_vtag = ::yew::virtual_dom::vtag::VTag::new("h1");
                    __yew_vtag.add_attributes(<[_]>::into_vec(box []));
                    __yew_vtag.add_listeners(<[_]>::into_vec(box []));
                    __yew_vtag.add_children(<[_]>::into_vec(box [::yew::virtual_dom::VNode::from("Yew Form Example")]));
                    ::yew::virtual_dom::VNode::from(__yew_vtag)
                }), ::yew::virtual_dom::VNode::from({
                    let mut __yew_vtag = ::yew::virtual_dom::vtag::VTag::new("p");
                    __yew_vtag.add_attributes(<[_]>::into_vec(box []));
                    __yew_vtag.add_listeners(<[_]>::into_vec(box []));
                    __yew_vtag.add_children(<[_]>::into_vec(box [::yew::virtual_dom::VNode::from({
                        {
                            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(&["Hello, ", " ", " and welcome to Yew Form!"], &match (&self.form.model().first_name, &self.form.model().last_name) { (arg0, arg1) => [::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt), ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt)], }));
                            res
                        }
                    })]));
                    ::yew::virtual_dom::VNode::from(__yew_vtag)
                }), ::yew::virtual_dom::VNode::from({
                    let mut __yew_vtag = ::yew::virtual_dom::vtag::VTag::new("form");
                    __yew_vtag.add_attributes(<[_]>::into_vec(box []));
                    __yew_vtag.add_listeners(<[_]>::into_vec(box []));
                    __yew_vtag.add_children(<[_]>::into_vec(box [::yew::virtual_dom::VNode::from({
                        let mut __yew_vtag = ::yew::virtual_dom::vtag::VTag::new("div");
                        __yew_vtag.set_classes("form-group");
                        __yew_vtag.add_attributes(<[_]>::into_vec(box []));
                        __yew_vtag.add_listeners(<[_]>::into_vec(box []));
                        __yew_vtag.add_children(<[_]>::into_vec(box [::yew::virtual_dom::VNode::from({
                            let mut __yew_vtag = ::yew::virtual_dom::vtag::VTag::new("label");
                            __yew_vtag.add_attributes(<[_]>::into_vec(box [("for".to_owned(), ("first_name").to_string())]));
                            __yew_vtag.add_listeners(<[_]>::into_vec(box []));
                            __yew_vtag.add_children(<[_]>::into_vec(box [::yew::virtual_dom::VNode::from("First Name: ")]));
                            ::yew::virtual_dom::VNode::from(__yew_vtag)
                        }), ::yew::virtual_dom::VNode::from({
                            if false {
                                trait __yew_validate_comp: ::yew::html::Component {} impl __yew_validate_comp for Field<Registration> {} let _ = |props: <Field<Registration> as ::yew::html::Component>::Properties| {
                                    props.field_name;
                                    props.form;
                                    props.oninput;
                                };
                            }
                            ::yew::virtual_dom::VChild::<Field<Registration>>::new(<<Field<Registration> as ::yew::html::Component>::Properties as ::yew::html::Properties>::builder().field_name(<::yew::virtual_dom::vcomp::VComp as ::yew::virtual_dom::Transformer<_, _>>::transform("first_name")).form(<::yew::virtual_dom::vcomp::VComp as ::yew::virtual_dom::Transformer<_, _>>::transform(&self.form)).oninput(<::yew::virtual_dom::vcomp::VComp as ::yew::virtual_dom::Transformer<_, _>>::transform(self.link.callback(|_: InputData| AppMessage::Update))).build(), ::yew::html::NodeRef::default())
                        }), ::yew::virtual_dom::VNode::from({
                            let mut __yew_vtag = ::yew::virtual_dom::vtag::VTag::new("div");
                            __yew_vtag.set_classes("invalid-feedback");
                            __yew_vtag.add_attributes(<[_]>::into_vec(box []));
                            __yew_vtag.add_listeners(<[_]>::into_vec(box []));
                            __yew_vtag.add_children(<[_]>::into_vec(box [::yew::virtual_dom::VNode::from({ &self.form.field_message("first_name") })]));
                            ::yew::virtual_dom::VNode::from(__yew_vtag)
                        })]));
                        ::yew::virtual_dom::VNode::from(__yew_vtag)
                    }), ::yew::virtual_dom::VNode::from({
                        let mut __yew_vtag = ::yew::virtual_dom::vtag::VTag::new("div");
                        __yew_vtag.set_classes("form-group");
                        __yew_vtag.add_attributes(<[_]>::into_vec(box []));
                        __yew_vtag.add_listeners(<[_]>::into_vec(box []));
                        __yew_vtag.add_children(<[_]>::into_vec(box [::yew::virtual_dom::VNode::from({
                            let mut __yew_vtag = ::yew::virtual_dom::vtag::VTag::new("label");
                            __yew_vtag.add_attributes(<[_]>::into_vec(box [("for".to_owned(), ("last_name").to_string())]));
                            __yew_vtag.add_listeners(<[_]>::into_vec(box []));
                            __yew_vtag.add_children(<[_]>::into_vec(box [::yew::virtual_dom::VNode::from("Last Name: ")]));
                            ::yew::virtual_dom::VNode::from(__yew_vtag)
                        }), ::yew::virtual_dom::VNode::from({
                            if false {
                                trait __yew_validate_comp: ::yew::html::Component {} impl __yew_validate_comp for Field<Registration> {} let _ = |props: <Field<Registration> as ::yew::html::Component>::Properties| {
                                    props.field_name;
                                    props.form;
                                    props.oninput;
                                };
                            }
                            ::yew::virtual_dom::VChild::<Field<Registration>>::new(<<Field<Registration> as ::yew::html::Component>::Properties as ::yew::html::Properties>::builder().field_name(<::yew::virtual_dom::vcomp::VComp as ::yew::virtual_dom::Transformer<_, _>>::transform("last_name")).form(<::yew::virtual_dom::vcomp::VComp as ::yew::virtual_dom::Transformer<_, _>>::transform(&self.form)).oninput(<::yew::virtual_dom::vcomp::VComp as ::yew::virtual_dom::Transformer<_, _>>::transform(self.link.callback(|_: InputData| AppMessage::Update))).build(), ::yew::html::NodeRef::default())
                        }), ::yew::virtual_dom::VNode::from({
                            let mut __yew_vtag = ::yew::virtual_dom::vtag::VTag::new("div");
                            __yew_vtag.set_classes("invalid-feedback");
                            __yew_vtag.add_attributes(<[_]>::into_vec(box []));
                            __yew_vtag.add_listeners(<[_]>::into_vec(box []));
                            __yew_vtag.add_children(<[_]>::into_vec(box [::yew::virtual_dom::VNode::from({ &self.form.field_message("last_name") })]));
                            ::yew::virtual_dom::VNode::from(__yew_vtag)
                        })]));
                        ::yew::virtual_dom::VNode::from(__yew_vtag)
                    }), ::yew::virtual_dom::VNode::from({
                        let mut __yew_vtag = ::yew::virtual_dom::vtag::VTag::new("div");
                        __yew_vtag.set_classes("form-group");
                        __yew_vtag.add_attributes(<[_]>::into_vec(box []));
                        __yew_vtag.add_listeners(<[_]>::into_vec(box []));
                        __yew_vtag.add_children(<[_]>::into_vec(box [::yew::virtual_dom::VNode::from({
                            let mut __yew_vtag = ::yew::virtual_dom::vtag::VTag::new("label");
                            __yew_vtag.add_attributes(<[_]>::into_vec(box [("for".to_owned(), ("last_name").to_string())]));
                            __yew_vtag.add_listeners(<[_]>::into_vec(box []));
                            __yew_vtag.add_children(<[_]>::into_vec(box [::yew::virtual_dom::VNode::from("Quantity: ")]));
                            ::yew::virtual_dom::VNode::from(__yew_vtag)
                        }), ::yew::virtual_dom::VNode::from({
                            if false {
                                trait __yew_validate_comp: ::yew::html::Component {} impl __yew_validate_comp for Field<Registration> {} let _ = |props: <Field<Registration> as ::yew::html::Component>::Properties| {
                                    props.field_name;
                                    props.form;
                                    props.oninput;
                                };
                            }
                            ::yew::virtual_dom::VChild::<Field<Registration>>::new(<<Field<Registration> as ::yew::html::Component>::Properties as ::yew::html::Properties>::builder().field_name(<::yew::virtual_dom::vcomp::VComp as ::yew::virtual_dom::Transformer<_, _>>::transform("quantity")).form(<::yew::virtual_dom::vcomp::VComp as ::yew::virtual_dom::Transformer<_, _>>::transform(&self.form)).oninput(<::yew::virtual_dom::vcomp::VComp as ::yew::virtual_dom::Transformer<_, _>>::transform(self.link.callback(|_: InputData| AppMessage::Update))).build(), ::yew::html::NodeRef::default())
                        }), ::yew::virtual_dom::VNode::from({
                            let mut __yew_vtag = ::yew::virtual_dom::vtag::VTag::new("div");
                            __yew_vtag.set_classes("invalid-feedback");
                            __yew_vtag.add_attributes(<[_]>::into_vec(box []));
                            __yew_vtag.add_listeners(<[_]>::into_vec(box []));
                            __yew_vtag.add_children(<[_]>::into_vec(box [::yew::virtual_dom::VNode::from({ &self.form.field_message("quantity") })]));
                            ::yew::virtual_dom::VNode::from(__yew_vtag)
                        })]));
                        ::yew::virtual_dom::VNode::from(__yew_vtag)
                    }), ::yew::virtual_dom::VNode::from({
                        let mut __yew_vtag = ::yew::virtual_dom::vtag::VTag::new("div");
                        __yew_vtag.set_classes("form-group");
                        __yew_vtag.add_attributes(<[_]>::into_vec(box []));
                        __yew_vtag.add_listeners(<[_]>::into_vec(box []));
                        __yew_vtag.add_children(<[_]>::into_vec(box [::yew::virtual_dom::VNode::from({
                            let mut __yew_vtag = ::yew::virtual_dom::vtag::VTag::new("label");
                            __yew_vtag.add_attributes(<[_]>::into_vec(box [("for".to_owned(), ("last_name").to_string())]));
                            __yew_vtag.add_listeners(<[_]>::into_vec(box []));
                            __yew_vtag.add_children(<[_]>::into_vec(box [::yew::virtual_dom::VNode::from("Price: ")]));
                            ::yew::virtual_dom::VNode::from(__yew_vtag)
                        }), ::yew::virtual_dom::VNode::from({
                            if false {
                                trait __yew_validate_comp: ::yew::html::Component {} impl __yew_validate_comp for Field<Registration> {} let _ = |props: <Field<Registration> as ::yew::html::Component>::Properties| {
                                    props.field_name;
                                    props.form;
                                    props.oninput;
                                };
                            }
                            ::yew::virtual_dom::VChild::<Field<Registration>>::new(<<Field<Registration> as ::yew::html::Component>::Properties as ::yew::html::Properties>::builder().field_name(<::yew::virtual_dom::vcomp::VComp as ::yew::virtual_dom::Transformer<_, _>>::transform("price")).form(<::yew::virtual_dom::vcomp::VComp as ::yew::virtual_dom::Transformer<_, _>>::transform(&self.form)).oninput(<::yew::virtual_dom::vcomp::VComp as ::yew::virtual_dom::Transformer<_, _>>::transform(self.link.callback(|_: InputData| AppMessage::Update))).build(), ::yew::html::NodeRef::default())
                        }), ::yew::virtual_dom::VNode::from({
                            let mut __yew_vtag = ::yew::virtual_dom::vtag::VTag::new("div");
                            __yew_vtag.set_classes("invalid-feedback");
                            __yew_vtag.add_attributes(<[_]>::into_vec(box []));
                            __yew_vtag.add_listeners(<[_]>::into_vec(box []));
                            __yew_vtag.add_children(<[_]>::into_vec(box [::yew::virtual_dom::VNode::from({ &self.form.field_message("price") })]));
                            ::yew::virtual_dom::VNode::from(__yew_vtag)
                        })]));
                        ::yew::virtual_dom::VNode::from(__yew_vtag)
                    }), ::yew::virtual_dom::VNode::from({
                        let mut __yew_vtag = ::yew::virtual_dom::vtag::VTag::new("div");
                        __yew_vtag.add_attributes(<[_]>::into_vec(box []));
                        __yew_vtag.add_listeners(<[_]>::into_vec(box []));
                        __yew_vtag.add_children(<[_]>::into_vec(box [::yew::virtual_dom::VNode::from("Total: "), ::yew::virtual_dom::VNode::from({
                            {
                                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1_formatted(&[""], &match (&self.form.model().total(), ) { (arg0, ) => [::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt)], }, &[::core::fmt::rt::v1::Argument { position: 0usize, format: ::core::fmt::rt::v1::FormatSpec { fill: ' ', align: ::core::fmt::rt::v1::Alignment::Unknown, flags: 0u32, precision: ::core::fmt::rt::v1::Count::Is(2usize), width: ::core::fmt::rt::v1::Count::Implied } }]));
                                res
                            }
                        })]));
                        ::yew::virtual_dom::VNode::from(__yew_vtag)
                    }), ::yew::virtual_dom::VNode::from({
                        let mut __yew_vtag = ::yew::virtual_dom::vtag::VTag::new("div");
                        __yew_vtag.set_classes("form-group");
                        __yew_vtag.add_attributes(<[_]>::into_vec(box []));
                        __yew_vtag.add_listeners(<[_]>::into_vec(box []));
                        __yew_vtag.add_children(<[_]>::into_vec(box [::yew::virtual_dom::VNode::from({
                            let mut __yew_vtag = ::yew::virtual_dom::vtag::VTag::new("label");
                            __yew_vtag.add_attributes(<[_]>::into_vec(box [("for".to_owned(), ("address.street").to_string())]));
                            __yew_vtag.add_listeners(<[_]>::into_vec(box []));
                            __yew_vtag.add_children(<[_]>::into_vec(box [::yew::virtual_dom::VNode::from("Street: ")]));
                            ::yew::virtual_dom::VNode::from(__yew_vtag)
                        }), ::yew::virtual_dom::VNode::from({
                            if false {
                                trait __yew_validate_comp: ::yew::html::Component {} impl __yew_validate_comp for Field<Registration> {} let _ = |props: <Field<Registration> as ::yew::html::Component>::Properties| {
                                    props.field_name;
                                    props.form;
                                    props.oninput;
                                };
                            }
                            ::yew::virtual_dom::VChild::<Field<Registration>>::new(<<Field<Registration> as ::yew::html::Component>::Properties as ::yew::html::Properties>::builder().field_name(<::yew::virtual_dom::vcomp::VComp as ::yew::virtual_dom::Transformer<_, _>>::transform("address.street")).form(<::yew::virtual_dom::vcomp::VComp as ::yew::virtual_dom::Transformer<_, _>>::transform(&self.form)).oninput(<::yew::virtual_dom::vcomp::VComp as ::yew::virtual_dom::Transformer<_, _>>::transform(self.link.callback(|_: InputData| AppMessage::Update))).build(), ::yew::html::NodeRef::default())
                        }), ::yew::virtual_dom::VNode::from({
                            let mut __yew_vtag = ::yew::virtual_dom::vtag::VTag::new("div");
                            __yew_vtag.set_classes("invalid-feedback");
                            __yew_vtag.add_attributes(<[_]>::into_vec(box []));
                            __yew_vtag.add_listeners(<[_]>::into_vec(box []));
                            __yew_vtag.add_children(<[_]>::into_vec(box [::yew::virtual_dom::VNode::from({ &self.form.field_message("address.street") })]));
                            ::yew::virtual_dom::VNode::from(__yew_vtag)
                        })]));
                        ::yew::virtual_dom::VNode::from(__yew_vtag)
                    }), ::yew::virtual_dom::VNode::from({
                        let mut __yew_vtag = ::yew::virtual_dom::vtag::VTag::new("div");
                        __yew_vtag.set_classes("form-group");
                        __yew_vtag.add_attributes(<[_]>::into_vec(box []));
                        __yew_vtag.add_listeners(<[_]>::into_vec(box []));
                        __yew_vtag.add_children(<[_]>::into_vec(box [::yew::virtual_dom::VNode::from({
                            let mut __yew_vtag = ::yew::virtual_dom::vtag::VTag::new("label");
                            __yew_vtag.add_attributes(<[_]>::into_vec(box [("for".to_owned(), ("address.city").to_string())]));
                            __yew_vtag.add_listeners(<[_]>::into_vec(box []));
                            __yew_vtag.add_children(<[_]>::into_vec(box [::yew::virtual_dom::VNode::from("City: ")]));
                            ::yew::virtual_dom::VNode::from(__yew_vtag)
                        }), ::yew::virtual_dom::VNode::from({
                            if false {
                                trait __yew_validate_comp: ::yew::html::Component {} impl __yew_validate_comp for Field<Registration> {} let _ = |props: <Field<Registration> as ::yew::html::Component>::Properties| {
                                    props.field_name;
                                    props.form;
                                    props.oninput;
                                };
                            }
                            ::yew::virtual_dom::VChild::<Field<Registration>>::new(<<Field<Registration> as ::yew::html::Component>::Properties as ::yew::html::Properties>::builder().field_name(<::yew::virtual_dom::vcomp::VComp as ::yew::virtual_dom::Transformer<_, _>>::transform("address.city")).form(<::yew::virtual_dom::vcomp::VComp as ::yew::virtual_dom::Transformer<_, _>>::transform(&self.form)).oninput(<::yew::virtual_dom::vcomp::VComp as ::yew::virtual_dom::Transformer<_, _>>::transform(self.link.callback(|_: InputData| AppMessage::Update))).build(), ::yew::html::NodeRef::default())
                        }), ::yew::virtual_dom::VNode::from({
                            let mut __yew_vtag = ::yew::virtual_dom::vtag::VTag::new("div");
                            __yew_vtag.set_classes("invalid-feedback");
                            __yew_vtag.add_attributes(<[_]>::into_vec(box []));
                            __yew_vtag.add_listeners(<[_]>::into_vec(box []));
                            __yew_vtag.add_children(<[_]>::into_vec(box [::yew::virtual_dom::VNode::from({ &self.form.field_message("address.city") })]));
                            ::yew::virtual_dom::VNode::from(__yew_vtag)
                        })]));
                        ::yew::virtual_dom::VNode::from(__yew_vtag)
                    }), ::yew::virtual_dom::VNode::from({
                        let mut __yew_vtag = ::yew::virtual_dom::vtag::VTag::new("div");
                        __yew_vtag.set_classes("form-group");
                        __yew_vtag.add_attributes(<[_]>::into_vec(box []));
                        __yew_vtag.add_listeners(<[_]>::into_vec(box []));
                        __yew_vtag.add_children(<[_]>::into_vec(box [::yew::virtual_dom::VNode::from({
                            let mut __yew_vtag = ::yew::virtual_dom::vtag::VTag::new("label");
                            __yew_vtag.add_attributes(<[_]>::into_vec(box [("for".to_owned(), ("address.province").to_string())]));
                            __yew_vtag.add_listeners(<[_]>::into_vec(box []));
                            __yew_vtag.add_children(<[_]>::into_vec(box [::yew::virtual_dom::VNode::from("Province: ")]));
                            ::yew::virtual_dom::VNode::from(__yew_vtag)
                        }), ::yew::virtual_dom::VNode::from({
                            if false {
                                trait __yew_validate_comp: ::yew::html::Component {} impl __yew_validate_comp for Field<Registration> {} let _ = |props: <Field<Registration> as ::yew::html::Component>::Properties| {
                                    props.field_name;
                                    props.form;
                                    props.oninput;
                                };
                            }
                            ::yew::virtual_dom::VChild::<Field<Registration>>::new(<<Field<Registration> as ::yew::html::Component>::Properties as ::yew::html::Properties>::builder().field_name(<::yew::virtual_dom::vcomp::VComp as ::yew::virtual_dom::Transformer<_, _>>::transform("address.province")).form(<::yew::virtual_dom::vcomp::VComp as ::yew::virtual_dom::Transformer<_, _>>::transform(&self.form)).oninput(<::yew::virtual_dom::vcomp::VComp as ::yew::virtual_dom::Transformer<_, _>>::transform(self.link.callback(|_: InputData| AppMessage::Update))).build(), ::yew::html::NodeRef::default())
                        }), ::yew::virtual_dom::VNode::from({
                            let mut __yew_vtag = ::yew::virtual_dom::vtag::VTag::new("div");
                            __yew_vtag.set_classes("invalid-feedback");
                            __yew_vtag.add_attributes(<[_]>::into_vec(box []));
                            __yew_vtag.add_listeners(<[_]>::into_vec(box []));
                            __yew_vtag.add_children(<[_]>::into_vec(box [::yew::virtual_dom::VNode::from({ &self.form.field_message("address.province") })]));
                            ::yew::virtual_dom::VNode::from(__yew_vtag)
                        })]));
                        ::yew::virtual_dom::VNode::from(__yew_vtag)
                    }), ::yew::virtual_dom::VNode::from({
                        let mut __yew_vtag = ::yew::virtual_dom::vtag::VTag::new("div");
                        __yew_vtag.set_classes("form-group");
                        __yew_vtag.add_attributes(<[_]>::into_vec(box []));
                        __yew_vtag.add_listeners(<[_]>::into_vec(box []));
                        __yew_vtag.add_children(<[_]>::into_vec(box [::yew::virtual_dom::VNode::from({
                            let mut __yew_vtag = ::yew::virtual_dom::vtag::VTag::new("label");
                            __yew_vtag.add_attributes(<[_]>::into_vec(box [("for".to_owned(), ("address.country").to_string())]));
                            __yew_vtag.add_listeners(<[_]>::into_vec(box []));
                            __yew_vtag.add_children(<[_]>::into_vec(box [::yew::virtual_dom::VNode::from("Country (optional): ")]));
                            ::yew::virtual_dom::VNode::from(__yew_vtag)
                        }), ::yew::virtual_dom::VNode::from({
                            if false {
                                trait __yew_validate_comp: ::yew::html::Component {} impl __yew_validate_comp for Field<Registration> {} let _ = |props: <Field<Registration> as ::yew::html::Component>::Properties| {
                                    props.field_name;
                                    props.form;
                                    props.oninput;
                                };
                            }
                            ::yew::virtual_dom::VChild::<Field<Registration>>::new(<<Field<Registration> as ::yew::html::Component>::Properties as ::yew::html::Properties>::builder().field_name(<::yew::virtual_dom::vcomp::VComp as ::yew::virtual_dom::Transformer<_, _>>::transform("address.country")).form(<::yew::virtual_dom::vcomp::VComp as ::yew::virtual_dom::Transformer<_, _>>::transform(&self.form)).oninput(<::yew::virtual_dom::vcomp::VComp as ::yew::virtual_dom::Transformer<_, _>>::transform(self.link.callback(|_: InputData| AppMessage::Update))).build(), ::yew::html::NodeRef::default())
                        }), ::yew::virtual_dom::VNode::from({
                            let mut __yew_vtag = ::yew::virtual_dom::vtag::VTag::new("div");
                            __yew_vtag.set_classes("invalid-feedback");
                            __yew_vtag.add_attributes(<[_]>::into_vec(box []));
                            __yew_vtag.add_listeners(<[_]>::into_vec(box []));
                            __yew_vtag.add_children(<[_]>::into_vec(box [::yew::virtual_dom::VNode::from({ &self.form.field_message("address.country") })]));
                            ::yew::virtual_dom::VNode::from(__yew_vtag)
                        })]));
                        ::yew::virtual_dom::VNode::from(__yew_vtag)
                    }), ::yew::virtual_dom::VNode::from({
                        let mut __yew_vtag = ::yew::virtual_dom::vtag::VTag::new("div");
                        __yew_vtag.set_classes("form-group");
                        __yew_vtag.add_attributes(<[_]>::into_vec(box []));
                        __yew_vtag.add_listeners(<[_]>::into_vec(box []));
                        __yew_vtag.add_children(<[_]>::into_vec(box [::yew::virtual_dom::VNode::from({
                            if false {
                                trait __yew_validate_comp: ::yew::html::Component {} impl __yew_validate_comp for CheckBox<Registration> {} let _ = |props: <CheckBox<Registration> as ::yew::html::Component>::Properties| {
                                    props.field_name;
                                    props.form;
                                };
                            }
                            ::yew::virtual_dom::VChild::<CheckBox<Registration>>::new(<<CheckBox<Registration> as ::yew::html::Component>::Properties as ::yew::html::Properties>::builder().field_name(<::yew::virtual_dom::vcomp::VComp as ::yew::virtual_dom::Transformer<_, _>>::transform("accept_terms")).form(<::yew::virtual_dom::vcomp::VComp as ::yew::virtual_dom::Transformer<_, _>>::transform(&self.form)).build(), ::yew::html::NodeRef::default())
                        }), ::yew::virtual_dom::VNode::from({
                            let mut __yew_vtag = ::yew::virtual_dom::vtag::VTag::new("label");
                            __yew_vtag.set_classes("form-check-label");
                            __yew_vtag.add_attributes(<[_]>::into_vec(box [("for".to_owned(), ("accept_terms").to_string())]));
                            __yew_vtag.add_listeners(<[_]>::into_vec(box []));
                            __yew_vtag.add_children(<[_]>::into_vec(box [::yew::virtual_dom::VNode::from("Accept Terms and Conditions: ")]));
                            ::yew::virtual_dom::VNode::from(__yew_vtag)
                        }), ::yew::virtual_dom::VNode::from({
                            let mut __yew_vtag = ::yew::virtual_dom::vtag::VTag::new("div");
                            __yew_vtag.set_classes("invalid-feedback");
                            __yew_vtag.add_attributes(<[_]>::into_vec(box []));
                            __yew_vtag.add_listeners(<[_]>::into_vec(box []));
                            __yew_vtag.add_children(<[_]>::into_vec(box [::yew::virtual_dom::VNode::from({ &self.form.field_message("accept_terms") })]));
                            ::yew::virtual_dom::VNode::from(__yew_vtag)
                        })]));
                        ::yew::virtual_dom::VNode::from(__yew_vtag)
                    }), ::yew::virtual_dom::VNode::from({
                        let mut __yew_vtag = ::yew::virtual_dom::vtag::VTag::new("div");
                        __yew_vtag.set_classes("form-group");
                        __yew_vtag.add_attributes(<[_]>::into_vec(box []));
                        __yew_vtag.add_listeners(<[_]>::into_vec(box []));
                        __yew_vtag.add_children(<[_]>::into_vec(box [::yew::virtual_dom::VNode::from({
                            let mut __yew_vtag = ::yew::virtual_dom::vtag::VTag::new("button");
                            __yew_vtag.set_kind(&("button"));
                            __yew_vtag.add_attributes(<[_]>::into_vec(box []));
                            __yew_vtag.add_listeners(<[_]>::into_vec(box [::std::rc::Rc::new({
                                ::yew::html::onclick::Wrapper::new(<::yew::virtual_dom::vtag::VTag as ::yew::virtual_dom::Transformer<_, _>>::transform(self.link.callback(|e: ClickEvent| {
                                    e.prevent_default();
                                    AppMessage::Submit
                                })))
                            })]));
                            __yew_vtag.add_children(<[_]>::into_vec(box [::yew::virtual_dom::VNode::from("Submit")]));
                            ::yew::virtual_dom::VNode::from(__yew_vtag)
                        })]));
                        ::yew::virtual_dom::VNode::from(__yew_vtag)
                    })]));
                    ::yew::virtual_dom::VNode::from(__yew_vtag)
                }), ::yew::virtual_dom::VNode::from({
                    let mut __yew_vtag = ::yew::virtual_dom::vtag::VTag::new("div");
                    if !self.submitted { __yew_vtag.add_attribute(&"hidden", &"hidden"); }
                    __yew_vtag.add_attributes(<[_]>::into_vec(box []));
                    __yew_vtag.add_listeners(<[_]>::into_vec(box []));
                    __yew_vtag.add_children(<[_]>::into_vec(box [::yew::virtual_dom::VNode::from({
                        let mut __yew_vtag = ::yew::virtual_dom::vtag::VTag::new("h2");
                        __yew_vtag.add_attributes(<[_]>::into_vec(box []));
                        __yew_vtag.add_listeners(<[_]>::into_vec(box []));
                        __yew_vtag.add_children(<[_]>::into_vec(box [::yew::virtual_dom::VNode::from("Form data")]));
                        ::yew::virtual_dom::VNode::from(__yew_vtag)
                    }), ::yew::virtual_dom::VNode::from({
                        let mut __yew_vtag = ::yew::virtual_dom::vtag::VTag::new("p");
                        __yew_vtag.add_attributes(<[_]>::into_vec(box []));
                        __yew_vtag.add_listeners(<[_]>::into_vec(box []));
                        __yew_vtag.add_children(<[_]>::into_vec(box [::yew::virtual_dom::VNode::from("First Name: "), ::yew::virtual_dom::VNode::from({ &self.form.model().first_name })]));
                        ::yew::virtual_dom::VNode::from(__yew_vtag)
                    }), ::yew::virtual_dom::VNode::from({
                        let mut __yew_vtag = ::yew::virtual_dom::vtag::VTag::new("p");
                        __yew_vtag.add_attributes(<[_]>::into_vec(box []));
                        __yew_vtag.add_listeners(<[_]>::into_vec(box []));
                        __yew_vtag.add_children(<[_]>::into_vec(box [::yew::virtual_dom::VNode::from("Last Name: "), ::yew::virtual_dom::VNode::from({ &self.form.model().last_name })]));
                        ::yew::virtual_dom::VNode::from(__yew_vtag)
                    }), ::yew::virtual_dom::VNode::from({
                        let mut __yew_vtag = ::yew::virtual_dom::vtag::VTag::new("p");
                        __yew_vtag.add_attributes(<[_]>::into_vec(box []));
                        __yew_vtag.add_listeners(<[_]>::into_vec(box []));
                        __yew_vtag.add_children(<[_]>::into_vec(box [::yew::virtual_dom::VNode::from("Street: "), ::yew::virtual_dom::VNode::from({ &self.form.model().address.street })]));
                        ::yew::virtual_dom::VNode::from(__yew_vtag)
                    }), ::yew::virtual_dom::VNode::from({
                        let mut __yew_vtag = ::yew::virtual_dom::vtag::VTag::new("p");
                        __yew_vtag.add_attributes(<[_]>::into_vec(box []));
                        __yew_vtag.add_listeners(<[_]>::into_vec(box []));
                        __yew_vtag.add_children(<[_]>::into_vec(box [::yew::virtual_dom::VNode::from("City: "), ::yew::virtual_dom::VNode::from({ &self.form.model().address.city })]));
                        ::yew::virtual_dom::VNode::from(__yew_vtag)
                    }), ::yew::virtual_dom::VNode::from({
                        let mut __yew_vtag = ::yew::virtual_dom::vtag::VTag::new("p");
                        __yew_vtag.add_attributes(<[_]>::into_vec(box []));
                        __yew_vtag.add_listeners(<[_]>::into_vec(box []));
                        __yew_vtag.add_children(<[_]>::into_vec(box [::yew::virtual_dom::VNode::from("Province: "), ::yew::virtual_dom::VNode::from({ &self.form.model().address.province })]));
                        ::yew::virtual_dom::VNode::from(__yew_vtag)
                    }), ::yew::virtual_dom::VNode::from({
                        let mut __yew_vtag = ::yew::virtual_dom::vtag::VTag::new("p");
                        __yew_vtag.add_attributes(<[_]>::into_vec(box []));
                        __yew_vtag.add_listeners(<[_]>::into_vec(box []));
                        __yew_vtag.add_children(<[_]>::into_vec(box [::yew::virtual_dom::VNode::from("Country: "), ::yew::virtual_dom::VNode::from({ &self.form.model().address.country })]));
                        ::yew::virtual_dom::VNode::from(__yew_vtag)
                    }), ::yew::virtual_dom::VNode::from({
                        let mut __yew_vtag = ::yew::virtual_dom::vtag::VTag::new("p");
                        __yew_vtag.add_attributes(<[_]>::into_vec(box []));
                        __yew_vtag.add_listeners(<[_]>::into_vec(box []));
                        __yew_vtag.add_children(<[_]>::into_vec(box [::yew::virtual_dom::VNode::from("Accepted Terms: "), ::yew::virtual_dom::VNode::from({ self.form.model().accept_terms })]));
                        ::yew::virtual_dom::VNode::from(__yew_vtag)
                    })]));
                    ::yew::virtual_dom::VNode::from(__yew_vtag)
                })]));
                ::yew::virtual_dom::VNode::from(__yew_vtag)
            })
        }
    }
}

fn main() {
    h
    yew::start_app::<App>();
}
