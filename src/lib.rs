use worker::*;
use serde::{Deserialize, Serialize};

const MAX_DREAM_LENGTH: usize = 5000;

#[derive(Deserialize)]
struct DreamRequest {
    #[serde(rename = "dreamPrompt")]
    dream_prompt: String,
}

#[derive(Serialize)]
struct AnalysisContent {
    response: String,
}

#[derive(Serialize)]
struct DreamResponse {
    analysis: AnalysisContent,
}

#[derive(Serialize)]
struct AiMessage {
    role: String,
    content: String,
}

#[derive(Serialize)]
struct AiChat {
    messages: Vec<AiMessage>,
}

const SYSTEM_PROMPT: &str = "You are a knowledgeable and approachable sleep and dream expert.
                      Analyze dream descriptions and provide insights, but maintain a tone
                      that suggests you're offering possibilities rather than definitive answers.
                      Suggest a few potential reasons for why the dream might have occurred.

                      The response should read just like another human directly responding naturally.

                      This is a one-off response and must not prompt the user to continue the conversation.";

fn get_cors_headers() -> Headers {
    let mut headers = Headers::new();
    headers.set("Access-Control-Allow-Origin", "*").unwrap();
    headers.set("Access-Control-Allow-Methods", "POST, OPTIONS").unwrap();
    headers.set("Access-Control-Allow-Headers", "Content-Type").unwrap();
    headers.set("Access-Control-Max-Age", "86400").unwrap();
    headers
}

#[event(fetch)]
async fn main(mut req: Request, env: Env, _ctx: Context) -> Result<Response> {
    if req.method() == Method::Options {
        return Response::empty()
            .map(|resp| {
                resp.with_headers(get_cors_headers())
            });
    }

    if req.method() != Method::Post {
        return Response::error("Please use POST method", 405)
            .map(|resp| {
                resp.with_headers(get_cors_headers())
            });
    }

    let body = match req.json::<DreamRequest>().await {
        Ok(body) => body,
        Err(e) => {
            return Response::error(format!("Invalid JSON: {}", e), 400)
                .map(|resp| resp.with_headers(get_cors_headers()));
        }
    };

    let dream_prompt = body.dream_prompt.trim();
    if dream_prompt.is_empty() {
        return Response::error("Missing dreamPrompt in request body", 400)
            .map(|resp| resp.with_headers(get_cors_headers()));
    }

    if dream_prompt.len() > MAX_DREAM_LENGTH {
        return Response::error(
            format!("Dream prompt is too long. Maximum length is {} characters", MAX_DREAM_LENGTH),
            400
        ).map(|resp| resp.with_headers(get_cors_headers()));
    }

    let chat = AiChat {
        messages: vec![
            AiMessage {
                role: "system".to_string(),
                content: SYSTEM_PROMPT.to_string(),
            },
            AiMessage {
                role: "user".to_string(),
                content: format!("Analyze this dream: {}", dream_prompt),
            },
        ],
    };

    let ai = match env.ai("AI") {
        Ok(ai) => ai,
        Err(e) => {
            return Response::error(format!("Failed to get AI binding: {}", e), 500)
                .map(|resp| resp.with_headers(get_cors_headers()));
        }
    };

    #[derive(Serialize)]
    struct AiRequest {
        messages: Vec<AiMessage>,
        max_tokens: u32,
    }

    let request = AiRequest {
        messages: chat.messages,
        max_tokens: 1024,
    };

    let ai_response: serde_json::Value = match ai
        .run("@hf/mistral/mistral-7b-instruct-v0.2", &request)
        .await
    {
        Ok(response) => response,
        Err(e) => {
            return Response::error(format!("AI error: {}", e), 500)
                .map(|resp| resp.with_headers(get_cors_headers()));
        }
    };

    let analysis = if let Some(text) = ai_response.as_str() {
        text.to_string()
    } else if let Some(obj) = ai_response.as_object() {
        obj.get("response")
            .or_else(|| obj.get("result"))
            .or_else(|| obj.get("output"))
            .and_then(|v| v.as_str())
            .unwrap_or("Unable to analyze the dream at this time.")
            .to_string()
    } else {
        "Unable to analyze the dream at this time.".to_string()
    };

    Response::from_json(&DreamResponse {
        analysis: AnalysisContent {
            response: analysis
        }
    })
    .map(|resp| {
        resp.with_headers(get_cors_headers())
    })
}
