use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Materials {
    pub wood: i64,
}

impl Materials {
    pub fn init() -> Materials {
        return Materials { wood: 100 };
    }

    pub fn get_all_materials(&self) -> &Materials {
        return self;
    }

    pub fn add_per_tick(&mut self) -> () {
        self.wood += 5;
    }
}
