use std::collections::HashMap;

/// metrics 的数据结构
#[derive(Debug)]
pub struct Metrics {
    data: HashMap<String, i64>,
}

impl Metrics {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn inc(&mut self, key: impl Into<String>) {
        let counter = self.data.entry(key.into()).or_insert(0);
        *counter += 1;
    }

    pub fn dec(&mut self, key: impl Into<String>) {
        let counter = self.data.entry(key.into()).or_insert(0);
        *counter -= 1;
    }

    pub fn snapshot(&self) -> HashMap<String, i64> {
        self.data.clone()
    }
}
