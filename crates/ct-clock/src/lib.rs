//! Robust per-partition clock correction.
use ct_errors::{Error, Result};
use ct_types::Event;
use std::collections::BTreeMap;

pub fn estimate_offsets(events: &[Event]) -> Result<BTreeMap<String, i64>> {
    if events.is_empty() {
        return Err(Error::InvalidInput("cannot estimate clocks without events".into()));
    }
    let mut samples: BTreeMap<String, Vec<i64>> = BTreeMap::new();
    for event in events {
        let offset = event.attributes.get("clock_offset_ms")
            .and_then(|value| value.parse::<i64>().ok()).unwrap_or(0);
        samples.entry(event.partition.clone()).or_default().push(offset);
    }
    Ok(samples.into_iter().map(|(partition, mut values)| {
        values.sort_unstable();
        (partition, values[values.len() / 2])
    }).collect())
}

pub fn apply_offsets(events: &[Event], offsets: &BTreeMap<String, i64>) -> Vec<Event> {
    events.iter().cloned().map(|mut event| {
        event.corrected_ms = event.timestamp_ms.saturating_sub(*offsets.get(&event.partition).unwrap_or(&0));
        event
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn median_resists_one_outlier() {
        let make = |id: &str, value: &str| Event { id:id.into(), service:"s".into(), partition:"p".into(),
            timestamp_ms:0, corrected_ms:0, kind:"k".into(), severity:0.0,
            attributes:[("clock_offset_ms".into(),value.into())].into() };
        let got=estimate_offsets(&[make("1","10"),make("2","11"),make("3","999")]).unwrap();
        assert_eq!(got["p"],11);
    }
}

