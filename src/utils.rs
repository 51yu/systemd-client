use crate::{Result, SYSTEMD_UNIT_CONFIGURATION_DIRECTORY};

use std::io::Write;

pub fn create_unit_configuration_file(unit_name: &str, buffer: &[u8]) -> Result<()> {
    let mut path = std::path::PathBuf::from(SYSTEMD_UNIT_CONFIGURATION_DIRECTORY);
    path.push(unit_name);
    let file = std::fs::File::create(path.as_path())?;
    let mut writer = std::io::BufWriter::new(file);
    writer.write_all(buffer)?;
    writer.flush()?;
    Ok(())
}

pub fn delete_unit_configuration_file(unit_name: &str) -> Result<()> {
    let mut path = std::path::PathBuf::from(SYSTEMD_UNIT_CONFIGURATION_DIRECTORY);
    path.push(unit_name);
    std::fs::remove_file(path.as_path())?;
    Ok(())
}
