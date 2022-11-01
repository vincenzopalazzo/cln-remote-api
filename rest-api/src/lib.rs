//! Core Lightning Rest API for Keet!
//!
//! This crate support the following RPC call
//! `sendPayment`, `addInvoice`, `decodeInvoice`, `listInvoices`, `subscribeToInvoices`, `nodeInfo`
use clightningrpc_common::client::Client as RPCClient;
use rocket_okapi::openapi_get_routes;
use rocket_okapi::swagger_ui::*;
use std::sync::Once;

#[macro_use]
extern crate rocket;

mod info;
mod pay_api;
pub mod utils;
use info::*;
use pay_api::*;

static INIT: Once = Once::new();
pub(crate) static mut CLN: Option<RPCClient> = None;

pub async fn run_rocket(path: &str) {
    INIT.call_once(|| unsafe { CLN = Some(RPCClient::new(&path)) });
    let _ = rocket::build()
        .mount(
            "/",
            openapi_get_routes![
                node_info,
                get_invoice,
                pay_invoice,
                decode_invoice,
                list_invoices
            ],
        )
        .mount(
            "/swagger-ui/",
            make_swagger_ui(&SwaggerUIConfig {
                url: "../openapi.json".to_owned(),
                ..Default::default()
            }),
        )
        .launch()
        .await;
}
