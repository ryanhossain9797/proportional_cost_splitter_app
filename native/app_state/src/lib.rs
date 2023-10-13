use std::collections::HashMap;
use std::sync::Mutex;
use std::sync::OnceLock;

use proportional_cost_splitter_lib::scale_to_total;

#[derive(Clone)]
pub struct CostEntry {
    pub name: String,
    pub initial_cost: f32,
}

pub struct ReadingInputState {
    pub current_entries: HashMap<String, CostEntry>,
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
            current_entries: HashMap::new(),
        })
    }
}

pub static STATE: OnceLock<Mutex<AppState>> = OnceLock::new();

pub struct AddCostEntryAction {
    pub name: String,
    pub initial_cost: f32,
}

pub struct RemoveCostEntryAction {
    pub name: String,
}

pub struct CalculateAction {
    pub final_total: f32,
}

pub enum AppAction {
    AddCostEntryAction(AddCostEntryAction),
    RemoveCostEntryAction(RemoveCostEntryAction),
    CalculateAction(CalculateAction),
    ResetAction,
}

pub async fn handle_app_action(action: AppAction) {
    match action {
        AppAction::AddCostEntryAction(add_cost_entry_action) => {
            add_cost_entry_impl(add_cost_entry_action).await
        }
        AppAction::RemoveCostEntryAction(remove_cost_entry_action) => {
            remove_cost_entry_impl(remove_cost_entry_action).await
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
            let cost_entry = match reading_input.current_entries.get(action.name.as_str()) {
                Some(entry) => CostEntry {
                    name: action.name,
                    initial_cost: action.initial_cost + entry.initial_cost,
                },
                None => CostEntry {
                    name: action.name,
                    initial_cost: action.initial_cost,
                },
            };

            let mut entries = reading_input.current_entries.clone();
            entries.insert(cost_entry.name.clone(), cost_entry);

            *state = AppState::ReadingInputState(ReadingInputState {
                current_entries: entries,
            })
        }
        _ => {}
    }
}

async fn remove_cost_entry_impl(action: RemoveCostEntryAction) {
    let mut state = STATE
        .get_or_init(|| Mutex::new(AppState::default()))
        .lock()
        .unwrap();

    match &*state {
        AppState::ReadingInputState(reading_input) => {
            let mut entries = reading_input.current_entries.clone();
            entries.remove(&action.name);

            *state = AppState::ReadingInputState(ReadingInputState {
                current_entries: entries,
            })
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
                    .map(|(_, entry)| (Some(entry.name.clone()), entry.initial_cost as f64))
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
        current_entries: HashMap::new(),
    });

    let mut state = STATE
        .get_or_init(|| Mutex::new(AppState::default()))
        .lock()
        .unwrap();

    *state = new_state;
}
