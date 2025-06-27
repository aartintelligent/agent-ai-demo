use std::fmt;

/// Represents the possible states of a chat agent
#[derive(Debug, Clone, PartialEq)]
pub enum AgentState {
    /// Ready to receive input
    Ready,
    /// Processing a user message
    Processing,
    /// Processing messages from the queue
    ProcessingQueue,
    /// Error state when something goes wrong
    Error(String),
}

impl fmt::Display for AgentState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AgentState::Ready => write!(f, "Ready"),
            AgentState::Processing => write!(f, "Processing"),
            AgentState::ProcessingQueue => write!(f, "Processing Queue"),
            AgentState::Error(msg) => write!(f, "Error: {}", msg),
        }
    }
}
