use systemd_client::{
    build_blocking_client, manager::blocking::OrgFreedesktopSystemd1Manager, Result,
    SystemdObjectType,
};

fn main() -> Result<()> {
    let client = build_blocking_client(SystemdObjectType::Manager)?;
    let units = client.list_units()?;
    for unit in units {
        println!("{:?}", unit);
    }
    Ok(())
}
