//! This module runs the corresponding function
//! when a `RustRequest` was received from Dart
//! and returns `RustResponse`.

use prost::Message;

use crate::bridge::api::{RustRequestUnique, RustResponse, RustResponseUnique, RustSignal};
use crate::bridge::send_rust_signal;
use crate::functions;
use crate::messages::state::app_state::State;
use crate::messages::state::{AppState, ReadingInputState, ID};
use crate::messages::*;
use crate::messages::{self, state};

pub async fn handle_request(request_unique: RustRequestUnique) -> RustResponseUnique {
    // Get the request data.
    let rust_request = request_unique.request;
    let interaction_id = request_unique.id;

    // Run the function that corresponds to the address.
    let rust_resource = rust_request.resource;
    let rust_response = match rust_resource {
        calculate_action::ID => functions::calculate_final_costs(rust_request).await,
        reset_action::ID => {
            let signal_message = AppState {
                state: Some(State::ReadingInput(ReadingInputState {})),
            };
            let rust_signal = RustSignal {
                resource: state::ID,
                bytes: signal_message.encode_to_vec(),
            };

            send_rust_signal(rust_signal);
            let response_message = messages::reset_action::ResetActionResult {};
            let empty_response = RustResponse {
                successful: true,
                bytes: response_message.encode_to_vec(),
            };
            empty_response
        }
        _ => RustResponse::default(),
    };

    // Return the response.
    RustResponseUnique {
        id: interaction_id,
        response: rust_response,
    }
}
