const BASE_URL: &str = "archiveofourown.org/";
const PROTOCOL: &str = "https://";
const WORK: &str = "works/";
pub struct URLBuilder {}
impl URLBuilder {
    pub fn get_work_url_by_id(&self, id: &str) -> String {
        format!("{PROTOCOL}{BASE_URL}{WORK}/{id}")
    }
}
