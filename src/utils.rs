use crate::Result;
use dbus::{blocking, nonblock};
use dbus_tokio::connection;
use std::{ops::Deref, sync::Arc, time::Duration};
use tokio::task::JoinHandle;
use tracing::error;

const DESTINATION_SYSTEMD: &str = "org.freedesktop.systemd1";
const OBJECT_PATH_SYSTEMD_MANAGER: &str = "/org/freedesktop/systemd1";
const CONNECTION_TIMEOUT_SECS: u64 = 5;

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
