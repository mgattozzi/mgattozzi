use std::collections::BTreeMap;
use std::fs::File;
use std::io::Read;
use util::mkpath;
use toml::{Parser, Value};

pub type Config = BTreeMap<String, Value>;

pub fn parse_config() -> Option<Config> {
    let config = mkpath("Site.toml");
    let mut f = File::open(config).expect("Unable to open Site.toml");
    let mut buff = String::new();
    let _ = f.read_to_string(&mut buff);
    Parser::new(&buff).parse()
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

#[derive(PartialEq, Eq)]
pub enum PreProc {
    Less,
    Sass,
}
