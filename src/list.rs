use philipshue::Bridge;

pub fn list(bridge: Bridge) {
    match bridge.get_all_lights() {
        Ok(lights) => {
            let max_name_len = lights.values().map(|l| l.name.len()).chain(Some(4)).max().unwrap();
            println!("id {0:1$} on  bri",
                     "name",
                     max_name_len);
            for (id, light) in lights.iter() {
                println!("{:2} {:name_len$} {:3} {:3}",
                         id,
                         light.name,
                         if light.state.on { "on" } else { "off" },
                         light.state.bri,
                         name_len = max_name_len);
            }
        }
        Err(err) => println!("Error: {}", err),
    }
}
