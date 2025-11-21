use async_trait::async_trait;

#[async_trait]
pub trait LlmGenerater: Send + Sync {
    async fn generate_commit_message(&self, diff: String) -> anyhow::Result<String>;
}
