use std::any::{Any, TypeId};
use std::collections::HashMap;

use pitha_core::CoreResult;

use crate::error::FoundationError;

/// Type-safe state registry for application state.
pub struct StateRegistry {
    states: HashMap<TypeId, Box<dyn Any>>,
}

impl StateRegistry {
    pub fn new() -> Self {
        Self {
            states: HashMap::new(),
        }
    }

    /// Insert a value into the registry.
    pub fn insert<T: 'static>(&mut self, value: T) {
        self.states.insert(TypeId::of::<T>(), Box::new(value));
    }

    /// Get a reference to a value from the registry.
    pub fn get<T: 'static>(&self) -> CoreResult<&T> {
        self.states
            .get(&TypeId::of::<T>())
            .and_then(|boxed| boxed.downcast_ref::<T>())
            .ok_or_else(|| {
                FoundationError::state(format!(
                    "state not found for type {}",
                    std::any::type_name::<T>()
                ))
                .into()
            })
    }

    /// Get a mutable reference to a value from the registry.
    pub fn get_mut<T: 'static>(&mut self) -> CoreResult<&mut T> {
        self.states
            .get_mut(&TypeId::of::<T>())
            .and_then(|boxed| boxed.downcast_mut::<T>())
            .ok_or_else(|| {
                FoundationError::state(format!(
                    "state not found for type {}",
                    std::any::type_name::<T>()
                ))
                .into()
            })
    }

    /// Remove a value from the registry.
    pub fn remove<T: 'static>(&mut self) -> Option<T> {
        self.states
            .remove(&TypeId::of::<T>())
            .and_then(|boxed| boxed.downcast::<T>().ok())
            .map(|boxed| *boxed)
    }

    /// Check if a value exists in the registry.
    pub fn contains<T: 'static>(&self) -> bool {
        self.states.contains_key(&TypeId::of::<T>())
    }
}

impl Default for StateRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_registry_insert_get() {
        let mut registry = StateRegistry::new();
        registry.insert(42u32);
        assert_eq!(*registry.get::<u32>().unwrap(), 42);
    }

    #[test]
    fn test_state_registry_get_mut() {
        let mut registry = StateRegistry::new();
        registry.insert(String::from("hello"));
        let value = registry.get_mut::<String>().unwrap();
        value.push_str(" world");
        assert_eq!(registry.get::<String>().unwrap(), "hello world");
    }

    #[test]
    fn test_state_registry_remove() {
        let mut registry = StateRegistry::new();
        registry.insert(42u32);
        let removed = registry.remove::<u32>();
        assert_eq!(removed, Some(42));
        assert!(!registry.contains::<u32>());
    }

    #[test]
    fn test_state_registry_not_found() {
        let registry = StateRegistry::new();
        assert!(registry.get::<u32>().is_err());
    }
}
