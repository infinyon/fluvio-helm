use std::process::Command;

/// Uninstaller Argument
#[derive(Debug)]
pub struct UninstallArg {
    pub release: String,
    pub namespace: Option<String>,
    pub ignore_not_found: bool,
    pub dry_run: bool,
    pub timeout: Option<String>,
}

impl UninstallArg {
    pub fn new(release: String) -> Self {
        Self {
            release,
            namespace: None,
            ignore_not_found: false,
            dry_run: false,
            timeout: None,
        }
    }

    /// set namepsace
    pub fn namespace(mut self, ns: String) -> Self {
        self.namespace = Some(ns);
        self
    }

    /// set ignore not found
    pub fn ignore_not_found(mut self) -> Self {
        self.ignore_not_found = true;
        self
    }

    /// set dry tun
    pub fn dry_run(mut self) -> Self {
        self.dry_run = true;
        self
    }

    /// set timeout
    pub fn timeout(mut self, timeout: String) -> Self {
        self.timeout = Some(timeout);
        self
    }
}

impl From<UninstallArg> for Command {
    fn from(arg: UninstallArg) -> Self {
        let mut command = Command::new("helm");
        command.args(&["uninstall", &arg.release]);

        if let Some(namespace) = &arg.namespace {
            command.args(&["--namespace", namespace]);
        }

        if arg.dry_run {
            command.arg("--dry-run");
        }

        for timeout in &arg.timeout {
            command.arg("--timeout").arg(timeout);
        }

        command
    }
}
