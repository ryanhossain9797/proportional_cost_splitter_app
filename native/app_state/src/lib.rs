use std::sync::Mutex;
use std::sync::OnceLock;

use proportional_cost_splitter_lib::scale_to_total;

pub struct CostEntry {
    pub name: Option<String>,
    pub initial_cost: f32,
}

pub struct FinalCost {
    pub name: String,
    pub final_cost: f32,
}

pub struct CalculatedState {
    pub final_costs: Vec<FinalCost>,
}

pub enum AppState {
    ReadingInputState,
    CalculatedState(CalculatedState),
}

impl Default for AppState {
    fn default() -> Self {
        AppState::ReadingInputState
    }
}

pub static STATE: OnceLock<Mutex<AppState>> = OnceLock::new();

pub struct CalculateAction {
    pub initial_costs: Vec<CostEntry>,
    pub final_total: f32,
}

pub enum AppAction {
    CalculateAction(CalculateAction),
    ResetAction,
}

pub async fn handle_app_action(action: AppAction) {
    match action {
        AppAction::CalculateAction(calculate_action) => {
            calculate_final_costs_impl(calculate_action).await
        }
        AppAction::ResetAction => reset_impl().await,
    };
}

async fn calculate_final_costs_impl(action: CalculateAction) {
    // We import message structs in this match condition
    // because schema will differ by the operation type.

    let result = scale_to_total(
        action
            .initial_costs
            .into_iter()
            .map(|entry| (entry.name, entry.initial_cost as f64))
            .collect(),
        action.final_total as f64,
    )
    .into_iter()
    .map(|(name, final_cost)| FinalCost {
        name,
        final_cost: final_cost as f32,
    })
    .collect::<Vec<_>>();

    let new_state = AppState::CalculatedState(CalculatedState {
        final_costs: result,
    });

    let mut state = STATE
        .get_or_init(|| Mutex::new(AppState::default()))
        .lock()
        .unwrap();

    *state = new_state;
}

async fn reset_impl() {
    let new_state = AppState::ReadingInputState;

    let mut state = STATE
        .get_or_init(|| Mutex::new(AppState::default()))
        .lock()
        .unwrap();

    *state = new_state;
}
