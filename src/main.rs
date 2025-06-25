use goose::prelude::*;
use async_openai::{
    Client,
    config::OpenAIConfig,
    types::{CreateChatCompletionRequest, ChatCompletionRequestMessage, Role},
};
use std::time::Duration;

const OLLAMA_API_BASE_URL: &str = "http://127.0.0.1:9001/v1";
const LLAMAEDGE_API_BASE_URL: &str = "http://127.0.0.1:9002/v1";

async fn ollama_loadtest_task(user: &mut GooseUser) -> GooseTaskResult {
    let config = OpenAIConfig::new().with_api_base(OLLAMA_API_BASE_URL).with_api_key("ollama");
    let client = Client::with_config(config);

    let messages = vec![
        ChatCompletionRequestMessage {
            role: Role::User,
            content: Some("Why is the sky blue?".to_string()),
            ..Default::default()
        }
    ];

    let request = CreateChatCompletionRequest {
        model: "llama3".to_string(),
        messages,
        ..Default::default()
    };

    let _response = client.chat().create(request).await.map_err(|e| {
        eprintln!("Ollama API Error: {}", e);
        Box::new(e)
    })?;

    Ok(())
}

async fn llamaedge_loadtest_task(user: &mut GooseUser) -> GooseTaskResult {
    let config = OpenAIConfig::new().with_api_base(LLAMAEDGE_API_BASE_URL).with_api_key("llamaedge");
    let client = Client::with_config(config);

    let messages = vec![
        ChatCompletionRequestMessage {
            role: Role::User,
            content: Some("Why is the sky blue?".to_string()),
            ..Default::default()
        }
    ];

    let request = CreateChatCompletionRequest {
        model: "llama3".to_string(),
        messages,
        ..Default::default()
    };

    let _response = client.chat().create(request).await.map_err(|e| {
        eprintln!("LlamaEdge API Error: {}", e);
        Box::new(e)
    })?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), GooseError> {
    GooseAttack::initialize()?
        .register_scenario(
            scenario!("ollama")
                .register_transaction(transaction!(ollama_loadtest_task).set_name("ollama_chat"))
        )
        .register_scenario(
            scenario!("llamaedge")
                .register_transaction(transaction!(llamaedge_loadtest_task).set_name("llamaedge_chat"))
        )
        .execute()
        .await?;

    Ok(())
}
