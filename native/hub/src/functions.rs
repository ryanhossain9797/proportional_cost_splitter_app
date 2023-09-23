//! This module is only for demonstration purposes.
//! You might want to remove this module in production.

use crate::bridge::api::RustResponse;
use crate::messages::calculate_action::CalculateActionDto;
use crate::messages::calculate_action::CostEntryDto;
use app_state::AppAction;
use app_state::CalculateAction;
use app_state::CostEntry;

impl Into<CostEntry> for CostEntryDto {
    fn into(self) -> CostEntry {
        CostEntry {
            name: self.name,
            initial_cost: self.initial_cost,
        }
    }
}

impl Into<AppAction> for CalculateActionDto {
    fn into(self) -> AppAction {
        AppAction::CalculateAction(CalculateAction {
            initial_costs: self
                .initial_costs
                .into_iter()
                .map(|cost_entry_dto| cost_entry_dto.into())
                .collect::<Vec<_>>(),
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
