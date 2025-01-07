use scraper::{Html, Selector};
pub fn scrape_header(document: String) {
    let html = Html::parse_document(document.as_str());
    scrape_header_stats(html);
}
fn scrape_header_stats(document: Html) {}
