use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PullArg {
    pub chart: String,
    pub version: String,
}

impl Into<Command> for PullArg {
    fn into(self) -> Command {
        let mut command = Command::new("helm");
        command.args(&["chart"]);
        command.args(&["pull"]);

        command.args(&[format!("{}:{}", &self.chart, &self.version)]);

        command
    }
}
