pub mod udisks;

// use anyhow::Result;
// use udisks::DriveModel;

// #[tokio::main]
// async fn main() -> Result<()> {
//     tracing_subscriber::fmt().init();

//     let drives = DriveModel::get_drives().await?;
//     for drive in drives {
//         println!("Drive: {:?} ({:?})", drive.name, drive.block_path);
//         for partition in drive.partitions {
//             println!(" - {}", partition.path);
//         }
//     }

//     Ok(())
// }
