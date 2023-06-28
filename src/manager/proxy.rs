#![allow(clippy::type_complexity)]

use zbus::{blocking, zvariant::OwnedObjectPath, Connection};

use crate::{Result, UnitTuple};

#[zbus::dbus_proxy(
    interface = "org.freedesktop.systemd1.Manager",
    default_service = "org.freedesktop.systemd1",
    default_path = "/org/freedesktop/systemd1"
)]
trait SystemdManager {
    fn get_unit(&self, name: &str) -> zbus::Result<OwnedObjectPath>;
    fn list_units(&self) -> zbus::Result<Vec<UnitTuple>>;
    fn load_unit(&self, name: &str) -> zbus::Result<OwnedObjectPath>;
    fn reload_unit(&self, name: &str, mode: &str) -> zbus::Result<OwnedObjectPath>;
    fn restart_unit(&self, name: &str, mode: &str) -> zbus::Result<OwnedObjectPath>;
    fn start_unit(&self, name: &str, mode: &str) -> zbus::Result<OwnedObjectPath>;
    fn stop_unit(&self, name: &str, mode: &str) -> zbus::Result<OwnedObjectPath>;
    fn enable_unit_files(
        &self,
        files: &[&str],
        runtime: bool,
        force: bool,
    ) -> zbus::Result<(bool, Vec<(String, String, String)>)>;
    fn disable_unit_files(
        &self,
        files: &[&str],
        runtime: bool,
    ) -> zbus::Result<Vec<(String, String, String)>>;
    fn get_unit_file_state(&self, arg_1: &str) -> zbus::Result<String>;
    #[dbus_proxy(property)]
    fn architecture(&self) -> zbus::Result<String>;
    #[dbus_proxy(property)]
    fn environment(&self) -> zbus::Result<Vec<String>>;
}

pub async fn build_nonblock_proxy() -> Result<SystemdManagerProxy<'static>> {
    let connection = Connection::system().await?;
    let proxy = SystemdManagerProxy::new(&connection).await?;
    Ok(proxy)
}

pub fn build_blocking_proxy() -> Result<SystemdManagerProxyBlocking<'static>> {
    let connection = blocking::Connection::system()?;
    let proxy = SystemdManagerProxyBlocking::new(&connection)?;
    Ok(proxy)
}
