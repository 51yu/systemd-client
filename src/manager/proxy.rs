use crate::{Result, UnitTuple};

#[zbus::dbus_proxy(
    interface = "org.freedesktop.systemd1.Manager",
    default_service = "org.freedesktop.systemd1",
    default_path = "/org/freedesktop/systemd1"
)]
trait SystemdManager {
    fn get_unit(&self, name: &str) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;
    fn list_units(&self) -> zbus::Result<Vec<UnitTuple>>;
    fn load_unit(&self, name: &str) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;
    fn reload_unit(&self, name: &str, mode: &str) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;
    fn restart_unit(&self, name: &str, mode: &str) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;
    fn start_unit(&self, name: &str, mode: &str) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;
    fn stop_unit(&self, name: &str, mode: &str) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;
    #[dbus_proxy(property)]
    fn architecture(&self) -> zbus::Result<String>;
    #[dbus_proxy(property)]
    fn environment(&self) -> zbus::Result<Vec<String>>;
}

pub async fn build_nonblock_proxy() -> Result<SystemdManagerProxy<'static>> {
    let connection = zbus::Connection::system().await?;
    let proxy = SystemdManagerProxy::new(&connection).await?;
    Ok(proxy)
}

pub fn build_blocking_proxy() -> Result<SystemdManagerProxyBlocking<'static>> {
    let connection = zbus::blocking::Connection::system()?;
    let proxy = SystemdManagerProxyBlocking::new(&connection)?;
    Ok(proxy)
}
