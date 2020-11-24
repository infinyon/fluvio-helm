use std::io::Error as IoError;
use std::string::FromUtf8Error;

#[derive(thiserror::Error, Debug)]
pub enum HelmError {
    #[error(
    r#"Unable to find 'helm' executable
  Please make sure helm is installed and in your PATH.
  See https://helm.sh/docs/intro/install/ for more help"#
    )]
    HelmNotInstalled(IoError),
    #[error("Failed to read helm client version: {0}")]
    HelmVersionNotFound(String),
    #[error("Failed to connect to Kubernetes")]
    FailedToConnect,
    #[error("Failed to parse helm output as UTF8")]
    Utf8Error(#[from] FromUtf8Error),
    #[error("Failed to parse JSON from helm output")]
    Serde(#[from] serde_json::Error),
}
