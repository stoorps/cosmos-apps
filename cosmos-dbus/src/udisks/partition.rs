use std::collections::HashMap;

use anyhow::Result;
use serde::Deserialize;
use tracing::{info, warn};
use udisks2::drive;
use zbus::{
    zvariant::{self, OwnedObjectPath},
    Connection, Proxy,
};

use super::Usage;

#[derive(Debug, Clone, Deserialize)]
pub struct PartitionModel {
    pub is_contained: bool,
    pub is_container: bool,
    pub table_path: OwnedObjectPath,
    pub name: String,
    pub partition_type: String,
    pub uuid: String,
    pub number: u32,
    pub flags: u64,
    pub offset: u64,
    pub size: u64,
    pub path: OwnedObjectPath,
    pub device_path: String,
    pub usage: Option<Usage>,
}

impl PartitionModel {
    pub fn pretty_name(&self) -> String {
        let mut name = self.name.clone();
        if name.len() == 0 {
            name = format!("Partition {}", &self.number);
        } else {
            name = format!("Partition {}: {}", &self.number, name);
        }

        name
    }
}
