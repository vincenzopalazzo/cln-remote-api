use clightningrpc_plugin::commands::json_utils;
use clightningrpc_plugin::commands::RPCCommand;
use clightningrpc_plugin::plugin::Plugin;
use clightningrpc_plugin::types::LogLevel;
use rest_api::run_rocket;
use serde_json::{from_value, Value};

#[derive(Clone)]
pub(crate) struct PluginState;

#[derive(Clone)]
struct OnShutdown;

impl RPCCommand<PluginState> for OnShutdown {
    fn call_void<'c>(&self, _plugin: &mut Plugin<PluginState>, _request: &'c Value) {}
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
            let server_port = conf.options[0].to_owned();
            let server_port: i32 = from_value(server_port).unwrap();
            if server_port >= 0 {
                plugin.log(LogLevel::Info, "running rest server on port {port}");
                let path = conf.configuration.lightning_dir.as_str();
                run_rocket(path);
            }
            json_utils::init_payload()
        })
        .clone()
}
