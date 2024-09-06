use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::Arc;

#[derive(Debug)]
pub struct AtomicMetrics {
    data: Arc<HashMap<&'static str, AtomicI64>>,
}

impl AtomicMetrics {
    // 一开始就要将 metric name 全部确定好
    pub fn new(metric_names: &[&'static str]) -> AtomicMetrics {
        let map = metric_names
            .iter()
            .map(|&name| (name, AtomicI64::new(0)))
            // 这里也可以不写，让其从下面的 Arc::new()中去推断
            .collect::<HashMap<_, _>>();

        AtomicMetrics {
            data: Arc::new(map),
        }
    }

    pub fn inc(&self, key: impl AsRef<str>) -> Result<()> {
        let key = key.as_ref();

        let counter = self
            .data
            .get(key)
            .ok_or_else(|| anyhow!("key {} not found", key))?;

        counter.fetch_add(1, Ordering::Relaxed);
        Ok(())
    }
}

impl Clone for AtomicMetrics {
    fn clone(&self) -> Self {
        AtomicMetrics {
            data: Arc::clone(&self.data),
        }
    }
}

impl Display for AtomicMetrics {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (key, value) in self.data.iter() {
            writeln!(f, "{}: {}", key, value.load(Ordering::Relaxed))?;
        }

        Ok(())
    }
}
