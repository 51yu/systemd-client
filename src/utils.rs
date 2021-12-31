use crate::Result;
use dbus::{blocking, nonblock};
use dbus_tokio::connection;
use std::{io::Write, ops::Deref, sync::Arc, time::Duration};
use tokio::task::JoinHandle;
use tracing::error;

const DESTINATION_SYSTEMD: &str = "org.freedesktop.systemd1";
const OBJECT_PATH_SYSTEMD_MANAGER: &str = "/org/freedesktop/systemd1";
const CONNECTION_TIMEOUT_SECS: u64 = 5;
const SYSTEMD_UNIT_CONFIGURATION_DIRECTORY: &str = "/etc/systemd/system";

pub enum SystemdObjectType {
    Manager,
}

pub struct DerefContainer<T> {
    target: T,
}

impl<T> Deref for DerefContainer<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.target
    }
}

impl<T> DerefContainer<T> {
    pub fn new(target: T) -> Self {
        DerefContainer { target }
    }
}

pub fn build_blocking_client(
    object_ty: SystemdObjectType,
) -> Result<blocking::Proxy<'static, DerefContainer<blocking::Connection>>> {
    let object_path = match object_ty {
        SystemdObjectType::Manager => OBJECT_PATH_SYSTEMD_MANAGER,
    };
    let conn = blocking::Connection::new_system()?;
    let proxy = blocking::Proxy::new(
        DESTINATION_SYSTEMD,
        object_path,
        Duration::from_secs(CONNECTION_TIMEOUT_SECS),
        DerefContainer::new(conn),
    );
    Ok(proxy)
}

pub fn build_nonblock_client(
    object_ty: SystemdObjectType,
) -> Result<(
    nonblock::Proxy<'static, Arc<nonblock::SyncConnection>>,
    JoinHandle<()>,
)> {
    let object_path = match object_ty {
        SystemdObjectType::Manager => OBJECT_PATH_SYSTEMD_MANAGER,
    };
    let (resource, conn) = connection::new_system_sync()?;
    let jh = tokio::spawn(async {
        let err = resource.await;
        error!("dbus connection lost, details: '{:?}'", err);
    });
    let proxy = nonblock::Proxy::new(
        DESTINATION_SYSTEMD,
        object_path,
        Duration::from_secs(CONNECTION_TIMEOUT_SECS),
        conn,
    );
    Ok((proxy, jh))
}

pub fn path_to_string(path: dbus::Path) -> Result<String> {
    let path = path.into_cstring().into_string()?;
    Ok(path)
}

pub fn create_unit_configuration_file(unit_name: &str, buffer: &[u8]) -> Result<()> {
    let mut path = std::path::PathBuf::from(SYSTEMD_UNIT_CONFIGURATION_DIRECTORY);
    path.push(unit_name);
    let file = std::fs::File::create(path.as_path())?;
    let mut writer = std::io::BufWriter::new(file);
    writer.write_all(buffer)?;
    writer.flush()?;
    Ok(())
}

pub fn delete_unit_configuration_file(unit_name: &str) -> Result<()> {
    let mut path = std::path::PathBuf::from(SYSTEMD_UNIT_CONFIGURATION_DIRECTORY);
    path.push(unit_name);
    std::fs::remove_file(path.as_path())?;
    Ok(())
}
