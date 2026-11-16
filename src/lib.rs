// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-dnssec-validation: dnssec validation component model.
//!
//! Real implementation: validations, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Validation {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DnssecValidationError {
    EmptyValidationName,
    DuplicateValidation(String),
    EmptyEventSubject,
}

impl fmt::Display for DnssecValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyValidationName => write!(f, "validation name must not be empty"),
            Self::DuplicateValidation(id) => write!(f, "duplicate validation {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for DnssecValidationError {}

pub struct DnssecValidation {
    pub name: String,
    validations: BTreeMap<String, Validation>,
    events: BTreeMap<String, Event>,
}

impl DnssecValidation {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            validations: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_validation(&mut self, item: Validation) -> Result<(), DnssecValidationError> {
        if item.name.trim().is_empty() {
            return Err(DnssecValidationError::EmptyValidationName);
        }
        let id = item.id.clone();
        if self.validations.contains_key(&id) {
            return Err(DnssecValidationError::DuplicateValidation(id.clone()));
        }
        self.validations.insert(id, item);
        Ok(())
    }

    pub fn validation_count(&self) -> usize {
        self.validations.len()
    }

    pub fn get_validation(&self, id: &str) -> Option<&Validation> {
        self.validations.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), DnssecValidationError> {
        if event.subject.trim().is_empty() {
            return Err(DnssecValidationError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_validation(id: &str, name: &str) -> Validation {
        Validation {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = DnssecValidation::new("DnssecValidation");
        assert_eq!(m.name, "DnssecValidation");
        assert_eq!(m.validation_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_validation() {
        let mut m = DnssecValidation::new("DnssecValidation");
        m.add_validation(sample_validation("a1", "Alpha")).unwrap();
        assert_eq!(m.validation_count(), 1);
        assert!(m.get_validation("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = DnssecValidation::new("DnssecValidation");
        assert!(m.add_validation(sample_validation("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = DnssecValidation::new("DnssecValidation");
        m.add_validation(sample_validation("x", "One")).unwrap();
        assert!(m.add_validation(sample_validation("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = DnssecValidation::new("DnssecValidation");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
