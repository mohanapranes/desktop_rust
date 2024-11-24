use serde::{Deserialize, Serialize};

/// Represents Git-related actions and their parameters
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "action")]
pub enum Git {
    /// Git commit action with associated parameters
    Commit {
        folder: String,
        host: String,
        branch: String,
        commit: String,
        files: String,
    },
    /// Git push action with associated parameters
    Push {
        folder: String,
        host: String,
        branch: String,
        remote: String,
    },
}

/// Represents Docker-related actions and their parameters
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "action")]
pub enum Docker {
    /// Docker push action with image name
    Push { image_name: String },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "service")]
pub enum Services {
    /// Git service actions
    Git(Git),
    /// Docker service actions
    Docker(Docker),
}
