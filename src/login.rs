use philipshue::bridge;
use philipshue::errors::{HueError, HueErrorKind, BridgeError};
use std;

pub fn login(bridge_ip: &str, device_type: &str) -> String {
    loop {
        match bridge::register_user(bridge_ip, device_type) {
            // A new user has succesfully been registered and the username is returned
            Ok(new_username) => {
                return new_username;
            },
            // Prompt the user to press the link button
            Err(HueError(HueErrorKind::BridgeError{error: BridgeError::LinkButtonNotPressed, ..}, _)) => {
                println!("Please, press the link on the bridge. Retrying in 5 seconds");
                std::thread::sleep(std::time::Duration::from_secs(5));
            },
            // Some other error happened
            Err(e) => {
                println!("Unexpected error occured: {:?}", e);
            }
        }
    }
}
