# systemd-client
[`systemd dbus`] client lib using [`dbus-codegen`]
## Examples
### Blocking
list units
```rust
use systemd_client::{
    build_blocking_client,
    manager::blocking::OrgFreedesktopSystemd1Manager,
    models::{IntoModel, Unit},
    Result, SystemdObjectType,
};

fn main() -> Result<()> {
    let client = build_blocking_client(SystemdObjectType::Manager)?;
    let units = client.list_units()?;
    for unit in units {
        let unit: Unit = unit.into_model()?;
        println!("{:#?}", unit);
    }
    Ok(())
}
```
create and start service
```rust
use systemd_client::{
    build_blocking_client, create_unit_configuration_file,
    manager::blocking::OrgFreedesktopSystemd1Manager, models::IntoModel,
    unit::blocking::UnitProperties, Result, ServiceConfiguration, ServiceUnitConfiguration,
    SystemdObjectType, UnitActiveStateType, UnitConfiguration, UnitLoadStateType, UnitProps,
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
    let client = build_blocking_client(SystemdObjectType::Manager)?;
    let job_path = client.start_unit("test.service", "replace")?;
    println!("{}", job_path);
    let svc_unit_path = client.get_unit("test.service")?;
    println!("{}", svc_unit_path);
    // verify unit state given unit path
    let client = build_blocking_client(SystemdObjectType::Unit(svc_unit_path))?;
    let unit_props = client.get_unit_properties()?;
    let unit_props: UnitProps = unit_props.into_model()?;
    println!("{:?}", unit_props);
    assert_eq!(unit_props.load_state, UnitLoadStateType::Loaded);
    assert_eq!(unit_props.active_state, UnitActiveStateType::Active);
    assert_eq!(unit_props.sub_state, UnitSubStateType::Running);
    std::thread::sleep(std::time::Duration::from_secs(4));
    // service should exit after 3 sec
    let unit_props = client.get_unit_properties()?;
    let unit_props: UnitProps = unit_props.into_model()?;
    println!("{:?}", unit_props);
    assert_eq!(unit_props.load_state, UnitLoadStateType::Loaded);
    assert_eq!(unit_props.active_state, UnitActiveStateType::Inactive);
    assert_eq!(unit_props.sub_state, UnitSubStateType::Dead);
    Ok(())
}
```
### Non Block
list units
```rust
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
```
create and start service
```rust
use systemd_client::{
    build_nonblock_client, create_unit_configuration_file,
    manager::nonblock::OrgFreedesktopSystemd1Manager, models::IntoModel,
    unit::nonblock::UnitProperties, Result, ServiceConfiguration, ServiceUnitConfiguration,
    SystemdObjectType, UnitActiveStateType, UnitConfiguration, UnitLoadStateType, UnitProps,
    UnitSubStateType,
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
    let svc_builder = ServiceConfiguration::builder().exec_start(vec!["/bin/sleep", "3"]);
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
    // verify unit state given unit path
    let (client, jh) = build_nonblock_client(SystemdObjectType::Unit(svc_unit_path))?;
    let unit_props = client.get_unit_properties().await?;
    let unit_props: UnitProps = unit_props.into_model()?;
    println!("{:?}", unit_props);
    assert_eq!(unit_props.load_state, UnitLoadStateType::Loaded);
    assert_eq!(unit_props.active_state, UnitActiveStateType::Active);
    assert_eq!(unit_props.sub_state, UnitSubStateType::Running);
    std::thread::sleep(std::time::Duration::from_secs(4));
    // service should exit after 3 sec
    let unit_props = client.get_unit_properties().await?;
    let unit_props: UnitProps = unit_props.into_model()?;
    println!("{:?}", unit_props);
    assert_eq!(unit_props.load_state, UnitLoadStateType::Loaded);
    assert_eq!(unit_props.active_state, UnitActiveStateType::Inactive);
    assert_eq!(unit_props.sub_state, UnitSubStateType::Dead);
    // close connection
    jh.abort();
    Ok(())
}
```
## Development
### Install Tools
```sh
sudo apt install libdbus-1-dev pkg-config
```
### Codegen
edit `build.rs` and create module for dbus object

[`systemd dbus`]: https://www.freedesktop.org/software/systemd/man/org.freedesktop.systemd1.html
[`dbus-codegen`]: https://github.com/diwic/dbus-rs/tree/master/dbus-codegen
