use crate::INTERFACE_SYSTEMD_UNIT;
#[allow(unused_imports)]
use dbus::arg;
use dbus::nonblock;
use dbus::{self as dbus, nonblock::stdintf::org_freedesktop_dbus::Properties};

pub trait UnitProperties {
    fn get_unit_properties(&self) -> nonblock::MethodReply<arg::PropMap>;
}

impl<'a, T: nonblock::NonblockReply, C: ::std::ops::Deref<Target = T>> UnitProperties
    for nonblock::Proxy<'a, C>
{
    fn get_unit_properties(&self) -> nonblock::MethodReply<arg::PropMap> {
        self.get_all(INTERFACE_SYSTEMD_UNIT)
    }
}
