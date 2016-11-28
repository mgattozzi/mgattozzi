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

pub fn css(conf: &Config) -> Option<PreProc> {
    if let Some(css) = conf.get("css") {

        match css {
            &Value::Table(ref tab) => {
                match tab.get("pre_processor").expect("No pre_processor field") {
                    &Value::Boolean(pp) => {
                        if pp {
                            match tab.get("css_processor").expect("No css_processor field") {
                                &Value::String(ref css_proc) => {
                                    if css_proc == "sass" {
                                        Some(PreProc::Sass)
                                    } else if css_proc == "less" {
                                        Some(PreProc::Less)
                                    } else {
                                        None
                                    }
                                },
                                _ => panic!("Incorrect type for pre_proccessor"),
                            }
                        } else {
                            None
                        }
                    },
                    _ => panic!("Incorrect type for pre_proccessor"),
                }
            },
            _ => panic!("No css table"),
        }
    } else {
        None
    }
}

pub fn update_duration(conf: &Config) -> u64 {
    let sleep_default = 5;

    match conf.get("site"){
        Some(&Value::Table(ref tab)) => {
            match tab.get("sleep_update") {
                Some(&Value::Integer(val)) => {
                    val as u64
                },
                _ => sleep_default,
            }
        },
        _ => sleep_default,
    }
}

#[derive(PartialEq, Eq)]
pub enum PreProc {
    Less,
    Sass,
}
