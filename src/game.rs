use std::error;
use serde::{Serialize, Deserialize};

/// Game result type.
pub type GameResult<T> = std::result::Result<T, Box<dyn error::Error>>;

pub const MULTIPLIER_COST: u64 = 100;
pub const PETTING_MACHINE_COST: u64 = 300;

/// Game.
#[derive(Serialize, Deserialize, Debug)]
#[serde(default = "Default::default")]
pub struct Game {
    /// Cat petted
    pub cat_petted: u64,
    // Cat petted multiplier
    pub multiplier: u64,
    // Cat petter machine
    pub petting_machine: u64,
}

impl Default for Game {
    fn default() -> Self {
        Self {
            cat_petted: 0,
            multiplier: 1,
            petting_machine: 0,
        }
    }
}

impl Game {
    /// Constructs a new instance of [`Game`].
    pub fn new() -> Self {
        Self::default()
    }

    // Pet the cat one time.
    pub fn pet_cat(&mut self) {
        if let Some(res) = self.cat_petted.checked_add(self.multiplier) {
            self.cat_petted = res;
        }
    }
    
    // Pet the cat with the petting machine.
    pub fn pet_cat_with_machine(&mut self) {
        if let Some(res) = self.cat_petted.checked_add(self.petting_machine) {
            self.cat_petted = res;
        }
    }

    // Buy a multiplier that will multiply the number of cat that can be petted one time.
    pub fn buy_multiplier(&mut self) {
        if let Some(res) = self.cat_petted.checked_sub(MULTIPLIER_COST) {
            self.cat_petted = res;
            self.multiplier += 1;
        }
    }

    // Buy a petting machine that will pet a cat automatically.
    pub fn buy_petting_machine(&mut self) {
        if let Some(res) = self.cat_petted.checked_sub(PETTING_MACHINE_COST) {
            self.cat_petted = res;
            self.petting_machine += 1;
        }
    }
}
