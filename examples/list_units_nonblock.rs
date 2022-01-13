use systemd_client::{manager, models::Unit, Result};

#[tokio::main]
pub async fn main() -> Result<()> {
    let client = manager::build_nonblock_proxy().await?;
    let units = client.list_units().await?;
    for unit in units {
        let unit: Unit = unit.into();
        println!("{:#?}", unit);
    }
    Ok(())
}
