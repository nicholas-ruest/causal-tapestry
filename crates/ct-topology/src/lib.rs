//! Service topology derived from span parent relationships.
use ct_types::{ServiceEdge, Timeline};
use std::collections::BTreeMap;

pub fn extract(timeline: &Timeline) -> Vec<ServiceEdge> {
    let owner: BTreeMap<_, _> = timeline
        .ordered
        .iter()
        .map(|event| (event.id.as_str(), event.service.as_str()))
        .collect();
    let mut weights: BTreeMap<(String, String), u32> = BTreeMap::new();
    for event in &timeline.ordered {
        if let Some(parent) = event.attributes.get("parent_id") {
            if let Some(caller) = owner.get(parent.as_str()) {
                if *caller != event.service {
                    *weights
                        .entry(((*caller).to_owned(), event.service.clone()))
                        .or_default() += 1;
                }
            }
        }
    }
    weights
        .into_iter()
        .map(|((caller, callee), weight)| ServiceEdge {
            caller,
            callee,
            weight,
        })
        .collect()
}

pub fn downstream(edges: &[ServiceEdge], service: &str) -> Vec<String> {
    edges
        .iter()
        .filter(|edge| edge.caller == service)
        .map(|edge| edge.callee.clone())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn edges_are_aggregated() {
        let event = |id: &str, s: &str, p: Option<&str>| ct_types::Event {
            id: id.into(),
            service: s.into(),
            partition: "p".into(),
            timestamp_ms: 0,
            corrected_ms: 0,
            kind: "k".into(),
            severity: 0.0,
            attributes: p
                .map(|v| [("parent_id".into(), v.into())].into())
                .unwrap_or_default(),
        };
        let t = Timeline {
            ordered: vec![event("a", "api", None), event("b", "db", Some("a"))],
            clock_offsets_ms: Default::default(),
        };
        assert_eq!(extract(&t)[0].callee, "db");
    }
}
