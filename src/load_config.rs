use std::fs::File;
use std::io::Read;
use log::warn;
use crate::Config;

pub fn load_config() -> Config {
    let config = File::open("./application.toml")
        .and_then(|mut file| {
            let mut buffer = String::new();
            file.read_to_string(&mut buffer)?;
            Ok(buffer)
        })
        .and_then(|buffer| {
            toml::from_str::<Config>(&buffer).map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))
        })
        .map_err(|err| {
            warn!("Can't read config file: {}", err);
        })
        .ok().expect("load application.toml failed");
    config
}