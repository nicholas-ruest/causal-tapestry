//! OpenTelemetry-shaped JSON adapter with strict validation.
use ct_errors::{Error, Result};
use ct_types::Event;
use serde_json::Value;
use std::collections::BTreeMap;

pub fn parse_line(line: &str) -> Result<Event> {
    let value: Value = serde_json::from_str(line)
        .map_err(|error| Error::InvalidInput(format!("invalid JSON: {error}")))?;
    let text = |name: &str| -> Result<String> {
        value
            .get(name)
            .and_then(Value::as_str)
            .filter(|v| !v.is_empty())
            .map(str::to_owned)
            .ok_or_else(|| Error::InvalidInput(format!("missing {name}")))
    };
    let timestamp_ms = value
        .get("timestamp_ms")
        .and_then(Value::as_i64)
        .ok_or_else(|| Error::InvalidInput("missing timestamp_ms".into()))?;
    let severity = value.get("severity").and_then(Value::as_f64).unwrap_or(0.5);
    if !(0.0..=1.0).contains(&severity) {
        return Err(Error::InvalidInput("severity must be within [0,1]".into()));
    }
    let attributes = value
        .get("attributes")
        .and_then(Value::as_object)
        .map(|map| {
            map.iter()
                .filter_map(|(k, v)| v.as_str().map(|s| (k.clone(), s.to_owned())))
                .collect()
        })
        .unwrap_or_else(BTreeMap::new);
    Ok(Event {
        id: text("id")?,
        service: text("service")?,
        partition: text("partition")?,
        timestamp_ms,
        corrected_ms: timestamp_ms,
        kind: text("kind")?,
        severity,
        attributes,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rejects_out_of_range_severity() {
        let input = r#"{"id":"a","service":"api","partition":"p","timestamp_ms":1,"kind":"error","severity":2}"#;
        assert!(parse_line(input).is_err());
    }
}
