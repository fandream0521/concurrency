use anyhow::Result;
use std::{
    collections::HashMap,
    fmt::Display,
    sync::{Arc, RwLock},
};

#[derive(Debug, Clone, Default)]
pub struct Metrix {
    data: Arc<RwLock<HashMap<String, i64>>>,
}

impl Metrix {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn inc(&self, key: &str) -> Result<()> {
        let mut lock = self
            .data
            .write()
            .map_err(|e| anyhow::anyhow!("Error to require lock: {:?}", e))?;
        let count = lock.entry(key.to_string()).or_insert(0);
        *count += 1;
        Ok(())
    }

    pub fn dec(&self, key: &str) -> Result<()> {
        let mut lock = self
            .data
            .write()
            .map_err(|e| anyhow::anyhow!("Error to require lock: {:?}", e))?;
        let count = lock.entry(key.to_string()).or_insert(0);
        *count -= 1;
        Ok(())
    }

    pub fn snapshot(&self) -> Result<HashMap<String, i64>> {
        let lock = self
            .data
            .read()
            .map_err(|e| anyhow::anyhow!("Error to require lock: {:?}", e));
        lock.map(|data| data.clone())
    }
}

impl Display for Metrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let data = self.data.read().map_err(|_| std::fmt::Error)?;
        write!(f, "{{")?;
        for (index, (key, value)) in data.iter().enumerate() {
            if data.len() - 1 == index {
                write!(f, "{}: {}", key, value)?;
            } else {
                write!(f, "{}: {}, ", key, value)?;
            }
        }
        writeln!(f, "}}")
    }
}
