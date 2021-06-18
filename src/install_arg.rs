use std::path::PathBuf;
use std::process::Command;

/// Installer Argument
#[derive(Debug)]
pub struct InstallArg {
    pub name: String,
    pub chart: String,
    pub version: Option<String>,
    pub namespace: Option<String>,
    pub opts: Vec<(String, String)>,
    pub values: Vec<PathBuf>,
    pub develop: bool,
}

impl InstallArg {
    pub fn new<N: Into<String>, C: Into<String>>(name: N, chart: C) -> Self {
        Self {
            name: name.into(),
            chart: chart.into(),
            version: None,
            namespace: None,
            opts: vec![],
            values: vec![],
            develop: false,
        }
    }

    /// set chart version
    pub fn version<S: Into<String>>(mut self, version: S) -> Self {
        self.version = Some(version.into());
        self
    }

    /// set namepsace
    pub fn namespace<S: Into<String>>(mut self, ns: S) -> Self {
        self.namespace = Some(ns.into());
        self
    }

    /// reset array of options
    pub fn opts(mut self, options: Vec<(String, String)>) -> Self {
        self.opts = options;
        self
    }

    /// set a single option
    pub fn opt<K: Into<String>, V: Into<String>>(mut self, key: K, value: V) -> Self {
        self.opts.push((key.into(), value.into()));
        self
    }

    /// set to use develop
    pub fn develop(mut self) -> Self {
        self.develop = true;
        self
    }

    /// set list of values
    pub fn values(mut self, values: Vec<PathBuf>) -> Self {
        self.values = values;
        self
    }

    /// set one value
    pub fn value(&mut self, value: PathBuf) -> &mut Self {
        self.values.push(value);
        self
    }

    pub fn install(&self) -> Command {
        let mut command = Command::new("helm");
        command.args(&["install", &self.name, &self.chart]);
        self.apply_args(&mut command);
        command
    }

    pub fn upgrade(&self) -> Command {
        let mut command = Command::new("helm");
        command.args(&["upgrade", "--install", &self.name, &self.chart]);
        self.apply_args(&mut command);
        command
    }

    fn apply_args(&self, command: &mut Command) {
        if let Some(namespace) = &self.namespace {
            command.args(&["--namespace", namespace]);
        }

        if self.develop {
            command.arg("--devel");
        }

        if let Some(version) = &self.version {
            command.args(&["--version", version]);
        }

        for value_path in &self.values {
            command.arg("--values").arg(value_path);
        }

        for (key, val) in &self.opts {
            command.arg("--set").arg(format!("{}={}", key, val));
        }
    }
}

impl From<InstallArg> for Command {
    fn from(arg: InstallArg) -> Self {
        let mut command = Command::new("helm");
        command.args(&["install", &arg.name, &arg.chart]);

        if let Some(namespace) = &arg.namespace {
            command.args(&["--namespace", namespace]);
        }

        if arg.develop {
            command.arg("--devel");
        }

        if let Some(version) = &arg.version {
            command.args(&["--version", version]);
        }

        for value_path in &arg.values {
            command.arg("--values").arg(value_path);
        }

        for (key, val) in &arg.opts {
            command.arg("--set").arg(format!("{}={}", key, val));
        }

        command
    }
}
