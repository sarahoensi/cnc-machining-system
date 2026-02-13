// domain/machining_strategy/finishing/finishing_execution_id.rs

use uuid::Uuid;

/// Unique identifier for a [`FinishingExecution`] aggregate.
///
/// This type wraps a [`Uuid`] to provide strong typing and
/// prevent accidental mixing with other identifiers.
///
/// # Identity
///
/// The identifier is immutable and uniquely represents
/// a finishing execution instance.
///
/// # Construction
///
/// - [`new`](Self::new) generates a random UUID (v4)
/// - [`from_uuid`](Self::from_uuid) allows reconstructing an ID
///   from external sources (e.g. persistence or transport layers)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FinishingExecutionId(pub Uuid);

impl FinishingExecutionId {
    /// Generates a new unique execution identifier.
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    /// Creates an identifier from an existing UUID.
    ///
    /// Typically used when reconstructing domain objects
    /// from persistence or external input.
    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }

    /// Returns the underlying UUID value.
    pub fn value(&self) -> Uuid {
        self.0
    }
}

/// Creates a new random identifier.
///
/// Equivalent to calling [`FinishingExecutionId::new`].
impl Default for FinishingExecutionId {
    fn default() -> Self {
        Self::new()
    }
}
