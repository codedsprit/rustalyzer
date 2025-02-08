use regex::Regex;
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TECH_FINGERPRINTS: HashMap<&'static str, Regex> = {
        let mut m = HashMap::new();
        m.insert("jQuery", Regex::new(r"jquery.*\.js").unwrap());
        m.insert("Google Analytics", Regex::new(r"googletagmanager|analytics\.js").unwrap());
        m.insert("Bootstrap", Regex::new(r"bootstrap.*\.js").unwrap());
        m
    };
}
