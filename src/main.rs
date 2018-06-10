#![feature(plugin, custom_derive, decl_macro)]
#![plugin(rocket_codegen)]

#[cfg(test)]
#[macro_use]
extern crate lazy_static;

extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde;
extern crate stripe;
extern crate log;

use rocket::Rocket;
use rocket_contrib::Template;

use std::env;
use rocket::request::LenientForm;
use rocket::response::Redirect;


#[derive(Serialize)]
struct TemplateContext {
    page_title: String,
    stripe_publishable_key: String,
    form_data_image: String,
    form_data_name: String,
    form_data_description: String,
    form_data_panel_label: String,
    form_data_label: String,
    form_data_collect_billing_address: String,
    form_data_allow_remember_me: String,
    form_data_locale: String,
    customer_id: String,
}


fn env(key: &str) -> String {
    env::var(key).unwrap_or(String::new())
}

#[get("/<customer_id>")]
fn index(customer_id: String) -> Template {
    let context = TemplateContext {
        page_title: env("PAGE_TITLE"),
        stripe_publishable_key: env("STRIPE_PUBLISHABLE_KEY"),
        form_data_name: env("FORM_DATA_NAME"),
        form_data_description: env("FORM_DATA_DESCRIPTION"),
        form_data_image: env("FORM_DATA_IMAGE"),
        form_data_panel_label: env("FORM_DATA_PANEL_LABEL"),
        form_data_label: env("FORM_DATA_LABEL"),
        form_data_collect_billing_address: env("FORM_DATA_COLLECT_BILLING_ADDRESS"),
        form_data_allow_remember_me: env("FORM_DATA_ALLOW_REMEMBER_ME"),
        form_data_locale: env("FORM_DATA_LOCALE"),
        customer_id,
    };

    Template::render("index", &context)
}

#[derive(FromForm, Debug)]
struct CardUpdate {
    #[form(field = "stripeToken")]
    stripe_token: String,
    #[form(field = "stripeEmail")]
    stripe_email: String
}


#[post("/<customer_id>", data = "<card_update_form>")]
fn update_card(customer_id: String, card_update_form: LenientForm<CardUpdate>) -> Result<Redirect, stripe::Error> {
    let card_update = card_update_form.get();
    let client = stripe::Client::new(env("STRIPE_SECRET_KEY"));


    // https://github.com/rapiditynetworks/stripe-rs
    let mut params = stripe::CustomerParams::default();
    params.email = Some(&card_update.stripe_email);
    params.source = Some(stripe::CustomerSource::Token(&card_update.stripe_token));

    stripe::Customer::update(&client, &customer_id, params).and_then(|_customer| {
        Ok(Redirect::to(&env("SUCCESS_REDIRECT_URL")))
    })
}

fn rocket() -> Rocket {
    rocket::ignite().attach(Template::fairing()).mount("/", routes![index, update_card])
}

fn main() {
    rocket().launch();
}
