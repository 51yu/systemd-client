use std::io::Write;

use dbus::blocking::{self, stdintf::org_freedesktop_dbus::Introspectable};
use dbus_codegen::{self, ConnectionType, GenOpts};

const SYSTEMD_DESTINATION: &str = "org.freedesktop.systemd1";
const SYSTEMD_MANAGER_OBJECT_PATH: &str = "/org/freedesktop/systemd1";
const INTROSPECT_TIMEOUT_SECS: u64 = 10;

const OUTPUT_MANAGER_OBJECT_BLOCKING_CLIENT: &str = "src/manager";
const OUTPUT_MANAGER_OBJECT_NONBLOCK_CLIENT: &str = "src/manager";

const MODULE_BLOCKING: &str = "blocking.rs";
const MODULE_NONBLOCK: &str = "nonblock.rs";

struct Generator<'a> {
    output: Option<&'a str>,
    object_path: Option<&'a str>,
    connection_type: Option<ConnectionType>,
}

impl<'a> Generator<'a> {
    pub fn new() -> Self {
        Generator {
            output: None,
            object_path: None,
            connection_type: None,
        }
    }

    pub fn output(mut self, output: &'a str) -> Self {
        self.output = Some(output);
        self
    }

    pub fn object_path(mut self, object_path: &'a str) -> Self {
        self.object_path = Some(object_path);
        self
    }

    pub fn connection_type(mut self, connection_type: ConnectionType) -> Self {
        self.connection_type = Some(connection_type);
        self
    }

    // fetch dbus xml definition of systemd
    fn introspect(object_path: &str) -> anyhow::Result<String> {
        let connection = blocking::Connection::new_system()?;
        let proxy = connection.with_proxy(
            SYSTEMD_DESTINATION,
            object_path,
            std::time::Duration::from_secs(INTROSPECT_TIMEOUT_SECS),
        );
        let definition = proxy.introspect()?;
        Ok(definition)
    }

    // write bytes to file
    fn write<P>(path: P, bytes: &[u8]) -> anyhow::Result<()>
    where
        P: AsRef<std::path::Path>,
    {
        let mut file = std::fs::File::create(path)?;
        file.write_all(bytes)?;
        file.flush()?;
        Ok(())
    }

    pub fn generate(self) -> anyhow::Result<()> {
        let output = self.output.expect("output directory undefined");
        let object_path = self.object_path.expect("object path undefined");
        let connection_type = self.connection_type.expect("connection type undefined");
        let output = match &connection_type {
            ConnectionType::Blocking => format!("{}/{}", output, MODULE_BLOCKING),
            ConnectionType::Nonblock => format!("{}/{}", output, MODULE_NONBLOCK),
            _ => unreachable!(),
        };
        let definition = Self::introspect(object_path)?;
        let opts = GenOpts {
            connectiontype: connection_type,
            methodtype: None,
            ..Default::default()
        };
        let code = dbus_codegen::generate(definition.as_str(), &opts).unwrap();
        Self::write(output, code.as_bytes())?;
        Ok(())
    }
}

fn main() {
    // generate manager object blocking proxy
    Generator::new()
        .object_path(SYSTEMD_MANAGER_OBJECT_PATH)
        .output(OUTPUT_MANAGER_OBJECT_BLOCKING_CLIENT)
        .connection_type(ConnectionType::Blocking)
        .generate()
        .unwrap();
    // generate manager nonblock proxy
    Generator::new()
        .object_path(SYSTEMD_MANAGER_OBJECT_PATH)
        .output(OUTPUT_MANAGER_OBJECT_NONBLOCK_CLIENT)
        .connection_type(ConnectionType::Nonblock)
        .generate()
        .unwrap();
}
