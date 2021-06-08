use std::process::Command;
use serde::{Serialize, Deserialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GetInstalledArg {
    pub all: Option<bool>,
    pub all_namespaces: Option<bool>,
    pub date: Option<bool>,
    pub pending: Option<bool>,
    pub reverse: Option<bool>,
    pub short: Option<bool>,
    pub superseded: Option<bool>,
    pub uninstalled: Option<bool>,
    pub uninstalling: Option<bool>,

    pub filter: Option<String>,
    pub selector: Option<String>,
    pub time_format: Option<String>,
    pub namespace: Option<String>,

    pub max: Option<u32>,
    pub offset: Option<u32>,
}

impl Into<Command> for GetInstalledArg {
    fn into(self) -> Command {
        let mut command = Command::new("helm");
        command.args(&["list"]);
        command.args(&["--output", "json"]);

        if let Some(all) = &self.all {
            if *all == true {
                command.args(&["--all"]);
            }
        }

        if let Some(all_namespaces) = &self.all_namespaces {
            if *all_namespaces == true {
                command.args(&["--all-namespaces"]);
            }
        }

        if let Some(date) = &self.date {
            if *date == true {
                command.args(&["--date"]);
            }
        }

        if let Some(pending) = &self.pending {
            if *pending == true {
                command.args(&["--pending"]);
            }
        }

        if let Some(reverse) = &self.reverse {
            if *reverse == true {
                command.args(&["--reverse"]);
            }
        }

        if let Some(short) = &self.short {
            if *short == true {
                command.args(&["--short"]);
            }
        }

        if let Some(superseded) = &self.superseded {
            if *superseded == true {
                command.args(&["--superseded"]);
            }
        }

        if let Some(uninstalled) = &self.uninstalled {
            if *uninstalled == true {
                command.args(&["--uninstalled"]);
            }
        }

        if let Some(uninstalling) = &self.uninstalling {
            if *uninstalling == true {
                command.args(&["--uninstalling"]);
            }
        }

        if let Some(filter) = &self.filter {
            command.args(&["--filter", filter.as_str()]);
        }

        if let Some(selector) = &self.selector {
            command.args(&["--selector", selector.as_str()]);
        }

        if let Some(time_format) = &self.time_format {
            command.args(&["--time-format", time_format.as_str()]);
        }

        if let Some(namespace) = &self.namespace {
            command.args(&["--namespace", namespace.as_str()]);
        }

        if let Some(max) = &self.max {
            command.args(&["--max", max.to_string().as_str()]);
        }

        if let Some(offset) = &self.offset {
            command.args(&["--offset", offset.to_string().as_str()]);
        }

        command
    }
}