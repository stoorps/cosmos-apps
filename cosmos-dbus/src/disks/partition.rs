use std::{collections::HashMap, path::Path};
use enumflags2::{bitflags, BitFlags};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use zbus::{
    zvariant::{OwnedObjectPath, Type}, Connection}
;
use zbus_macros::proxy;
use super::{filesystem::FilesystemProxy, DiskError, Usage};


#[derive(Debug, Clone)]
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
    pub device_path: Option<String>,
    pub usage: Option<Usage>,
    connection: Option<Connection>,

}


#[proxy(
    default_service = "org.freedesktop.UDisks2",
    interface = "org.freedesktop.UDisks2.PartitionTable"
)]
pub(crate) trait PartitionTable {
    #[zbus(property)]
    fn partitions(&self) -> zbus::Result<Vec<OwnedObjectPath>>;

    //CreatePartition (UInt64 offset, UInt64 size, String type, String name, Dict of {String, Variant} options) \u21a6 (Object Path created_partition)
    //CreatePartitionAndFormat (UInt64 offset, UInt64 size, String type, String name, Dict of {String, Variant} options, String format_type, Dict of {String, Variant} format_options) ↦ (Object Path created_partition)
}

/// Flags describing the partition.
#[bitflags]
#[repr(u64)]
#[derive(Type, Debug, PartialEq, Eq, Copy, Clone, Serialize, Deserialize)]
pub enum PartitionFlags {
    /// The partition is marked as a system partition.
    ///
    /// Known flag for `gpt` partitions.
    SystemPartition = 1 << 0,
    /// The partition is marked as a Legacy BIOS Bootable partition.
    ///
    /// Known flag for `gpt` partitions.
    LegacyBIOSBootable = 1 << 2,
    /// The partition is marked as bootable.
    ///
    /// Known flag for `dos` partitions.
    Bootable = 0x80,
    /// The partition is marked as read-only.
    ///
    /// Known flag for `gpt` partitions.
    ReadOnly = 1 << 60,
    /// The partition is marked as hidden.
    ///
    /// Known flag for `gpt` partitions.
    Hidden = 1 << 62,
    /// The partition is marked as Do not automount.
    ///
    /// Known flag for `gpt` partitions.
    NoAutoMount = 1 << 63,
}


#[proxy(
    default_service = "org.freedesktop.UDisks2",
    interface = "org.freedesktop.UDisks2.Partition"
)]
pub(crate) trait Partition {
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

   /// Deletes the partition.
    ///
    /// If the option `tear-down` is set to `true`, then the block device and all its children will be cleaned up before formatting.
    /// This cleanup consists of removing entries from `/etc/fstab` and `/etc/crypttab`, and locking of encrypted block devices.
    /// Entries in `/etc/fstab` and `/etc/crypttab` that have been created with the 'track-parents' options to AddConfigurationItem
    /// will be removed even if their block device is currently unavailable.
    fn delete(
        &self,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<()>;

    /// Resizes the partition.
    ///
    /// The partition will not change its position but might be slightly
    /// bigger than requested due to sector counts and alignment (e.g. 1MiB).
    /// If the requested size can't be allocated it results in an error.
    /// The maximal size can automatically be set by using 0 as size.
    fn resize(
        &self,
        size: u64,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<()>;

    /// Set the `flags` property.
    ///
    /// See [`PartitionFlags`] for more information.
    fn set_flags(
        &self,
        flags: BitFlags<PartitionFlags>,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<()>;

    /// Sets the partition name (label).
    fn set_name(
        &self,
        name: &str,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<()>;

    /// Sets the partition type. See the "Type" property for a description of known partition types.
    fn set_type(
        &self,
        type_: &str,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<()>;

    /// Sets the partition UUID (GPT only).
    #[zbus(name = "SetUUID")]
    fn set_uuid(
        &self,
        uuid: &str,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<()>;



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

    pub(crate) async fn from_proxy(partition_path: OwnedObjectPath, usage: Option<Usage>, partition_proxy: PartitionProxy<'_>) -> Result<Self>
    {
        let proposed = &format!("/dev/{}", partition_path.split("/").last().unwrap());

        let device_path = match Path::new(proposed).exists()
        {
            true => Some(proposed.to_owned()),

            //TODO: Figure out how to SOLIDLY resolve a device's "natural" path.
            false => None,
        };

     Ok(Self {
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
            device_path: device_path,
            usage,
            connection: Some(Connection::system().await?),
        })
    }


    pub async fn connect(&mut self) -> Result<()>
    {
        if self.connection.is_none()
        {
            self.connection = Some(Connection::system().await?);
        }

        Ok(())
    }


    pub async fn mount(&self) -> Result<()>
    {
        if self.connection.is_none()
        {
            return Err(DiskError::NotConnected(self.name.clone()).into())
        }

        let proxy = FilesystemProxy::new(&self.connection.as_ref().unwrap(), &self.path).await?;

        proxy.mount(HashMap::new()).await?;

        Ok(())
    }

    pub async fn unmount(&self) -> Result<()>
    {
        if self.connection.is_none()
        {
            return Err(DiskError::NotConnected(self.name.clone()).into())
        }

        let proxy = FilesystemProxy::new(&self.connection.as_ref().unwrap(), &self.path).await?;

        proxy.unmount(HashMap::new()).await?;

        Ok(())
    }


    pub async fn delete(&self) -> Result<()>
    {
        if self.connection.is_none()
        {
            return Err(DiskError::NotConnected(self.name.clone()).into())
        }

        //try to unmount first. If it fails, it's likely because it's already unmounted.
        //any other error with the partition should be caught by the delete operation.
        let _ = self.unmount().await;


        let proxy = PartitionProxy::new(&self.connection.as_ref().unwrap(), &self.path).await?;

        proxy.delete(HashMap::new()).await?;



        Ok(())
    }


    pub async fn format(&self, name: String, erase: bool, partion_type: String) -> Result<()>
    {
        if self.connection.is_none()
        {
            return Err(DiskError::NotConnected(self.name.clone()).into())
        }

        Ok(())
    }


    //TODO: implement
    pub async fn edit_partition(&self, partition_type: String, name: String, flags: u64) -> Result<()>
    {
        if self.connection.is_none()
        {
            return Err(DiskError::NotConnected(self.name.clone()).into())
        }

        Ok(())
    }



    //TODO: implement
    pub async fn edit_filesystem_label(&self, label: String) -> Result<()>
    {
        if self.connection.is_none()
        {
            return Err(DiskError::NotConnected(self.name.clone()).into())
        }
        Ok(())
    }


    //TODO: implement
    pub async fn change_passphrase(&self) ->Result<()>
    {
        if self.connection.is_none()
        {
            return Err(DiskError::NotConnected(self.name.clone()).into())
        }
        Ok(())
    }


    //TODO: implement
    pub async fn resize(&self, new_size_bytes: u64) ->Result<()>
    {
        if self.connection.is_none()
        {
            return Err(DiskError::NotConnected(self.name.clone()).into())
        }
        Ok(())
    }


    //TODO: implement
    pub async fn check_filesystem(&self) ->Result<()>
    {
        if self.connection.is_none()
        {
            return Err(DiskError::NotConnected(self.name.clone()).into())
        }
        Ok(())
    }


    //TODO: implement
    pub async fn repair_filesystem(&self) ->Result<()>
    {
        if self.connection.is_none()
        {
            return Err(DiskError::NotConnected(self.name.clone()).into())
        }
        Ok(())
    }


    //TODO: implement
    pub async fn take_ownership(&self, recursive: bool) ->Result<()>
    {
        if self.connection.is_none()
        {
            return Err(DiskError::NotConnected(self.name.clone()).into())
        }
        Ok(())
    }


    //TODO: implement. See how edit mount options -> User session defaults works in gnome-disks.
    pub async fn default_mount_options(&self) -> Result<()>
    {
        if self.connection.is_none()
        {
            return Err(DiskError::NotConnected(self.name.clone()).into())
        }
        Ok(())
    }


    //TODO: implement. Look at gnome-disks -> partition -> edit mount options. Likely make all params optional.
    pub async fn edit_mount_options(&self, mount_at_startup: bool, show_in_ui: bool, requre_auth: bool,display_name: Option<String>,
                                    icon_name: Option<String>, symbolic_icon_name: Option<String>,  options: String,
                                    mount_point: String, identify_as: String, file_system_type: String) ->Result<()>
    {
        if self.connection.is_none()
        {
            return Err(DiskError::NotConnected(self.name.clone()).into())
        }
        Ok(())
    }


    //TODO: implement
    pub async fn edit_encrytion_options(&self) ->Result<()>
    {
        if self.connection.is_none()
        {
            return Err(DiskError::NotConnected(self.name.clone()).into())
        }
        Ok(())
    }


    //TODO: implement. creates a *.img of self. 
    pub async fn create_image(&self, output_path: String) ->Result<()>
    {
        if self.connection.is_none()
        {
            return Err(DiskError::NotConnected(self.name.clone()).into())
        }
        Ok(())
    }




}
