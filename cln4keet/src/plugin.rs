use std::collections::HashMap;

use clightningrpc_plugin::commands::json_utils;
use clightningrpc_plugin::commands::RPCCommand;
use clightningrpc_plugin::plugin::Plugin;
use clightningrpc_plugin::types::LogLevel;
use rest_api::run_rocket;
use rocket::tokio::spawn;
use serde_json::{from_value, Value};

#[derive(Clone)]
pub(crate) struct PluginState;

#[derive(Clone)]
struct OnShutdown;

impl RPCCommand<PluginState> for OnShutdown {
    fn call_void<'c>(&self, _plugin: &mut Plugin<PluginState>, _request: &'c Value) {
        std::process::exit(0);
    }
}

// FIXME: register the on init method and if the port is > -1 run the server.
pub(crate) fn build_plugin() -> Plugin<PluginState> {
    Plugin::new(PluginState {}, true)
        // FIXME: store the init path
        .add_opt(
            "rest-port",
            "int",
            Some("-1".to_string()),
            "The port where the server will run.",
            false,
        )
        .register_notification("shutdown", OnShutdown {})
        .on_init(&|plugin| -> Value {
            plugin.log(LogLevel::Debug, "Custom init method called");
            // FIXME: improve the get option plugin API
            let conf = if let Some(conf_plugin) = &plugin.conf {
                conf_plugin
            } else {
                panic!("this should never happen")
            };
            let opts: HashMap<String, Value> = from_value(conf.options.clone()).unwrap();
            let server_port = opts["rest-port"].as_i64().unwrap().clone();
            if server_port >= 0 {
                plugin.log(LogLevel::Info, "running rest server on port {port}");
                let path = conf.configuration.lightning_dir.to_owned();
                let unix_file = conf.configuration.rpc_file.to_owned();
                spawn(async move {
                    let full_path = format!("{path}/{unix_file}");
                    run_rocket(full_path.as_str()).await;
                });
            }
            // FIXME: disable plugin is the port it is not enabled
            json_utils::init_payload()
        })
        .clone()
}
