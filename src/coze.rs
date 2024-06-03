use anyhow::Result as R;
use json::object;
use reqwest::{self, Response};

// curl --location --request POST 'https://api.coze.cn/open_api/v2/chat' --header 'Authorization: Bearer pat_FufCiN8D7bi0UWTFqcTlvSTG5uqIniAUFcRScauKyTWsSr9KT0pqFqsg4TzG2Xtj' --header 'Content-Type: application/json' --header 'Accept: */*' --header 'Host: api.coze.cn' --header 'Connection: keep-alive' --data-raw '{
//     "conversation_id": "123",
//     "bot_id": "7376178220695650356",
//     "user": "29032201862555",
//     "query": "123456",
//     "stream": false
// }'

pub async fn request_bot(bot_id: &str, token: &str, query: &str) -> R<Response> {
    let client = reqwest::Client::new();

    let payload = object! {
        "conversation_id": "123",
        "bot_id": bot_id,
        "user": "29032201862555",
        "query": query,
        "stream": false
    };

    let resp = client
        .post("https://api.coze.cn/open_api/v2/chat")
        .header("Authorization", format!("Bearer {}", token))
        .header("Content-Type", "application/json")
        .header("Accept", "*/*")
        .header("Host", "api.coze.cn")
        .header("Connection", "keep-alive")
        .body(payload.to_string())
        .send()
        .await?;

    Ok(resp)
}

pub async fn parse_commit_message(resp: Response) -> R<String> {
    let text = resp.text().await.unwrap();
    let obj = json::parse(&text)?;

    let message = obj["messages"][0]["content"].to_string();

    Ok(message)
}
