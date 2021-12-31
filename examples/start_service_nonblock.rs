use systemd_client::{
    build_nonblock_client, create_unit_configuration_file,
    manager::nonblock::OrgFreedesktopSystemd1Manager, Result, ServiceConfiguration,
    ServiceUnitConfiguration, SystemdObjectType, UnitConfiguration,
};

/*
 * Run example as superuser since we start a service
 * ```sh
 * cargo build --example start_service_nonblock
 * sudo ./target/debug/examples/start_service_nonblock
 * ```
 */
#[tokio::main]
async fn main() -> Result<()> {
    let unit_builder = UnitConfiguration::builder().description("test service");
    let svc_builder = ServiceConfiguration::builder().exec_start(vec!["/bin/sleep", "10"]);
    let svc_unit = ServiceUnitConfiguration::builder()
        .unit(unit_builder)
        .service(svc_builder)
        .build();
    let svc_unit_literal = format!("{}", svc_unit);
    // create /etc/systemd/system/test.service
    create_unit_configuration_file("test.service", svc_unit_literal.as_bytes())?;
    let (client, jh) = build_nonblock_client(SystemdObjectType::Manager)?;
    let job_path = client.start_unit("test.service", "replace").await?;
    println!("{}", job_path);
    let svc_unit_path = client.get_unit("test.service").await?;
    println!("{}", svc_unit_path);
    // close connection
    jh.abort();
    Ok(())
}
