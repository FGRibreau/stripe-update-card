#[macro_use]
extern crate serde_derive;
extern crate log;
extern crate serde;
extern crate serde_json;
extern crate stripe;

use std::env;

use actix_web::{get, post, App, HttpResponse, HttpServer, web};
use actix_web::body::BoxBody;
use actix_web::dev::ServiceResponse;
use actix_web::http::header::ContentType;
use actix_web::http::{StatusCode};
use actix_web::middleware::{ErrorHandlerResponse, ErrorHandlers};

use actix_web::Result;

use log::error;
use serde_json::json;
use stripe::TokenId;
use handlebars::Handlebars;

fn env(key: &str) -> String {
    env::var(key).unwrap_or(String::new())
}

#[get("/{customer_id}")]
async fn index(hb: web::Data<Handlebars<'_>>, path: web::Path<String>) -> HttpResponse {

    let customer_id = path.into_inner();

    let data = json!({
        "page_title": env("PAGE_TITLE"),
        "stripe_publishable_key": env("STRIPE_PUBLISHABLE_KEY"),
        "form_data_name": env("FORM_DATA_NAME"),
        "form_data_description": env("FORM_DATA_DESCRIPTION"),
        "form_data_image": env("FORM_DATA_IMAGE"),
        "form_data_panel_label": env("FORM_DATA_PANEL_LABEL"),
        "form_data_label": env("FORM_DATA_LABEL"),
        "form_data_collect_billing_address": env("FORM_DATA_COLLECT_BILLING_ADDRESS"),
        "form_data_allow_remember_me": env("FORM_DATA_ALLOW_REMEMBER_ME"),
        "form_data_locale": env("FORM_DATA_LOCALE"),
        "customer_id": customer_id
    });

    let body = hb.render("index", &data).unwrap();

    HttpResponse::Ok().body(body)
}


#[derive(Debug, Serialize, Deserialize)]
struct CardUpdate {
    #[serde(rename = "stripeToken")]
    stripe_token: String,
    #[serde(rename = "stripeEmail")]
    stripe_email: String,
}


#[post("/{customer_id}")]
async fn update_card(path: web::Path<String>, card_update_form: web::Form<CardUpdate> ) -> HttpResponse {
    let customer_id = path.into_inner();

    let client = stripe::Client::new(env("STRIPE_SECRET_KEY"));

    // https://github.com/rapiditynetworks/stripe-rs
    let mut params = stripe::UpdateCustomer::default();
    params.email = Some(&card_update_form.stripe_email);
    params.source = Some(stripe::PaymentSourceParams::Token(card_update_form.stripe_token.parse::<TokenId>().unwrap_or_default()));

    let res = stripe::Customer::update(&client, &customer_id.parse::<stripe::CustomerId>().unwrap_or_default(), params)
        .await;

    if let Err(error) = res{
        error!("Could not update customer customer_id={} response={:?}", customer_id, error);
        return HttpResponse::InternalServerError().finish();
    }

    HttpResponse::Ok().finish()
        /*.and_then(|_customer| {
        Ok(HttpResponse::Found()
            .insert_header((LOCATION, env("SUCCESS_REDIRECT_URL")))
            .finish())
    })*/
}

// Custom error handlers, to return HTML responses when an error occurs.
fn error_handlers() -> ErrorHandlers<BoxBody> {
    ErrorHandlers::new().handler(StatusCode::NOT_FOUND, not_found)
}

// Error handler for a 404 Page not found error.
fn not_found<B>(res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<BoxBody>> {
    let response = get_error_response(&res, "Page not found");
    Ok(ErrorHandlerResponse::Response(ServiceResponse::new(
        res.into_parts().0,
        response.map_into_left_body(),
    )))
}

// Generic error handler.
fn get_error_response<B>(res: &ServiceResponse<B>, error: &str) -> HttpResponse<BoxBody> {
    let request = res.request();

    // Provide a fallback to a simple plain text response in case an error occurs during the
    // rendering of the error page.
    let fallback = |err: &str| {
        HttpResponse::build(res.status())
            .content_type(ContentType::plaintext())
            .body(err.to_string())
    };

    let hb = request
        .app_data::<web::Data<Handlebars>>()
        .map(|t| t.get_ref());
    match hb {
        Some(hb) => {
            let data = json!({
                "error": error,
                "status_code": res.status().as_str()
            });
            let body = hb.render("error", &data);

            match body {
                Ok(body) => HttpResponse::build(res.status())
                    .content_type(ContentType::html())
                    .body(body),
                Err(_) => fallback(error),
            }
        }
        None => fallback(error),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Handlebars uses a repository for the compiled templates. This object must be
    // shared between the application threads, and is therefore passed to the
    // Application Builder as an atomic reference-counted pointer.
    let mut handlebars = Handlebars::new();
    handlebars
        .register_templates_directory(".html", "./static/templates")
        .unwrap();
    let handlebars_ref = web::Data::new(handlebars);

    HttpServer::new(move || {
        App::new().wrap(error_handlers()).app_data(handlebars_ref.clone())
            .service(index)
            .service(update_card)
    })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
