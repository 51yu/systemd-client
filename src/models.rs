use crate::{path_to_string, Result};

pub trait IntoModel<T> {
    fn into_model(self) -> Result<T>;
}

#[derive(Clone, Debug)]
pub struct Unit {
    pub name: String,
    pub description: String,
    pub load_state: String,
    pub active_state: String,
    pub sub_state: String,
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
        let load_state = self.2;
        let active_state = self.3;
        let sub_state = self.4;
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
