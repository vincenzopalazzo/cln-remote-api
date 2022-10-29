//! All the info API for cln.
use rocket::response::{content, status};
use rocket_okapi::openapi;
use std::collections::HashMap;

use crate::utils::run_cln_request_for_rocket;

#[openapi(tag = "Info")]
#[get("/cln/v1/nodeInfo")]
pub fn node_info() -> status::Custom<content::RawJson<String>> {
    run_cln_request_for_rocket("getinfo", &HashMap::<String, String>::new())
}
