use proportional_cost_splitter_lib::*;

pub fn split(initial_costs: Vec<(Option<String>, f64)>, final_cost: f64) -> Vec<(String, f64)> {
    scale_to_total(initial_costs, final_cost)
}


