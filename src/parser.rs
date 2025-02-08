use scraper::{Html, Selector};
use crate::fingerprints;

pub fn analyze_technologies(html: &str) -> Vec<String> {
    let document = Html::parse_document(html);
    let script_selector = Selector::parse("script").unwrap();
    let meta_selector = Selector::parse("meta").unwrap();

    let mut detected = Vec::new();

    // scan scripts
    for element in document.select(&script_selector) {
        if let Some(script_content) = element.value().attr("src") {
            for (tech, pattern) in fingerprints::TECH_FINGERPRINTS.iter() {
                if pattern.is_match(script_content) {
                    detected.push(tech.to_string());
                }
            }
        }
    }

    // scan metatags
    for element in document.select(&meta_selector) {
        if let Some(meta_content) = element.value().attr("content") {
            for (tech, pattern) in fingerprints::TECH_FINGERPRINTS.iter() {
                if pattern.is_match(meta_content) {
                    detected.push(tech.to_string());
                }
            }
        }
    }

    detected
}
