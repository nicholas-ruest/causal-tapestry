//! Partial-order timeline construction with parent references.
use ct_errors::{Error, Result};
use ct_types::Timeline;
use std::collections::{BTreeMap, BTreeSet};

pub fn causal_order(timeline: &Timeline) -> Result<Vec<String>> {
    let ids: BTreeSet<_> = timeline
        .ordered
        .iter()
        .map(|event| event.id.clone())
        .collect();
    let mut children: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut degree: BTreeMap<String, usize> = ids.iter().cloned().map(|id| (id, 0)).collect();
    for event in &timeline.ordered {
        if let Some(parent) = event.attributes.get("parent_id") {
            if ids.contains(parent) {
                children
                    .entry(parent.clone())
                    .or_default()
                    .push(event.id.clone());
                *degree.get_mut(&event.id).expect("event id was indexed") += 1;
            }
        }
    }
    let mut ready: BTreeSet<String> = degree
        .iter()
        .filter(|(_, d)| **d == 0)
        .map(|(id, _)| id.clone())
        .collect();
    let mut ordered = Vec::with_capacity(ids.len());
    while let Some(id) = ready.pop_first() {
        ordered.push(id.clone());
        for child in children.get(&id).into_iter().flatten() {
            let entry = degree.get_mut(child).expect("child id was indexed");
            *entry -= 1;
            if *entry == 0 {
                ready.insert(child.clone());
            }
        }
    }
    if ordered.len() != ids.len() {
        return Err(Error::InvalidInput("event parent cycle detected".into()));
    }
    Ok(ordered)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rejects_cycle() {
        let event = |id: &str, parent: &str| ct_types::Event {
            id: id.into(),
            service: "s".into(),
            partition: "p".into(),
            timestamp_ms: 1,
            corrected_ms: 1,
            kind: "k".into(),
            severity: 0.0,
            attributes: [("parent_id".into(), parent.into())].into(),
        };
        let t = Timeline {
            ordered: vec![event("a", "b"), event("b", "a")],
            clock_offsets_ms: Default::default(),
        };
        assert!(causal_order(&t).is_err());
    }
}
