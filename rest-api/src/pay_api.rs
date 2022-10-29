//! Core Lightning Pay API for Keet!
//!
//! This crate support all the Pay API that Keet required.
use crate::utils::run_cln_request_for_rocket;
use rocket::response::{content::RawJson, status::Custom};
use rocket::serde::{json::Json, Deserialize};
use rocket_okapi::okapi::schemars::{self, JsonSchema};
use rocket_okapi::openapi;
use serde::Serialize;

#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct GetInvoice<'r> {
    amount_msat: &'r str,
    label: &'r str,
    description: &'r str,
}

#[openapi(tag = "Pay")]
#[post("/cln/v1/invoice", data = "<get_invoice>")]
pub fn get_invoice(get_invoice: Json<GetInvoice<'_>>) -> Custom<RawJson<String>> {
    run_cln_request_for_rocket("invoice", &get_invoice.0)
}

#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct Invoice<'a> {
    bolt11: &'a str,
}

#[openapi(tag = "Pay")]
#[post("/cln/v1/pay", data = "<invoice>")]
pub fn pay_invoice(invoice: Json<Invoice<'_>>) -> Custom<RawJson<String>> {
    run_cln_request_for_rocket("pay", &invoice.0)
}

#[openapi(tag = "Pay")]
#[post("/cln/v1/decodeInvoice", data = "<invoice>")]
pub fn decode_invoice(invoice: Json<Invoice<'_>>) -> Custom<RawJson<String>> {
    run_cln_request_for_rocket("decodepay", &invoice.0)
}
