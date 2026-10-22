//! Bounded candidate generation from topology and anomalous evidence.
use ct_topology::downstream;
use ct_types::{Hypothesis, ServiceEdge, Timeline};
use std::collections::BTreeMap;

pub fn generate(timeline:&Timeline, edges:&[ServiceEdge], limit:usize)->Vec<Hypothesis>{
    let mut aggregate:BTreeMap<String,(f64,usize)>=BTreeMap::new();
    for event in &timeline.ordered {
        let entry=aggregate.entry(event.service.clone()).or_default();
        entry.0+=event.severity;
        entry.1+=1;
    }
    let mut ranked:Vec<_>=aggregate.into_iter().map(|(service,(sum,count))|{
        let mean=sum/count as f64;
        let effects=downstream(edges,&service);
        Hypothesis{id:format!("cause:{service}"),cause_service:service.clone(),
            effect_service:effects.first().cloned().unwrap_or(service),
            score:mean,e_value:1.0,stable_environments:0,
            rationale:format!("mean anomaly severity {mean:.3} across {count} observations")}
    }).collect();
    ranked.sort_by(|a,b|b.score.total_cmp(&a.score).then_with(||a.id.cmp(&b.id)));
    ranked.truncate(limit);
    ranked
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn highest_mean_ranks_first() {
        let event=|s:&str,x:f64|ct_types::Event{id:format!("{s}{x}"),service:s.into(),partition:"p".into(),
            timestamp_ms:0,corrected_ms:0,kind:"k".into(),severity:x,attributes:Default::default()};
        let t=Timeline{ordered:vec![event("a",0.1),event("b",0.9)],clock_offsets_ms:Default::default()};
        assert_eq!(generate(&t,&[],2)[0].cause_service,"b");
    }
}

