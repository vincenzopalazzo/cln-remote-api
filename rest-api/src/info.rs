//! All the info API for cln.
use clightningrpc_common::types::Response;
use rocket::http::Status;
use rocket::response::{content, status};
use rocket_okapi::openapi;
use serde_json::Value;
use std::collections::HashMap;

use crate::CLN;

#[openapi(tag = "Info")]
#[get("/cln/v1/nodeInfo")]
pub fn node_info() -> status::Custom<content::RawJson<String>> {
    unsafe {
        if let Some(cln) = &CLN {
            let resp = cln
                .send_request("getinfo", HashMap::<String, Value>::new())
                .and_then(|res: Response<HashMap<String, Value>>| res.into_result());
            return match resp {
                Ok(resp) => {
                    let raw = serde_json::to_string(&resp).unwrap();
                    status::Custom(Status::ImATeapot, content::RawJson(raw))
                }
                Err(err) => {
                    let raw = format!("{{err: \"{}\"}}", err);
                    status::Custom(Status::InternalServerError, content::RawJson(raw))
                }
            };
        }
    }
    panic!("Should nerver happens");
}
