use std::clone;

use scraper::ElementRef;
use scraper::Selector;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct WorkHeader {
    stats: Stats,
}
impl WorkHeader {}
#[derive(Serialize, Deserialize, Debug)]
pub struct Stats {
    language: String,
    words: String,
    current_chapter_count: String,
    chapter_max: String,
    comment: String,
    kudos: String,
    bookmarks: String,
    hits: String,
}
impl Stats {
    pub fn new() -> Self {
        Self {
            language: String::from(""),
            words: String::from(""),
            current_chapter_count: String::from(""),
            chapter_max: String::from(""),
            comment: String::from(""),
            kudos: String::from(""),
            bookmarks: String::from(""),
            hits: String::from(""),
        }
    }
    pub fn update(&mut self, el: ElementRef) {
        self.language = self.select_stat(el, "dd.language");
        self.words = self.select_stat(el, "dd.words");
        let chapters = self.select_stat(el, "dd.chapters");
        let temp = chapters.split("/").collect::<Vec<&str>>();
        self.current_chapter_count = String::from(temp[0]);
        self.chapter_max = String::from(temp[1]);
        self.comment = self.select_stat(el, "dd.comments");
        self.kudos = self.select_stat(el, "dd.kudos");
        self.bookmarks = self.select_stat(el, "dd.bookmarks");
        self.hits = self.select_stat(el, "dd.hits");
    }
    fn select_stat(&self, el: ElementRef, stat: &str) -> String {
        el.select(&self.get_sel(stat))
            .next()
            .unwrap()
            .text()
            .collect::<String>()
    }
    fn get_sel(&self, sel: &str) -> Selector {
        Selector::parse(sel).unwrap()
    }
}
