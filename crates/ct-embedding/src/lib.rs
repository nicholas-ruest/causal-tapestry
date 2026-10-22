//! Deterministic eight-dimensional incident embeddings.
use ct_types::{Hypothesis, Timeline};

pub const DIMENSION: usize = 8;

pub fn embed(timeline:&Timeline, hypotheses:&[Hypothesis])->Vec<f32>{
    let count=timeline.ordered.len().max(1) as f32;
    let severity=timeline.ordered.iter().map(|event|event.severity as f32).sum::<f32>()/count;
    let errors=timeline.ordered.iter().filter(|event|event.kind.contains("error")).count() as f32/count;
    let latency=timeline.ordered.iter().filter(|event|event.kind.contains("latency")).count() as f32/count;
    let partitions=timeline.ordered.iter().map(|event|&event.partition).collect::<std::collections::BTreeSet<_>>().len() as f32/16.0;
    let services=timeline.ordered.iter().map(|event|&event.service).collect::<std::collections::BTreeSet<_>>().len() as f32/32.0;
    let max_e=hypotheses.iter().map(|h|h.e_value as f32).fold(1.0,f32::max).ln_1p()/10.0;
    let duration=timeline.ordered.last().zip(timeline.ordered.first())
        .map_or(0.0,|(last,first)|(last.corrected_ms-first.corrected_ms).unsigned_abs() as f32/1_000_000.0);
    let linked=timeline.ordered.iter().filter(|event|event.attributes.contains_key("parent_id")).count() as f32/count;
    vec![severity,errors,latency,partitions,services,max_e,duration.min(1.0),linked]
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn vector_has_fixed_dimension() {
        let timeline=Timeline{ordered:vec![],clock_offsets_ms:Default::default()};
        assert_eq!(embed(&timeline,&[]).len(),DIMENSION);
    }
}

