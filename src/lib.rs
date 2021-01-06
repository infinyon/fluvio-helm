use serde::Deserialize;
use std::process::{Command, Stdio};
use tracing::{instrument, warn};

mod error;

pub use crate::error::HelmError;
use flv_util::cmd::CommandExt;

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
    /// The `opts` are passed to helm as `--set` arguments.
    #[instrument(skip(self, version, opts))]
    pub fn install(
        &self,
        namespace: &str,
        name: &str,
        chart: &str,
        version: Option<&str>,
        opts: &[(&str, &str)],
    ) -> Result<(), HelmError> {
        let sets: Vec<_> = opts
            .iter()
            .flat_map(|(key, val)| vec!["--set".to_string(), format!("{}={}", key, val)])
            .collect();

        let mut command = Command::new("helm");
        command
            .args(&["install", name, chart])
            .args(&["--namespace", namespace])
            .args(&["--devel"])
            .args(sets);

        if let Some(version) = version {
            command.args(&["--version", version]);
        }

        command.inherit();
        Ok(())
    }

    /// Uninstalls specified chart library
    pub fn uninstall(&self, name: &str, namespace: &str, ignore_not_found: bool) -> Result<(), HelmError> {
        if ignore_not_found {
            let app_charts = self.get_installed_chart_by_name(name,namespace)?;
            if app_charts.is_empty() {
                warn!("Chart does not exists, {}", &name);
                return Ok(());
            }
        }
        let mut command = Command::new("helm");
        command
            .args(&["uninstall", name])
            .args(&["--namespace", namespace]);

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
        namespace: &str,
    ) -> Result<Vec<InstalledChart>, HelmError> {
        let exact_match = format!("^{}$", name);
        let mut command = Command::new("helm");
        command
            .arg("list")
            .arg("--filter")
            .arg(exact_match)
            .args(&["--namespace", namespace])
            .arg("--output")
            .arg("json");
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
