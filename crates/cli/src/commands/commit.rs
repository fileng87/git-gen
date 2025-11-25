use anyhow::anyhow;
use git_gen_core::{CommitService, CommitServiceImpl, GitRepository, LlmGenerater};
use git_gen_git::GitRepositoryImpl;

use crate::config::AppConfig;
use crate::llm_provider::LlmProvider;

pub async fn run(
    provider: Option<LlmProvider>,
    apply: bool,
    config: AppConfig,
) -> anyhow::Result<()> {
    // Determine LLM provider
    let provider = match provider {
        Some(p) => {
            // Validate that the specified provider is configured
            let provider_name = match p {
                LlmProvider::OpenAI => "openai",
                LlmProvider::Gemini => "gemini",
            };
            if !config.has_provider(provider_name) {
                return Err(anyhow!(
                    "{} is not configured in config file or environment variables",
                    provider_name
                ));
            }
            p
        }
        None => LlmProvider::detect_from_config(&config).ok_or_else(|| {
            anyhow::anyhow!(
                "No LLM provider configured. Please configure a provider in config file or environment variables, or use --llm flag to specify provider."
            )
        })?,
    };

    // Create LLM generater from config
    let llm_generater: Box<dyn LlmGenerater> =
        provider.create_llm_generater_from_config(&config)?;

    // Create git repository
    let git_repository: Box<dyn GitRepository> = Box::new(GitRepositoryImpl::new());

    // Create commit service
    let commit_service = CommitServiceImpl::new(llm_generater, git_repository);

    // Generate and optionally apply commit
    match commit_service.commit(apply).await {
        Ok(message) => {
            println!("{}", message);
            if apply {
                println!("✓ Commit applied successfully");
            } else {
                println!("ℹ Use --apply to commit the changes");
            }
            Ok(())
        }
        Err(e) => Err(anyhow!("Failed to generate commit message: {}", e)),
    }
}
