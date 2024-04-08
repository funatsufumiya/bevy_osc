use bevy::prelude::*;
use bevy_mod_osc::osc_sender::OscSender;
// use rosc::OscType;

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .insert_resource(OscSender::new("[::1]", 1234))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut osc_sender: ResMut<OscSender>,
) {
    // osc send message
    osc_sender.send("/test", [1, 2, 3]);

    // or
    // osc_sender.send("/test", vec![OscType::Int(1), OscType::Int(2), OscType::Int(3)]);

    println!("Sent OSC message: /test 1 2 3 to {}:{}", osc_sender.host, osc_sender.port);

    std::process::exit(0);
}