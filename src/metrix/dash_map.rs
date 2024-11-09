use std::{fmt::Display, sync::Arc};

use anyhow::Result;
use dashmap::DashMap;

#[derive(Debug, Clone, Default)]
pub struct DashMapMetrix {
    data: Arc<DashMap<String, i64>>,
}

impl DashMapMetrix {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn inc(&self, key: &str) -> Result<()> {
        let mut count = self.data.entry(key.to_string()).or_insert(0);
        *count += 1;
        Ok(())
    }

    pub fn dec(&self, key: &str) -> Result<()> {
        let mut count = self.data.entry(key.to_string()).or_insert(0);
        *count -= 1;
        Ok(())
    }
}

impl Display for DashMapMetrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{")?;
        for (index, val) in self.data.iter().enumerate() {
            let (key, value) = val.pair();
            if self.data.len() - 1 == index {
                write!(f, "{}: {}", key, value)?;
            } else {
                write!(f, "{}: {}, ", key, value)?;
            }
        }
        write!(f, "}}")
    }
}
