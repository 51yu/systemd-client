use crate::{path_to_string, Result};
use dbus::arg;

// https://www.freedesktop.org/software/systemd/man/systemctl.html#Unit%20Commands%20(Introspection%20and%20Modification)
#[derive(Clone, Debug, PartialEq)]
pub enum UnitLoadStateType {
    Loaded,
    NotFound,
    BadSetting,
    Error,
    Masked,
    Other(String),
}

impl From<String> for UnitLoadStateType {
    fn from(origin: String) -> Self {
        match origin.as_str() {
            "loaded" => UnitLoadStateType::Loaded,
            "not-found" => UnitLoadStateType::NotFound,
            "bad-setting" => UnitLoadStateType::BadSetting,
            "error" => UnitLoadStateType::Error,
            "masked" => UnitLoadStateType::Masked,
            _ => UnitLoadStateType::Other(origin),
        }
    }
}

impl ToString for UnitLoadStateType {
    fn to_string(&self) -> String {
        match self {
            UnitLoadStateType::Loaded => String::from("loaded"),
            UnitLoadStateType::NotFound => String::from("not-found"),
            UnitLoadStateType::BadSetting => String::from("bad-setting"),
            UnitLoadStateType::Error => String::from("error"),
            UnitLoadStateType::Masked => String::from("masked"),
            UnitLoadStateType::Other(other) => other.to_owned(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum UnitActiveStateType {
    Active,
    Reloading,
    Inactive,
    Failed,
    Activating,
    Deactivating,
    Other(String),
}

impl From<String> for UnitActiveStateType {
    fn from(origin: String) -> Self {
        match origin.as_str() {
            "active" => UnitActiveStateType::Active,
            "activating" => UnitActiveStateType::Activating,
            "deactivating" => UnitActiveStateType::Deactivating,
            "failed" => UnitActiveStateType::Failed,
            "inactive" => UnitActiveStateType::Inactive,
            "reloading" => UnitActiveStateType::Reloading,
            _ => UnitActiveStateType::Other(origin),
        }
    }
}

impl ToString for UnitActiveStateType {
    fn to_string(&self) -> String {
        match self {
            UnitActiveStateType::Active => String::from("active"),
            UnitActiveStateType::Activating => String::from("activating"),
            UnitActiveStateType::Deactivating => String::from("deactivating"),
            UnitActiveStateType::Failed => String::from("failed"),
            UnitActiveStateType::Inactive => String::from("inactive"),
            UnitActiveStateType::Reloading => String::from("reloading"),
            UnitActiveStateType::Other(other) => other.to_owned(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum UnitSubStateType {
    Active,
    Dead,
    Exited,
    Failed,
    Mounted,
    Plugged,
    Listening,
    Running,
    Waiting,
    Other(String),
}

impl From<String> for UnitSubStateType {
    fn from(origin: String) -> Self {
        match origin.as_str() {
            "active" => UnitSubStateType::Active,
            "dead" => UnitSubStateType::Dead,
            "exited" => UnitSubStateType::Exited,
            "failed" => UnitSubStateType::Failed,
            "mounted" => UnitSubStateType::Mounted,
            "plugged" => UnitSubStateType::Plugged,
            "listening" => UnitSubStateType::Listening,
            "running" => UnitSubStateType::Running,
            "waiting" => UnitSubStateType::Waiting,
            _ => UnitSubStateType::Other(origin),
        }
    }
}

impl ToString for UnitSubStateType {
    fn to_string(&self) -> String {
        match self {
            UnitSubStateType::Active => String::from("active"),
            UnitSubStateType::Dead => String::from("dead"),
            UnitSubStateType::Exited => String::from("exited"),
            UnitSubStateType::Failed => String::from("failed"),
            UnitSubStateType::Mounted => String::from("mounted"),
            UnitSubStateType::Plugged => String::from("plugged"),
            UnitSubStateType::Listening => String::from("listening"),
            UnitSubStateType::Running => String::from("running"),
            UnitSubStateType::Waiting => String::from("waiting"),
            UnitSubStateType::Other(other) => other.to_owned(),
        }
    }
}

pub trait IntoModel<T> {
    fn into_model(self) -> Result<T>;
}

#[derive(Clone, Debug)]
pub struct Unit {
    pub name: String,
    pub description: String,
    pub load_state: UnitLoadStateType,
    pub active_state: UnitActiveStateType,
    pub sub_state: UnitSubStateType,
    pub follow_unit: Option<String>,
    pub object_path: String,
    pub job_id: u32,
    pub job_ty: String,
    pub job_object_path: String,
}

impl<'a> IntoModel<Unit>
    for (
        String,
        String,
        String,
        String,
        String,
        String,
        dbus::Path<'a>,
        u32,
        String,
        dbus::Path<'a>,
    )
{
    fn into_model(self) -> Result<Unit> {
        let name = self.0;
        let description = self.1;
        let load_state: UnitLoadStateType = self.2.into();
        let active_state: UnitActiveStateType = self.3.into();
        let sub_state: UnitSubStateType = self.4.into();
        let follow_unit = match self.5.is_empty() {
            true => None,
            false => Some(self.5),
        };
        let object_path = path_to_string(self.6)?;
        let job_id = self.7;
        let job_ty = self.8;
        let job_object_path = path_to_string(self.9)?;
        Ok(Unit {
            name,
            description,
            load_state,
            active_state,
            sub_state,
            follow_unit,
            object_path,
            job_id,
            job_ty,
            job_object_path,
        })
    }
}

#[derive(Clone, Debug)]
pub struct Job {
    pub id: u32,
    pub unit_name: String,
    pub ty: String,
    pub state: String,
    pub path: String,
    pub unit_path: String,
}

impl<'a> IntoModel<Job> for (u32, String, String, String, dbus::Path<'a>, dbus::Path<'a>) {
    fn into_model(self) -> Result<Job> {
        let id = self.0;
        let unit_name = self.1;
        let ty = self.2;
        let state = self.3;
        let path = path_to_string(self.4)?;
        let unit_path = path_to_string(self.5)?;
        Ok(Job {
            id,
            unit_name,
            ty,
            state,
            path,
            unit_path,
        })
    }
}

#[derive(Clone, Debug)]
pub struct UnitProps {
    pub id: String,
    pub description: String,
    pub load_state: UnitLoadStateType,
    pub active_state: UnitActiveStateType,
    pub sub_state: UnitSubStateType,
}

impl IntoModel<UnitProps> for arg::PropMap {
    fn into_model(self) -> Result<UnitProps> {
        let id = arg::prop_cast::<String>(&self, "Id")
            .expect("unit id undefined")
            .to_owned();
        let description = match arg::prop_cast::<String>(&self, "Description") {
            Some(description) => description.to_owned(),
            None => String::new(),
        };
        let load_state = arg::prop_cast::<String>(&self, "LoadState")
            .expect("load state undefined")
            .to_owned();
        let active_state = arg::prop_cast::<String>(&self, "ActiveState")
            .expect("active state undefined")
            .to_owned();
        let sub_state = arg::prop_cast::<String>(&self, "SubState")
            .expect("sub state undefined")
            .to_owned();
        Ok(UnitProps {
            id,
            description,
            load_state: load_state.into(),
            active_state: active_state.into(),
            sub_state: sub_state.into(),
        })
    }
}
