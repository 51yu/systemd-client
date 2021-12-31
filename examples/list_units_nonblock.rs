use systemd_client::{
    build_nonblock_client,
    manager::nonblock::OrgFreedesktopSystemd1Manager,
    models::{IntoModel, Unit},
    Result, SystemdObjectType,
};

#[tokio::main]
pub async fn main() -> Result<()> {
    let (client, jh) = build_nonblock_client(SystemdObjectType::Manager)?;
    let units = client.list_units().await?;
    for unit in units {
        let unit: Unit = unit.into_model()?;
        println!("{:#?}", unit);
    }
    // close connection
    jh.abort();
    Ok(())
}
