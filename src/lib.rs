mod chart;
mod error;
mod helm_client;
mod install_arg;
mod installed_chart;
mod uninstall_arg;

pub use crate::error::HelmError;

pub use chart::Chart;
pub use helm_client::HelmClient;
pub use install_arg::InstallArg;
pub use installed_chart::InstalledChart;
pub use uninstall_arg::UninstallArg;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_get_installed_charts() {
        const JSON_RESPONSE: &str = r#"[{"name":"test_chart","namespace":"default","revision":"50","updated":"2021-03-17 08:42:54.546347741 +0000 UTC","status":"deployed","chart":"test_chart-1.2.32-rc2","app_version":"1.2.32-rc2"}]"#;
        let installed_charts: Vec<InstalledChart> =
            serde_json::from_slice(JSON_RESPONSE.as_bytes()).expect("can not parse json");
        assert_eq!(installed_charts.len(), 1);
        let test_chart = installed_charts
            .get(0)
            .expect("can not grab the first result");
        assert_eq!(test_chart.name, "test_chart");
        assert_eq!(test_chart.chart, "test_chart-1.2.32-rc2");
    }
}
