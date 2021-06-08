use serde::{Deserialize, Serialize};

/// A representation of an installed chart.
#[derive(Debug, Serialize, Deserialize)]
pub struct InstalledChart {
    /// The chart name
    pub name: String,
    /// The version of the app this chart installed
    pub app_version: String,
    /// The chart revision
    pub revision: String,
    /// Date/time when the chart was last updated
    pub updated: String,
    /// Status of the installed chart
    pub status: String,
    /// The ID of the chart that is installed
    pub chart: String,
}
