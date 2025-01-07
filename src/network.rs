use core::panic;
use reqwest::blocking::get;

pub fn get_response(url: &str, retry: u8) -> String {
    let resp = get(url);
    if retry == 0 {
        panic!("Failed to get request even after retries");
    }
    match resp {
        Ok(response) => response.text().expect("Failed to get response"),
        Err(error) => {
            println!("Reqwest error: {error} \nRetries left: {retry}");
            get_response(url, retry - 1)
        }
    }
}
