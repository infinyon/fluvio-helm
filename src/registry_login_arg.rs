use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct RegistryLoginArg {
    pub host: String,
    pub username: String,
    pub password: String,
}

impl Into<Command> for RegistryLoginArg {
    fn into(self) -> Command {
        let mut command = Command::new("helm");
        command.args(&["registry"]);
        command.args(&["login"]);

        command.arg(self.host.as_str());

        command.args(&["--username", self.username.as_str()]);
        command.args(&["--password", self.password.as_str()]);

        command
    }
}
