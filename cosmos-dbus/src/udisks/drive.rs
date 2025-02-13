use std::{collections::HashMap, process::Command};

use anyhow::Result;
use serde::Deserialize;
use tracing::{info, warn};
use tracing_subscriber::fmt::format;
use udisks2::drive;
use zbus::{
    
    proxy::{self, SignalStream}, zvariant::{self, OwnedObjectPath}, Connection, Proxy
};
use zbus_macros::proxy;

use super::PartitionModel;

#[derive(Debug, Clone, Deserialize)]
pub struct Usage {
    pub filesystem: String,
    pub blocks: u64,
    pub used: u64,
    pub available: u64,
    pub percent: u32,
    pub mount_point: String,
}
pub fn get_usage_data() -> Result<Vec<Usage>> {
    let output = Command::new("df").arg("--block-size=1").output()?;

    let text = String::from_utf8(output.stdout)?;
    let lines: Vec<&str> = text.lines().collect();

    let mut usages = vec![];
    for ln in 1..lines.len() {
        let values: Vec<&str> = lines[ln].split_whitespace().collect();

        if values.len() == 6 {
            usages.push(Usage {
                filesystem: values[0].to_string(),
                blocks: values[1].parse()?,
                used: values[2].parse()?,
                available: values[3].parse()?,
                percent: values[4].trim_end_matches('%').parse()?,
                mount_point: values[5].to_string(),
            });
        }
    }

    Ok(usages)
}

#[derive(Debug, Clone, Deserialize)]
pub struct DriveModel {
    pub can_power_off: bool,
    pub ejectable: bool,
    pub media_available: bool,
    pub media_change_detected: bool,
    pub media_removable: bool,
    pub optical: bool,
    pub optical_blank: bool,
    pub removable: bool,
    pub id: String,
    pub model: String,
    pub revision: String,
    pub serial: String,
    pub vendor: String,
    pub size: u64,
    pub name: String,
    pub block_path: String,
    pub partitions: Vec<PartitionModel>,
    pub path: String,
}

async fn get_drive_proxy<'a>(path: &str) -> Result<drive::DriveProxy<'a>> {
    let client = udisks2::Client::new().await?;
    let path_str = path.to_owned();
    let object = client
        .object(path.clone())
        .expect(&format!("No {} device found", path_str));
    let block = object.block().await?;

    let drive = client.drive_for_block(&block).await?;
    Ok(drive)
}

// async fn get_partition<'a>() -> Result<PartitionModel> {}

impl DriveModel {
    pub fn pretty_name(&self) -> String {
        self.name.split("/").last().unwrap().replace("_", " ") //TODO: Handle unwrap
    }

    pub async fn get_drives() -> Result<Vec<DriveModel>> {
        let connection = Connection::system().await?;
        let client = udisks2::Client::new().await?;

        let p = Proxy::new(
            &connection,
            "org.freedesktop.UDisks2",
            "/org/freedesktop/UDisks2/Manager",
            "org.freedesktop.UDisks2.Manager",
        )
        .await?;

        let mut drive_paths: Vec<zvariant::OwnedObjectPath> = p
            .call(
                "GetBlockDevices",
                &std::collections::HashMap::<String, zvariant::Value>::new(),
            )
            .await?;

        let mut drives: HashMap<String, DriveModel> = HashMap::new();

        let mut usage_data = get_usage_data()?;

        //Build a list of drives with their partitions.
        for path in drive_paths {
            let short_name = path.as_str().split('/').last().unwrap().to_owned();

            let drive_proxy = match get_drive_proxy(&path).await {
                Ok(d) => d,
                Err(e) => {
                    info!("Error getting drive for {}: {}", path, e);
                    continue;
                }
            };

            let mut drive = DriveModel {
                name: drive_proxy.inner().path().to_string(),
                path: path.as_str().to_owned(),
                size: drive_proxy.size().await?,
                id: drive_proxy.id().await?,
                model: drive_proxy.model().await?,
                serial: drive_proxy.serial().await?,
                vendor: drive_proxy.vendor().await?,
                block_path: path.to_string(),
                partitions: vec![],
                can_power_off: drive_proxy.can_power_off().await?,
                ejectable: drive_proxy.ejectable().await?,
                media_available: drive_proxy.media_available().await?,
                media_change_detected: drive_proxy.media_change_detected().await?,
                media_removable: drive_proxy.media_removable().await?,
                optical: drive_proxy.optical().await?,
                optical_blank: drive_proxy.optical_blank().await?,
                removable: drive_proxy.removable().await?,
                revision: drive_proxy.revision().await?,
            };

            //Get the proxy for the partitionTable
            let partition_table_proxy = match Proxy::new(
                &connection,
                "org.freedesktop.UDisks2",
                &path,
                "org.freedesktop.UDisks2.PartitionTable",
            )
            .await
            {
                Ok(p) => p,
                Err(e) => {
                    warn!("Error getting partition table: {}", e);
                    continue;
                }
            };

            //Get the partitions, if any exist.
            let partition_table: zbus::Result<Vec<zvariant::OwnedObjectPath>> =
                partition_table_proxy.get_property("Partitions").await;

            let partition_table = match partition_table {
                Ok(p) => p,
                Err(e) => {
                    warn!("Error getting partitions for {}: {}", path, e);
                    //drives.push(drive);
                    continue;
                }
            };

            for partition_path in partition_table {
                let partition_proxy = match Proxy::new(
                    &connection,
                    "org.freedesktop.UDisks2",
                    &partition_path,
                    "org.freedesktop.UDisks2.Partition",
                )
                .await
                {
                    Ok(p) => p,
                    Err(e) => {
                        warn!("Error getting partition info: {}", e);
                        continue;
                    }
                };

                let name = partition_path.to_string();
                let short_name = name.split("/").last();

                let usage = match short_name {
                    Some(sn) => match usage_data.iter_mut().find(|u| u.filesystem.ends_with(sn)) {
                        Some(u) => Some(u.clone()),
                        None => None,
                    },
                    None => None,
                };

                println!("usage {:?}", &usage);

                drive.partitions.push(PartitionModel {
                    is_contained: partition_proxy.get_property("IsContained").await?,
                    is_container: partition_proxy.get_property("IsContainer").await?,
                    table_path: partition_proxy.get_property("Table").await?,
                    name: partition_proxy.get_property("Name").await?,
                    partition_type: partition_proxy.get_property("Type").await?,
                    uuid: partition_proxy.get_property("UUID").await?,
                    number: partition_proxy.get_property("Number").await?,
                    flags: partition_proxy.get_property("Flags").await?,
                    offset: partition_proxy.get_property("Offset").await?,
                    size: partition_proxy.get_property("Size").await?,
                    path: partition_path.clone(),
                    device_path: format!("/dev/{}", partition_path.split("/").last().unwrap()), //TODO: HANDLE UNWRAP
                    usage,
                });
                // proxy.get_property(property_name)
            }

            match drives.get_mut(&drive.name) {
                Some(d) => {
                    warn!("Appending");
                    d.partitions.append(&mut drive.partitions);
                }
                None => {
                    drives.insert(drive.name.clone(), drive);
                }
            }
        }

        Ok(drives.into_values().collect())
    }

}

// #[proxy(
//     default_service = "org.freedesktop.UDisks2",
//     default_path = "/org/freedesktop/UDisks2/Manager",
//     interface = "org.freedesktop.UDisks2.Manager"
// )]
// trait UDisks2Manager {
//     #[zbus(signal)]
//     fn device_added(&self, device: OwnedObjectPath) -> zbus::Result<()>;

//     #[zbus(signal)]
//     fn device_removed(&self, device: OwnedObjectPath) -> zbus::Result<()>;
// }

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn Error>> {
//     let connection = Connection::system().await?;
//     let udisks_proxy = UDisks2ManagerProxy::new(&connection).await?;

//     let mut device_added_stream = udisks_proxy.receive_device_added().await?;
//     let mut device_removed_stream = udisks_proxy.receive_device_removed().await?;

//     println!("Monitoring UDisks2 for device changes...");

//     loop {  // Use loop for concurrent stream handling
//         tokio::select! {
//             Some(msg) = device_added_stream.next() => {
//                 let args: DeviceAddedArgs = msg.args()?;
//                 println!("Device Added: {}", args.device);
//             }
//             Some(msg) = device_removed_stream.next() => {
//                 let args: DeviceRemovedArgs = msg.args()?;
//                 println!("Device Removed: {}", args.device);
//             }
//         }
//     }
// }