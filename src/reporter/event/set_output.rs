use crate::manifest::bare_version::BareVersion;
use crate::reporter::event::Message;
use crate::Event;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub struct SetOutputMessage {
    version: BareVersion,
    manifest_path: PathBuf,
}

impl SetOutputMessage {
    pub fn new(version: impl Into<BareVersion>, manifest_path: PathBuf) -> Self {
        Self {
            version: version.into(),
            manifest_path,
        }
    }

    pub fn version(&self) -> &BareVersion {
        &self.version
    }

    pub fn manifest_path(&self) -> &Path {
        &self.manifest_path
    }
}

impl From<SetOutputMessage> for Event {
    fn from(it: SetOutputMessage) -> Self {
        Message::SetOutput(it).into()
    }
}
