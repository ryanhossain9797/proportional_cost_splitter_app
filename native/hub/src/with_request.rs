//! This module runs the corresponding function
//! when a `RustRequest` was received from Dart
//! and returns `RustResponse`.

use crate::bridge::api::{
    RustOperation, RustRequestUnique, RustResponse, RustResponseUnique, RustSignal,
};
use crate::bridge::send_rust_signal;
use crate::functions;
use crate::messages::state::app_state_dto::State;
use crate::messages::state::{AppStateDto, ReadingInputStateDto};
use crate::messages::*;
use crate::messages::{self, state};
use prost::Message;

pub async fn handle_request(request_unique: RustRequestUnique) -> RustResponseUnique {
    // Get the request data.
    let rust_request = request_unique.request;
    let interaction_id = request_unique.id;

    // Run the function that corresponds to the address.
    let rust_resource = rust_request.resource;
    let rust_response = match rust_request.operation {
        RustOperation::Update => match rust_resource {
            calculate_action::ID => functions::calculate_final_costs(rust_request.message).await,
            reset_action::ID => functions::reset().await,
            _ => RustResponse::default(),
        },
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
