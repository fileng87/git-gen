use async_trait::async_trait;
use git_gen_core::GitRepository;
use std::path::{Path, PathBuf};
use std::process::Command;
use anyhow::{anyhow, Context};

/// Implementation of GitRepository using system git command
pub struct GitRepositoryImpl {
    repo_path: PathBuf,
}

impl Default for GitRepositoryImpl {
    fn default() -> Self {
        Self::new()
    }
}

impl GitRepositoryImpl {
    /// Create a new GitRepositoryImpl for the current directory
    pub fn new() -> Self {
        Self {
            repo_path: PathBuf::from("."),
        }
    }

    /// Create a new GitRepositoryImpl for a specific path
    pub fn with_path<P: AsRef<Path>>(path: P) -> Self {
        Self {
            repo_path: path.as_ref().to_path_buf(),
        }
    }
}

#[async_trait]
impl GitRepository for GitRepositoryImpl {
    async fn get_staged_diff(&self) -> anyhow::Result<String> {
        let repo_path = self.repo_path.clone();

        // Run in blocking thread pool since Command is synchronous
        tokio::task::spawn_blocking(move || {
            // Get staged diff using git diff --cached
            let output = Command::new("git")
                .args(&["diff", "--cached"])
                .current_dir(&repo_path)
                .output()
                .context("Failed to execute git diff. Make sure git is installed and in PATH")?;

            // git diff returns exit code 0 even when there are no changes
            // We need to check if the output is empty
            let diff_text = String::from_utf8_lossy(&output.stdout).to_string();

            if diff_text.trim().is_empty() {
                return Err(anyhow!("No staged changes found"));
            }

            // Check for error output
            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                return Err(anyhow!("Git diff failed: {}", stderr));
            }

            Ok(diff_text)
        })
        .await
        .map_err(|e| anyhow!("Task join error: {}", e))?
    }

    async fn commit(&self, message: &str) -> anyhow::Result<()> {
        let repo_path = self.repo_path.clone();
        let message = message.to_string();

        // Run in blocking thread pool since Command is synchronous
        tokio::task::spawn_blocking(move || {
            // First check if there are staged changes
            // git diff --cached --quiet returns exit code 0 if no changes, 1 if there are changes
            let diff_output = Command::new("git")
                .args(&["diff", "--cached", "--quiet"])
                .current_dir(&repo_path)
                .output()
                .context("Failed to execute git diff. Make sure git is installed and in PATH")?;

            // Exit code 0 means no staged changes
            if diff_output.status.code() == Some(0) {
                return Err(anyhow!("No staged changes to commit"));
            }

            // Check git user config before committing
            let name_output = Command::new("git")
                .args(&["config", "user.name"])
                .current_dir(&repo_path)
                .output()
                .context("Failed to check git user.name")?;

            if !name_output.status.success() || name_output.stdout.is_empty() {
                return Err(anyhow!("Git user.name is not configured. Please run: git config user.name \"Your Name\""));
            }

            let email_output = Command::new("git")
                .args(&["config", "user.email"])
                .current_dir(&repo_path)
                .output()
                .context("Failed to check git user.email")?;

            if !email_output.status.success() || email_output.stdout.is_empty() {
                return Err(anyhow!("Git user.email is not configured. Please run: git config user.email \"your.email@example.com\""));
            }

            // Execute git commit
            let output = Command::new("git")
                .args(&["commit", "-m", &message])
                .current_dir(&repo_path)
                .output()
                .context("Failed to execute git commit. Make sure git is installed and in PATH")?;

            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                return Err(anyhow!("Git commit failed: {}", stderr));
            }

            Ok(())
        })
        .await
        .map_err(|e| anyhow!("Task join error: {}", e))?
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_git_repository_creation() {
        let _repo = GitRepositoryImpl::new();
        // Just test that it can be created
        assert!(true);
    }
}
