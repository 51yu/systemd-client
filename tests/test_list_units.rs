use systemd_client::{manager, models::Unit};

#[test]
fn test_blocking() {
    let client = manager::build_blocking_proxy().expect("build blocking client failed");
    let units = client.list_units().expect("list units failed");
    for unit in units {
        let unit: Unit = unit.into();
        println!("{:#?}", unit);
    }
}

#[tokio::test]
async fn test_nonblock() {
    let client = manager::build_nonblock_proxy()
        .await
        .expect("build nonblock client failed");
    let units = client.list_units().await.expect("list units failed");
    for unit in units {
        let unit: Unit = unit.into();
        println!("{:#?}", unit);
    }
}
