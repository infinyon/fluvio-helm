use serde::{Deserialize, Serialize};
use std::{path::PathBuf, process::Command};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ChartExportArg {
    pub chart: String,
    pub version: String,
    pub destination: PathBuf,
}

impl Into<Command> for ChartExportArg {
    fn into(self) -> Command {
        let mut command = Command::new("helm");
        command.args(&["chart", "export"]);
        command.args(&[format!("{}:{}", &self.chart, &self.version)]);
        command.arg("--destination").arg(self.destination);

        command
    }
}
