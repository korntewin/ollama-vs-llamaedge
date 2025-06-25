use goose::prelude::*;
use reqwest::{header, Client};
use serde::Serialize;
use std::env;

const LLAMAEDGE_API_BASE_URL: &str = "http://localhost:11401";
const OLLAMA_API_BASE_URL: &str = "http://localhost:11400";
const API_PATH: &str = "/v1/chat/completions";

#[derive(Serialize)]
struct ChatMessage<'a> {
    role: &'a str,
    content: &'a str,
}

#[derive(Serialize)]
struct ChatRequest<'a> {
    model: &'a str,
    messages: Vec<ChatMessage<'a>>,
}

async fn setup_custom_client(user: &mut GooseUser) -> TransactionResult {
    let api_key = env::var("OPENAI_API_KEY").unwrap_or("".to_string());
    let auth_header = format!("Bearer {}", api_key);
    let mut headers = header::HeaderMap::new();
    headers.insert(
        "Authorization",
        header::HeaderValue::from_str(&auth_header).unwrap(),
    );

    let builder = Client::builder()
        .default_headers(headers)
        .gzip(true)
        .cookie_store(true);

    user.set_client_builder(builder).await?;

    Ok(())
}

async fn llamaedge_loadtest_task(user: &mut GooseUser) -> TransactionResult {
    let payload = ChatRequest {
        model: "llama3",
        messages: vec![
            ChatMessage {
                role: "system",
                content: "You are a helpful assistant.",
            },
            ChatMessage {
                role: "user",
                content: "How are you?",
            },
        ],
    };

    let _response = user.post_json(API_PATH, &payload).await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), GooseError> {
    let model = env::var("MODEL").unwrap();

    let scenario = scenario!("loadtest")
        .register_transaction(transaction!(llamaedge_loadtest_task).set_name("chat"))
        .register_transaction(
            transaction!(setup_custom_client)
                .set_name("setup_custom_client")
                .set_on_start(),
        );

    let goose_attack = GooseAttack::initialize()?.register_scenario(match model.as_str() {
        "llamaedge" => scenario.set_host(LLAMAEDGE_API_BASE_URL),
        "ollama" => scenario.set_host(OLLAMA_API_BASE_URL),
        _ => panic!("Invalid model specified, use 'llamaedge' or 'ollama'"),
    });

    goose_attack.execute().await?;

    Ok(())
}
