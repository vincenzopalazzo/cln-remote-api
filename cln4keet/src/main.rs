//! CLN plugin that expose some cln API
//! under a rest endpoint.
mod plugin;
use crate::plugin::build_plugin;

extern crate rocket;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    build_plugin().start();
    Ok(())
}
