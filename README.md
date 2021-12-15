# systemd-client
[`systemd dbus`] client lib using [`dbus-codegen`]
## Examples
blocking
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
nonblock
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
## Development
### Install Tools
```sh
sudo apt install libdbus-1-dev pkg-config
```
### Codegen
edit `build.rs` and save

[`systemd dbus`]: https://www.freedesktop.org/software/systemd/man/org.freedesktop.systemd1.html
[`dbus-codegen`]: https://github.com/diwic/dbus-rs/tree/master/dbus-codegen
