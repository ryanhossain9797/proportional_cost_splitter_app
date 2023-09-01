//! This module is only for demonstration purposes.
//! You might want to remove this module in production.

use crate::bridge::api::RustOperation;
use crate::bridge::api::RustRequest;
use crate::bridge::api::RustResponse;
use crate::bridge::api::RustSignal;
use crate::bridge::send_rust_signal;
use proportional_cost_splitter_lib::scale_to_total;
use prost::Message;

pub async fn calculate_something(rust_request: RustRequest) -> RustResponse {
    match rust_request.operation {
        RustOperation::Create => RustResponse::default(),
        RustOperation::Read => {
            // We import message structs in this match condition
            // because schema will differ by the operation type.
            use crate::messages::interaction::{CounterReadRequest, CounterReadResponse};

            // Decode raw bytes into a Rust message object.
            let request_message = CounterReadRequest::decode(&rust_request.bytes[..]).unwrap();

            // Perform a simple calculation.
            let after_value: i32 = request_message.before_number + 7;

            // Return the response that will be sent to Dart.
            let response_message = CounterReadResponse {
                after_number: after_value,
                dummy_one: request_message.dummy_one,
                dummy_two: request_message.dummy_two,
                dummy_three: request_message.dummy_three,
            };
            RustResponse {
                successful: true,
                bytes: response_message.encode_to_vec(),
            }
        }
        RustOperation::Update => RustResponse::default(),
        RustOperation::Delete => RustResponse::default(),
    }
}
