extern crate philipshue;
#[macro_use]
extern crate structopt;
#[macro_use]
extern crate clap;
extern crate preferences;

use structopt::StructOpt;

use philipshue::Bridge;
use preferences::{AppInfo, PreferencesMap, Preferences};

mod login;
use login::login;
mod list;
use list::list;
mod command;
use command::command;
use command::Command;

const APP_INFO: AppInfo = AppInfo{name: "hue-cli", author: "Josh Mcguigan"};
const PREFS_KEY: &str = "user_prefs";
const PREFS_KEY_USERNAME : &str = "username";

fn main() {
    #[derive(Debug, StructOpt, PartialEq)]
    enum Opt {
        #[structopt(name = "login")]
        Login,
        #[structopt(name = "ls")]
        List,
        #[structopt(name = "cmd")]
        Command {
            #[structopt(raw(possible_values="&Command::variants()", case_insensitive = "true"))]
            /// The command sent to the Hue bridge
            cmd: Command
        }
    }

    let cli_option = Opt::from_args();

    // Discover a bridge
    let bridge_ip = philipshue::bridge::discover()
        .expect("Unable to discover Hue bridge")
        .pop().expect("Unable to discover Hue bridge")
        .into_ip();

    let load_result = PreferencesMap::<String>::load(&APP_INFO, PREFS_KEY);

    let mut preferences = load_result.unwrap_or(PreferencesMap::new());

    if cli_option == Opt::Login {
        let username = login(&bridge_ip, &APP_INFO.name);
        preferences.insert(PREFS_KEY_USERNAME.to_owned(), username);
        preferences.save(&APP_INFO, PREFS_KEY).expect("Failed to save login information");
        println!("Login successful");
        return;
    }

    let username: String = preferences.get(PREFS_KEY_USERNAME).map(|x| x.to_owned()).expect("Not logged in");
    let bridge = Bridge::new(bridge_ip, username);

    match cli_option {
        Opt::List => list(bridge),
        Opt::Command{cmd} => command(bridge, cmd),
        _ => {}
    }

}
