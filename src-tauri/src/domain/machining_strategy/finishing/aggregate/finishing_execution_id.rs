// domain/machining_strategy/finishing/finishing_execution_id.rs

use uuid::Uuid;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FinishingExecutionId(Uuid);

impl FinishingExecutionId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn value(&self) -> Uuid {
        self.0
    }
}

