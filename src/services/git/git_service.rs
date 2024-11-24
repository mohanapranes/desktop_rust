use crate::services::models::models::Git;
use std::error::Error;

/// Handles Git-related actions
///
/// # Arguments
/// * `action` - Git action to be performed with its parameters
pub fn handle_git_action(action: Git) -> Result<(), Box<dyn Error>> {
    match action {
        Git::Commit {
            folder,
            host,
            branch,
            commit: _,
            files: _,
        } => {
            println!(
                "Executing Git commit in folder '{}' on branch '{}' hosted at '{}'",
                folder, branch, host
            );
            Ok(())
        }
        Git::Push {
            folder,
            host,
            branch,
            remote,
        } => {
            println!(
                "Executing Git push in folder '{}' on branch '{}' to remote '{}' hosted at '{}'",
                folder, branch, remote, host
            );
            Ok(())
        }
    }
}
