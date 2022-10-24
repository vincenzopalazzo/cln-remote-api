//! Core Lightning Rest API for Keet!
//!
//! This crate support the following RPC call
//! `sendPayment`, `addInvoice`, `decodeInvoice`, `listInvoices`, `subscribeToInvoices`, `nodeInfo`
use clightningrpc_common::client::Client as RPCClient;
use once_cell::sync::OnceCell;

#[macro_use]
extern crate rocket;

mod info;

pub(crate) static CLN: OnceCell<RPCClient> = OnceCell::new();

pub fn run_rocket(path: &str) {
    let _ = CLN.get_or_init(|| RPCClient::new(&path));
    let _ = rocket::build().mount("/cln/v1/nodeInfo", routes![info::node_info]);
}
