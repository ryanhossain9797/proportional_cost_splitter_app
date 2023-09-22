//! This module is only for demonstration purposes.
//! You might want to remove this module in production.

use crate::bridge::api::RustResponse;
use crate::bridge::api::RustSignal;
use crate::bridge::send_rust_signal;
use crate::messages::calculate_action::{CalculateActionDto, CostEntryDto};
use crate::messages::state;
use crate::messages::state::app_state_dto;
use crate::messages::state::{AppStateDto, CalculatedStateDto, FinalCostDto, ReadingInputStateDto};
use app_state::{calculate_final_costs_impl, CostEntry};
use proportional_cost_splitter_lib::scale_to_total;
use prost::Message;

pub async fn calculate_final_costs(message_data: Option<Vec<u8>>) -> RustResponse {
    match message_data {
        Some(message) => {
            // Decode raw bytes into a Rust message object.
            let request_message = CalculateActionDto::decode(&message[..]).unwrap();

            app_state::calculate_final_costs_impl(
                request_message
                    .initial_costs
                    .iter()
                    .map(|dto| CostEntry {
                        name: dto.name.clone(),
                        initial_cost: dto.initial_cost,
                    })
                    .collect::<Vec<_>>(),
                request_message.final_total_cost,
            )
            .await;

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
    crate::print!("reset");
    app_state::reset_impl().await;

    crate::print!("reset complete");

    let empty_response = RustResponse {
        successful: true,
        message: None,
        blob: None,
    };
    empty_response
}
