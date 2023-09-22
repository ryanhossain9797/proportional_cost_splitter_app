//! This module is only for demonstration purposes.
//! You might want to remove this module in production.

use crate::bridge::api::RustOperation;
use crate::bridge::api::RustRequest;
use crate::bridge::api::RustResponse;
use crate::bridge::api::RustSignal;
use crate::bridge::send_rust_signal;
use crate::messages::state;
use crate::messages::state::app_state;
use crate::messages::state::{AppState, CalculatedState, FinalCost, ReadingInputState};
use proportional_cost_splitter_lib::scale_to_total;
use prost::Message;

pub async fn calculate_final_costs(message_data: Option<Vec<u8>>) -> RustResponse {
    // We import message structs in this match condition
    // because schema will differ by the operation type.
    use crate::messages::calculate_action::CalculateAction;

    match message_data {
        Some(message) => {
            // Decode raw bytes into a Rust message object.
            let request_message = CalculateAction::decode(&message[..]).unwrap();

            let result = scale_to_total(
                request_message
                    .initial_costs
                    .into_iter()
                    .map(|entry| (entry.name, entry.initial_cost as f64))
                    .collect(),
                request_message.final_total_cost as f64,
            )
            .into_iter()
            .map(|(name, final_cost)| FinalCost {
                name,
                final_cost: final_cost as f32,
            })
            .collect::<Vec<_>>();

            let state = AppState {
                state: Some(app_state::State::Calculated(CalculatedState {
                    final_costs: result,
                })),
            };

            let rust_signal = RustSignal {
                resource: crate::messages::state::ID,
                message: Some(state.encode_to_vec()),
                blob: None,
            };

            send_rust_signal(rust_signal);

            RustResponse {
                successful: true,
                message: None,
                blob: None,
            }
        }
        None => RustResponse::default(),
    }
}
pub async fn reset() -> RustResponse {
    let signal_message = AppState {
        state: Some(app_state::State::ReadingInput(ReadingInputState {})),
    };
    let rust_signal = RustSignal {
        resource: state::ID,
        message: Some(signal_message.encode_to_vec()),
        blob: None,
    };

    send_rust_signal(rust_signal);

    let empty_response = RustResponse {
        successful: true,
        message: None,
        blob: None,
    };
    empty_response
}
