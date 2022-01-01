use crate::INTERFACE_SYSTEMD_UNIT;
#[allow(unused_imports)]
use dbus::arg;
use dbus::blocking;
use dbus::{self as dbus, blocking::stdintf::org_freedesktop_dbus::Properties};

pub trait UnitProperties {
    fn get_unit_properties(&self) -> Result<arg::PropMap, dbus::Error>;
}

impl<'a, T: blocking::BlockingSender, C: ::std::ops::Deref<Target = T>> UnitProperties
    for blocking::Proxy<'a, C>
{
    fn get_unit_properties(&self) -> Result<arg::PropMap, dbus::Error> {
        self.get_all(INTERFACE_SYSTEMD_UNIT)
    }
}
