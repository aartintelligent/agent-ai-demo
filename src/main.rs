mod machine;
mod state;

use machine::ChatAgentStateMachine;
use quick_xml::de::from_str;
use reqwest;
use rig::client::CompletionClient;
use rig::completion::ToolDefinition;
use rig::providers::ollama;
use rig::tool::Tool;
use serde::{Deserialize, Serialize};
use serde_json::json;
use state::AgentState;
use std::time::Duration;

#[derive(Debug, Deserialize)]
struct SearchArgs {
    query: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ArxivResult {
    title: String,
    summary: String,
    link: String,
}

#[derive(Debug, Deserialize)]
struct ArxivApiResponse {
    #[serde(rename = "entry")]
    entries: Vec<Entry>,
}

#[derive(Debug, Deserialize)]
struct Entry {
    title: String,
    summary: String,
    id: String,
}

#[derive(Debug, thiserror::Error)]
#[error("Search error: {0}")]
struct SearchError(String);

#[derive(Clone)]
struct ArxivSearch {
    client: reqwest::Client,
}

impl ArxivSearch {
    fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    async fn search(&self, query: &str) -> Result<Vec<ArxivResult>, SearchError> {
        let url = format!(
            "http://export.arxiv.org/api/query?search_query=all:{}&start=0&max_results=5",
            urlencoding::encode(query)
        );

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| SearchError(e.to_string()))?;
        let response_text = response
            .text()
            .await
            .map_err(|e| SearchError(e.to_string()))?;

        let response_json: Result<ArxivApiResponse, _> = from_str(&response_text);

        match response_json {
            Ok(response_json) => {
                let results = response_json
                    .entries
                    .into_iter()
                    .map(|entry| ArxivResult {
                        title: entry.title,
                        summary: entry.summary,
                        link: entry.id,
                    })
                    .collect();
                Ok(results)
            }
            Err(_) => Err(SearchError(
                "Failed to parse the response. The structure might have unexpected namespaces or formats."
                    .to_string(),
            )),
        }
    }
}

impl Tool for ArxivSearch {
    const NAME: &'static str = "arxiv_search";
    type Error = SearchError;
    type Args = SearchArgs;
    type Output = Vec<ArxivResult>;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Search for academic papers on arXiv.".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "query": {
                        "type": "string",
                        "description": "The search query to look for papers on arXiv"
                    }
                },
                "required": ["query"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        self.search(&args.query).await
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Research Assistant State Machine Demo ===\n");
    dotenvy::dotenv().ok();
    let ollama_host = std::env::var("OLLAMA_HOST").expect("OLLAMA_HOST not found");
    let ollama_client = ollama::Client::from_url(&ollama_host);
    let arxiv_search_tool = ArxivSearch::new();
    let agent = ollama_client
        .agent("mistral-small3.2")
        .preamble(
            "You are a helpful assistant with academic search capabilities using arXiv. \
            When provided with information about a paper, you summarize the main points \
            and present a concise summary of the key information. You speech frensh.",
        )
        .build();
    let mut state_machine = ChatAgentStateMachine::new(agent);
    state_machine.set_response_callback(|response| {
        println!("\n🤖 Assistant:");
        println!("\n{}", response);
    });
    let mut state_rx = state_machine.subscribe_to_state_changes();
    tokio::spawn(async move {
        while let Ok(state) = state_rx.recv().await {
            println!("\n📍 State: {}", state);
        }
    });
    let query = "llm agents";
    println!("🔍 Searching arXiv for '{}'", query);
    let results = arxiv_search_tool.search(query).await?;
    for (index, result) in results.iter().enumerate() {
        println!("\nProcessing result {}...", index + 1);
        let message = format!(
            "Veuillez résumer le document suivant:\nTitre: {}\nRésumé: {}\nLien: {}",
            result.title, result.summary, result.link
        );
        state_machine.process_message(&message).await?;
        while state_machine.current_state() != &AgentState::Ready {
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
        tokio::time::sleep(Duration::from_millis(500)).await;
    }
    println!("\n=== Demo Complete ===");
    Ok(())
}
