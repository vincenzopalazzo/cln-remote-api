use clightningrpc_plugin::commands::RPCCommand;
use clightningrpc_plugin::plugin::Plugin;
use rest_api::run_rocket;
use serde_json::Value;

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
        .clone()
}
