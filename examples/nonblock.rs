use systemd_client::{
    build_nonblock_client, manager::nonblock::OrgFreedesktopSystemd1Manager, Result,
    SystemdObjectType,
};

#[tokio::main]
pub async fn main() -> Result<()> {
    let (client, jh) = build_nonblock_client(SystemdObjectType::Manager)?;
    let units = client.list_units().await?;
    for unit in units {
        println!("{:?}", unit);
    }
    // close connection
    jh.abort();
    Ok(())
}
