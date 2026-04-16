#[cfg(feature = "plugins")]
use teamtalk::extensions::plugins::{PluginFlow, PluginManager, TeamTalkPlugin};
#[cfg(feature = "plugins")]
use teamtalk::{Client, Event, Message};

#[cfg(feature = "plugins")]
struct GreeterPlugin;

#[cfg(feature = "plugins")]
impl TeamTalkPlugin for GreeterPlugin {
    fn on_load(
        &mut self,
        _client: &Client,
    ) -> Result<(), teamtalk::extensions::plugins::PluginError> {
        println!("GreeterPlugin loaded");
        Ok(())
    }

    fn on_unload(&mut self) {
        println!("GreeterPlugin unloaded");
    }

    fn on_event(&mut self, event: Event, _message: &Message) -> PluginFlow {
        println!("GreeterPlugin received event: {event:?}");
        PluginFlow::Continue
    }

    fn on_command(&mut self, command: &str, args: &[String]) -> PluginFlow {
        println!("GreeterPlugin command: {command} {:?}", args);
        PluginFlow::Continue
    }
}

#[cfg(feature = "plugins")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut manager = PluginManager::new();

    manager.register_plugin("greeter", Box::new(GreeterPlugin));
    println!("static plugins: {:?}", manager.plugin_names());

    let dyn_path = std::path::Path::new("plugins/sample_plugin.dll");
    if dyn_path.exists() {
        manager.load_plugin(dyn_path)?;
        println!("dynamic plugins: {:?}", manager.plugin_names());
    }

    if manager.is_loaded("greeter") {
        manager.unload("greeter")?;
    }

    if manager.is_loaded("sample") {
        manager.unload("sample")?;
    }

    Ok(())
}

#[cfg(not(feature = "plugins"))]
fn main() {
    eprintln!("Enable plugins feature: cargo run --example plugin_loader --features plugins");
}
