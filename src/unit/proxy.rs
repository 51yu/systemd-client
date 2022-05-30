use zbus::{blocking, zvariant::OwnedObjectPath, Connection};

use crate::{Result, UnitProps};

#[zbus::dbus_proxy(
    interface = "org.freedesktop.systemd1.Unit",
    default_service = "org.freedesktop.systemd1"
)]
trait SystemdUnit {
    #[dbus_proxy(property)]
    fn id(&self) -> zbus::Result<String>;
    #[dbus_proxy(property)]
    fn description(&self) -> zbus::Result<String>;
    #[dbus_proxy(property)]
    fn load_state(&self) -> zbus::Result<String>;
    #[dbus_proxy(property)]
    fn active_state(&self) -> zbus::Result<String>;
    #[dbus_proxy(property)]
    fn sub_state(&self) -> zbus::Result<String>;
}

impl SystemdUnitProxyBlocking<'_> {
    pub fn get_properties(&self) -> zbus::Result<UnitProps> {
        let id = self.id()?;
        let description = self.description()?;
        let load_state = self.load_state()?;
        let active_state = self.active_state()?;
        let sub_state = self.sub_state()?;
        let unit_props = UnitProps::builder()
            .id(id)
            .description(description)
            .load_state(load_state)
            .active_state(active_state)
            .sub_state(sub_state)
            .build();
        Ok(unit_props)
    }
}

impl SystemdUnitProxy<'_> {
    pub async fn get_properties(&self) -> zbus::Result<UnitProps> {
        let id = self.id().await?;
        let description = self.description().await?;
        let load_state = self.load_state().await?;
        let active_state = self.active_state().await?;
        let sub_state = self.sub_state().await?;
        let unit_props = UnitProps::builder()
            .id(id)
            .description(description)
            .load_state(load_state)
            .active_state(active_state)
            .sub_state(sub_state)
            .build();
        Ok(unit_props)
    }
}

pub async fn build_nonblock_proxy(object: OwnedObjectPath) -> Result<SystemdUnitProxy<'static>> {
    let connection = Connection::system().await?;
    let proxy = SystemdUnitProxy::builder(&connection)
        .path(object)?
        .build()
        .await?;
    Ok(proxy)
}

pub fn build_blocking_proxy(object: OwnedObjectPath) -> Result<SystemdUnitProxyBlocking<'static>> {
    let connection = blocking::Connection::system()?;
    let proxy = SystemdUnitProxyBlocking::builder(&connection)
        .path(object)?
        .build()?;
    Ok(proxy)
}
