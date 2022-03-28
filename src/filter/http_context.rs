use crate::configuration::FilterConfig;
use crate::envoy::{
    RLA_action_specifier, RateLimitRequest, RateLimitResponse, RateLimitResponse_Code,
};
use crate::utils::{descriptor_from_actions, request_process_failure, UtilsErr};
use log::{debug, info, warn};
use protobuf::Message;
use proxy_wasm::traits::{Context, HttpContext};
use proxy_wasm::types::Action;
use std::time::Duration;

const RATELIMIT_SERVICE_NAME: &str = "envoy.service.ratelimit.v3.RateLimitService";
const RATELIMIT_METHOD_NAME: &str = "ShouldRateLimit";

// we cannot determine using Istio's naming scheme because this cluster is patched
// by the kuadrant-controller and should match whatever value is patched.
const DEFAULT_UPSTREAM_CLUSTER: &str = "rate-limit-cluster";

struct RequestInfo {
    pub host: String,
    pub path: String,
    pub method: String,
}

pub struct Filter {
    pub context_id: u32,
    pub config: FilterConfig,
}

impl Filter {
    fn config(&self) -> &FilterConfig {
        &self.config
    }

    fn context_id(&self) -> u32 {
        self.context_id
    }

    fn fetch_request_info(&self) -> RequestInfo {
        // TODO(rahulanand16nov): Handle error
        let host_bytes = self.get_property(vec!["request", "host"]).unwrap();
        let mut host = String::from_utf8(host_bytes).unwrap();

        // make sure port is removed from host before processing the request.
        let split_host = host.split(':').collect::<Vec<_>>();
        host = split_host[0].to_owned();

        let path_bytes = self.get_property(vec!["request", "path"]).unwrap();
        let path = String::from_utf8(path_bytes).unwrap();

        let method_bytes = self.get_property(vec!["request", "method"]).unwrap();
        let method = String::from_utf8(method_bytes).unwrap();

        RequestInfo { host, path, method }
    }

    fn create_ratelimit_request(
        &self,
        domain: &str,
        actions: &[RLA_action_specifier],
    ) -> Result<RateLimitRequest, UtilsErr> {
        let mut rl_req = RateLimitRequest::new();

        rl_req.set_domain(domain.into());

        rl_req.set_hits_addend(1);

        rl_req
            .mut_descriptors()
            .push(descriptor_from_actions(self, actions)?);

        Ok(rl_req)
    }
}

impl HttpContext for Filter {
    fn on_http_request_headers(&mut self, _: usize) -> Action {
        let context_id = self.context_id();
        info!("context #{}: on_http_request_headers called", context_id);

        let req_info = self.fetch_request_info();
        let mut actions: Vec<RLA_action_specifier> = Vec::new();
        let mut upstream_cluster = "";
        let mut domain = "";

        for (rlp_name, rlp) in self.config().ratelimitpolicies() {
            if !rlp.hosts().is_match(&req_info.host) {
                continue;
            }
            info!(
                "context #{}: match found in {} RateLimitPolicy",
                context_id, rlp_name
            );

            if let Some(rules) = rlp.rules() {
                for rule in rules {
                    let matched_operation = rule
                        .operations()
                        .map(|operations| {
                            operations.iter().find(|operation| {
                                operation.paths.is_match(&req_info.path)
                                    && operation.methods.is_match(&req_info.method)
                            })
                        })
                        .flatten();

                    // Without the operation match, actions won't be included.
                    if matched_operation.is_none() {
                        continue;
                    }

                    debug!(
                        "context #{}: matched operation: {:?}",
                        context_id, matched_operation
                    );

                    if let Some(ref matched_actions) = rule.actions() {
                        actions.append(&mut matched_actions.to_vec());
                    }
                    break; // only first match is allowed.
                }
            }

            if let Some(global_actions) = rlp.global_actions() {
                actions.append(&mut global_actions.to_vec());
            }
            upstream_cluster = rlp.upstream_cluster().unwrap_or(DEFAULT_UPSTREAM_CLUSTER);
            domain = rlp.domain().unwrap_or(rlp_name);
            break;
        }

        if actions.is_empty() {
            info!(
                "context #{}: Allowing request to pass because zero descriptors generated",
                context_id
            );
            return Action::Continue;
        }

        // Initiate a call to the limitador
        let rl_request = self.create_ratelimit_request(domain, &actions).unwrap(); // TODO(rahulanand16nov): Error Handling
        let rl_req_serialized = Message::write_to_bytes(&rl_request).unwrap(); // TODO(rahulanand16nov): Error Handling

        match self.dispatch_grpc_call(
            upstream_cluster,
            RATELIMIT_SERVICE_NAME,
            RATELIMIT_METHOD_NAME,
            Vec::new(),
            Some(&rl_req_serialized),
            Duration::from_secs(5),
        ) {
            Ok(call_id) => info!("Initiated gRPC call (id# {}) to Limitador", call_id),
            Err(e) => {
                warn!("gRPC call to Limitador failed! {:?}", e);
                request_process_failure(self.config().failure_mode_deny());
            }
        }
        Action::Pause
    }
}

impl Context for Filter {
    fn on_grpc_call_response(&mut self, token_id: u32, status_code: u32, resp_size: usize) {
        info!(
            "received gRPC call response: token: {}, status: {}",
            token_id, status_code
        );

        let res_body_bytes = match self.get_grpc_call_response_body(0, resp_size) {
            Some(bytes) => bytes,
            None => {
                warn!("grpc response body is empty!");
                request_process_failure(self.config().failure_mode_deny());
                return;
            }
        };

        let rl_resp: RateLimitResponse = match Message::parse_from_bytes(&res_body_bytes) {
            Ok(res) => res,
            Err(e) => {
                warn!(
                    "failed to parse grpc response body into RateLimitResponse message: {}",
                    e
                );
                request_process_failure(self.config().failure_mode_deny());
                return;
            }
        };

        match rl_resp.get_overall_code() {
            RateLimitResponse_Code::UNKNOWN => {
                request_process_failure(self.config().failure_mode_deny());
                return;
            }
            RateLimitResponse_Code::OVER_LIMIT => {
                self.send_http_response(429, vec![], Some(b"Too Many Requests\n"));
                return;
            }
            RateLimitResponse_Code::OK => {}
        }
        self.resume_http_request();
    }
}
