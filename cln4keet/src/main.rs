//! CLN plugin that expose some cln API
//! under a rest endpoint.
mod plugin;
use crate::plugin::build_plugin;

fn main() {
    build_plugin().start();
}
