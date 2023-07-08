// Hutto, C.J. & Gilbert, E.E. (2014). 
// VADER: A Parsimonious Rule-based Model for Sentiment Analysis of Social Media Text. 
// Eighth International Conference on Weblogs and Social Media (ICWSM-14). Ann Arbor, MI, June 2014.

extern crate vader_sentiment;

pub async fn get_sentiment(content: &str) -> f32 {
    let analyzer = vader_sentiment::SentimentIntensityAnalyzer::new();
    let result = analyzer.polarity_scores(content);

    let pos = result["pos"] as f32;
    let neg = result["neg"] as f32;
    if pos > neg { 
        pos
    } else if neg > pos { 
        -1.0 * neg
    } else {
        0.0
    }
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_negative() {

    }
}