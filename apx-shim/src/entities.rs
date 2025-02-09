use anyhow::Result;
use serde::{Deserialize, Serialize};
use tracing::debug;

use crate::{command::run_apx, error::ApxError};

#[derive(Serialize, Deserialize)]
pub struct Stack {
    #[serde(alias = "Name")]
    pub name: String,
    #[serde(alias = "Base")]
    pub base: String,
    #[serde(alias = "Packages")]
    pub packages: Vec<String>,
    #[serde(alias = "PkgManager")]
    pub package_manager: String,
    #[serde(alias = "BuiltIn")]
    pub built_in: bool,
}

impl Stack {
    pub fn get_all() -> Result<Vec<Stack>> {
        let json = run_apx("stacks list --json", false)?;

        match serde_json::from_str(&json) {
            Ok(v) => Ok(v),
            Err(e) => Err(e.into()),
        }
    }

    pub fn create(&mut self) -> Result<()> {
        run_apx(
            &format!(
                "stacks new --name {} --base {} --packages {} --pkg-manager {}",
                self.name,
                self.base,
                self.packages.join(" "),
                self.package_manager
            ),
            false,
        )?;

        let all = Self::get_all()?;

        let matched_entries: Vec<&Stack> = all.iter().filter(|e| e.name == self.name).collect();

        //Make sure we have added correctly, pulling any updated info.
        match matched_entries.first() {
            Some(e) => {
                self.base = e.base.clone();
                self.package_manager = e.package_manager.clone();
                self.packages = e.packages.clone();
                Ok(())
            }
            None => Err(ApxError::CommandError {
                error: "Failed to create Stack".into(),
            }
            .into()),
        }
    }

    pub fn update(&self) -> Result<()> {
        let res = run_apx(
            &format!(
                "stacks update --name {} --base {} --packages {} --pkg-manager {}",
                self.name,
                self.base,
                self.packages.join(" "),
                self.package_manager
            ),
            false,
        );

        match res {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub fn remove(&self, force: bool) -> Result<()> {
        let mut command = format!("stacks rm --name {}", self.name);

        if force {
            command.push_str(" --force");
        }

        let res = run_apx(&command, false);

        match res {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Subsystem {
    #[serde(alias = "InternalName")]
    pub internal_name: String,
    #[serde(alias = "Name")]
    pub name: String,
    #[serde(alias = "Stack")]
    pub stack: Stack,
    #[serde(alias = "Home")]
    pub home: String,
    #[serde(alias = "Status")]
    pub status: String,
}

impl Subsystem {
    pub fn get_all() -> Result<Vec<Subsystem>> {
        let json = run_apx("subsystems list --json", false)?;

        match serde_json::from_str(&json) {
            Ok(v) => Ok(v),
            Err(e) => Err(e.into()),
        }
    }

    pub fn create(&mut self) -> Result<()> {
        let mut command = format!(
            "subsystems new --name {} --stack {}",
            self.name, self.stack.name
        );

        if self.home.len() > 0 {
            command.push_str(&format!("--home {}", self.home));
        }

        let res = run_apx(&command, false);

        match res {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub fn update(&self) -> Result<()> {
        let res = run_apx(
            &format!(
                "subsystems --name {} --stack {}",
                self.name, self.stack.name
            ),
            false,
        );

        match res {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub fn remove(&self, force: bool) -> Result<()> {
        let mut command = format!("subsystems rm --name {}", self.name);

        if force {
            command.push_str(" --force");
        }

        let res = run_apx(&command, false);

        match res {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub fn running(&self) -> bool {
        self.status.contains("Up") || self.status.contains("running")
    }

    pub fn start(&self) -> Result<()> {
        let res = run_apx(&format!("{} start", self.name), false);

        match res {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub fn stop(&self) -> Result<()> {
        let res = run_apx(&format!("{} stop", self.name), false);

        match res {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub fn reset(&self, force: bool) -> Result<()> {
        let mut command = format!("subsystems reset --name {}", self.name);

        if force {
            command.push_str(" --force");
        }

        let res = run_apx(&command, false);

        match res {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub fn autoremove(&self) -> Result<()> {
        let res = run_apx(&format!("{} autoremove", self.name), false);

        match res {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub fn clean(&self) -> Result<()> {
        let res = run_apx(&format!("{} clean", self.name), false);

        match res {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct PackageManager {
    #[serde(alias = "Name")]
    pub name: String,
    #[serde(alias = "NeedSudo")]
    pub need_sudo: bool,
    #[serde(alias = "BuiltIn")]
    pub built_in: bool,
    #[serde(alias = "CmdAutoRemove")]
    pub cmd_auto_remove: String,
    #[serde(alias = "CmdClean")]
    pub cmd_clean: String,
    #[serde(alias = "CmdInstall")]
    pub cmd_install: String,
    #[serde(alias = "CmdList")]
    pub cmd_list: String,
    #[serde(alias = "CmdPurge")]
    pub cmd_purge: String,
    #[serde(alias = "CmdRemove")]
    pub cmd_remove: String,
    #[serde(alias = "CmdSearch")]
    pub cmd_search: String,
    #[serde(alias = "CmdShow")]
    pub cmd_show: String,
    #[serde(alias = "CmdUpdate")]
    pub cmd_update: String,
    #[serde(alias = "CmdUpgrade")]
    pub cmd_upgrade: String,
}

impl PackageManager {
    pub fn get_all() -> Result<Vec<PackageManager>> {
        let json = run_apx("pkgmanagers list --json", false)?;

        match serde_json::from_str(&json) {
            Ok(v) => Ok(v),
            Err(e) => Err(e.into()),
        }
    }

    pub fn create(&mut self) -> Result<()> {
        let command = format!(
            "pkgmanagers new --name '{}' --need-sudo '{}' 
             --autoremove '{}' --clean '{}' --install '{}' --list '{}' --purge '{}' --remove '{}'
             --search '{}' --show '{}' --update '{}' --upgrade '{}'",
            self.name,
            self.need_sudo,
            self.cmd_auto_remove,
            self.cmd_clean,
            self.cmd_install,
            self.cmd_list,
            self.cmd_purge,
            self.cmd_remove,
            self.cmd_search,
            self.cmd_show,
            self.cmd_update,
            self.cmd_upgrade
        );

        run_apx(&command, false)?;

        let all = Self::get_all()?;

        let matched_entries: Vec<&PackageManager> =
            all.iter().filter(|e| e.name == self.name).collect();

        //Make sure we have added correctly, pulling any updated info.
        match matched_entries.first() {
            Some(e) => {
                self.need_sudo = e.need_sudo.clone();
                self.cmd_auto_remove = e.cmd_auto_remove.clone();
                self.cmd_clean = e.cmd_clean.clone();
                self.cmd_install = e.cmd_install.clone();
                self.cmd_list = e.cmd_list.clone();
                self.cmd_purge = e.cmd_purge.clone();
                self.cmd_remove = e.cmd_remove.clone();
                self.cmd_search = e.cmd_search.clone();
                self.cmd_show = e.cmd_show.clone();
                self.cmd_update = e.cmd_update.clone();
                self.cmd_upgrade = e.cmd_upgrade.clone();
                Ok(())
            }
            None => Err(ApxError::CommandError {
                error: "Failed to create Package Manager".into(),
            }
            .into()),
        }
    }

    pub fn update(&self) -> Result<()> {
        let command = format!("pkgmanagers update --name '{}' --need-sudo '{}' --autoremove '{}' --clean '{}' --install '{}' --list '{}' --purge '{}' --remove '{}' --search '{}' --show '{}' --update '{}' --upgrade '{}'",
        self.name,
        self.need_sudo,
        self.cmd_auto_remove,
        self.cmd_clean,
        self.cmd_install,
        self.cmd_list,
        self.cmd_purge,
        self.cmd_remove,
        self.cmd_search,
        self.cmd_show,
        self.cmd_update,
        self.cmd_upgrade
    );

        debug!("command: {}", command);

        let res = run_apx(&command, false);

        match res {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub fn remove(&self, force: bool) -> Result<()> {
        let mut command = format!("pkgmanagers rm --name '{}'", self.name);

        debug!("command: {command}");

        if force {
            command.push_str(" --force");
        }

        let res = run_apx(&command, false);

        match res {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
}
