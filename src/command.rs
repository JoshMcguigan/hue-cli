use philipshue::Bridge;
use philipshue::hue;

arg_enum!{
        #[derive(Debug, PartialEq)]
        pub enum Command {
            On,
            Off,
            Toggle
        }
    }

pub fn command(bridge: Bridge, cmd: Command) {

    let bridge_cmd = match cmd {
        Command::On => hue::LightCommand::default().on(),
        Command::Off => hue::LightCommand::default().off(),
        Command::Toggle => {
            let mut light_state = None;
            match bridge.get_all_lights() {
                Ok(lights) => {
                    light_state = Some(false);
                    for (_i, light) in lights.iter() {
                        if light.state.on {
                            light_state = Some(true);
                        }
                    }
                }
                Err(err) => println!("Error: {}", err),
            }

            if light_state == Some(true) {
                hue::LightCommand::default().off()
            } else {
                hue::LightCommand::default()
                    .on()
                    .with_bri(u8::max_value())
            }
        }
    };

    // group ID 0 sends command to all known lights
    bridge.set_group_state(0,&bridge_cmd).expect("Command to bridge failed");

}

