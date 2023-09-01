//! This module is only for demonstration purposes.
//! You might want to remove this module in production.

use crate::bridge::api::RustOperation;
use crate::bridge::api::RustRequest;
use crate::bridge::api::RustResponse;
use crate::bridge::api::RustSignal;
use crate::bridge::send_rust_signal;
use proportional_cost_splitter_lib::scale_to_total;
use prost::Message;

pub async fn calculate_final_costs(rust_request: RustRequest) -> RustResponse {
    match rust_request.operation {
        RustOperation::Create => RustResponse::default(),
        RustOperation::Read => {
            // We import message structs in this match condition
            // because schema will differ by the operation type.
            use crate::messages::interaction::{RustCalculateRequest, RustCalculateResponse};
            use crate::messages::schemas::{RustCostEntry, RustFinalCost};

            // Decode raw bytes into a Rust message object.
            let request_message = RustCalculateRequest::decode(&rust_request.bytes[..]).unwrap();

            let result = scale_to_total(
                request_message
                    .initial_costs
                    .into_iter()
                    .map(|entry| (entry.name, entry.initial_cost as f64))
                    .collect(),
                request_message.final_total_cost as f64,
            );

            let final_costs = result
                .into_iter()
                .map(|(name, final_cost)| RustFinalCost {
                    name,
                    final_cost: final_cost as f32,
                })
                .collect::<Vec<_>>();

            // Return the response that will be sent to Dart.
            let response_message = RustCalculateResponse { final_costs };
            RustResponse {
                successful: true,
                bytes: response_message.encode_to_vec(),
            }
        }
        RustOperation::Update => RustResponse::default(),
        RustOperation::Delete => RustResponse::default(),
    }
}
