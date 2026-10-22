//! Deterministic normalization from raw corpus to corrected events.
use ct_clock::{apply_offsets, estimate_offsets};
use ct_errors::Result;
use ct_types::{Corpus, Timeline};

pub fn normalize(corpus: &Corpus) -> Result<Timeline> {
    let offsets = estimate_offsets(&corpus.events)?;
    let mut ordered = apply_offsets(&corpus.events, &offsets);
    ordered.sort_by(|a, b| (a.corrected_ms, &a.id).cmp(&(b.corrected_ms, &b.id)));
    Ok(Timeline {
        ordered,
        clock_offsets_ms: offsets,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sort_is_stable_by_id() {
        let event = |id: &str| ct_types::Event {
            id: id.into(),
            service: "s".into(),
            partition: "p".into(),
            timestamp_ms: 1,
            corrected_ms: 1,
            kind: "k".into(),
            severity: 0.0,
            attributes: Default::default(),
        };
        let out = normalize(&Corpus {
            incident_id: "i".into(),
            events: vec![event("b"), event("a")],
        })
        .unwrap();
        assert_eq!(out.ordered[0].id, "a");
    }
}
