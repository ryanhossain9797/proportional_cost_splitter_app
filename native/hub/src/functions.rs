//! This module is only for demonstration purposes.
//! You might want to remove this module in production.

use crate::bridge::api::RustResponse;
use crate::messages::add_cost_entry_action::AddCostEntryActionDto;
use crate::messages::calculate_action::CalculateActionDto;
use app_state::AddCostEntryAction;
use app_state::AppAction;
use app_state::CalculateAction;
use app_state::CostEntry;

impl Into<AppAction> for AddCostEntryActionDto {
    fn into(self) -> AppAction {
        AppAction::AddCostEntryAction(AddCostEntryAction {
            name: self.name,
            initial_cost: self.initial_cost,
        })
    }
}

impl Into<AppAction> for CalculateActionDto {
    fn into(self) -> AppAction {
        AppAction::CalculateAction(CalculateAction {
            final_total: self.final_total_cost,
        })
    }
}

pub fn successful_empty_rust_response() -> RustResponse {
    RustResponse {
        successful: true,
        message: None,
        blob: None,
    }
}
