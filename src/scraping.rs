use crate::selector_map::SelectorMap;

use scraper::{Html, Selector};
pub fn scrape_header(document: String, mut selectors: SelectorMap) {
    let html = Html::parse_document(document.as_str());
    let work_selector = selectors.get("li.work");
    println!("{}", html.select(work_selector).count());
    for work in html.select(work_selector) {
        println!("yay")
    }
    scrape_header_stats(html, selectors);
}
fn scrape_header_stats(document: Html, selectors: SelectorMap) {}
