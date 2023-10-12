//! This module runs the corresponding function
//! when a `RustRequest` was received from Dart
//! and returns `RustResponse`.

use crate::bridge::api::{
    RustOperation, RustRequestUnique, RustResponse, RustResponseUnique, RustSignal,
};
use crate::bridge::send_rust_signal;
use crate::functions::successful_empty_rust_response;
use crate::messages::add_cost_entry_action::AddCostEntryActionDto;
use crate::messages::calculate_action::CalculateActionDto;
use crate::messages::state::app_state_dto::State;
use crate::messages::state::{
    app_state_dto, AppStateDto, CalculatedStateDto, CurrentCostEntryDto, FinalCostDto,
    ReadingInputStateDto,
};
use crate::messages::*;
use crate::messages::{self, state};
use app_state::{handle_app_action, AppAction, AppState};
use prost::Message;
use std::sync::Mutex;

pub async fn handle_request(request_unique: RustRequestUnique) -> RustResponseUnique {
    let rust_request = request_unique.request;
    let interaction_id = request_unique.id;

    // Run the function that corresponds to the address.
    let rust_resource = rust_request.resource;
    let rust_response = match (rust_request.operation, rust_request.message) {
        (RustOperation::Update, Some(message)) => {
            let rust_response = match rust_resource {
                add_cost_entry_action::ID => {
                    crate::debug_print!("AddCostEntryAction");
                    let dto = AddCostEntryActionDto::decode(&message[..]).unwrap();
                    handle_app_action(dto.into()).await;
                    successful_empty_rust_response()
                }
                calculate_action::ID => {
                    crate::debug_print!("CalculateAction");
                    let dto = CalculateActionDto::decode(&message[..]).unwrap();
                    handle_app_action(dto.into()).await;
                    successful_empty_rust_response()
                }
                reset_action::ID => {
                    crate::debug_print!("ResetAction");
                    handle_app_action(AppAction::ResetAction).await;
                    successful_empty_rust_response()
                }
                _ => RustResponse::default(),
            };

            let new_state = app_state::STATE
                .get_or_init(|| Mutex::new(AppState::default()))
                .lock()
                .unwrap();

            match &*new_state {
                AppState::ReadingInputState(state) => {
                    crate::debug_print!("ReadingInputState");
                    let signal_message = AppStateDto {
                        state: Some(app_state_dto::State::ReadingInput(ReadingInputStateDto {
                            padding: 1.,
                            current_cost_entries: state
                                .current_entries
                                .iter()
                                .map(|(_, current_cost_entry)| CurrentCostEntryDto {
                                    name: current_cost_entry.name.clone(),
                                    cost: current_cost_entry.initial_cost,
                                })
                                .collect::<Vec<_>>(),
                        })),
                    };
                    let rust_signal = RustSignal {
                        resource: state::ID,
                        message: Some(signal_message.encode_to_vec()),
                        blob: None,
                    };

                    send_rust_signal(rust_signal);
                }

                AppState::CalculatedState(state) => {
                    crate::debug_print!("CalculatedState");
                    let state = AppStateDto {
                        state: Some(app_state_dto::State::Calculated(CalculatedStateDto {
                            final_costs: state
                                .final_costs
                                .iter()
                                .map(|data| FinalCostDto {
                                    name: data.name.clone(),
                                    final_cost: data.final_cost,
                                })
                                .collect::<Vec<_>>(),
                        })),
                    };

                    let rust_signal = RustSignal {
                        resource: crate::messages::state::ID,
                        message: Some(state.encode_to_vec()),
                        blob: None,
                    };

                    send_rust_signal(rust_signal);
                }
            }

            rust_response
        }
        (RustOperation::Update, None) => RustResponse::default(),
        (RustOperation::Create, _) => RustResponse::default(),
        (RustOperation::Read, _) => RustResponse::default(),
        (RustOperation::Delete, _) => RustResponse::default(),
    };

    // Return the response.
    RustResponseUnique {
        id: interaction_id,
        response: rust_response,
    }
}
