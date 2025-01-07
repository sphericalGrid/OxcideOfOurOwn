use crate::selector_map::SelectorMap;

use super::network;
use super::scraping;
pub struct WorkHeader {
    stats: Stats,
}
impl WorkHeader {}
struct Stats {
    language: String,
    words: String,
    current_chapter_count: String,
    chapter_max: String,
    comment: String,
    kudos: String,
    hits: String,
}
impl Stats {
    pub fn new() -> Self {
        Self {
            language: "".to_owned(),
            words: "".to_owned(),
            current_chapter_count: "".to_owned(),
            chapter_max: "".to_owned(),
            comment: "".to_owned(),
            kudos: "".to_owned(),
            hits: "".to_owned(),
        }
    }
}
