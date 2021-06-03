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
    pub values: Option<Vec<PathBuf>>,
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
            values: None,
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
    pub fn values(mut self, values: Option<Vec<PathBuf>>) -> Self {
        self.values = values;
        self
    }

    /// set one value
    pub fn value(&mut self, value: PathBuf) -> &mut Self {
        if self.values.is_none() {
            self.values = Some(vec![]);
        }
        self.values.as_mut().unwrap().push(value);
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
        command.args(&["--output", "json"]);
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

        if let Some(values) = &self.values {
            for value_path in values {
                command.arg("--values").arg(value_path);
            }
        }

        for (key, val) in &self.opts {
            command.arg("--set").arg(format!(r#"{}="{}""#, key, val));
        }
    }
}

impl Into<Command> for InstallArg {
    fn into(self) -> Command {
        let mut command = Command::new("helm");
        command.args(&["install", &self.name, &self.chart]);
        command.args(&["--output", "json"]);

        if let Some(namespace) = &self.namespace {
            command.args(&["--namespace", namespace]);
        }

        if self.develop {
            command.arg("--devel");
        }

        if let Some(version) = &self.version {
            command.args(&["--version", version]);
        }

        if let Some(values) = &self.values {
            for value_path in values {
                command.arg("--values").arg(value_path);
            }
        }

        for (key, val) in &self.opts {
            command.arg("--set").arg(format!("{}={}", key, val));
        }

        command
    }
}

