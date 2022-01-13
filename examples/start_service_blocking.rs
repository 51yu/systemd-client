use systemd_client::{
    create_unit_configuration_file, manager, unit, Result, ServiceConfiguration,
    ServiceUnitConfiguration, UnitActiveStateType, UnitConfiguration, UnitLoadStateType, UnitProps,
    UnitSubStateType,
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
    let svc_builder = ServiceConfiguration::builder().exec_start(vec!["/bin/sleep", "3"]);
    let svc_unit = ServiceUnitConfiguration::builder()
        .unit(unit_builder)
        .service(svc_builder)
        .build();
    let svc_unit_literal = format!("{}", svc_unit);
    // create /etc/systemd/system/test.service
    create_unit_configuration_file("test.service", svc_unit_literal.as_bytes())?;
    let client = manager::build_blocking_proxy()?;
    let job_path = client.start_unit("test.service", "replace")?;
    println!("{}", job_path.as_str());
    let svc_unit_path = client.get_unit("test.service")?;
    println!("{}", svc_unit_path.as_str());
    // verify unit state given unit path
    let client = unit::build_blocking_proxy(svc_unit_path)?;
    let unit_props = client.get_properties()?;
    let unit_props: UnitProps = unit_props.into();
    println!("{:?}", unit_props);
    assert_eq!(unit_props.load_state, UnitLoadStateType::Loaded);
    assert_eq!(unit_props.active_state, UnitActiveStateType::Active);
    assert_eq!(unit_props.sub_state, UnitSubStateType::Running);
    std::thread::sleep(std::time::Duration::from_secs(4));
    // service should exit after 3 sec
    let unit_props = client.get_properties()?;
    let unit_props: UnitProps = unit_props.into();
    println!("{:?}", unit_props);
    assert_eq!(unit_props.load_state, UnitLoadStateType::Loaded);
    assert_eq!(unit_props.active_state, UnitActiveStateType::Inactive);
    assert_eq!(unit_props.sub_state, UnitSubStateType::Dead);
    Ok(())
}
