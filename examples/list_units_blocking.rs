use systemd_client::{manager, models::Unit, Result};

fn main() -> Result<()> {
    let client = manager::build_blocking_proxy()?;
    let units = client.list_units()?;
    for unit in units {
        let unit: Unit = unit.into();
        println!("{:#?}", unit);
    }
    Ok(())
}
