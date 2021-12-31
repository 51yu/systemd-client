use systemd_client::{
    build_blocking_client,
    create_unit_configuration_file,
    manager::blocking::OrgFreedesktopSystemd1Manager,
    // models::{IntoModel, Unit},
    Result,
    ServiceConfiguration,
    ServiceUnitConfiguration,
    SystemdObjectType,
    UnitConfiguration,
};

/*
 * Run example as superuser since we start a service
 * ```sh
 * cargo build --example start_service_blocking
 * sudo ./target/debug/examples/start_service_blocking
 * ```
 */
fn main() -> Result<()> {
    let unit_builder = UnitConfiguration::builder().description("test service");
    let svc_builder = ServiceConfiguration::builder().exec_start(vec!["/bin/sleep", "10"]);
    let svc_unit = ServiceUnitConfiguration::builder()
        .unit(unit_builder)
        .service(svc_builder)
        .build();
    let svc_unit_literal = format!("{}", svc_unit);
    create_unit_configuration_file("test.service", svc_unit_literal.as_bytes())?;
    let client = build_blocking_client(SystemdObjectType::Manager)?;
    let job_path = client.start_unit("test.service", "replace")?;
    println!("{}", job_path);
    let svc_unit_path = client.get_unit("test.service")?;
    println!("{}", svc_unit_path);
    Ok(())
}
