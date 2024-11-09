use anyhow::Result;
use std::{
    collections::HashMap,
    fmt::Display,
    sync::{atomic::AtomicI64, Arc},
};
#[derive(Debug, Clone)]
pub struct AtomicMetrix {
    data: Arc<HashMap<&'static str, AtomicI64>>,
}

impl AtomicMetrix {
    pub fn new(metrixs: &[&'static str]) -> Self {
        let mut metrix_map = HashMap::new();
        for metrix in metrixs {
            metrix_map.insert(*metrix, AtomicI64::new(0));
        }
        Self {
            data: Arc::new(metrix_map),
        }
    }

    pub fn inc(&self, key: &'static str) -> Result<()> {
        let count = self
            .data
            .get(key)
            .ok_or(anyhow::anyhow!("Key not existed"))?;
        count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        Ok(())
    }

    pub fn dec(&self, key: &'static str) -> Result<()> {
        let count = self
            .data
            .get(key)
            .ok_or(anyhow::anyhow!("Key not existed"))?;
        count.fetch_sub(1, std::sync::atomic::Ordering::Relaxed);
        Ok(())
    }
}

impl Display for AtomicMetrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{")?;
        for (index, (key, value)) in self.data.iter().enumerate() {
            if self.data.len() - 1 == index {
                write!(
                    f,
                    "{}: {}",
                    key,
                    value.load(std::sync::atomic::Ordering::Relaxed)
                )?;
            } else {
                write!(
                    f,
                    "{}: {}, ",
                    key,
                    value.load(std::sync::atomic::Ordering::Relaxed)
                )?;
            }
        }
        writeln!(f, "}}")
    }
}
