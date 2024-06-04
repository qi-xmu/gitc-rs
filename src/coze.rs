use anyhow::Result as R;
use json::object;
use reqwest::{self, Response};

use crate::config::Config;

// curl --location --request POST 'https://api.coze.cn/open_api/v2/chat' --header 'Authorization: Bearer pat_FufCiN8D7bi0UWTFqcTlvSTG5uqIniAUFcRScauKyTWsSr9KT0pqFqsg4TzG2Xtj' --header 'Content-Type: application/json' --header 'Accept: */*' --header 'Host: api.coze.cn' --header 'Connection: keep-alive' --data-raw '{
//     "conversation_id": "123",
//     "bot_id": "7376178220695650356",
//     "user": "29032201862555",
//     "query": "123456",
//     "stream": false
// }'

async fn request_bot(bot_id: &str, token: &str, query: &str) -> R<Response> {
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

async fn parse_commit_message(resp: Response) -> R<String> {
    let text = resp.text().await.unwrap();
    let obj = json::parse(&text)?;
    let code = obj["code"].to_string();
    if code == "0" {
        let message = obj["messages"][0]["content"].to_string();
        if message.is_empty() {
            return Err(anyhow::anyhow!("Empty message."));
        }
        Ok(message)
    } else {
        Err(anyhow::anyhow!("Empty messages."))
    }
}

pub async fn coze_commit_message(config: &Config, diff: &String) -> R<String> {
    println!("Requesting bot...");
    let resp = request_bot(&config.bot_id, &config.token, diff)
        .await
        .expect("Request bot failed. Please check your config.");

    let message = parse_commit_message(resp)
        .await
        .expect("Parse message failed.");
    Ok(message)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config;

    #[tokio::test]
    async fn test_request_bot() {
        let config = config::read_config().unwrap();
        let resp = request_bot(&config.bot_id, &config.token, "test")
            .await
            .unwrap();

        // println!("{:?}", resp.text().await.unwrap());
        let message = parse_commit_message(resp)
            .await
            .expect("Parse message failed.");
        println!("{:?}", message);
    }
}
