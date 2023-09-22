//! This module is only for demonstration purposes.
//! You might want to remove this module in production.

use crate::bridge::api::RustResponse;
use crate::bridge::api::RustSignal;
use crate::bridge::send_rust_signal;
use crate::messages::calculate_action::CalculateActionDto;
use crate::messages::state;
use crate::messages::state::app_state_dto;
use crate::messages::state::{AppStateDto, CalculatedStateDto, FinalCostDto, ReadingInputStateDto};
use proportional_cost_splitter_lib::scale_to_total;
use prost::Message;

pub async fn calculate_final_costs(message_data: Option<Vec<u8>>) -> RustResponse {
    // We import message structs in this match condition
    // because schema will differ by the operation type.

    match message_data {
        Some(message) => {
            // Decode raw bytes into a Rust message object.
            let request_message = CalculateActionDto::decode(&message[..]).unwrap();

            let result = scale_to_total(
                request_message
                    .initial_costs
                    .into_iter()
                    .map(|entry| (entry.name, entry.initial_cost as f64))
                    .collect(),
                request_message.final_total_cost as f64,
            )
            .into_iter()
            .map(|(name, final_cost)| FinalCostDto {
                name,
                final_cost: final_cost as f32,
            })
            .collect::<Vec<_>>();

            let state = AppStateDto {
                state: Some(app_state_dto::State::Calculated(CalculatedStateDto {
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
    let signal_message = AppStateDto {
        state: Some(app_state_dto::State::ReadingInput(ReadingInputStateDto {})),
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
