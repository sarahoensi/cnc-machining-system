//! Infrastructure adapters for finishing workflow persistence.
//!
//! This module provides concrete repository implementations used by the
//! finishing application use cases to store and retrieve execution state.

// infrastructure/finishing/mod.rs
 
 mod in_memory_finishing_execution_repository;
 
 pub use in_memory_finishing_execution_repository::InMemoryFinishingExecutionRepository;
