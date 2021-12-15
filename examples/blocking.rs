use systemd_client::{
    build_blocking_client,
    manager::blocking::OrgFreedesktopSystemd1Manager,
    models::{IntoModel, Unit},
    Result, SystemdObjectType,
};

fn main() -> Result<()> {
    let client = build_blocking_client(SystemdObjectType::Manager)?;
    let units = client.list_units()?;
    for unit in units {
        let unit: Unit = unit.into_model()?;
        println!("{:#?}", unit);
    }
    Ok(())
}
