pub mod command;
pub mod entities;
pub mod error;
pub use entities::{PackageManager, Stack, Subsystem};

// fn main() -> anyhow::Result<()>{
//     println!("Hello, world!");

//     let subsystems = Subsystem::get_all()?;
//     println!("There are {} subsystems", subsystems.len());

//     let pkgmanagers = PackageManager::get_all()?;
//     println!("There are {} pkgmanagers", pkgmanagers.len());

//     let stacks = Stack::get_all()?;
//     println!("There are {} stacks", stacks.len());

//     Ok(())

// }
