use std::collections::HashMap;




#[derive(Debug, Clone)]
pub struct Environement {
    labels: HashMap<String, String>,
}

impl Environement {
    pub fn new() -> Self {
        Self {
            labels: HashMap::new(),
        }
    }

    pub fn add_label(&mut self, name: String, label: String) {
        self.labels.insert(name, label);
    }

    pub fn get_label(&self, name: &String) -> Option<&String> {
        self.labels.get(name)
    }
}
