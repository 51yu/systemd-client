use systemd_client::{
    build_blocking_client, build_nonblock_client,
    models::{IntoModel, Unit},
    SystemdObjectType,
};

#[test]
fn test_blocking() {
    use systemd_client::manager::blocking::OrgFreedesktopSystemd1Manager;

    let client =
        build_blocking_client(SystemdObjectType::Manager).expect("build blocking client failed");
    let units = client.list_units().expect("list units failed");
    for unit in units {
        let unit: Unit = unit.into_model().expect("into model failed");
        println!("{:#?}", unit);
    }
}

#[tokio::test]
async fn test_nonblock() {
    use systemd_client::manager::nonblock::OrgFreedesktopSystemd1Manager;

    let (client, jh) =
        build_nonblock_client(SystemdObjectType::Manager).expect("build nonblock client failed");
    let units = client.list_units().await.expect("list units failed");
    for unit in units {
        let unit: Unit = unit.into_model().expect("into model failed");
        println!("{:#?}", unit);
    }
    // close connection
    jh.abort();
}
