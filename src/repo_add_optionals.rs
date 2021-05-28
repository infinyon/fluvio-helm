use std::process::Command;

#[derive(Debug, Default)]
pub struct RepoAddArg {
    chart: String,
    location: String,

    force_update: Option<bool>,
    insecure_skip_tls_verify: Option<bool>,
    no_update: Option<bool>,

    ca_file: Option<String>,
    cert_file: Option<String>,
    key_file: Option<String>,
    password: Option<String>,
    username: Option<String>
}

impl RepoAddArg {
    pub fn new(chart: &str, location: &str) -> RepoAddArg {
        RepoAddArg{
            chart: chart.into(),
            location: location.into(),
            ..RepoAddArg::default()
        }
    }

    pub fn set_force_update<S: Into<bool>>(mut self, value: S) -> Self {
        self.force_update = Some(value.into());
        self
    }

    pub fn set_insecure_skip_tls_verify<S: Into<bool>>(mut self, value: S) -> Self {
        self.insecure_skip_tls_verify = Some(value.into());
        self
    }

    pub fn set_no_update<S: Into<bool>>(mut self, value: S) -> Self {
        self.no_update = Some(value.into());
        self
    }

    pub fn set_ca_file<S: Into<String>>(mut self, value: S) -> Self {
        self.ca_file = Some(value.into());
        self
    }

    pub fn set_cert_file<S: Into<String>>(mut self, value: S) -> Self {
        self.cert_file = Some(value.into());
        self
    }

    pub fn set_key_file<S: Into<String>>(mut self, value: S) -> Self {
        self.key_file = Some(value.into());
        self
    }

    pub fn set_password<S: Into<String>>(mut self, value: S) -> Self {
        self.password = Some(value.into());
        self
    }

    pub fn set_username<S: Into<String>>(mut self, value: S) -> Self {
        self.username = Some(value.into());
        self
    }
}

impl Into<Command> for RepoAddArg {

    fn into(self) -> Command {
        let mut command = Command::new("helm");

        command.args(&["repo", "add", self.chart.as_str(), self.location.as_str()]);
        command.args(&["--output", "json"]);

        if let Some(force_update) = &self.force_update {
            if *force_update == true {
                command.args(&["--force-update"]);
            }
        }

        if let Some(insecure_skip_tls_verify) = &self.insecure_skip_tls_verify {
            if *insecure_skip_tls_verify == true {
                command.args(&["--insecure-skip-tls-verify"]);
            }
        }

        if let Some(no_update) = &self.no_update {
            if *no_update == true {
                command.args(&["--no-update"]);
            }
        }

        if let Some(ca_file) = &self.ca_file {
            command.args(&["--ca-file", ca_file.as_str()]);
        }

        if let Some(cert_file) = &self.cert_file {
            command.args(&["--cert-file", cert_file.as_str()]);
        }

        if let Some(key_file) = &self.key_file {
            command.args(&["--key-file", key_file.as_str()]);
        }

        if let Some(password) = &self.password {
            command.args(&["--password", password.as_str()]);
        }

        if let Some(username) = &self.username {
            command.args(&["--username", username.as_str()]);
        }
        command
    }

}