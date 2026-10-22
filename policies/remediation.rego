package causal_tapestry.remediation

default allow := false

allow if {
  input.reversible == true
  input.estimated_blast_radius <= 2
  input.e_value >= 1.5
}

reason := "proposal accepted by bounded reversible policy" if allow
reason := "proposal denied: evidence, reversibility, or blast radius constraint" if not allow

