use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ClubModel {
    pub name: String,
}
