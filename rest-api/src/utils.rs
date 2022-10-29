//! Utils crate that contains all the utils method
//! used inside the repository.
use crate::CLN;
use clightningrpc_common::types::Response;
use rocket::http::Status;
use rocket::response::content::RawJson;
use rocket::response::status::Custom;
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;

/// Wrap the cln method to run a custom request and return
/// a rocket response
pub fn run_cln_request_for_rocket<Req: Serialize>(
    method: &str,
    req: &Req,
) -> Custom<RawJson<String>> {
    unsafe {
        if let Some(cln) = &CLN {
            let resp = cln
                .send_request(method, req)
                .and_then(|res: Response<HashMap<String, Value>>| res.into_result());
            return match resp {
                Ok(resp) => {
                    let raw = serde_json::to_string(&resp).unwrap();
                    Custom(Status::ImATeapot, RawJson(raw))
                }
                Err(err) => {
                    let raw = format!("{{err: \"{}\"}}", err);
                    Custom(Status::InternalServerError, RawJson(raw))
                }
            };
        }
        panic!("This must not happens!");
    }
}
