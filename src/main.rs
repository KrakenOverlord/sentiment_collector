mod analyzer;
mod database;

use analyzer::Analyzer;
use database::Database;
use dotenv::dotenv;
use log::{error, info};
use nostr_sdk::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    env_logger::init();

    // Print the type of analyzer that will be used
    info!("Using {}", Analyzer::analyzer_type());

    // Setup DB
    let database = Database::new().await?;

    // Get the NOSTR client and start receiving events
    let nostr_client = nostr_client().await?;
    nostr_client.handle_notifications(|notification| async {
        handle_notification(&database, notification).await;
        Ok(())
    })
    .await?;

    Ok(())
}

async fn nostr_client() -> Result<Client> {
    let keys = Keys::generate();
    let opts = Options::new().wait_for_send(false);
    let client = Client::with_opts(&keys, opts);

    client.add_relay("wss://relay.damus.io", None).await?;
    client.add_relay("wss://eden.nostr.land", None).await?;
    client.add_relay("wss://relay.snort.social", None).await?;
    client.add_relay("wss://nostr.orangepill.dev", None).await?;
    client.add_relay("wss://nos.lol", None).await?;
    client.add_relay("wss://nostr.oxtr.dev", None).await?;
    client.add_relay("wss://nostr.mom", None).await?;

    client.connect().await;

    // let metadata = Metadata::new()
    //     .name("nostr-sdk-bot-example")
    //     .display_name("Nostr SDK Bot Example")
    //     .website(Url::parse("https://github.com/rust-nostr/nostr")?);
    // client.set_metadata(metadata).await?;

    let subscription = Filter::new()
        .kind(Kind::TextNote)
        .since(Timestamp::now());

    client.subscribe(vec![subscription]).await;

    Ok(client)
}

// Forwards TextNote events for further processing
async fn handle_notification(database: &Database, notification: RelayPoolNotification) {
    if let RelayPoolNotification::Event(_url, event) = notification {
        if event.kind == Kind::TextNote {
            handle_textnote_event(database, &event).await;
        }
    }
}

// Forwards textnote events with bitcoin content for further processing
async fn handle_textnote_event(database: &Database, event: &Event) {
    if event.content.to_lowercase().contains("bitcoin") {
        handle_bitcoin_event(database, event).await;
    }
}

async fn handle_bitcoin_event(database: &Database, event: &Event) {
    // Print content
    let content = event.content.clone();
    info!("{}", content);

    // Get and print sentiment
    let sentiment = match Analyzer::get_sentiment(&content).await {
        Ok(s) => s,
        Err(e) => {
            error!("{} {}", "Error getting sentiment:", e);
            return;
        },
    };
    if sentiment == 0.0 {
        info!("{}", "Not recording sentiment because it's zero.");
        return;
    } else {
        info!("Sentiment: {}", sentiment);
    }

    let sentiment_event = database::SentimentEvent {
        id:         event.id.to_string(),
        sentiment:  sentiment,
    };

    match database.record_event(&sentiment_event).await {
        Ok(_) => (),
        Err(e) => {
            if e.to_string().contains("Duplicate entry") {
                info!("{}", e);
            } else {
                error!("{}", e);
            }
        },
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_handle_event() {

    }
}