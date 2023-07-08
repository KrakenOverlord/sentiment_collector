use anyhow::{Result, ensure, Context, bail};
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ApiResponse {
    pub data:    Option<Vec<Model>>,
    pub choices: Option<Vec<Choice>>,
    pub text:    Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Model {
    pub id: String,
}

#[derive(Debug, Deserialize)]
pub struct Choice {
    pub message: Message,
}

#[derive(Debug, Deserialize)]
pub struct Message {
    pub content: String,
}

// POST https://api.openai.com/v1/chat/completions
pub async fn get_sentiment(content: &str) -> Result<f32> {
    let body = serde_json::json!(
        {
            "model":"gpt-3.5-turbo",
            "messages":[
                {
                    "role": "user",
                    "content": format!("Describe the sentiment in a table with one row and column. {}", content),
                }
            ],
            "temperature": 0.7,
        }
    );

    let response = reqwest::Client::new()
        .post("https://api.openai.com/v1/chat/completions")
        .bearer_auth(api_key()?)
        .json(&body)   
        .send()
        .await?;

    ensure!(response.status() == reqwest::StatusCode::OK, response.text().await?);
    let response: ApiResponse = response.json().await?;
    let choices = response.choices.context("Error retrieving choices from OpenAI response.")?;
    let choice = choices.first().context("Error retrieving choice from OpenAI response.")?;

    Ok(quantify_sentiment(&choice.message.content)?)
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
    let var = std::env::var("OPENAI_API_KEY")?;
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
}