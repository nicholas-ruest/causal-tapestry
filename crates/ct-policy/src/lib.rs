//! Fail-closed policy application boundary.
use ct_errors::Result;
use ct_regorus::evaluate;
use ct_types::{PolicyDecision, Proposal};

pub fn authorize(policy:&str, proposal:&Proposal, e_value:f64, threshold:f64)->Result<PolicyDecision>{
    if e_value<threshold {
        return Ok(PolicyDecision{allowed:false,reasons:vec![format!("e-value {e_value:.3} below threshold {threshold:.3}")]});
    }
    evaluate(policy,proposal,e_value)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn threshold_short_circuits(){
        let proposal=Proposal{action:"x".into(),target_service:"s".into(),estimated_blast_radius:1,reversible:true};
        assert!(!authorize("invalid",&proposal,1.0,2.0).unwrap().allowed);
    }
}

