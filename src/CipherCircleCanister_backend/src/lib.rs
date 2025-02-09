use ic_cdk::{
    api::management_canister::http_request::{
        http_request, CanisterHttpRequestArgument, HttpHeader, HttpMethod, HttpResponse, TransformContext, TransformArgs,
    },
    println, api::print,
};

use serde::{Deserialize, Serialize};
use serde_json::{self, json, Value};

#[ic_cdk::update]
async fn ask_ollama(prompt: String) -> String {
    // Set up local Ollama endpoint and headers
    let url = "http://localhost:11434/api/generate";
    let request_headers = vec![
        HttpHeader {
            name: "Content-Type".to_string(),
            value: "application/json".to_string(),
        },
    ];

  
    let payload = json!({
        "model": "llama3.2:latest",
        "prompt": prompt,
        "max_tokens": 500,
        "stream": false
    })
    .to_string();
    
    let request_body = Some(payload.into_bytes());

    let request = CanisterHttpRequestArgument {
        url: url.to_string(),
        max_response_bytes: Some(2_000_000),
        method: HttpMethod::POST,
        headers: request_headers,
        body: request_body,
        transform: None, 
    };

    let cycles: u128 = 2_000_000_000;
    match http_request(request, cycles).await {
        Ok((response,)) => {
            let str_body = String::from_utf8(response.body)
                .unwrap_or_else(|err| format!("Failed to decode response: {:?}", err));
            print(format!("Response from Ollama: {}", str_body));
            str_body
        }
        Err((r, m)) => format!("HTTP request error: {:?}, message: {}", r, m),
    }
}
