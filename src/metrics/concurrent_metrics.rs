use anyhow::Result;
use dashmap::DashMap;
use std::fmt::Display;
use std::sync::Arc;

/// metrics 的数据结构
/// 如果加了 clone ，clone后，就完全是一个新的 metrics
#[derive(Debug, Clone)] // clone 是对 Arc 进行 clone
pub struct ConcurrentMetrics {
    // 锁相关的操作，全部被封装在 DashMap 的内部
    data: Arc<DashMap<String, i64>>,
}

impl ConcurrentMetrics {
    pub fn new() -> Self {
        Self {
            data: Arc::new(DashMap::new()),
        }
    }

    // 任何东西，只要能够转换为 String 都可以作为 key
    pub fn inc(&self, key: impl Into<String>) -> Result<()> {
        // `MutexGuard<'_, HashMap<String, i64>>` cannot be sent between threads safely
        let mut counter = self
            .data
            // lock()? 返回的错误无法在线程之间安全的传递
            .entry(key.into())
            .or_insert(0);

        *counter += 1;

        Ok(())
    }

    pub fn dec(&self, key: impl Into<String>) -> Result<()> {
        let mut counter = self.data.entry(key.into()).or_insert(0);

        *counter -= 1;

        Ok(())
    }

    // pub fn snapshot(&self) -> Result<HashMap<String, i64>> {
    //     Ok(self
    //         .data
    //         .read()
    //         .map_err(|e| anyhow!(e.to_string()))?
    //         .clone())
    // }
}

impl Display for ConcurrentMetrics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for entry in self.data.iter() {
            writeln!(f, "{}: {}", entry.key(), entry.value())?;
        }

        Ok(())
    }
}
