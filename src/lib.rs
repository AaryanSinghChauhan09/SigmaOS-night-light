// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-night-light: night light component model.
//!
//! Real implementation: lights, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Light {
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
pub enum NightLightError {
    EmptyLightName,
    DuplicateLight(String),
    EmptyEventSubject,
}

impl fmt::Display for NightLightError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyLightName => write!(f, "light name must not be empty"),
            Self::DuplicateLight(id) => write!(f, "duplicate light {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for NightLightError {}

pub struct NightLight {
    pub name: String,
    lights: BTreeMap<String, Light>,
    events: BTreeMap<String, Event>,
}

impl NightLight {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            lights: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_light(&mut self, item: Light) -> Result<(), NightLightError> {
        if item.name.trim().is_empty() {
            return Err(NightLightError::EmptyLightName);
        }
        let id = item.id.clone();
        if self.lights.contains_key(&id) {
            return Err(NightLightError::DuplicateLight(id.clone()));
        }
        self.lights.insert(id, item);
        Ok(())
    }

    pub fn light_count(&self) -> usize {
        self.lights.len()
    }

    pub fn get_light(&self, id: &str) -> Option<&Light> {
        self.lights.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), NightLightError> {
        if event.subject.trim().is_empty() {
            return Err(NightLightError::EmptyEventSubject);
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

    fn sample_light(id: &str, name: &str) -> Light {
        Light {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = NightLight::new("NightLight");
        assert_eq!(m.name, "NightLight");
        assert_eq!(m.light_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_light() {
        let mut m = NightLight::new("NightLight");
        m.add_light(sample_light("a1", "Alpha")).unwrap();
        assert_eq!(m.light_count(), 1);
        assert!(m.get_light("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = NightLight::new("NightLight");
        assert!(m.add_light(sample_light("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = NightLight::new("NightLight");
        m.add_light(sample_light("x", "One")).unwrap();
        assert!(m.add_light(sample_light("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = NightLight::new("NightLight");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
