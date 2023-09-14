//! This module runs the corresponding function
//! when a `RustRequest` was received from Dart
//! and returns `RustResponse`.

use crate::bridge::api::{RustRequestUnique, RustResponse, RustResponseUnique};
use crate::functions;
use crate::messages;

pub async fn handle_request(request_unique: RustRequestUnique) -> RustResponseUnique {
    // Get the request data.
    let rust_request = request_unique.request;
    let interaction_id = request_unique.id;

    // Run the function that corresponds to the address.
    let rust_resource = rust_request.resource;
    let rust_response = match rust_resource {
        messages::calculate::ID => functions::calculate_final_costs(rust_request).await,
        _ => RustResponse::default(),
    };

    // Return the response.
    RustResponseUnique {
        id: interaction_id,
        response: rust_response,
    }
}
