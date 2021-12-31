use systemd_client::{ServiceConfiguration, ServiceUnitConfiguration, UnitConfiguration};

#[test]
fn test_service_template() {
    let unit_builder = UnitConfiguration::builder().description("test service");
    let svc_builder = ServiceConfiguration::builder()
        .exec_start(vec!["/bin/echo", "aloha"])
        .working_directory("/path/to/directory")
        .user("guest")
        .group("guest");
    let svc_unit = ServiceUnitConfiguration::builder()
        .unit(unit_builder)
        .service(svc_builder)
        .build();
    let actual_svc_unit = format!("{}", svc_unit);
    let buffer =
        std::fs::read("tests/resources/test.service").expect("open file 'test.service' failed");
    let expected_svc_unit = String::from_utf8(buffer).expect("invalid utf-8 in 'test.service'");
    assert_eq!(expected_svc_unit, actual_svc_unit);
}
