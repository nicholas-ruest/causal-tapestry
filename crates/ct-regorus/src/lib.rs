//! Microsoft Regorus policy engine adapter.
use ct_errors::{Error, Result};
use ct_types::{PolicyDecision, Proposal};
use regorus::{Engine, Value};

pub fn evaluate(policy:&str, proposal:&Proposal, e_value:f64)->Result<PolicyDecision>{
    let mut engine=Engine::new();
    engine.add_policy("remediation.rego".into(),policy.into())
        .map_err(|error|Error::Policy(format!("Regorus add_policy: {error}")))?;
    let input=serde_json::json!({
        "action":proposal.action,
        "target_service":proposal.target_service,
        "estimated_blast_radius":proposal.estimated_blast_radius,
        "reversible":proposal.reversible,
        "e_value":e_value
    });
    engine.set_input(Value::from_json_str(&input.to_string())
        .map_err(|error|Error::Policy(format!("Regorus input: {error}")))?);
    let value=engine.eval_rule("data.causal_tapestry.remediation.allow".into())
        .map_err(|error|Error::Policy(format!("Regorus eval: {error}")))?;
    let allowed=value==Value::Bool(true);
    Ok(PolicyDecision{allowed,reasons:vec![if allowed {
        "proposal accepted by bounded reversible policy".into()
    } else {
        "proposal denied: evidence, reversibility, or blast radius constraint".into()
    }]})
}

#[cfg(test)]
mod tests {
    use super::*;
    const POLICY:&str=include_str!("../../../policies/remediation.rego");
    #[test]
    fn high_evidence_bounded_action_allowed(){
        let proposal=Proposal{action:"isolate".into(),target_service:"api".into(),estimated_blast_radius:1,reversible:true};
        assert!(evaluate(POLICY,&proposal,2.0).unwrap().allowed);
    }
    #[test]
    fn wide_blast_radius_denied(){
        let proposal=Proposal{action:"isolate".into(),target_service:"api".into(),estimated_blast_radius:9,reversible:true};
        assert!(!evaluate(POLICY,&proposal,2.0).unwrap().allowed);
    }
}

