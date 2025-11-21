use async_trait::async_trait;

/// Interface for git repository operations
#[async_trait]
pub trait GitRepository: Send + Sync {
    /// Get the diff of staged changes
    async fn get_staged_diff(&self) -> anyhow::Result<String>;

    /// Execute git commit with the given message
    async fn commit(&self, message: &str) -> anyhow::Result<()>;
}
