#[path = "./analyzer/bard_api.rs"]
mod bard_api;

#[path = "./analyzer/openai_api.rs"]
mod openai_api;

#[path = "./analyzer/vadar.rs"]
mod vadar;

enum AnalyzerType {
    Vadar,
    OpenAI,
    Bard,
}

impl AnalyzerType {
    pub fn new() -> Self {
        let analyzer = std::env::var("ANALYZER").expect("Missing ANALYZER environmental variable.");
        match analyzer.as_str() {
            "VADAR" => AnalyzerType::Vadar,
            "OPENAI" => AnalyzerType::OpenAI,
            "BARD" => AnalyzerType::Bard,
            _ => panic!("Invalid ANALYZER environmental variable value"),
        }
    }

    pub fn to_string() -> String {
        let analyzer = std::env::var("ANALYZER").expect("Missing ANALYZER environmental variable.");
        match analyzer.as_str() {
            "VADAR" => String::from("Vadar"),
            "OPENAI" => String::from("OpenAI"),
            "BARD" => String::from("Bard"),
            _ => panic!("Invalid ANALYZER environmental variable value"),
        }
    }
}

pub struct Analyzer {}

impl Analyzer {
    pub fn analyzer_type() -> String {
        AnalyzerType::to_string()
    }

    // Returns the sentiment from an API about the content
    pub async fn get_sentiment(content: &str) -> anyhow::Result<f32> {
        let analyzer = AnalyzerType::new();

        let response = match analyzer {
            AnalyzerType::Vadar => vadar::get_sentiment(content).await,
            AnalyzerType::Bard => bard_api::get_sentiment(content).await?,
            AnalyzerType::OpenAI => openai_api::get_sentiment(content).await?,
        };

        Ok(response)
    }
}
