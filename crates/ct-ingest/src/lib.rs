//! Corpus acquisition with duplicate and capacity defenses.
use ct_errors::{Error, Result};
use ct_otel::parse_line;
use ct_types::Corpus;
use std::collections::HashSet;

pub fn ingest_json_lines(incident_id: &str, input: &str, max_events: usize) -> Result<Corpus> {
    if incident_id.trim().is_empty() {
        return Err(Error::InvalidInput("incident_id is empty".into()));
    }
    let mut events = Vec::new();
    let mut ids = HashSet::new();
    for line in input.lines().filter(|line| !line.trim().is_empty()) {
        if events.len() == max_events {
            return Err(Error::InvalidInput(format!("event ceiling {max_events} exceeded")));
        }
        let event = parse_line(line)?;
        if !ids.insert(event.id.clone()) {
            return Err(Error::InvalidInput(format!("duplicate event id {}", event.id)));
        }
        events.push(event);
    }
    if events.is_empty() {
        return Err(Error::InvalidInput("corpus has no events".into()));
    }
    Ok(Corpus { incident_id: incident_id.to_owned(), events })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn duplicate_ids_fail_closed() {
        let line = r#"{"id":"a","service":"api","partition":"p","timestamp_ms":1,"kind":"error","severity":1}"#;
        assert!(ingest_json_lines("i", &format!("{line}\n{line}"), 3).is_err());
    }
}

