use core::panic;
use reqwest::get;

pub fn get_response(url: &str, retry: u8) -> String {
    let resp = get(url);
    "".to_owned()
}
