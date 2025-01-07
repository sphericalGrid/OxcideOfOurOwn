use super::network;
use super::scraping;
pub struct WorkHeader {
    stats: Stats,
}
impl WorkHeader {
    pub fn from(url: &str) -> Self {
        Self {
            stats: Stats::from(url),
        }
    }
}
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
    pub fn from(url: &str) -> Self {
        println!("Scraping: {url}");
        let response = network::get_response(url, 5);
        scraping::scrape_header(response);
        Stats::new()
    }
}
