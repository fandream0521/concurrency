use anyhow::Result;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

#[derive(Debug, Clone, Default)]
pub struct Metrix {
    data: Arc<Mutex<HashMap<String, i64>>>,
}

impl Metrix {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn inc(&self, key: &str) -> Result<()> {
        let mut lock = self
            .data
            .lock()
            .map_err(|e| anyhow::anyhow!("Error to require lock: {:?}", e))?;
        let count = lock.entry(key.to_string()).or_insert(0);
        *count += 1;
        Ok(())
    }

    pub fn dec(&self, key: &str) -> Result<()> {
        let mut lock = self
            .data
            .lock()
            .map_err(|e| anyhow::anyhow!("Error to require lock: {:?}", e))?;
        let count = lock.entry(key.to_string()).or_insert(0);
        *count -= 1;
        Ok(())
    }

    pub fn snapshot(&self) -> Result<HashMap<String, i64>> {
        let lock = self
            .data
            .lock()
            .map_err(|e| anyhow::anyhow!("Error to require lock: {:?}", e));
        lock.map(|data| data.clone())
    }
}
