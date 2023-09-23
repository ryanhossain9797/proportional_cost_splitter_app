//! This module is only for demonstration purposes.
//! You might want to remove this module in production.

use crate::bridge::api::RustResponse;
use crate::messages::calculate_action::CalculateActionDto;
use app_state::handle_app_action;
use app_state::AppAction;
use app_state::CalculateAction;
use app_state::CostEntry;
use prost::Message;

pub async fn calculate_final_costs(message_data: Option<Vec<u8>>) -> RustResponse {
    match message_data {
        Some(message) => {
            // Decode raw bytes into a Rust message object.
            let request_message = CalculateActionDto::decode(&message[..]).unwrap();

            handle_app_action(AppAction::CalculateAction(CalculateAction {
                initial_costs: request_message
                    .initial_costs
                    .iter()
                    .map(|dto| CostEntry {
                        name: dto.name.clone(),
                        initial_cost: dto.initial_cost,
                    })
                    .collect::<Vec<_>>(),
                final_total: request_message.final_total_cost,
            }))
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
    handle_app_action(AppAction::ResetAction).await;

    let empty_response = RustResponse {
        successful: true,
        message: None,
        blob: None,
    };
    empty_response
}
