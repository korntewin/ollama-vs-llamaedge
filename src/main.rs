mod dto;

use crate::dto::{ChatMessage, ChatRequest, ChatResponse};
use goose::prelude::*;
use reqwest::{header, Client};
use std::env;
use std::sync::{LazyLock, Mutex};

static TOTAL_COMPLETION_TOKENS: LazyLock<Mutex<u64>> = LazyLock::new(|| Mutex::new(0));

const LMSTUDIO_API_BASE_URL: &str = "http://localhost:11402";
const LLAMAEDGE_API_BASE_URL: &str = "http://localhost:11401";
const OLLAMA_API_BASE_URL: &str = "http://localhost:11400";
const API_PATH: &str = "/v1/chat/completions";

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

    let goose_response = user.post_json(API_PATH, &payload).await?;

    if let Ok(reqwest_response) = goose_response.response {
        if let Ok(response_json) = reqwest_response.json::<ChatResponse>().await {
            let mut total_tokens = TOTAL_COMPLETION_TOKENS.lock().unwrap();
            *total_tokens += response_json.usage.completion_tokens;
        }
    }

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
        "lmstudio" => scenario.set_host(LMSTUDIO_API_BASE_URL),
        _ => panic!("Invalid model specified, use 'llamaedge' or 'ollama'"),
    });

    let goose_metrics = goose_attack.execute().await?;

    let total_tokens = *TOTAL_COMPLETION_TOKENS.lock().unwrap();
    let total_time_in_secs = goose_metrics.duration as f64;

    if total_time_in_secs > 0.0 {
        let tokens_per_sec = total_tokens as f64 / total_time_in_secs;
        println!();
        println!("##################################################");
        println!("# Summary                                        #");
        println!("##################################################");
        println!("Total completion tokens: {}", total_tokens);
        println!("Total time: {:.2}s", total_time_in_secs);
        println!("Tokens per second: {:.2}", tokens_per_sec);
        println!("##################################################");
    } else {
        println!("Not enough time elapsed to calculate tokens per second.");
    }

    Ok(())
}
