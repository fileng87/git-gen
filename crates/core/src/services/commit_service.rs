use async_trait::async_trait;

use crate::interfaces::llm_generater::LlmGenerater;
use crate::interfaces::git_repository::GitRepository;

#[async_trait]
pub trait CommitService {
    /// Generate and optionally apply commit message
    async fn commit(&self, apply: bool) -> anyhow::Result<String>;
}

pub struct CommitServiceImpl {
    llm_generater: Box<dyn LlmGenerater>,
    git_repository: Box<dyn GitRepository>,
}

impl CommitServiceImpl {
    pub fn new(
        llm_generater: Box<dyn LlmGenerater>,
        git_repository: Box<dyn GitRepository>,
    ) -> Self {
        Self {
            llm_generater,
            git_repository,
        }
    }

    /// Internal helper to generate commit message from staged changes
    async fn generate_commit_message(&self) -> anyhow::Result<String> {
        // 1. Get git diff
        let diff = self.git_repository.get_staged_diff().await?;

        // 2. Generate commit message using LLM
        self.llm_generater.generate_commit_message(diff).await
    }
}

#[async_trait]
impl CommitService for CommitServiceImpl {
    async fn commit(&self, apply: bool) -> anyhow::Result<String> {
        // 1. Generate commit message
        let message = self.generate_commit_message().await?;

        // 2. Optionally apply the commit
        if apply {
            self.git_repository.commit(&message).await?;
        }

        Ok(message)
    }
}
