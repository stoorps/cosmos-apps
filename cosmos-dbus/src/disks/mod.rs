mod drive;
mod partition;
mod usage;
mod manager;

pub use drive::*;
pub use partition::PartitionModel;
pub use usage::*;
pub use manager::*;
// async fn get_size(path: impl Into<String> + std::fmt::Display) -> Result<String> {
//     let client = udisks2::Client::new().await?;
//     let object = client
//         .object(format!(
//             "/org/freedesktop/UDisks2/block_devices/{}",
//             path.to_string()
//         ))
//         .expect(&format!("No {} device found", path));
//     let block = object.block().await?;
//     let drive = client.drive_for_block(&block).await?;
//     Ok(client.size_for_display(drive.size().await?, true, true))
// }
