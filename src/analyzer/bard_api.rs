use anyhow::{Result, bail};
use serde::{Serialize};

// POST https://api.bardapi.dev/chat
pub async fn get_sentiment(content: &str) -> Result<f32> {
    #[derive(Serialize)]
    struct Query {
        input: String,
    }

    // Some data structure.
    let query = Query {
        input: format!("Describe the sentiment in a table with one row and column. {}", content),
    };

    // Serialize it to a JSON string.
    let body = serde_json::to_string(&query)?;

    let response = reqwest::Client::new()
        .post("https://api.bardapi.dev/chat")
        .bearer_auth(api_key()?)
        .header("Content-Type", "text/plain")
        .body(body)
        .send()
        .await?;

    let status = response.status();
    let body = response.text().await?;

    if status != reqwest::StatusCode::OK {
        bail!("Error response: {:?}", body);
    }
    Ok(quantify_sentiment(&body)?)
}

fn quantify_sentiment(sentiment: &str) -> anyhow::Result<f32> {
    let sentiment = sentiment.to_lowercase();
    let negative = sentiment.contains("negative");
    let neutral = sentiment.contains("neutral");
    let positive = sentiment.contains("positive");

    if negative && !neutral && !positive {
        return Ok(-1.0);
    } else if !negative && neutral && !positive {
        return Ok(0.0);
    } else if !negative && !neutral && positive {
        return Ok(1.0);
    } else {
        bail!("Could not quantify response.")
    }
}

fn api_key() ->  Result<String> {
    let var = std::env::var("BARD_API_KEY")?;
    Ok(var)
}

#[cfg(test)]
mod tests {
    use dotenv::dotenv;

    #[tokio::test]
    async fn test_positive_sentiment() {
        dotenv().ok();
        let response = super::get_sentiment("Bitcoin is great.").await.unwrap();
        println!("{:?}", response);
    }

    #[tokio::test]
    async fn test_neutral_sentiment() {
        dotenv().ok();
        let response = super::get_sentiment("Bitcoin is ok.").await.unwrap();
        println!("{:?}", response);
    }

    #[tokio::test]
    async fn test_negative_sentiment() {
        dotenv().ok();
        let response = super::get_sentiment("Bitcoin sucks.").await.unwrap();
        println!("{:?}", response);
    }
}