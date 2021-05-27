use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ChartPullArg {
    pub chart: String,
    pub version: String,
}

impl Into<Command> for ChartPullArg {
    fn into(self) -> Command {
        let mut command = Command::new("helm");
        command.args(&["chart"]);
        command.args(&["pull"]);

        command.args(&[format!("{}:{}", &self.chart, &self.version)]);

        command
    }
}
