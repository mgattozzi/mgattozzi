use std::collections::BTreeMap;
use std::fs::File;
use std::io::Read;
use util::mkpath;
use toml::{Parser, Value};
use slog::Logger;

pub type Config = BTreeMap<String, Value>;

pub fn parse_config(log: &Logger) -> Option<Config> {
    let config = mkpath("Site.toml");
    if let Ok(mut f) = File::open(config) {
        let mut buff = String::new();
        let _ = f.read_to_string(&mut buff);
        let parse = Parser::new(&buff).parse();
        match parse {
            Some(_) => {
                info!(log, "Configuration parsed succesfully");
                parse
            },
            None => {
                error!(log, "Configuration not parsed");
                None
            },
        }
    } else {
        error!(log, "Unable to open Site.toml file.");
        None
    }
}

// Note that these lookups aren't using lookup because the functions for the
// library were not implemented with & versions.

pub fn css(conf: &Config) -> Option<PreProc> {
    conf.get("css")
        .and_then(Value::as_table)
        .and_then(|x| x.get("css_processor"))
        .and_then(Value::as_str)
        .and_then(|css_proc|
            match css_proc {
                "sass" => Some(PreProc::Sass),
                "less" => Some(PreProc::Less),
                _ => None,
            })
}

pub fn update_duration(conf: &Config) -> u64 {
    let sleep_default = 5;

    conf.get("site")
        .and_then(Value::as_table)
        .and_then(|x| x.get("sleep_update"))
        .and_then(Value::as_integer)
        .unwrap_or(sleep_default) as u64
}

pub fn get_port(conf: &Config) -> u64 {
    let port_default = 3000;

    conf.get("site")
        .and_then(Value::as_table)
        .and_then(|x| x.get("port"))
        .and_then(Value::as_integer)
        .unwrap_or(port_default) as u64
}

#[derive(PartialEq, Eq)]
pub enum PreProc {
    Less,
    Sass,
}
