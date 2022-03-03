use std::collections::HashMap;
use crate::glob::GlobPatternSet;
use crate::envoy::RLA_action_specifier;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Operation {
    pub paths: GlobPatternSet,
    pub hosts: GlobPatternSet,
    pub methods: GlobPatternSet,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Rule {
    pub opertion: Operation,
    pub actions: Vec<RLA_action_specifier>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct RateLimitPolicy {
    rules: Vec<Rule>,
    global_actions: Vec<RLA_action_specifier>,
    upstream_cluster: String,
    domain: String,
}

impl RateLimitPolicy {
    pub fn rules(&self) -> &[Rule] {
        self.rules.as_ref()
    }

    pub fn global_actions(&self) -> &[RLA_action_specifier] {
        &self.global_actions
    }

    pub fn upstream_cluster(&self) -> &str {
        &self.upstream_cluster
    }

    pub fn domain(&self) -> &str {
        &self.domain
    }
}

// TODO(rahulanand16nov): We can convert the structure of config in such a way
// that it's optimized for lookup in the runtime. For e.g., keying on virtualhost
// to sort through ratelimitpolicies and then further operations.

#[derive(Deserialize, Debug, Clone)]
pub struct FilterConfig {
    ratelimitpolicies: HashMap<String, RateLimitPolicy>,
    // Deny request when faced with an irrecoverable failure.
    failure_mode_deny: bool,
}

impl FilterConfig {
    pub fn new() -> Self {
        FilterConfig {
            ratelimitpolicies: HashMap::new(), 
            failure_mode_deny: true, 
        }
    }

    pub fn ratelimitpolicies(&self) -> &HashMap<String, RateLimitPolicy> {
        &self.ratelimitpolicies
    }

    pub fn failure_mode_deny(&self) -> bool {
        self.failure_mode_deny
    }
}
