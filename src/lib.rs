use std::{
    path::PathBuf,
    process::{Command, Stdio},
};

use serde::Deserialize;
use tracing::{instrument, warn};

mod error;

pub use crate::error::HelmError;
use flv_util::cmd::CommandExt;

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
    pub fn new(name: String, chart: String) -> Self {
        Self {
            name,
            chart,
            version: None,
            namespace: None,
            opts: vec![],
            values: vec![],
            develop: false,
        }
    }

    /// set chart version
    pub fn version(mut self, version: String) -> Self {
        self.version = Some(version);
        self
    }

    /// set namepsace
    pub fn namespace(mut self, ns: String) -> Self {
        self.namespace = Some(ns);
        self
    }

    /// reset array of options
    pub fn opts(mut self, options: Vec<(String, String)>) -> Self {
        self.opts = options;
        self
    }

    /// set a single option
    pub fn opt(mut self, key: String, value: String) -> Self {
        self.opts.push((key, value));
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

    pub fn valu(&mut self, value: PathBuf) -> &mut Self {
        self.values.push(value);
        self
    }
}

impl Into<Command> for InstallArg {
    fn into(self) -> Command {
        let mut command = Command::new("helm");
        command.args(&["install", &self.name, &self.chart]);

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

        command
    }
}

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

impl Into<Command> for UninstallArg {
    fn into(self) -> Command {
        let mut command = Command::new("helm");
        command.args(&["uninstall", &self.release]);

        if let Some(namespace) = &self.namespace {
            command.args(&["--namespace", namespace]);
        }

        if self.dry_run {
            command.arg("--dry-run");
        }

        for timeout in &self.timeout {
            command.arg("--timeout").arg(timeout);
        }

        command
    }
}

/// Client to manage helm operations
#[derive(Debug)]
#[non_exhaustive]
pub struct HelmClient {}

impl HelmClient {
    /// Creates a Rust client to manage our helm needs.
    ///
    /// This only succeeds if the helm command can be found.
    pub fn new() -> Result<Self, HelmError> {
        let output = Command::new("helm")
            .arg("version")
            .print()
            .output()
            .map_err(HelmError::HelmNotInstalled)?;

        // Convert command output into a string
        let out_str = String::from_utf8(output.stdout).map_err(HelmError::Utf8Error)?;

        // Check that the version command gives a version.
        // In the future, we can parse the version string and check
        // for compatible CLI client version.
        if !out_str.contains("version") {
            return Err(HelmError::HelmVersionNotFound(out_str));
        }

        // If checks succeed, create Helm client
        Ok(Self {})
    }

    /// Installs the given chart under the given name.
    ///
    #[instrument(skip(self))]
    pub fn install(&self, install: InstallArg) -> Result<(), HelmError> {
        let mut command: Command = install.into();
        command.inherit();
        Ok(())
    }

    /// Uninstalls specified chart library
    pub fn uninstall(&self, uninstall: UninstallArg) -> Result<(), HelmError> {
        if uninstall.ignore_not_found {
            let app_charts = self
                .get_installed_chart_by_name(&uninstall.release, uninstall.namespace.as_deref())?;
            if app_charts.is_empty() {
                warn!("Chart does not exists, {}", &uninstall.release);
                return Ok(());
            }
        }
        let mut command: Command = uninstall.into();

        command.inherit();
        Ok(())
    }

    /// Adds a new helm repo with the given chart name and chart location
    #[instrument(skip(self))]
    pub fn repo_add(&self, chart: &str, location: &str) -> Result<(), HelmError> {
        Command::new("helm")
            .args(&["repo", "add", chart, location])
            .stdout(Stdio::inherit())
            .stdout(Stdio::inherit())
            .inherit();
        Ok(())
    }

    /// Updates the local helm repository
    #[instrument(skip(self))]
    pub fn repo_update(&self) -> Result<(), HelmError> {
        Command::new("helm").args(&["repo", "update"]).inherit();
        Ok(())
    }

    /// Searches the repo for the named helm chart
    #[instrument(skip(self))]
    pub fn search_repo(&self, chart: &str, version: &str) -> Result<Vec<Chart>, HelmError> {
        let mut command = Command::new("helm");
        command
            .args(&["search", "repo", chart])
            .args(&["--version", version])
            .args(&["--output", "json"]);

        let output = command
            .print()
            .output()
            .map_err(HelmError::HelmNotInstalled)?;

        check_helm_stderr(output.stderr)?;
        serde_json::from_slice(&output.stdout).map_err(HelmError::Serde)
    }

    /// Get all the available versions
    #[instrument(skip(self))]
    pub fn versions(&self, chart: &str) -> Result<Vec<Chart>, HelmError> {
        let mut command = Command::new("helm");
        command
            .args(&["search", "repo"])
            .args(&["--versions", chart])
            .args(&["--output", "json", "--devel"]);
        let output = command
            .print()
            .output()
            .map_err(HelmError::HelmNotInstalled)?;

        check_helm_stderr(output.stderr)?;
        serde_json::from_slice(&output.stdout).map_err(HelmError::Serde)
    }

    /// Checks that a given version of a given chart exists in the repo.
    #[instrument(skip(self))]
    pub fn chart_version_exists(&self, name: &str, version: &str) -> Result<bool, HelmError> {
        let versions = self.search_repo(name, version)?;
        let count = versions
            .iter()
            .filter(|chart| chart.name == name && chart.version == version)
            .count();
        Ok(count > 0)
    }

    /// Returns the list of installed charts by name
    #[instrument(skip(self))]
    pub fn get_installed_chart_by_name(
        &self,
        name: &str,
        namespace: Option<&str>,
    ) -> Result<Vec<InstalledChart>, HelmError> {
        let exact_match = format!("^{}$", name);
        let mut command = Command::new("helm");
        command
            .arg("list")
            .arg("--filter")
            .arg(exact_match)
            .arg("--output")
            .arg("json");
        if let Some(ns) = namespace {
            command.args(&["--namespace", ns]);
        }

        let output = command
            .print()
            .output()
            .map_err(HelmError::HelmNotInstalled)?;

        check_helm_stderr(output.stderr)?;
        serde_json::from_slice(&output.stdout).map_err(HelmError::Serde)
    }

    /// get helm package version
    #[instrument(skip(self))]
    pub fn get_helm_version(&self) -> Result<String, HelmError> {
        let helm_version = Command::new("helm")
            .arg("version")
            .arg("--short")
            .output()
            .map_err(HelmError::HelmNotInstalled)?;
        let version_text = String::from_utf8(helm_version.stdout).map_err(HelmError::Utf8Error)?;
        Ok(version_text[1..].trim().to_string())
    }
}

/// Check for errors in Helm's stderr output
///
/// Returns `Ok(())` if everything is fine, or `HelmError` if something is wrong
fn check_helm_stderr(stderr: Vec<u8>) -> Result<(), HelmError> {
    if !stderr.is_empty() {
        let stderr = String::from_utf8(stderr)?;
        if stderr.contains("Kubernetes cluster unreachable") {
            return Err(HelmError::FailedToConnect);
        }
    }

    Ok(())
}

/// A representation of a chart definition in a repo.
#[derive(Debug, Deserialize)]
pub struct Chart {
    /// The chart name
    name: String,
    /// The chart version
    version: String,
}

impl Chart {
    pub fn version(&self) -> &str {
        &self.version
    }
    pub fn name(&self) -> &str {
        &self.name
    }
}

/// A representation of an installed chart.
#[derive(Debug, Deserialize)]
pub struct InstalledChart {
    /// The chart name
    pub name: String,
    /// The version of the app this chart installed
    pub app_version: String,
}
