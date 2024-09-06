use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::fmt::Display;
use std::sync::{Arc, RwLock};

/// metrics 的数据结构
/// 如果加了 clone ，clone后，就完全是一个新的 metrics
#[derive(Debug, Clone)] // clone 是对 Arc 进行 clone
pub struct Metrics {
    data: Arc<RwLock<HashMap<String, i64>>>,
}

impl Metrics {
    pub fn new() -> Self {
        Self {
            data: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    // 任何东西，只要能够转换为 String 都可以作为 key
    pub fn inc(&self, key: impl Into<String>) -> Result<()> {
        // `MutexGuard<'_, HashMap<String, i64>>` cannot be sent between threads safely
        let mut mutex_map = self
            .data
            // lock()? 返回的错误无法在线程之间安全的传递
            .write()
            .map_err(|e| {
                let error = anyhow!(e.to_string());
                error
            })?;

        let counter = mutex_map.entry(key.into()).or_insert(0);
        *counter += 1;

        Ok(())
    }

    pub fn dec(&self, key: impl Into<String>) -> Result<()> {
        let mut mutex_map = self.data.write().map_err(|e| anyhow!(e.to_string()))?;

        let counter = mutex_map.entry(key.into()).or_insert(1);
        *counter -= 1;

        Ok(())
    }

    pub fn snapshot(&self) -> Result<HashMap<String, i64>> {
        Ok(self
            .data
            .read()
            .map_err(|e| anyhow!(e.to_string()))?
            .clone())
    }
}

impl Display for Metrics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let data = self
            .data
            .read()
            // error 无法转换，直接生成一个新的 fmt::Error
            .map_err(|_e| std::fmt::Error {})?;

        for (key, value) in data.iter() {
            writeln!(f, "{}: {}", key, value)?;
        }

        Ok(())
    }
}
