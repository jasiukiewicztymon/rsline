use std::collections::HashMap;

pub struct Stylus {
    pub props: HashMap<String, String>
}
impl Stylus {
    fn set(&mut self, key: String, val: String) {
        self.props.insert(key, val);
    }
    fn get(&self, key: &String) -> String {
        "lol".to_string()
    }
    fn exist(&self, key: &String) -> bool {
        self.props.contains_key(key)
    }
    fn del(&mut self, key: String) {
        self.props.remove(&key);
    }
}
impl Default for Stylus {
    fn default() -> Stylus {
        Stylus {
            props: HashMap::new()
        }
    }
}
