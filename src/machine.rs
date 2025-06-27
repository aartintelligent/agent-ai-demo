use crate::state::AgentState;
use rig::completion::{Chat, Message, PromptError};
use std::collections::VecDeque;
use tokio::sync::broadcast;
use tracing::{debug, error, info};

/// A state machine for a chat agent that can process messages in a queue
pub struct ChatAgentStateMachine<A: Chat> {
    /// Current state of the agent
    current_state: AgentState,
    /// The underlying agent that handles the chat
    agent: A,
    /// Channel for broadcasting state changes
    state_tx: broadcast::Sender<AgentState>,
    /// Chat history
    history: Vec<Message>,
    /// Queue of messages to process
    queue: VecDeque<String>,
    /// Optional response callback to handle outputs
    response_callback: Option<Box<dyn Fn(String) + Send + Sync>>,
}

impl<A: Chat> ChatAgentStateMachine<A> {
    pub fn new(agent: A) -> Self {
        let (state_tx, _) = broadcast::channel(32);
        let machine = Self {
            current_state: AgentState::Ready,
            agent,
            state_tx,
            history: Vec::new(),
            queue: VecDeque::new(),
            response_callback: None,
        };

        info!("Agent initialized in state: {}", machine.current_state);

        machine
    }

    pub fn set_response_callback<F>(&mut self, callback: F)
    where
        F: Fn(String) + Send + Sync + 'static,
    {
        self.response_callback = Some(Box::new(callback));
    }

    pub async fn process_message(&mut self, message: &str) -> Result<(), PromptError> {
        debug!("Enqueuing message: {}", message);
        self.queue.push_back(message.to_string());

        if self.current_state == AgentState::Ready {
            self.process_queue().await;
        }

        Ok(())
    }

    async fn process_queue(&mut self) {
        self.transition_to(AgentState::ProcessingQueue);

        while let Some(message) = self.queue.pop_front() {
            self.transition_to(AgentState::Processing);

            match self.process_single_message(&message).await {
                Ok(response) => {
                    // Handle the response (e.g., send it to the user)
                    if let Some(callback) = &self.response_callback {
                        callback(response);
                    } else {
                        println!("Response: {}", response);
                    }
                }
                Err(e) => {
                    error!("Error processing message: {}", e);
                    self.transition_to(AgentState::Error(e.to_string()));
                    // Decide whether to continue processing or break
                    // For this example, we'll break on error
                    break;
                }
            }
        }

        // After processing the queue, transition back to Ready
        self.transition_to(AgentState::Ready);
    }

    pub async fn process_single_message(&mut self, message: &str) -> Result<String, PromptError> {
        debug!("Processing message: {}", message);
        self.history.push(Message::user(message));
        match self.agent.chat(message, self.history.clone()).await {
            Ok(response) => {
                self.history.push(Message::assistant(response.clone()));
                debug!("Successfully processed message");
                Ok(response)
            }
            Err(e) => {
                error!("Error processing message: {}", e);
                Err(e)
            }
        }
    }

    pub fn current_state(&self) -> &AgentState {
        &self.current_state
    }

    pub fn subscribe_to_state_changes(&self) -> broadcast::Receiver<AgentState> {
        self.state_tx.subscribe()
    }

    pub fn transition_to(&mut self, new_state: AgentState) {
        debug!("State transition: {} -> {}", self.current_state, new_state);
        self.current_state = new_state.clone();
        let _ = self.state_tx.send(new_state);
    }
}
