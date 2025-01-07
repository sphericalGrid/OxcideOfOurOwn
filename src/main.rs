mod network;
mod scraping;
mod selector_map;
mod utils;
mod work;
use std::rc::Rc;
fn main() {
    let resp = network::get_response("https://archiveofourown.org/works?commit=Sort+and+Filter&work_search[sort_column]=kudos_count&work_search[other_tag_names]=&work_search[excluded_tag_names]=&work_search[crossover]=&work_search[complete]=&work_search[words_from]=&work_search[words_to]=&work_search[date_from]=&work_search[date_to]=&work_search[query]=&work_search[language_id]=&tag_id=Harry+Potter+-+J*d*+K*d*+Rowling", 5);
    scraping::scrape_header(resp);
    let _ = utils::URLBuilder::new().get_work_url_by_id("da");
}
