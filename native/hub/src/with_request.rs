//! This module runs the corresponding function
//! when a `RustRequest` was received from Dart
//! and returns `RustResponse`.

use crate::bridge::api::{
    RustOperation, RustRequestUnique, RustResponse, RustResponseUnique, RustSignal,
};
use crate::bridge::send_rust_signal;
use crate::functions;
use crate::messages::state::app_state_dto::State;
use crate::messages::state::{
    app_state_dto, AppStateDto, CalculatedStateDto, FinalCostDto, ReadingInputStateDto,
};
use crate::messages::*;
use crate::messages::{self, state};
use app_state::AppState;
use prost::Message;
use std::sync::Mutex;

pub async fn handle_request(request_unique: RustRequestUnique) -> RustResponseUnique {
    let rust_request = request_unique.request;
    let interaction_id = request_unique.id;

    // Run the function that corresponds to the address.
    let rust_resource = rust_request.resource;
    let rust_response = match rust_request.operation {
        RustOperation::Update => {
            let rust_response = match rust_resource {
                calculate_action::ID => {
                    functions::calculate_final_costs(rust_request.message).await
                }
                reset_action::ID => functions::reset().await,
                _ => RustResponse::default(),
            };

            let new_state = app_state::STATE
                .get_or_init(|| Mutex::new(AppState::default()))
                .lock()
                .unwrap();

            match &*new_state {
                AppState::ReadingInputState => {
                    crate::print!("ReadingInputState");
                    let signal_message = AppStateDto {
                        state: Some(app_state_dto::State::ReadingInput(ReadingInputStateDto {})),
                    };
                    let rust_signal = RustSignal {
                        resource: state::ID,
                        message: Some(signal_message.encode_to_vec()),
                        blob: None,
                    };

                    send_rust_signal(rust_signal);
                }

                AppState::CalculatedState(state) => {
                    crate::print!("CalculatedState");
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
        RustOperation::Create => RustResponse::default(),
        RustOperation::Read => RustResponse::default(),
        RustOperation::Delete => RustResponse::default(),
    };

    // Return the response.
    RustResponseUnique {
        id: interaction_id,
        response: rust_response,
    }
}
