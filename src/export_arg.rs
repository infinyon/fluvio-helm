use serde::{Deserialize, Serialize};
use std::{path::PathBuf, process::Command};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ExportArg {
    pub chart: String,
    pub version: String,
    pub destination: PathBuf,
}

impl Into<Command> for ExportArg {
    fn into(self) -> Command {
        let mut command = Command::new("helm");
        command.args(&["chart"]);
        command.args(&["export"]);

        command.args(&[format!("{}:{}", &self.chart, &self.version)]);

        command.arg("--destination").arg(self.destination);

        command
    }
}
