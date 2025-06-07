use atelier_core::data::Dataset;

pub trait Forecaster {
    // Online forecast: f(X^n): R^n -> R
    fn forecast(features: Vec<f64>) -> f64;
}

#[derive(Debug)]
pub enum Action {
    Forecast,
}

#[derive(Debug)]
pub struct Agent {
    pub agent_id: String,
    pub data: Dataset,
}

impl Agent {
    pub fn new() -> AgentBuilder {
        AgentBuilder::new()
    }
}

pub struct AgentBuilder {
    agent_id: Option<String>,
    data: Option<Dataset>,
}

impl AgentBuilder {
    pub fn new() -> Self {
        AgentBuilder {
            agent_id: None,
            data: None,
        }
    }

    pub fn agent_id(mut self, agent_id: String) -> Self {
        self.agent_id = Some(agent_id);
        self
    }

    pub fn data(mut self, data: Dataset) -> Self {
        self.data = Some(data);
        self
    }

    pub fn build(self) -> Result<Agent, &'static str> {
        let agent_id = self.agent_id.ok_or("Missing agent_id value")?;
        let data = self.data.ok_or("Missing data value")?;

        Ok(Agent { agent_id, data })
    }
}
