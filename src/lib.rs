// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-rt-scheduler: rt scheduler component model.
//!
//! Real implementation: schedulers, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Scheduler {
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
pub enum RtSchedulerError {
    EmptySchedulerName,
    DuplicateScheduler(String),
    EmptyEventSubject,
}

impl fmt::Display for RtSchedulerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptySchedulerName => write!(f, "scheduler name must not be empty"),
            Self::DuplicateScheduler(id) => write!(f, "duplicate scheduler {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for RtSchedulerError {}

pub struct RtScheduler {
    pub name: String,
    schedulers: BTreeMap<String, Scheduler>,
    events: BTreeMap<String, Event>,
}

impl RtScheduler {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            schedulers: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_scheduler(&mut self, item: Scheduler) -> Result<(), RtSchedulerError> {
        if item.name.trim().is_empty() {
            return Err(RtSchedulerError::EmptySchedulerName);
        }
        let id = item.id.clone();
        if self.schedulers.contains_key(&id) {
            return Err(RtSchedulerError::DuplicateScheduler(id.clone()));
        }
        self.schedulers.insert(id, item);
        Ok(())
    }

    pub fn scheduler_count(&self) -> usize {
        self.schedulers.len()
    }

    pub fn get_scheduler(&self, id: &str) -> Option<&Scheduler> {
        self.schedulers.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), RtSchedulerError> {
        if event.subject.trim().is_empty() {
            return Err(RtSchedulerError::EmptyEventSubject);
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

    fn sample_scheduler(id: &str, name: &str) -> Scheduler {
        Scheduler {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = RtScheduler::new("RtScheduler");
        assert_eq!(m.name, "RtScheduler");
        assert_eq!(m.scheduler_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_scheduler() {
        let mut m = RtScheduler::new("RtScheduler");
        m.add_scheduler(sample_scheduler("a1", "Alpha")).unwrap();
        assert_eq!(m.scheduler_count(), 1);
        assert!(m.get_scheduler("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = RtScheduler::new("RtScheduler");
        assert!(m.add_scheduler(sample_scheduler("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = RtScheduler::new("RtScheduler");
        m.add_scheduler(sample_scheduler("x", "One")).unwrap();
        assert!(m.add_scheduler(sample_scheduler("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = RtScheduler::new("RtScheduler");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
