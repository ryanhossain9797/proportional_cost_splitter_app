use std::sync::Mutex;
use std::sync::OnceLock;

use proportional_cost_splitter_lib::scale_to_total;

#[derive(Clone)]
pub struct CostEntry {
    pub name: String,
    pub initial_cost: f32,
}

pub struct ReadingInputState {
    pub current_entries: Vec<CostEntry>,
}

pub struct FinalCost {
    pub name: String,
    pub final_cost: f32,
}

pub struct CalculatedState {
    pub final_costs: Vec<FinalCost>,
}

pub enum AppState {
    ReadingInputState(ReadingInputState),
    CalculatedState(CalculatedState),
}

impl Default for AppState {
    fn default() -> Self {
        AppState::ReadingInputState(ReadingInputState {
            current_entries: Vec::new(),
        })
    }
}

pub static STATE: OnceLock<Mutex<AppState>> = OnceLock::new();

pub struct AddCostEntryAction {
    pub name: String,
    pub initial_cost: f32,
}

pub struct CalculateAction {
    pub final_total: f32,
}

pub enum AppAction {
    AddCostEntryAction(AddCostEntryAction),
    CalculateAction(CalculateAction),
    ResetAction,
}

pub async fn handle_app_action(action: AppAction) {
    match action {
        AppAction::AddCostEntryAction(add_cost_entry_action) => {
            add_cost_entry_impl(add_cost_entry_action).await
        }
        AppAction::CalculateAction(calculate_action) => {
            calculate_final_costs_impl(calculate_action).await
        }
        AppAction::ResetAction => reset_impl().await,
    };
}

async fn add_cost_entry_impl(action: AddCostEntryAction) {
    let mut state = STATE
        .get_or_init(|| Mutex::new(AppState::default()))
        .lock()
        .unwrap();

    match &*state {
        AppState::ReadingInputState(reading_input) => {
            let new_cost_entry = CostEntry {
                name: action.name,
                initial_cost: action.initial_cost,
            };

            let mut current_entries = reading_input.current_entries.to_vec();
            current_entries.push(new_cost_entry);

            let new_state = AppState::ReadingInputState(ReadingInputState { current_entries });

            *state = new_state;
        }
        _ => {}
    }
}

async fn calculate_final_costs_impl(action: CalculateAction) {
    let mut state = STATE
        .get_or_init(|| Mutex::new(AppState::default()))
        .lock()
        .unwrap();

    match &*state {
        AppState::ReadingInputState(reading_input) => {
            let result = scale_to_total(
                reading_input
                    .current_entries
                    .iter()
                    .map(|entry| (Some(entry.name.clone()), entry.initial_cost as f64))
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

            *state = new_state;
        }
        _ => {}
    }
}

async fn reset_impl() {
    let new_state = AppState::ReadingInputState(ReadingInputState {
        current_entries: Vec::new(),
    });

    let mut state = STATE
        .get_or_init(|| Mutex::new(AppState::default()))
        .lock()
        .unwrap();

    *state = new_state;
}
