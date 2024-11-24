use crate::services::{git::git_service, models::models::Services};
use std::error::Error;

/// Parses JSON input and routes to appropriate service handler
///
/// # Arguments
/// * `json_data` - JSON string containing service configuration
pub fn parse_json(json_data: &str) -> Result<(), Box<dyn Error>> {
    match serde_json::from_str::<Services>(json_data) {
        Ok(Services::Git(git_action)) => {
            git_service::handle_git_action(git_action)?;
            Ok(())
        }
        Ok(Services::Docker(_)) => {
            println!("Docker service action received");
            Ok(())
        }
        Err(e) => Err(Box::new(e)),
    }
}
