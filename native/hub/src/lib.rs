use crate::bridge::api::RustSignal;
use crate::bridge::send_rust_signal;
use crate::messages::state::{AppState, ReadingInputState, ID};
use bridge::respond_to_dart;
use prost::Message;
use web_alias::*;
use with_request::handle_request;

mod bridge;
mod functions;
mod messages;
mod web_alias;
mod with_request;

/// This `hub` crate is the entry point for the Rust logic.
/// Always use non-blocking async functions such as `tokio::fs::File::open`.
async fn main() {
    // This is `tokio::sync::mpsc::Reciver` that receives the requests from Dart.
    let mut request_receiver = bridge::get_request_receiver();

    while let Some(request_unique) = request_receiver.recv().await {
        crate::print!("REQUEST RECEIVED: {}", request_unique.id);
        crate::spawn(async {
            let response_unique = handle_request(request_unique).await;
            respond_to_dart(response_unique);
        });
    }
}
