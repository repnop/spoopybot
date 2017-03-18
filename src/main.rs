#[macro_use]
extern crate serenity;
extern crate toml;

use serenity::client::Client;
use toml::Value;
use std::fs::File;
use std::io::Read;

fn main() {

    let mut config_file = File::open("config.toml").expect("Cannot open config.toml\n");
    let mut config_str = String::new();
    config_file.read_to_string(&mut config_str).expect("Could not read from config.toml\n");

    let config_str = config_str.parse::<Value>().expect("Could not parse config_str\n");

    let mut client = Client::login_bot(&config_str["token"].as_str().expect("No token provided"));
    client.with_framework(|f| f.configure(|c| c.prefix("!"))
                          .on("ping", ping)
                          .on("itdatboi", datboi)
                          .on("quit", quit));

    let _ = client.start();
}

command!(ping(_context, message) {
    let _ = message.reply("Pong!");
});

command!(datboi(_context, message) {
    let _ = message.reply("oh shit waddup");
});

command!(quit(_context, message) {
    std::process::exit(0);
});
