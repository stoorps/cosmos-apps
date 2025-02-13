use anyhow::Result;
use serde::Deserialize;
use zbus::
    zvariant::OwnedObjectPath
;
use zbus_macros::proxy;
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


#[proxy(
    default_service = "org.freedesktop.UDisks2",
    interface = "org.freedesktop.UDisks2.PartitionTable"
)]
pub(crate) trait UDisks2PartitionTable {
    #[zbus(property)]
    fn partitions(&self) -> zbus::Result<Vec<OwnedObjectPath>>;
}

#[proxy(
    default_service = "org.freedesktop.UDisks2",
    interface = "org.freedesktop.UDisks2.Partition"
)]
pub(crate) trait UDisks2Partition {
    #[zbus(property)]
    fn is_contained(&self) -> zbus::Result<bool>;
    #[zbus(property)]
    fn is_container(&self) -> zbus::Result<bool>;
    #[zbus(property)]
    fn table(&self) -> zbus::Result<OwnedObjectPath>;
    #[zbus(property)]
    fn name(&self) -> zbus::Result<String>;
    #[zbus(property)]
    fn type_(&self) -> zbus::Result<String>; // Note the underscore because "type" is a Rust keyword.
    #[zbus(property)]
    #[allow(non_snake_case)]
    fn UUID(&self) -> zbus::Result<String>;
    #[zbus(property)]
    fn number(&self) -> zbus::Result<u32>;
    #[zbus(property)]
    fn flags(&self) -> zbus::Result<u64>;
    #[zbus(property)]
    fn offset(&self) -> zbus::Result<u64>;
    #[zbus(property)]
    fn size(&self) -> zbus::Result<u64>;
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

    pub(crate) async fn from_proxy(partition_path: OwnedObjectPath, usage: Option<Usage>, partition_proxy: UDisks2PartitionProxy<'_>) -> Result<Self>
    {
     Ok(   Self {
            is_contained: partition_proxy.is_contained().await?,
            is_container: partition_proxy.is_container().await?,
            table_path: partition_proxy.table().await?,
            name: partition_proxy.name().await?,
            partition_type: partition_proxy.type_().await?, // Use type_()
            uuid: partition_proxy.UUID().await?,
            number: partition_proxy.number().await?,
            flags: partition_proxy.flags().await?,
            offset: partition_proxy.offset().await?,
            size: partition_proxy.size().await?,
            path: partition_path.clone(),
            device_path: format!("/dev/{}", partition_path.split("/").last().unwrap()),
            usage,
        })
    }
}
