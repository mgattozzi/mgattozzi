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
