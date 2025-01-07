use crate::work::{Stats, WorkHeader};
use reqwest::blocking::get;
use scraper::{ElementRef, Html, Selector};
use std::{collections::HashMap, rc::Rc};
pub fn scrape_header(document: String) {
    let html = Html::parse_document(document.as_str());
    let work_selector = &Selector::parse("li.work").unwrap();
    for work in html.select(&work_selector) {
        scrape_header_stats(work);
    }
}
fn scrape_header_stats(document: ElementRef) {
    let el = document
        .select(&Selector::parse("dl.stats").unwrap())
        .next()
        .unwrap();
    let mut temp = Stats::new();
    temp.update(el);
    println!("{:#?}", temp)
}
