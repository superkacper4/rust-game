use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Player {
    pub cash: f64,
    pub name: String,
}

impl Player {
    pub fn get_cash(&self) -> f64 {
        self.cash
    }

    pub fn subtract_cash(&mut self, amount: f64) {
        self.cash -= amount;
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn init() -> Player {
        return Player {
            cash: 10000000.0,
            name: "Kacper".to_owned(),
        };
    }
}
