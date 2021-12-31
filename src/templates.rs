use std::fmt::Display;

// configuration templates of systemd
pub struct UnitConfiguration<'a> {
    pub description: &'a str,
    pub after: Vec<&'a str>,
    pub start_limit_interval_sec: u64,
}

impl<'a> Display for UnitConfiguration<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "[Unit]")?;
        writeln!(f, "Description={}", self.description)?;
        writeln!(f, "After={}", self.after.join(" "))?;
        writeln!(f, "startLimitIntervalSec={}", self.start_limit_interval_sec)
    }
}

impl<'a> UnitConfiguration<'a> {
    pub fn builder() -> UnitConfigurationBuilder<'a> {
        UnitConfigurationBuilder::default()
    }
}

#[derive(Default)]
pub struct UnitConfigurationBuilder<'a> {
    pub description: &'a str,
    pub after: Vec<&'a str>,
    pub start_limit_interval_sec: u64,
}

impl<'a> UnitConfigurationBuilder<'a> {
    pub fn description(mut self, description: &'a str) -> Self {
        self.description = description;
        self
    }

    pub fn after(mut self, after: &'a str) -> Self {
        self.after.push(after);
        self
    }

    pub fn start_limit_interval_sec(mut self, start_limit_interval_sec: u64) -> Self {
        self.start_limit_interval_sec = start_limit_interval_sec;
        self
    }

    pub fn build(self) -> UnitConfiguration<'a> {
        let description = self.description;
        let after = self.after;
        let start_limit_interval_sec = self.start_limit_interval_sec;
        UnitConfiguration {
            description,
            after,
            start_limit_interval_sec,
        }
    }
}

pub enum ServiceType {
    Simple,
    Exec,
    Forking,
}

impl Display for ServiceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ty = match self {
            ServiceType::Simple => "simple",
            ServiceType::Exec => "exec",
            ServiceType::Forking => "forking",
        };
        write!(f, "{}", ty)
    }
}

pub enum RestartPolicy {
    No,
    OnSuccess,
    OnFailure,
    Always,
}

impl Display for RestartPolicy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let policy = match self {
            RestartPolicy::No => "no",
            RestartPolicy::OnSuccess => "on-success",
            RestartPolicy::OnFailure => "on-failure",
            RestartPolicy::Always => "always",
        };
        write!(f, "{}", policy)
    }
}

// https://www.freedesktop.org/software/systemd/man/systemd.service.html#Service%20Templates
pub struct ServiceConfiguration<'a> {
    pub ty: ServiceType,
    pub exec_start: Vec<&'a str>,
    pub restart_policy: RestartPolicy,
    // a unit-less value in seconds, or a time span value such as "5min 20s"
    pub restart_sec: &'a str,
    pub working_directory: Option<&'a str>,
}

impl<'a> Display for ServiceConfiguration<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "[Service]")?;
        if let Some(working_directory) = self.working_directory {
            writeln!(f, "WorkingDirectory={}", working_directory)?;
        }
        writeln!(f, "ExecStart={}", self.exec_start.join(" "))?;
        writeln!(f, "Restart={}", self.restart_policy)?;
        writeln!(f, "RestartSec={}", self.restart_sec)
    }
}

impl<'a> ServiceConfiguration<'a> {
    pub fn builder() -> ServiceConfigurationBuilder<'a> {
        ServiceConfigurationBuilder::default()
    }
}

pub struct ServiceConfigurationBuilder<'a> {
    pub ty: ServiceType,
    pub exec_start: Vec<&'a str>,
    pub restart_policy: RestartPolicy,
    pub restart_sec: &'a str,
    pub working_directory: Option<&'a str>,
}

impl<'a> Default for ServiceConfigurationBuilder<'a> {
    fn default() -> Self {
        ServiceConfigurationBuilder {
            ty: ServiceType::Simple,
            exec_start: vec![],
            restart_policy: RestartPolicy::No,
            restart_sec: "100ms",
            working_directory: None,
        }
    }
}

impl<'a> ServiceConfigurationBuilder<'a> {
    pub fn ty(mut self, ty: ServiceType) -> Self {
        self.ty = ty;
        self
    }

    pub fn exec_start(mut self, exec_start: Vec<&'a str>) -> Self {
        self.exec_start = exec_start;
        self
    }

    pub fn restart_policy(mut self, restart_policy: RestartPolicy) -> Self {
        self.restart_policy = restart_policy;
        self
    }

    pub fn restart_sec(mut self, restart_sec: &'a str) -> Self {
        self.restart_sec = restart_sec;
        self
    }

    pub fn working_directory(mut self, working_directory: &'a str) -> Self {
        self.working_directory = Some(working_directory);
        self
    }

    pub fn build(self) -> ServiceConfiguration<'a> {
        let ty = self.ty;
        let exec_start = self.exec_start;
        let restart_policy = self.restart_policy;
        let restart_sec = self.restart_sec;
        let working_directory = self.working_directory;
        ServiceConfiguration {
            ty,
            exec_start,
            restart_policy,
            restart_sec,
            working_directory,
        }
    }
}

pub struct InstallConfiguration<'a> {
    pub wanted_by: Vec<&'a str>,
}

impl<'a> Display for InstallConfiguration<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "[Install]")?;
        writeln!(f, "WantedBy={}", self.wanted_by.join(" "))
    }
}

impl<'a> InstallConfiguration<'a> {
    pub fn builder() -> InstallConfigurationBuilder<'a> {
        InstallConfigurationBuilder::default()
    }
}

pub struct InstallConfigurationBuilder<'a> {
    pub wanted_by: Vec<&'a str>,
}

impl<'a> Default for InstallConfigurationBuilder<'a> {
    fn default() -> Self {
        InstallConfigurationBuilder {
            // https://unix.stackexchange.com/questions/404667/systemd-service-what-is-multi-user-target
            wanted_by: vec!["multi-user.target"],
        }
    }
}

impl<'a> InstallConfigurationBuilder<'a> {
    pub fn wanted_by(mut self, wanted_by: &'a str) -> Self {
        self.wanted_by.push(wanted_by);
        self
    }

    pub fn build(self) -> InstallConfiguration<'a> {
        let wanted_by = self.wanted_by;
        InstallConfiguration { wanted_by }
    }
}

// https://www.freedesktop.org/software/systemd/man/systemd.service.html#Service%20Templates
pub struct ServiceUnitConfiguration<'a> {
    pub unit: UnitConfiguration<'a>,
    pub service: ServiceConfiguration<'a>,
    pub install: InstallConfiguration<'a>,
}

impl<'a> Display for ServiceUnitConfiguration<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n{}\n{}", self.unit, self.service, self.install)
    }
}

impl<'a> ServiceUnitConfiguration<'a> {
    pub fn builder() -> ServiceUnitConfigurationBuilder<'a> {
        ServiceUnitConfigurationBuilder::default()
    }
}

#[derive(Default)]
pub struct ServiceUnitConfigurationBuilder<'a> {
    pub unit: UnitConfigurationBuilder<'a>,
    pub service: ServiceConfigurationBuilder<'a>,
    pub install: InstallConfigurationBuilder<'a>,
}

impl<'a> ServiceUnitConfigurationBuilder<'a> {
    pub fn unit(mut self, unit: UnitConfigurationBuilder<'a>) -> Self {
        self.unit = unit;
        self
    }

    pub fn service(mut self, service: ServiceConfigurationBuilder<'a>) -> Self {
        self.service = service;
        self
    }

    pub fn install(mut self, install: InstallConfigurationBuilder<'a>) -> Self {
        self.install = install;
        self
    }

    pub fn build(self) -> ServiceUnitConfiguration<'a> {
        let unit = self.unit.build();
        let service = self.service.build();
        let install = self.install.build();
        ServiceUnitConfiguration {
            unit,
            service,
            install,
        }
    }
}
