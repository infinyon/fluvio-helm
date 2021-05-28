use std::process::Command;
use fluvio_command::CommandExt;
use tracing::{instrument, warn};

use super::RegistryLoginArg;
use super::ChartPullArg;
use super::ChartExportArg;
use super::InstallArg;
use super::UninstallArg;
use super::HelmError;
use super::Chart;
use super::InstalledChart;
use super::RepoAddArg;
use super::GetInstalledArg;

/// Client to manage helm operations
#[derive(Debug)]
#[non_exhaustive]
pub struct HelmClient {}

impl HelmClient {
    /// Creates a Rust client to manage our helm needs.
    ///
    /// This only succeeds if the helm command can be found.
    pub fn new() -> Result<Self, HelmError> {
        let output = Command::new("helm").arg("version").result()?;

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

    pub fn registry(&self, args: RegistryLoginArg) -> Result<(), HelmError> {
        let mut command: Command = args.into();
        command.result()?;
        Ok(())
    }

    /// Download a chart from a remote registry
    #[instrument(skip(self))]
    pub fn chart_pull(&self, args: ChartPullArg) -> Result<(), HelmError> {
        let mut command: Command = args.into();
        command.result()?;
        Ok(())
    }

    /// Export a chart stored in local registry cache
    #[instrument(skip(self))]
    pub fn export(&self, args: ChartExportArg) -> Result<(), HelmError> {
        let mut command: Command = args.into();
        command.result()?;
        Ok(())
    }

    /// Installs the given chart under the given name.
    ///
    #[instrument(skip(self))]
    pub fn install(&self, args: &InstallArg) -> Result<(), HelmError> {
        let mut command = args.install();
        command.result()?;
        Ok(())
    }

    /// Upgrades the given chart
    #[instrument(skip(self))]
    pub fn upgrade(&self, args: &InstallArg) -> Result<(), HelmError> {
        let mut command = args.upgrade();
        command.result()?;
        Ok(())
    }

    /// Uninstalls specified chart library
    pub fn uninstall(&self, uninstall: UninstallArg) -> Result<(), HelmError> {
        if uninstall.ignore_not_found {
            let app_charts = self
                .get_installed_chart_by_name(&uninstall.release, uninstall.namespace.clone())?;
            if app_charts.is_empty() {
                warn!("Chart does not exists, {}", &uninstall.release);
                return Ok(());
            }
        }
        let mut command: Command = uninstall.into();
        command.result()?;
        Ok(())
    }

    /// Adds a new helm repo with the given chart name and chart location
    #[instrument(skip(self))]
    pub fn repo_add(&self, chart: &str, location: &str) -> Result<(), HelmError> {
        Command::new("helm")
            .args(&["repo", "add", chart, location])
            .args(&["--output", "json"])
            .result()?;
        Ok(())
    }

    pub fn repo_add_with_optionals(&self, optionals: RepoAddArg) -> Result<(), HelmError> {
        let mut command: Command = optionals.into();
        command.result()?;
        Ok(())
    }

    /// Updates the local helm repository
    #[instrument(skip(self))]
    pub fn repo_update(&self) -> Result<(), HelmError> {
        Command::new("helm")
        .args(&["repo", "update"])
        .args(&["--output", "json"])
        .result()?;
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

        let output = command.result()?;

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
        let output = command.result()?;

        check_helm_stderr(output.stderr)?;
        serde_json::from_slice(&output.stdout).map_err(HelmError::Serde)
    }

    /// Checks that a given version of a given chart exists in the repo.
    #[instrument(skip(self))]
    pub fn chart_version_exists(&self, name: &str, version: &str) -> Result<bool, HelmError> {
        let versions = self.search_repo(name, version)?;
        let count = versions
            .iter()
            .filter(|chart| chart.name() == name && chart.version() == version)
            .count();
        Ok(count > 0)
    }

    /// Returns the list of installed charts by name
    #[instrument(skip(self))]
    pub fn get_installed_chart_by_name(
        &self,
        name: &str,
        namespace: Option<String>,
    ) -> Result<Vec<InstalledChart>, HelmError> {
        let exact_match = format!("^{}$", name);
        let mut command = Command::new("helm");
        command
            .arg("list")
            .arg("--filter")
            .arg(exact_match)
            .args(&["--output", "json"]);
        if let Some(ns) = namespace {
            command.args(&["--namespace", ns.as_str()]);
        }

        let output = command.result()?;
        check_helm_stderr(output.stderr)?;
        serde_json::from_slice(&output.stdout).map_err(HelmError::Serde)
    }

    #[instrument(skip(self))]
    pub fn list_installed(&self, args: GetInstalledArg) -> Result<Vec<InstalledChart>, HelmError> {
        let mut command: Command = args.into();

        let output = command.result()?;
        check_helm_stderr(output.stderr)?;
        serde_json::from_slice(&output.stdout).map_err(HelmError::Serde)
    }

    /// get helm package version
    #[instrument(skip(self))]
    pub fn get_helm_version(&self) -> Result<String, HelmError> {
        let helm_version = Command::new("helm")
            .arg("version")
            .arg("--short")
            .args(&["--output", "json"])
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