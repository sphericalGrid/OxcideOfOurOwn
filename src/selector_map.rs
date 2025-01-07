use std::collections::HashMap;
use std::rc::Rc;

use scraper::Selector;

pub struct SelectorMap {
    map: HashMap<String, Selector>,
}

impl SelectorMap {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
    pub fn get(&mut self, key: &str) -> &Selector {
        let key = key.to_owned();
        if !self.map.contains_key(&key) {
            self.map
                .insert(key.clone(), Selector::parse(key.as_str()).unwrap());
        }
        self.map.get(&key).unwrap()
    }
}
